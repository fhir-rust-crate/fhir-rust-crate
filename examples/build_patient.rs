//! Build a FHIR R5 `Patient` in Rust and serialize it to canonical FHIR JSON.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example build_patient
//! ```
//!
//! This example shows the everyday shape of working with the crate: construct a
//! resource struct, wrap primitive values in their newtypes, and let `serde`
//! produce the JSON. Because every resource derives `Default`, you only set the
//! fields you care about and spread `..Default::default()` for the rest.

use fhir::r5::coded::Coded;
use fhir::r5::codes::{AdministrativeGender, ContactPointSystem};
use fhir::r5::resources::Patient;
use fhir::r5::types::{Boolean, Code, ContactPoint, HumanName, String as FhirString};

fn main() {
    // FHIR primitives are transparent newtypes: `String(std::string::String)`,
    // `Boolean(bool)`, `Code(String)`, and so on. Wrapping keeps the model
    // type-safe while still serializing to a plain JSON value.
    let patient = Patient {
        id: Some(FhirString("pat-1".to_string())),
        active: Some(Boolean(true)),
        // `gender` has a required binding, so it is typed as the code enum
        // (wrapped in `Coded`, which tolerates codes outside the value set).
        gender: Some(Coded::Known(AdministrativeGender::Male)),

        // `name` is `0..*`, so it is `Option<Vec<HumanName>>`.
        name: Some(vec![HumanName {
            family: Some(FhirString("Chalmers".to_string())),
            // `given` is `0..*` but non-nullable in this struct, so a bare Vec.
            given: vec![
                FhirString("Peter".to_string()),
                FhirString("James".to_string()),
            ],
            ..Default::default()
        }]),

        // `telecom` demonstrates another repeating datatype.
        telecom: Some(vec![ContactPoint {
            system: Some(Coded::Known(ContactPointSystem::Phone)),
            value: Some(FhirString("(03) 5555 6473".to_string())),
            use1: Some(Code("work".to_string())),
            ..Default::default()
        }]),

        // Everything else stays at its default (`None` / empty).
        ..Default::default()
    };

    // `skip_serializing_none` means unset optional fields are omitted entirely,
    // producing clean, canonical FHIR JSON.
    let json = serde_json::to_string_pretty(&patient).expect("serialize Patient");
    println!("{json}");
}
