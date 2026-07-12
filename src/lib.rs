//! # FHIR R5 for Rust
//!
//! `fhir` is a Rust implementation of the **HL7 FHIR® Release 5 (R5)** data
//! model, together with a code generator that produces it from the official
//! FHIR specification JSON files.
//!
//! Fast Healthcare Interoperability Resources (FHIR) is the HL7 standard for
//! exchanging electronic health records. This crate gives you:
//!
//! - **Every R5 resource** (Patient, Observation, Encounter, …) as a Rust
//!   `struct` under [`r5::resources`], each round-tripping to and from the
//!   canonical FHIR JSON via `serde`.
//! - **Every R5 datatype** (Period, HumanName, CodeableConcept, …) under
//!   [`r5::types`], including the primitive newtypes ([`Code`](r5::types::Code),
//!   [`Id`](r5::types::Id), [`DateTime`](r5::types::DateTime), …).
//! - **Type-safe code systems** — 400+ FHIR `CodeSystem`s as Rust enums under
//!   [`r5::codes`].
//! - **A polymorphic [`Resource`](r5::resources::Resource) enum** tagged by
//!   `resourceType`, for reading a resource whose type you do not know ahead of
//!   time.
//! - **Lightweight validation** via the [`Validate`](r5::validate::Validate)
//!   trait and `#[derive(Validate)]`.
//! - **A code generator** under [`r5::parse`] that reads the FHIR spec JSON
//!   shipped in [`DEFINITIONS_DIR`] and emits Rust source.
//!
//! FHIR® is a registered trademark of Health Level Seven International. This
//! crate is not affiliated with or endorsed by HL7.
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! fhir = "0.1"
//! serde_json = "1" # or any other serde data format
//! ```
//!
//! ## Design in one paragraph
//!
//! Everything in the data model derives [`serde::Serialize`] and
//! [`serde::Deserialize`], so you work with the crate almost entirely through
//! `serde_json`. Resources and complex datatypes are plain structs; FHIR
//! *primitive* types (`code`, `id`, `dateTime`, …) are thin newtypes such as
//! `Code(String)` that serialize *transparently* to their underlying JSON
//! value. Optional elements are `Option<T>`, repeating elements are `Vec<T>`,
//! and FHIR's `value[x]` choice elements are a generated enum per element. See
//! [Cardinality and choice types](#cardinality-and-choice-types) below.
//!
//! ## Quick start
//!
//! Build a `Patient`, serialize it to canonical FHIR JSON, and parse it back:
//!
//! ```
//! use fhir::r5::resources::Patient;
//! use fhir::r5::coded::Coded;
//! use fhir::r5::codes::AdministrativeGender;
//! use fhir::r5::types::{Boolean, HumanName, String as FhirString};
//!
//! let patient = Patient {
//!     id: Some(FhirString("pat-1".to_string())),
//!     active: Some(Boolean(true)),
//!     gender: Some(Coded::Known(AdministrativeGender::Male)),
//!     name: vec![HumanName {
//!         family: Some(FhirString("Chalmers".to_string())),
//!         given: vec![FhirString("Peter".to_string())],
//!         ..Default::default()
//!     }],
//!     ..Default::default()
//! };
//!
//! // Serialize to canonical FHIR JSON. `None` fields are omitted.
//! let json = serde_json::to_string_pretty(&patient).unwrap();
//! assert!(json.contains("\"family\": \"Chalmers\""));
//!
//! // Parse it back — a perfect round trip.
//! let parsed: Patient = serde_json::from_str(&json).unwrap();
//! assert_eq!(parsed, patient);
//! ```
//!
//! ## Primitives are transparent newtypes
//!
//! A FHIR primitive is a wrapper around a Rust value, so its JSON form is just
//! that value — no wrapper object:
//!
//! ```
//! use fhir::r5::types::{Boolean, Code, Integer64};
//!
//! assert_eq!(serde_json::to_value(Code("final".to_string())).unwrap(), "final");
//! assert_eq!(serde_json::to_value(Boolean(true)).unwrap(), true);
//! // `integer64` is serialized as a JSON *string* per the FHIR spec:
//! assert_eq!(serde_json::to_value(Integer64(9_000_000_000)).unwrap(), "9000000000");
//! ```
//!
//! ## Validation
//!
//! The [`Validate`](r5::validate::Validate) trait reports every problem it
//! finds as a [`ValidationIssue`](r5::validate::ValidationIssue). Primitive
//! types check their FHIR regex constraints; `#[derive(Validate)]` makes
//! complex types and resources validate recursively, prefixing each nested
//! issue's `path` with the field name.
//!
//! ```
//! use fhir::r5::resources::Patient;
//! use fhir::r5::types::{Id, Uri};
//! use fhir::r5::validate::Validate;
//!
//! // Primitive format checks:
//! assert!(Id("patient-1".to_string()).is_valid());
//! assert!(!Id("has spaces".to_string()).is_valid());
//!
//! // Recursive validation of a whole resource:
//! let mut patient = Patient::default();
//! assert!(patient.validate().is_empty());
//!
//! // A `uri` may not be surrounded by whitespace.
//! patient.implicit_rules = Some(Uri(" http://bad ".to_string()));
//! let issues = patient.validate();
//! assert_eq!(issues.len(), 1);
//! // The path is prefixed with the field name, then the primitive's own label.
//! assert_eq!(issues[0].path, "implicit_rules.uri");
//! ```
//!
//! ## Code systems as enums
//!
//! Coded values are available as type-safe enums that serialize to their
//! canonical FHIR code strings:
//!
//! ```
//! use fhir::r5::codes::AdministrativeGender;
//!
//! let gender = AdministrativeGender::Female;
//! assert_eq!(serde_json::to_value(&gender).unwrap(), "female");
//!
//! let parsed: AdministrativeGender = serde_json::from_value("male".into()).unwrap();
//! assert_eq!(parsed, AdministrativeGender::Male);
//! ```
//!
//! ## Reading a resource of unknown type
//!
//! When you receive JSON but do not know its `resourceType`, deserialize into
//! the [`Resource`](r5::resources::Resource) enum. It is tagged by
//! `resourceType`, so serde picks the right variant for you:
//!
//! ```
//! use fhir::r5::resources::Resource;
//!
//! let json = serde_json::json!({
//!     "resourceType": "Patient",
//!     "id": "pat-1",
//!     "active": true
//! });
//!
//! match serde_json::from_value(json).unwrap() {
//!     Resource::Patient(patient) => {
//!         assert_eq!(patient.id.unwrap().0, "pat-1");
//!     }
//!     other => panic!("expected a Patient, got {other:?}"),
//! }
//! ```
//!
//! ## Cardinality and choice types
//!
//! FHIR element cardinality maps to Rust types as follows:
//!
//! | FHIR cardinality | Rust type        |
//! |------------------|------------------|
//! | `0..1`           | `Option<T>`      |
//! | `1..1`           | `T`              |
//! | `0..*`           | `Option<Vec<T>>` |
//! | `1..*`           | `Vec<T>`         |
//!
//! A `value[x]` *choice* element becomes one generated enum with a variant per
//! allowed type — for example `Observation.value` is `Option<ObservationValue>`
//! with variants `Quantity`, `String`, `Boolean`, … so exactly one is set.
//! Required-binding coded fields are their [`r5::codes`] enum wrapped in
//! [`Coded`](r5::coded::Coded). Every resource and datatype has a
//! `Type::builder()`, and [`prelude`] re-exports the common items.
//!
//! ## More examples
//!
//! Runnable programs live in the `examples/` directory. Run one with, e.g.:
//!
//! ```sh
//! cargo run --example build_patient
//! cargo run --example validate_resource
//! cargo run --example read_bundle
//! cargo run --example primitive_extensions
//! cargo run --example operation_outcome
//! cargo run --example extensions
//! cargo run --example transaction_bundle
//! ```
//!
//! ## Crate layout
//!
//! - [`r5::resources`] — the 158 R5 resources, plus the [`Resource`](r5::resources::Resource) enum.
//! - [`r5::types`] — the ~50 complex datatypes and 21 primitive newtypes.
//! - [`r5::codes`] — FHIR `CodeSystem`s as enums.
//! - [`r5::validate`] — the [`Validate`](r5::validate::Validate) trait and primitive constraints.
//! - [`r5::meta`] — per-element specification metadata (cardinality, bindings, choice types, reference targets).
//! - [`r5::parse`] — the code generator that reads [`DEFINITIONS_DIR`].

// Enable Clippy's pedantic lint group across the whole crate.
#![warn(clippy::pedantic)]
// This crate deliberately nests each concept in a directory beside a
// same-named module file (e.g. `resource/resource.rs`), so allow the
// resulting module-inception pattern crate-wide.
#![allow(clippy::module_inception)]
// The type/field doc comments are long-form FHIR prose (mentioning terms such
// as ValueSet, ConceptMap, JSON, XHTML, etc.); backticking every such term
// would harm readability without adding value, so allow these doc lints.
#![allow(clippy::doc_markdown)]
#![allow(clippy::doc_link_with_quotes)]
// The generated/parse modules deliberately glob-import their sibling namespace
// (`use crate::r5::parse::…::*;`) as a convenience; keep that pattern.
#![allow(clippy::wildcard_imports)]
// These fire only on the work-in-progress code-generator helpers (which unwrap
// freely and return owned values). Documenting every panic/`Result`/`#[must_use]`
// on WIP internals is premature churn; revisit once the generator API stabilizes.
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
// The datatype/resource doc comments are long-form FHIR prose that contains
// bare specification URLs (e.g. `http://hl7.org/fhir/...`) and FHIR choice
// notation such as `value[x]`. rustdoc would otherwise treat these as bare
// URLs or (broken) intra-doc links. Backticking/wrapping every occurrence
// across ~200 modules would not improve the docs, so allow these rustdoc lints.
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::broken_intra_doc_links)]

/// An async FHIR REST client (feature `client`).
#[cfg(feature = "client")]
pub mod client;
pub mod prelude;
pub mod r5;
pub mod util;

/// Absolute path to the directory holding the FHIR R5 specification JSON files
/// that ship with this crate.
pub static DEFINITIONS_DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("doc")
            .join("fhir-specifications")
            .join("r5")
            .join("fhir-definitions-json")
    });

/// Literate programming: a block of generated Rust source code, as a string.
pub type SourceCodeString = String;
