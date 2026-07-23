//! The shape of the per-element metadata table, shared by every release.
//!
//! Each release generates its own table of [`ElementMeta`] — the facts the
//! specification states about an element that the Rust types cannot carry, such
//! as whether a repeating field was `0..*` or `1..*`, which value set a code is
//! bound to, and which resources a `Reference` may point at. The *types* in
//! that table, and the lookups over it, do not vary by release, so they are
//! defined once here and used by [`r4::meta`](crate::r4::meta) and
//! [`r5::meta`](crate::r5::meta).
//!
//! ```
//! use fhir::r5::meta;
//!
//! let gender = meta::element("Patient.gender").unwrap();
//! assert_eq!(gender.binding.unwrap().strength, fhir::meta::BindingStrength::Required);
//! ```

use std::collections::HashMap;

/// Binding strength for a coded element
/// (`ElementDefinition.binding.strength`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingStrength {
    /// The value must come from the bound value set.
    Required,
    /// Codes from the value set should be used; others allowed if none fit.
    Extensible,
    /// The value set is a suggestion.
    Preferred,
    /// The value set is illustrative only.
    Example,
}

impl BindingStrength {
    /// Parse a FHIR strength token (`"required"`, …); unknown tokens map to
    /// [`Example`](Self::Example).
    #[must_use]
    pub fn from_token(token: &str) -> Self {
        match token {
            "required" => Self::Required,
            "extensible" => Self::Extensible,
            "preferred" => Self::Preferred,
            _ => Self::Example,
        }
    }
}

/// A value-set binding on a coded element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingMeta {
    /// How strictly the value set applies.
    pub strength: BindingStrength,
    /// Canonical `ValueSet` URL (may carry a `|version` suffix), if declared.
    pub value_set: Option<&'static str>,
}

/// One allowed type for an element (an entry of `ElementDefinition.type`).
///
/// A `value[x]` choice element has one `TypeRef` per allowed type; a reference
/// element carries its allowed target resource profiles.
#[derive(Debug, Clone, Copy)]
pub struct TypeRef {
    /// FHIR type code, e.g. `"Quantity"`, `"string"`, `"Reference"`.
    pub code: &'static str,
    /// For `Reference`/`canonical` types, the allowed target resource profiles
    /// as canonical URLs; empty otherwise.
    pub target_profiles: &'static [&'static str],
}

impl TypeRef {
    /// The bare target resource names (final path segment of each profile URL).
    ///
    /// ```
    /// use fhir::r5::meta;
    /// let subject = meta::element("Observation.subject").unwrap();
    /// let targets: Vec<_> = subject.types[0].target_names().collect();
    /// assert!(targets.contains(&"Patient"));
    /// ```
    pub fn target_names(&self) -> impl Iterator<Item = &'static str> {
        self.target_profiles
            .iter()
            .map(|url| url.rsplit(['/', '#']).next().unwrap_or(url))
    }
}

/// Metadata for one element of a FHIR resource or datatype, keyed by its full
/// `ElementDefinition` path.
#[derive(Debug, Clone, Copy)]
pub struct ElementMeta {
    /// Full FHIR path, e.g. `"Patient.gender"` or `"Observation.value[x]"`.
    pub path: &'static str,
    /// Minimum cardinality.
    pub min: u32,
    /// Maximum cardinality as the raw FHIR token: `"0"`, `"1"`, `"*"`, or a
    /// number.
    pub max: &'static str,
    /// Whether the element is part of the summary view
    /// (`ElementDefinition.isSummary`).
    pub is_summary: bool,
    /// Coded-value binding, if any.
    pub binding: Option<BindingMeta>,
    /// Allowed types; more than one for a `value[x]` choice element.
    pub types: &'static [TypeRef],
}

impl ElementMeta {
    /// Whether the element is mandatory (minimum cardinality ≥ 1).
    #[must_use]
    pub fn is_required(&self) -> bool {
        self.min >= 1
    }

    /// Whether the element repeats (maximum cardinality greater than one).
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        self.max != "0" && self.max != "1"
    }

    /// Whether the element is a `value[x]`-style choice element.
    #[must_use]
    pub fn is_choice(&self) -> bool {
        self.path.ends_with("[x]")
    }

    /// The FHIR type codes allowed for this element.
    pub fn type_codes(&self) -> impl Iterator<Item = &'static str> {
        self.types.iter().map(|t| t.code)
    }
}

/// Look up an element by full FHIR path in a release's table.
///
/// The table is generated sorted by path, so this is a binary search.
#[must_use]
pub fn find(table: &'static [ElementMeta], path: &str) -> Option<&'static ElementMeta> {
    table.binary_search_by(|e| e.path.cmp(path)).ok().map(|i| &table[i])
}

/// Map every generated struct name to the FHIR path prefix it represents, e.g.
/// `"AppointmentParticipant"` to `"Appointment.participant"`.
///
/// Backbone struct names are the PascalCase concatenation of their path
/// segments, which is not reversible on its own — `PatientContact` could split
/// in several places — so the mapping is built from the paths that actually
/// exist.
#[must_use]
pub fn struct_prefixes(table: &'static [ElementMeta]) -> HashMap<String, &'static str> {
    use ::convert_case::{Case, Casing};

    let mut map = HashMap::new();
    for e in table {
        let seg_count = e.path.split('.').count();
        for take in 1..seg_count {
            let name: String =
                e.path.split('.').take(take).map(|s| s.to_case(Case::Pascal)).collect();
            if let Some((end, _)) = e.path.match_indices('.').nth(take - 1) {
                map.entry(name).or_insert(&e.path[..end]);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    static TABLE: &[ElementMeta] = &[
        ElementMeta { path: "Patient.active", min: 0, max: "1", is_summary: true, binding: None, types: &[TypeRef { code: "boolean", target_profiles: &[] }] },
        ElementMeta { path: "Patient.contact", min: 0, max: "*", is_summary: false, binding: None, types: &[TypeRef { code: "BackboneElement", target_profiles: &[] }] },
        ElementMeta { path: "Patient.contact.name", min: 0, max: "1", is_summary: false, binding: None, types: &[TypeRef { code: "HumanName", target_profiles: &[] }] },
        ElementMeta { path: "Patient.link.other", min: 1, max: "1", is_summary: false, binding: None, types: &[TypeRef { code: "Reference", target_profiles: &["http://hl7.org/fhir/StructureDefinition/Patient"] }] },
    ];

    #[test]
    fn lookup_by_path() {
        assert_eq!(find(TABLE, "Patient.active").unwrap().max, "1");
        assert!(find(TABLE, "Patient.nope").is_none());
    }

    #[test]
    fn cardinality_helpers() {
        let active = find(TABLE, "Patient.active").unwrap();
        assert!(!active.is_required());
        assert!(!active.is_multiple());
        let contact = find(TABLE, "Patient.contact").unwrap();
        assert!(contact.is_multiple());
        assert!(find(TABLE, "Patient.link.other").unwrap().is_required());
    }

    #[test]
    fn target_names_strip_the_profile_url() {
        let other = find(TABLE, "Patient.link.other").unwrap();
        let names: Vec<&str> = other.types[0].target_names().collect();
        assert_eq!(names, ["Patient"]);
    }

    #[test]
    fn struct_names_map_back_to_paths() {
        let prefixes = struct_prefixes(TABLE);
        assert_eq!(prefixes.get("Patient").copied(), Some("Patient"));
        assert_eq!(prefixes.get("PatientContact").copied(), Some("Patient.contact"));
    }

    #[test]
    fn strength_tokens_parse() {
        assert_eq!(BindingStrength::from_token("required"), BindingStrength::Required);
        assert_eq!(BindingStrength::from_token("nonsense"), BindingStrength::Example);
    }
}
