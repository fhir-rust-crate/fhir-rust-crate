//! Naming a FHIR release in generic code.
//!
//! The data model is release-specific — an R4 `Patient` and an R5 `Patient` are
//! different Rust types — but code that merely *moves resources around* (the
//! REST client, for example) is not. [`Release`] lets such code name "the
//! `Bundle` of whichever release the caller chose" without duplicating itself
//! per release.
//!
//! Each enabled release module provides a marker type implementing this trait:
//! [`r4::R4`](crate::r4::R4) and [`r5::R5`](crate::r5::R5).
//!
//! ```
//! use fhir::release::Release;
//! use fhir::r5::R5;
//!
//! assert_eq!(R5::LABEL, "R5");
//! assert_eq!(R5::VERSION, "5.0.0");
//!
//! // The associated types name that release's resources.
//! let bundle = <R5 as Release>::Bundle::default();
//! assert!(bundle.entry.is_empty());
//! ```

use ::serde::de::DeserializeOwned;

/// A FHIR release, as a type: a marker naming that release's core resources.
///
/// Implemented by [`r4::R4`](crate::r4::R4) and [`r5::R5`](crate::r5::R5).
pub trait Release {
    /// The release label, e.g. `"R5"`.
    const LABEL: &'static str;

    /// The full FHIR version, e.g. `"5.0.0"`.
    const VERSION: &'static str;

    /// The polymorphic resource enum, tagged by `resourceType`.
    type Resource: DeserializeOwned + std::fmt::Debug;

    /// The `Bundle` resource.
    type Bundle: DeserializeOwned + std::fmt::Debug + Default;

    /// The `CapabilityStatement` resource returned by `GET [base]/metadata`.
    type CapabilityStatement: DeserializeOwned + std::fmt::Debug;

    /// The `OperationOutcome` resource servers return to describe errors.
    type OperationOutcome: DeserializeOwned + std::fmt::Debug;
}
