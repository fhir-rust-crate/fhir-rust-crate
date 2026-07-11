//! FHIR Release 5 (R5).
//!
//! This module holds the entire FHIR R5 implementation. The pieces you use
//! day to day are:
//!
//! - [`resources`] — the 158 R5 resources plus the polymorphic
//!   [`Resource`](resources::Resource) enum.
//! - [`types`] — the ~50 complex datatypes and 21 primitive newtypes.
//! - [`codes`] — FHIR `CodeSystem`s as type-safe enums.
//! - [`validate`] — the [`Validate`](validate::Validate) trait and the
//!   primitive-format constraints.
//!
//! The remaining modules ([`parse`], [`abstract_types`], [`properties`],
//! [`resource`], [`todo`]) support the code generator that produces the model
//! from the official specification JSON, and are not needed for consuming the
//! data model.
//!
//! See the [crate-level guide](crate) for a task-oriented walkthrough.

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

/// Per-element metadata extracted from the FHIR R5 specification (cardinality,
/// bindings, choice types, reference targets, summary membership).
pub mod meta;

/// Support types for `value[x]` choice elements (see `spec/11-choice-types.md`).
pub mod choice;
