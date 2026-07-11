//! Element metadata extracted from the FHIR R5 specification.
//!
//! This module is a queryable, compile-time table of per-element facts drawn
//! from the official `ElementDefinition`s: cardinality, coded-value bindings,
//! the allowed types of `value[x]` choice elements, reference target profiles,
//! and summary membership. It is the foundation the type-safety and validation
//! layers build on (choice enums, required-binding checks, typed references,
//! summary serialization).
//!
//! The data is **generated** from the specification JSON by
//! [`crate::r5::parse::meta`] into [`mod@self::generated`]; do not hand-edit it.
//! Regenerate with `cargo run` (or the `regenerate` test in
//! `crate::r5::parse::meta`).
//!
//! Elements are keyed by their full FHIR path, e.g. `"Patient.gender"`,
//! `"Observation.value[x]"`, `"Patient.contact.name"`.
//!
//! ```
//! use fhir::r5::meta;
//!
//! // Patient.gender is bound, with `required` strength, to administrative-gender.
//! let gender = meta::element("Patient.gender").unwrap();
//! let binding = gender.binding.unwrap();
//! assert_eq!(binding.strength, meta::BindingStrength::Required);
//! assert!(binding.value_set.unwrap().contains("administrative-gender"));
//!
//! // Observation.value[x] is a choice element with several allowed types.
//! let value = meta::element("Observation.value[x]").unwrap();
//! assert!(value.is_choice());
//! assert!(value.type_codes().any(|c| c == "Quantity"));
//! ```

mod generated;

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

/// Look up the metadata for an element by its full FHIR path.
///
/// Returns `None` if the path is not a known element.
#[must_use]
pub fn element(path: &str) -> Option<&'static ElementMeta> {
    generated::ELEMENTS
        .binary_search_by(|e| e.path.cmp(path))
        .ok()
        .map(|i| &generated::ELEMENTS[i])
}

/// Every extracted element, sorted by path.
#[must_use]
pub fn elements() -> &'static [ElementMeta] {
    generated::ELEMENTS
}

/// Every element belonging to a given resource or datatype, i.e. whose path is
/// `"<type_name>.…"` (including nested backbone paths).
pub fn elements_of(type_name: &str) -> impl Iterator<Item = &'static ElementMeta> {
    let prefix = format!("{type_name}.");
    generated::ELEMENTS
        .iter()
        .filter(move |e| e.path.starts_with(&prefix))
}

/// The FHIR path prefix a generated Rust struct corresponds to, e.g.
/// `"AppointmentParticipant"` → `"Appointment.participant"`, `"Patient"` →
/// `"Patient"`. Backbone struct names are the PascalCase concatenation of their
/// path segments. Returns `None` for a name that is not a FHIR type/backbone.
#[must_use]
pub fn struct_prefix(struct_name: &str) -> Option<&'static str> {
    use ::convert_case::{Case, Casing};
    use std::collections::HashMap;
    use std::sync::LazyLock;

    static PREFIXES: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
        let mut map = HashMap::new();
        for e in generated::ELEMENTS {
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
    });

    PREFIXES.get(struct_name).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_non_empty_and_sorted() {
        let all = elements();
        assert!(all.len() > 5000, "expected thousands of elements, got {}", all.len());
        assert!(
            all.windows(2).all(|w| w[0].path <= w[1].path),
            "ELEMENTS must be sorted by path for binary search"
        );
    }

    // Acceptance fact 1: Patient.gender binding=required -> administrative-gender.
    #[test]
    fn patient_gender_binding() {
        let gender = element("Patient.gender").expect("Patient.gender");
        let binding = gender.binding.expect("gender has a binding");
        assert_eq!(binding.strength, BindingStrength::Required);
        assert!(
            binding.value_set.unwrap().contains("administrative-gender"),
            "value_set was {:?}",
            binding.value_set
        );
    }

    // Acceptance fact 2: Observation.value[x] is a choice with the R5 type set.
    // (The task text said 11 types from R4; R5 adds Attachment and Reference,
    // for 13.)
    #[test]
    fn observation_value_choice_types() {
        let value = element("Observation.value[x]").expect("Observation.value[x]");
        assert!(value.is_choice());
        assert_eq!(value.types.len(), 13, "R5 Observation.value[x] type count");
        for code in ["Quantity", "CodeableConcept", "string", "Attachment", "Reference"] {
            assert!(value.type_codes().any(|c| c == code), "missing type {code}");
        }
    }

    // Acceptance fact 3: Observation.subject targets Patient|Group|Device|Location.
    #[test]
    fn observation_subject_targets() {
        let subject = element("Observation.subject").expect("Observation.subject");
        let targets: Vec<&str> = subject
            .types
            .iter()
            .flat_map(TypeRef::target_names)
            .collect();
        for want in ["Patient", "Group", "Device", "Location"] {
            assert!(targets.contains(&want), "subject should target {want}: {targets:?}");
        }
    }

    #[test]
    fn cardinality_helpers() {
        // Observation.status is 1..1 (required, single).
        let status = element("Observation.status").expect("Observation.status");
        assert!(status.is_required());
        assert!(!status.is_multiple());
        // Observation.identifier is 0..* (optional, repeating).
        let identifier = element("Observation.identifier").expect("Observation.identifier");
        assert!(!identifier.is_required());
        assert!(identifier.is_multiple());
    }

    #[test]
    fn elements_of_a_type() {
        let count = elements_of("Patient").count();
        assert!(count > 20, "Patient should have many elements, got {count}");
        assert!(elements_of("Patient").all(|e| e.path.starts_with("Patient.")));
    }
}
