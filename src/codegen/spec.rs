//! Permissive views of the official FHIR definition JSON.
//!
//! The definition bundles mix `StructureDefinition` with `OperationDefinition`,
//! `SearchParameter`, `CompartmentDefinition` and more, and each release adds
//! fields the previous one did not have. These structs therefore deserialize
//! only what the generator uses and ignore everything else — no
//! `deny_unknown_fields` — which is why one set of types reads both the R4 and
//! R5 bundles unchanged.

use std::collections::BTreeMap;
use std::path::Path;

use ::serde::Deserialize;

/// A definition bundle file: a FHIR `Bundle` of definition resources.
#[derive(Debug, Deserialize)]
pub struct Bundle {
    /// The bundle's entries; each wraps one definition resource.
    #[serde(default)]
    pub entry: Vec<Entry>,
}

/// One `Bundle.entry`.
#[derive(Debug, Deserialize)]
pub struct Entry {
    /// The contained definition resource, left as raw JSON so that entries of
    /// an unwanted `resourceType` cost nothing to skip.
    pub resource: ::serde_json::Value,
}

/// A FHIR `StructureDefinition`: one datatype or resource.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinition {
    /// The definition's name, e.g. `"Patient"`.
    pub name: String,
    /// The FHIR type this defines, e.g. `"Patient"`. Equals the root element path.
    #[serde(rename = "type")]
    pub type_name: String,
    /// `primitive-type`, `complex-type`, `resource`, or `logical`.
    pub kind: String,
    /// Whether this is an abstract base (`Resource`, `DomainResource`, …).
    #[serde(default, rename = "abstract")]
    pub is_abstract: bool,
    /// The canonical URL, e.g. `http://hl7.org/fhir/StructureDefinition/Patient`.
    pub url: String,
    /// The release version this definition was published in, e.g. `"4.0.1"`.
    pub version: Option<String>,
    /// The specification's prose description of the type.
    pub description: Option<String>,
    /// The fully resolved element list. Definitions without one are skipped.
    pub snapshot: Option<Snapshot>,
}

/// A `StructureDefinition.snapshot`: the fully resolved element list.
#[derive(Debug, Clone, Deserialize)]
pub struct Snapshot {
    /// Every element, in specification order, starting with the root.
    #[serde(default)]
    pub element: Vec<ElementDefinition>,
}

/// One `ElementDefinition` — a single element of a datatype or resource.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition {
    /// Dotted FHIR path, e.g. `"Patient.contact.name"`. Choice elements end
    /// in `[x]`.
    pub path: String,
    /// Minimum cardinality.
    #[serde(default)]
    pub min: u32,
    /// Maximum cardinality: a number, or `"*"` for unbounded.
    pub max: Option<String>,
    /// The one-line summary shown in the specification's tables.
    pub short: Option<String>,
    /// The full prose definition.
    pub definition: Option<String>,
    /// A pointer to another element whose children this one reuses, e.g.
    /// `"#Observation.referenceRange"` (R4) or a full URL with the same
    /// fragment (R5).
    pub content_reference: Option<String>,
    /// The allowed types. A choice element has more than one; a backbone
    /// element has `BackboneElement` or `Element`.
    #[serde(default, rename = "type")]
    pub types: Vec<ElementType>,
    /// The value-set binding, when the element is coded.
    pub binding: Option<Binding>,
    /// Whether the element is part of the `_summary=true` view.
    pub is_summary: Option<bool>,
}

/// One allowed type of an element (`ElementDefinition.type`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementType {
    /// The FHIR type code, e.g. `"Quantity"`, `"string"`, or a FHIRPath system
    /// type URL such as `http://hl7.org/fhirpath/System.String`.
    pub code: String,
    /// For `Reference`/`canonical`, the resource profiles that may be targeted.
    #[serde(default)]
    pub target_profile: Vec<String>,
}

/// An element's value-set binding (`ElementDefinition.binding`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// `required`, `extensible`, `preferred`, or `example`.
    pub strength: String,
    /// The canonical `ValueSet` URL, possibly with a `|version` suffix.
    pub value_set: Option<String>,
}

/// A FHIR `CodeSystem`, the source of the generated `codes` enums.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem {
    /// The code system's name, e.g. `"AdministrativeGender"`.
    pub name: Option<String>,
    /// The canonical URL that value sets reference.
    pub url: Option<String>,
    /// The specification's prose description.
    pub description: Option<String>,
    /// `complete`, `example`, `fragment`, or `not-present`. Only `complete`
    /// systems can become an exhaustive Rust enum.
    pub content: Option<String>,
    /// The concept tree; nested concepts are flattened by [`CodeSystem::codes`].
    #[serde(default)]
    pub concept: Vec<Concept>,
}

/// One `CodeSystem.concept`.
#[derive(Debug, Clone, Deserialize)]
pub struct Concept {
    /// The code as it appears on the wire.
    pub code: String,
    /// A short human-readable label.
    pub display: Option<String>,
    /// The concept's prose definition.
    pub definition: Option<String>,
    /// Child concepts, which FHIR nests but Rust enums flatten.
    #[serde(default)]
    pub concept: Vec<Concept>,
}

impl CodeSystem {
    /// Every concept in the system, flattened depth-first and de-duplicated by
    /// code (FHIR permits a code to appear once, but hierarchies are nested).
    #[must_use]
    pub fn codes(&self) -> Vec<&Concept> {
        fn walk<'a>(concepts: &'a [Concept], out: &mut Vec<&'a Concept>) {
            for concept in concepts {
                out.push(concept);
                walk(&concept.concept, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.concept, &mut out);
        let mut seen = std::collections::HashSet::new();
        out.retain(|c| seen.insert(c.code.clone()));
        out
    }
}

impl ElementDefinition {
    /// Whether the element repeats (`max` is `*` or greater than one).
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        match self.max.as_deref() {
            Some("*") => true,
            Some(other) => other.parse::<u32>().is_ok_and(|n| n > 1),
            None => false,
        }
    }

    /// Whether this is a `value[x]`-style choice element.
    #[must_use]
    pub fn is_choice(&self) -> bool {
        self.path.ends_with("[x]")
    }

    /// The element path with any `[x]` suffix removed.
    #[must_use]
    pub fn base_path(&self) -> &str {
        self.path.strip_suffix("[x]").unwrap_or(&self.path)
    }

    /// The last path segment, without any `[x]` suffix.
    #[must_use]
    pub fn leaf(&self) -> &str {
        self.base_path().rsplit('.').next().unwrap_or_default()
    }

    /// The path of the element that owns this one, e.g. `"Patient.contact"`
    /// for `"Patient.contact.name"`. `None` for a root element.
    #[must_use]
    pub fn owner_path(&self) -> Option<&str> {
        self.base_path().rsplit_once('.').map(|(owner, _)| owner)
    }

    /// The element path a `contentReference` points at, e.g.
    /// `"Observation.referenceRange"`.
    ///
    /// R4 writes a bare fragment (`#Observation.referenceRange`) and R5 a full
    /// canonical URL with the same fragment, so both reduce to the text after
    /// the `#`.
    #[must_use]
    pub fn content_reference_path(&self) -> Option<&str> {
        self.content_reference.as_deref()?.rsplit('#').next()
    }
}

/// Read a definition bundle and return every `StructureDefinition` in it.
///
/// Entries of any other `resourceType`, and definitions without a snapshot, are
/// skipped: the generator can only work from fully resolved element lists.
pub fn read_structure_definitions(path: &Path) -> std::io::Result<Vec<StructureDefinition>> {
    Ok(read_resources::<StructureDefinition>(path, "StructureDefinition")?
        .into_iter()
        .filter(|sd| sd.snapshot.is_some())
        .collect())
}

/// Read a definition bundle and return every `CodeSystem` in it.
pub fn read_code_systems(path: &Path) -> std::io::Result<Vec<CodeSystem>> {
    read_resources::<CodeSystem>(path, "CodeSystem")
}

/// Read a bundle and deserialize every entry whose `resourceType` matches.
fn read_resources<T: for<'de> Deserialize<'de>>(
    path: &Path,
    resource_type: &str,
) -> std::io::Result<Vec<T>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let bundle: Bundle = ::serde_json::from_reader(reader)
        .map_err(|e| std::io::Error::other(format!("{}: {e}", path.display())))?;
    Ok(bundle
        .entry
        .into_iter()
        .filter(|entry| entry.resource.get("resourceType").and_then(|v| v.as_str()) == Some(resource_type))
        .filter_map(|entry| ::serde_json::from_value::<T>(entry.resource).ok())
        .collect())
}

/// Index the given definitions by FHIR type name, e.g. `"Patient"`.
#[must_use]
pub fn by_type_name(definitions: &[StructureDefinition]) -> BTreeMap<String, StructureDefinition> {
    definitions
        .iter()
        .map(|sd| (sd.type_name.clone(), sd.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn element(path: &str, max: &str) -> ElementDefinition {
        ElementDefinition {
            path: path.to_string(),
            min: 0,
            max: Some(max.to_string()),
            short: None,
            definition: None,
            content_reference: None,
            types: Vec::new(),
            binding: None,
            is_summary: None,
        }
    }

    #[test]
    fn path_parts() {
        let el = element("Observation.component.value[x]", "1");
        assert!(el.is_choice());
        assert_eq!(el.base_path(), "Observation.component.value");
        assert_eq!(el.leaf(), "value");
        assert_eq!(el.owner_path(), Some("Observation.component"));
        assert!(!el.is_multiple());
    }

    #[test]
    fn root_has_no_owner() {
        assert_eq!(element("Patient", "1").owner_path(), None);
    }

    #[test]
    fn multiplicity() {
        assert!(element("Patient.name", "*").is_multiple());
        assert!(!element("Patient.gender", "1").is_multiple());
        assert!(element("X.y", "5").is_multiple());
    }

    #[test]
    fn content_reference_forms_agree() {
        let mut el = element("Observation.component.referenceRange", "*");
        el.content_reference = Some("#Observation.referenceRange".to_string());
        assert_eq!(el.content_reference_path(), Some("Observation.referenceRange"));
        el.content_reference = Some(
            "http://hl7.org/fhir/StructureDefinition/Observation#Observation.referenceRange"
                .to_string(),
        );
        assert_eq!(el.content_reference_path(), Some("Observation.referenceRange"));
    }

    #[test]
    fn code_system_flattens_nested_concepts() {
        let system: CodeSystem = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "CodeSystem",
            "name": "Example",
            "content": "complete",
            "concept": [
                { "code": "a", "concept": [ { "code": "b" } ] },
                { "code": "c" }
            ]
        }))
        .unwrap();
        let codes: Vec<&str> = system.codes().iter().map(|c| c.code.as_str()).collect();
        assert_eq!(codes, ["a", "b", "c"]);
    }
}
