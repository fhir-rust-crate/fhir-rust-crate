//! Command-line entry point for the FHIR specifications parser / generator.

use fhir_specifications_parser::DEFINITIONS_DIR;
use fhir_specifications_parser::r5::parse;
use std::fs::File;
use std::io::BufReader;

/// Read `profiles-types.json` and generate a Rust source file for each FHIR R5
/// datatype it defines.
fn generate_profiles_types() {
    let path = DEFINITIONS_DIR.join("profiles-types.json");
    let file = File::open(path).expect("open");
    let reader = BufReader::new(file);
    let bundle: parse::profiles_types::Bundle = ::serde_json::from_reader(reader).unwrap();
    bundle
        .entry
        .into_iter()
        .map(|resource_head| resource_head.resource)
        .for_each(|resource| {
            parse::profiles_types::resource_into_rust(&resource).expect("resource_into_rust");
        });
}

fn main() {
    generate_profiles_types();
}
