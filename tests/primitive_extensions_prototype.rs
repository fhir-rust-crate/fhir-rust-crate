//! T6 prototype: verify the `_field` primitive-extension *sibling* approach
//! round-trips FHIR primitive extensions with no data loss.
//!
//! The chosen representation adds, next to a primitive field `x`, a sibling
//! field `_x: Option<Element>` (scalar) or `_x: Option<Vec<Option<Element>>>`
//! (repeating), serde-renamed to the FHIR `_x` key. These tests exercise all
//! three shapes on Patient, Observation, and Questionnaire.
//!
//! See `spec/09-primitive-extensions.md` for the design comparison and decision.

use fhir::r5::resources::Resource;
use serde_json::{Value, json};

/// Assert that `original` deserializes into the polymorphic [`Resource`] enum
/// (which carries the `resourceType` tag), re-serializes, and matches
/// `original` exactly (order-independent `Value` equality). This mirrors how
/// resources are actually round-tripped by the official-examples suite.
fn assert_roundtrips(original: Value) {
    let parsed: Resource = serde_json::from_value(original.clone()).expect("deserialize");
    let reserialized = serde_json::to_value(&parsed).expect("serialize");
    assert_eq!(reserialized, original, "round-trip mismatch");
}

#[test]
fn patient_scalar_primitive_extensions() {
    // `_birthDate` (with the standard patient-birthTime extension), `_gender`
    // (element id), and `_active` all present alongside their values.
    let patient = json!({
        "resourceType": "Patient",
        "id": "pat-1",
        "active": true,
        "_active": { "id": "active-elem" },
        "gender": "female",
        "_gender": {
            "extension": [{
                "url": "http://hl7.org/fhir/StructureDefinition/data-absent-reason",
                "valueCode": "masked"
            }]
        },
        "birthDate": "1970-03-25",
        "_birthDate": {
            "extension": [{
                "url": "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
                "valueDateTime": "1970-03-25T14:35:00-05:00"
            }]
        }
    });
    assert_roundtrips(patient);
}

#[test]
fn patient_extension_only_no_value() {
    // FHIR permits a primitive extension with *no* value: only `_gender` present.
    let patient = json!({
        "resourceType": "Patient",
        "_gender": {
            "extension": [{
                "url": "http://hl7.org/fhir/StructureDefinition/data-absent-reason",
                "valueCode": "unknown"
            }]
        }
    });
    assert_roundtrips(patient);
}

#[test]
fn observation_required_primitive_extension() {
    // `status` is a required 1..1 primitive; it can still carry `_status`.
    let observation = json!({
        "resourceType": "Observation",
        "status": "final",
        "_status": { "id": "status-1" },
        "code": { "text": "Body temperature" }
    });
    assert_roundtrips(observation);
}

#[test]
fn questionnaire_repeating_primitive_extensions() {
    // Repeating primitive: `subjectType` array aligns with `_subjectType`,
    // where `null` marks entries without an extension. This is the case that
    // forces `Vec<Option<Element>>`.
    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "status": "active",
        "subjectType": ["Patient", "Group", "Practitioner"],
        "_subjectType": [
            null,
            { "extension": [{
                "url": "http://example.org/fhir/StructureDefinition/note",
                "valueString": "groups only in pilot"
            }]},
            null
        ]
    });
    assert_roundtrips(questionnaire);
}

#[test]
fn questionnaire_no_extensions_still_clean() {
    // Without `_subjectType`, nothing extra is emitted (skip_serializing_none).
    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "status": "active",
        "subjectType": ["Patient"]
    });
    assert_roundtrips(questionnaire);
}
