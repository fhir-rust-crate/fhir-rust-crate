//! Identifier
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Identifier
//!
//! Version: 5.0.0
//!
//! Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Identifier is a FHIR R5 complex datatype that identifies some entity
/// uniquely and unambiguously. It is most commonly used to carry business
/// identifiers such as medical record numbers, order numbers, and driver's
/// license numbers, distinguishing them from the technical logical identifiers
/// assigned by FHIR servers. Each Identifier can pin the value to a namespace
/// via `system`, classify its role via `use` and `type`, and record the period
/// of validity and the organization that assigned it.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::identifier::Identifier;
///
/// let value = Identifier::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Identifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// usual | official | temp | secondary | old (If known)
    pub r#use: Option<types::Code>,

    /// Description of identifier
    pub r#type: Option<types::CodeableConcept>,

    /// The namespace for the identifier value
    pub system: Option<types::Uri>,

    /// The value that is unique
    pub value: Option<types::String>,

    /// Time period when id is/was valid for use
    pub period: Option<types::Period>,

    /// Organization that issued id (may be just text)
    pub assigner: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Identifier;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
