//! A coded value that is usually a known enum variant but tolerates any code.
//!
//! FHIR elements with a `required` binding draw their value from a value set, so
//! this crate types them as the matching [`codes`](crate::r5::codes) enum. But
//! real data occasionally carries a code outside the set (a newer code, a local
//! extension, or simply invalid data), and a data-exchange library must not fail
//! to parse it. [`Coded<E>`] wraps the enum with an [`Unknown`](Coded::Unknown)
//! fallback so every wire value round-trips. See `spec/05-code-systems.md`.
//!
//! ```
//! use fhir::r5::coded::Coded;
//! use fhir::r5::codes::AdministrativeGender;
//!
//! // A known code parses into the enum variant.
//! let known: Coded<AdministrativeGender> =
//!     serde_json::from_value(serde_json::json!("female")).unwrap();
//! assert_eq!(known, Coded::Known(AdministrativeGender::Female));
//!
//! // An unrecognized code is preserved as-is rather than rejected.
//! let unknown: Coded<AdministrativeGender> =
//!     serde_json::from_value(serde_json::json!("robot")).unwrap();
//! assert_eq!(unknown, Coded::Unknown("robot".to_string()));
//!
//! // Both round-trip to their original code string.
//! assert_eq!(serde_json::to_value(&known).unwrap(), "female");
//! assert_eq!(serde_json::to_value(&unknown).unwrap(), "robot");
//! ```

use ::serde::{Deserialize, Serialize};

use crate::r5::validate::{Validate, ValidationIssue};

/// A coded value: a known [`codes`](crate::r5::codes) enum variant `E`, or any
/// other code string preserved verbatim.
///
/// Deserialization tries `E` first (untagged) and falls back to
/// [`Unknown`](Self::Unknown); serialization emits the underlying code string in
/// either case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Coded<E> {
    /// A recognized code from the bound value set.
    Known(E),
    /// A code outside the value set, preserved for round-tripping.
    Unknown(String),
}

impl<E: Default> Default for Coded<E> {
    fn default() -> Self {
        Coded::Known(E::default())
    }
}

impl<E> Coded<E> {
    /// The known enum variant, if this value is recognized.
    pub fn known(&self) -> Option<&E> {
        match self {
            Coded::Known(e) => Some(e),
            Coded::Unknown(_) => None,
        }
    }

    /// Whether this value is an unrecognized code.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Coded::Unknown(_))
    }
}

impl<E: Serialize> Coded<E> {
    /// The underlying FHIR code string (the enum's canonical code, or the raw
    /// unknown code).
    #[must_use]
    pub fn code(&self) -> String {
        match self {
            Coded::Known(e) => ::serde_json::to_value(e)
                .ok()
                .and_then(|v| v.as_str().map(str::to_string))
                .unwrap_or_default(),
            Coded::Unknown(s) => s.clone(),
        }
    }
}

impl<E> Validate for Coded<E> {
    // A recognized variant is valid by construction; membership checks for the
    // `Unknown` fallback belong to the validation-depth work (T13).
    fn validate(&self) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r5::codes::AdministrativeGender;

    #[test]
    fn known_and_unknown_roundtrip() {
        for (json, expect) in [
            (serde_json::json!("male"), Coded::Known(AdministrativeGender::Male)),
            (serde_json::json!("xyz"), Coded::Unknown("xyz".to_string())),
        ] {
            let parsed: Coded<AdministrativeGender> =
                serde_json::from_value(json.clone()).unwrap();
            assert_eq!(parsed, expect);
            assert_eq!(serde_json::to_value(&parsed).unwrap(), json);
        }
    }

    #[test]
    fn default_is_known() {
        assert_eq!(
            Coded::<AdministrativeGender>::default(),
            Coded::Known(AdministrativeGender::default())
        );
    }
}
