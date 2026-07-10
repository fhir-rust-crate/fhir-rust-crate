//! FHIR Release 5

/// Parse FHIR R5 specifications JSON file.
pub mod parse;

/// FHIR R5 classes - the complete set - TODO figure out where these go.
pub mod abstract_types;

// TODO move anywhere better
pub mod properties;
pub mod resource;
pub mod todo;

/// FHIR R5 datatypes
pub mod types;

/// FHIR R5 resources
pub mod resources;

/// FHIR R5 code systems as type-safe enums
pub mod codes;

/// Lightweight FHIR R5 validation
pub mod validate;
