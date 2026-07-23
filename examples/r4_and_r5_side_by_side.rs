//! Use the R4 and R5 models in one program, and convert between them.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example r4_and_r5_side_by_side --features "r4 r5"
//! ```
//!
//! `fhir::r4::resources::Patient` and `fhir::r5::resources::Patient` are
//! different Rust types. That is deliberate: the releases genuinely disagree,
//! and a type that quietly stood for both would let an R4 value be written to
//! an R5 endpoint with elements that mean something else there.
//!
//! To move data between them, go through JSON and decide explicitly what to do
//! with anything that does not carry over. This example shows a Patient, whose
//! shape is stable across the two releases, and then points at an element that
//! is not.

use fhir::r4::types::String as R4String;
use fhir::r5::types::String as R5String;

fn main() {
    // An R4 Patient, as it might arrive from an R4 server.
    let r4_patient: fhir::r4::resources::Patient = serde_json::from_value(serde_json::json!({
        "resourceType": "Patient",
        "id": "pat-1",
        "active": true,
        "gender": "female",
        "name": [{ "family": "Chalmers", "given": ["Jane"] }]
    }))
    .expect("valid R4 Patient");
    println!("R4 id: {}", r4_patient.id.as_ref().map_or("", |s| &s.0));

    // Convert by re-parsing the JSON as R5. Serde reports anything the target
    // release does not accept, rather than dropping it silently.
    let json = serde_json::to_value(&r4_patient).expect("serialize R4");
    let r5_patient: fhir::r5::resources::Patient =
        serde_json::from_value(json).expect("this Patient is shape-compatible across releases");
    println!("R5 id: {}", r5_patient.id.as_ref().map_or("", |s| &s.0));

    assert_eq!(r4_patient.name[0].family, Some(R4String("Chalmers".to_string())));
    assert_eq!(r5_patient.name[0].family, Some(R5String("Chalmers".to_string())));

    // Not every resource carries over so cleanly. `Observation.value[x]` allows
    // 11 types in R4 and 13 in R5, and the metadata tables say so:
    let r4_value = fhir::r4::meta::element("Observation.value[x]").expect("R4 element");
    let r5_value = fhir::r5::meta::element("Observation.value[x]").expect("R5 element");
    println!(
        "Observation.value[x] allows {} types in R4 and {} in R5",
        r4_value.types.len(),
        r5_value.types.len()
    );

    let r5_only: Vec<&str> = r5_value
        .type_codes()
        .filter(|code| !r4_value.type_codes().any(|c| c == *code))
        .collect();
    println!("types R5 added: {r5_only:?}");
    assert!(r5_only.contains(&"Attachment"));
}
