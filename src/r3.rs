//! FHIR Release 4 (R3).
//!
//! This module holds the FHIR R3 (3.0.2) implementation, laid out exactly like
//! [`crate::r5`] so that porting between releases is a matter of changing one
//! path segment. The pieces you use day to day are:
//!
//! - [`resources`] — the 146 R3 resources plus the polymorphic
//!   [`Resource`](resources::Resource) enum.
//! - [`types`] — the 43 complex datatypes and 20 primitive newtypes.
//! - [`codes`] — FHIR `CodeSystem`s as type-safe enums.
//! - [`validate`] — the R3 primitive-format constraints, over the shared
//!   [`Validate`](crate::validate::Validate) trait.
//!
//! Unlike [`crate::r5`], every model module here is **generated** from the
//! official R3 definition JSON by [`crate::codegen`]; regenerate with
//! `cargo run -- r3` rather than editing `src/r3/types` or `src/r3/resources`
//! by hand.
//!
//! # R3 is not R5
//!
//! The releases are deliberately separate types, because they disagree in ways
//! that silently corrupt data if conflated. For example `Observation.value[x]`
//! allows 11 types in R3 and 13 in R5; `MedicationRequest.medication[x]` is a
//! choice in R3 but a `CodeableReference` in R5; and R3 has no `integer64`,
//! `CodeableReference`, or `RatioRange` datatype at all. Convert between them
//! explicitly, through JSON, rather than assuming they interoperate.
//!
//! See the [crate-level guide](crate) for a task-oriented walkthrough.

/// FHIR R3 datatypes.
pub mod types;

/// FHIR R3 resources.
pub mod resources;

/// FHIR R3 code systems as type-safe enums.
pub mod codes;

/// Lightweight FHIR R3 validation.
pub mod validate;

/// Per-element metadata extracted from the FHIR R3 specification (cardinality,
/// bindings, choice types, reference targets, summary membership).
pub mod meta;

/// Support types for `value[x]` choice elements (see `spec/11-choice-types.md`).
pub mod choice;

/// The [`Coded`](coded::Coded) wrapper for `required`-binding coded fields.
pub mod coded;

/// Support for the generated `#[derive(Builder)]` builders.
pub mod builder;

/// Ergonomic extension accessors ([`ExtensionExt`](extension_ext::ExtensionExt)).
pub mod extension_ext;

/// Utilities for `Bundle`s: iteration, paging, and transaction/batch building.
pub mod bundle_util;

/// Summary serialization (the FHIR `_summary=true` view).
pub mod summary;

/// Parsing and precision-aware comparison for the date/time primitives.
pub mod temporal;

/// Common imports for working with FHIR R3.
pub mod prelude;

/// An async FHIR R3 REST client (feature `client`).
#[cfg(feature = "client")]
pub mod client;

/// FHIR XML serialization (feature `xml`).
#[cfg(feature = "xml")]
pub mod xml;

/// The FHIR R3 release, as a type.
///
/// Marker for release-parameterized code such as
/// [`ReleaseClient`](crate::client::ReleaseClient); see [`crate::release`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct R3;

impl crate::release::Release for R3 {
    const LABEL: &'static str = "R3";
    const VERSION: &'static str = "3.0.2";
    type Resource = resources::Resource;
    type Bundle = resources::Bundle;
    type CapabilityStatement = resources::CapabilityStatement;
    type OperationOutcome = resources::OperationOutcome;
}
