//! Support for the generated resource/datatype builders.
//!
//! `#[derive(fhir_derive_macros::Builder)]` generates a `<Type>Builder` with a
//! chainable setter per field and a `build()` that returns
//! [`Result<T, BuilderError>`], failing if a required (`1..1`) field was not set.
//! Optional (`0..1`) and repeating fields default to absent/empty.
//!
//! ```
//! use fhir::r5::resources::Observation;
//! use fhir::r5::coded::Coded;
//! use fhir::r5::codes::ObservationStatus;
//! use fhir::r5::types::CodeableConcept;
//!
//! // `status` and `code` are required (1..1); omitting them fails build().
//! assert!(Observation::builder().build().is_err());
//!
//! let obs = Observation::builder()
//!     .status(Coded::Known(ObservationStatus::Final))
//!     .code(CodeableConcept::default())
//!     .build()
//!     .expect("both required fields set");
//! assert!(matches!(obs.status, Coded::Known(ObservationStatus::Final)));
//! ```

use std::fmt;

/// Error returned by a builder's `build()` when a required field is missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuilderError {
    /// The name of the required field that was not set.
    pub missing_field: String,
}

impl BuilderError {
    /// A missing required field.
    #[must_use]
    pub fn missing(field: &str) -> Self {
        Self { missing_field: field.to_string() }
    }
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "required field `{}` was not set", self.missing_field)
    }
}

impl std::error::Error for BuilderError {}
