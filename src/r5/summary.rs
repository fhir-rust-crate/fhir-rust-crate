//! Summary serialization (the FHIR `_summary=true` view).
//!
//! FHIR lets a client request only the *summary* elements of a resource — those
//! marked `isSummary` in the specification, plus mandatory elements. This module
//! prunes a serialized resource down to that set using the
//! [`meta`](crate::r5::meta) table.
//!
//! ```
//! use fhir::r5::resources::Patient;
//! use fhir::r5::summary::to_summary_value;
//!
//! let patient: Patient = serde_json::from_value(serde_json::json!({
//!     "resourceType": "Patient",
//!     "id": "pat-1",
//!     "gender": "male",           // isSummary
//!     "photo": [{ "title": "x" }] // NOT isSummary
//! })).unwrap();
//!
//! let summary = to_summary_value(&patient, "Patient");
//! assert!(summary.get("gender").is_some());
//! assert!(summary.get("photo").is_none());
//! ```

use ::serde::Serialize;
use ::serde_json::Value;

use crate::r5::meta;

/// Serialize `resource` (a value of FHIR type `resource_type`) and prune it to
/// the summary view: top-level elements that are `isSummary` or mandatory, plus
/// `resourceType`. Non-summary elements — and their `_field` extension siblings
/// — are removed.
#[must_use]
pub fn to_summary_value<T: Serialize>(resource: &T, resource_type: &str) -> Value {
    let mut value = ::serde_json::to_value(resource).unwrap_or(Value::Null);
    prune_to_summary(&mut value, resource_type);
    // A bare resource struct does not serialize `resourceType` (the `Resource`
    // enum adds it); inject it so the summary is a valid resource.
    if let Value::Object(map) = &mut value {
        map.entry("resourceType".to_string())
            .or_insert_with(|| Value::String(resource_type.to_string()));
    }
    value
}

/// Prune a serialized resource object in place to its summary elements.
pub fn prune_to_summary(value: &mut Value, resource_type: &str) {
    let Value::Object(map) = value else { return };
    let bases = summary_bases(resource_type);
    map.retain(|key, _| keep_in_summary(key, &bases));
}

/// A direct child element: (camelCase base name, is-summary-or-mandatory, is-choice).
fn summary_bases(resource_type: &str) -> Vec<(String, bool, bool)> {
    let prefix_len = resource_type.len() + 1;
    meta::elements_of(resource_type)
        .filter(|e| !e.path[prefix_len..].contains('.')) // direct children only
        .map(|e| {
            let leaf = &e.path[prefix_len..];
            let is_choice = leaf.ends_with("[x]");
            let base = leaf.trim_end_matches("[x]").to_string();
            (base, e.is_summary || e.min >= 1, is_choice)
        })
        .collect()
}

fn keep_in_summary(key: &str, bases: &[(String, bool, bool)]) -> bool {
    if key == "resourceType" {
        return true;
    }
    // A `_field` sibling follows its base element.
    let bare = key.strip_prefix('_').unwrap_or(key);
    bases.iter().any(|(base, is_summary, is_choice)| {
        *is_summary
            && (bare == base
                || (*is_choice
                    && bare.starts_with(base.as_str())
                    && bare[base.len()..].chars().next().is_some_and(char::is_uppercase)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r5::resources::Patient;

    #[test]
    fn patient_summary_keeps_summary_drops_the_rest() {
        let patient: Patient = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Patient",
            "id": "p1",
            "gender": "male",
            "birthDate": "1970-01-01",
            "name": [{ "family": "Chalmers" }],
            "photo": [{ "title": "portrait" }],
            "maritalStatus": { "text": "married" },
            "communication": [{ "language": { "text": "en" } }]
        }))
        .unwrap();

        let s = to_summary_value(&patient, "Patient");
        // Summary elements are kept.
        for k in ["resourceType", "id", "gender", "birthDate", "name"] {
            assert!(s.get(k).is_some(), "expected {k} in summary");
        }
        // Non-summary elements are dropped.
        for k in ["photo", "maritalStatus", "communication"] {
            assert!(s.get(k).is_none(), "expected {k} absent from summary");
        }
    }

    #[test]
    fn choice_element_summary() {
        // Patient.deceased[x] is isSummary; multipleBirth[x] is not.
        let patient: Patient = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Patient",
            "deceasedBoolean": true,
            "multipleBirthInteger": 2
        }))
        .unwrap();
        let s = to_summary_value(&patient, "Patient");
        assert!(s.get("deceasedBoolean").is_some());
        assert!(s.get("multipleBirthInteger").is_none());
    }
}
