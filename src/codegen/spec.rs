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
    ///
    /// Empty when the definition states no code at all. R3 does this on a
    /// primitive's own `value` element, where the type is carried only by a
    /// `structuredefinition-json-type` extension — those elements are not
    /// modelled, because a primitive's Rust representation comes from
    /// [`super::primitives`] rather than from the snapshot.
    #[serde(default)]
    pub code: String,
    /// For `Reference`/`canonical`, the resource profiles that may be targeted.
    ///
    /// R3 writes a single string here and repeats the whole type entry once per
    /// target; R4 and R5 write a list. Both are read into a list.
    #[serde(default, deserialize_with = "string_or_seq")]
    pub target_profile: Vec<String>,
}

/// An element's value-set binding (`ElementDefinition.binding`).
///
/// The releases spell the bound value set three different ways, so read
/// [`Binding::value_set`] rather than any one field.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// `required`, `extensible`, `preferred`, or `example`.
    pub strength: String,
    /// R4/R5: the canonical `ValueSet` URL, possibly with a `|version` suffix.
    #[serde(rename = "valueSet")]
    value_set_canonical: Option<String>,
    /// R3: the value set as a `Reference`.
    value_set_reference: Option<BindingReference>,
    /// R3: the value set as a bare URI.
    value_set_uri: Option<String>,
}

/// The `Reference` form of an R3 binding's value set.
#[derive(Debug, Clone, Deserialize)]
pub struct BindingReference {
    /// The referenced `ValueSet` URL.
    pub reference: Option<String>,
}

impl Binding {
    /// The bound `ValueSet` URL, whichever way this release spells it.
    #[must_use]
    pub fn value_set(&self) -> Option<&str> {
        self.value_set_canonical
            .as_deref()
            .or_else(|| self.value_set_reference.as_ref()?.reference.as_deref())
            .or(self.value_set_uri.as_deref())
    }
}

/// Deserialize a field that may be either a single string or a list of them.
fn string_or_seq<'de, D: ::serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    Ok(match OneOrMany::deserialize(deserializer)? {
        OneOrMany::One(s) => vec![s],
        OneOrMany::Many(v) => v,
    })
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

    /// Whether this element is FHIR *infrastructure* rather than a primitive
    /// element that can carry its own extensions.
    ///
    /// `Element.id`, every `<Type>.id`, and `Extension.url` are serialized as
    /// bare JSON attributes with no `_field` sibling. R4 and R5 say so by
    /// giving them a FHIRPath system type (`http://hl7.org/fhirpath/System.*`);
    /// R3 predates that convention and types them as ordinary `string`, `id` or
    /// `uri`, so the rule is expressed structurally and holds for all three.
    #[must_use]
    pub fn is_system_element(&self) -> bool {
        if self.types.iter().any(|t| t.code.starts_with("http://hl7.org/fhirpath/System.")) {
            return true;
        }
        self.leaf() == "id" || self.path == "Extension.url"
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
    let mut out = Vec::new();
    for entry in bundle.entry {
        if entry.resource.get("resourceType").and_then(|v| v.as_str()) != Some(resource_type) {
            continue;
        }
        // Silently skipping a definition that fails to parse would drop a whole
        // resource from the generated model, which is far worse than stopping.
        let name = entry
            .resource
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("<unnamed>")
            .to_string();
        let parsed = ::serde_json::from_value::<T>(entry.resource).map_err(|e| {
            std::io::Error::other(format!(
                "{}: could not read {resource_type} {name:?}: {e}",
                path.display()
            ))
        })?;
        out.push(parsed);
    }
    Ok(out)
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
    fn system_elements_are_recognized_in_every_release() {
        // R4/R5 mark them with a FHIRPath system type.
        let mut el = element("Element.id", "1");
        el.types = vec![ElementType {
            code: "http://hl7.org/fhirpath/System.String".to_string(),
            target_profile: Vec::new(),
        }];
        assert!(el.is_system_element());

        // R3 types the same element as a plain `string`.
        let mut el = element("Element.id", "1");
        el.types = vec![ElementType { code: "string".to_string(), target_profile: Vec::new() }];
        assert!(el.is_system_element());

        // As it does `Extension.url`, which R3 calls a `uri`.
        let mut el = element("Extension.url", "1");
        el.types = vec![ElementType { code: "uri".to_string(), target_profile: Vec::new() }];
        assert!(el.is_system_element());

        // An ordinary primitive element is not one.
        let mut el = element("Patient.birthDate", "1");
        el.types = vec![ElementType { code: "date".to_string(), target_profile: Vec::new() }];
        assert!(!el.is_system_element());
    }

    #[test]
    fn target_profile_reads_both_shapes() {
        // R3 writes one string; R4/R5 write a list.
        let one: ElementType =
            ::serde_json::from_value(::serde_json::json!({ "code": "Reference",
                "targetProfile": "http://hl7.org/fhir/StructureDefinition/Patient" }))
            .unwrap();
        assert_eq!(one.target_profile, ["http://hl7.org/fhir/StructureDefinition/Patient"]);

        let many: ElementType =
            ::serde_json::from_value(::serde_json::json!({ "code": "Reference",
                "targetProfile": ["a", "b"] }))
            .unwrap();
        assert_eq!(many.target_profile, ["a", "b"]);

        let none: ElementType =
            ::serde_json::from_value(::serde_json::json!({ "code": "string" })).unwrap();
        assert!(none.target_profile.is_empty());
    }

    #[test]
    fn binding_value_set_reads_every_spelling() {
        // R4/R5: a canonical string.
        let b: Binding = ::serde_json::from_value(::serde_json::json!({
            "strength": "required", "valueSet": "http://x/vs|4.0.1" }))
        .unwrap();
        assert_eq!(b.value_set(), Some("http://x/vs|4.0.1"));

        // R3: a Reference.
        let b: Binding = ::serde_json::from_value(::serde_json::json!({
            "strength": "required", "valueSetReference": { "reference": "http://x/vs" } }))
        .unwrap();
        assert_eq!(b.value_set(), Some("http://x/vs"));

        // R3: a bare URI.
        let b: Binding = ::serde_json::from_value(::serde_json::json!({
            "strength": "required", "valueSetUri": "http://x/vs" }))
        .unwrap();
        assert_eq!(b.value_set(), Some("http://x/vs"));

        // No value set at all.
        let b: Binding =
            ::serde_json::from_value(::serde_json::json!({ "strength": "example" })).unwrap();
        assert_eq!(b.value_set(), None);
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
