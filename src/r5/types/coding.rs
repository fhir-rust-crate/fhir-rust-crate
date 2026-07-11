//! Coding
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coding
//!
//! Version: 5.0.0
//!
//! Coding Type: A reference to a code defined by a terminology system.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A Coding is a single, atomic value representing a code from a specific terminology system.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A reference to a code defined by a terminology system, capturing the code system,
/// version, code value, human-readable display text, and whether the code was picked
/// directly by a user. Coding is used within CodeableConcept and directly on elements
/// wherever a single coded value from a known system needs to be represented.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::coding::Coding;
///
/// let value = Coding::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Coding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Coding {
    /// The identification of the code system that defines the meaning of the symbol in the code.
    pub system: Option<types::Uri>,
    /// The version of the code system which was used when choosing this code.
    pub version: Option<types::String>,
    /// A symbol in the syntax defined by the code system, representing the specific code.
    pub code: Option<types::Code>, // « C »
    /// A human-readable representation of the meaning of the code, following the code system's rules.
    pub display: Option<String>,   // « C »
    /// Indicates that this coding was chosen by a user directly, rather than by an algorithm.
    pub user_selected: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coding;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            system: None,
            version: None,
            code: None,
            display: None,
            user_selected: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
