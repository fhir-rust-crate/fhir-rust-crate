//! FHIR specifications parser
//!
//! A Fast Healthcare Interoperability Resources (FHIR) parser and code
//! generator for FHIR Release 5 (R5).
//!
//! This crate exposes two things:
//!
//! - The FHIR R5 data model as Rust types under [`r5::types`], which
//!   serialize to and from the canonical FHIR JSON representation via `serde`.
//! - A code generator (under [`r5::parse`]) that reads the official FHIR R5
//!   specification JSON files and emits Rust source code.
//!
//! # Examples
//!
//! Round-trip a FHIR datatype through JSON:
//!
//! ```
//! use fhir_specifications_parser::r5::types::period::Period;
//!
//! let period = Period::default();
//! let json = ::serde_json::to_value(&period).unwrap();
//! let back: Period = ::serde_json::from_value(json).unwrap();
//! assert_eq!(period, back);
//! ```

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
