//! ExtendedContactDetail
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExtendedContactDetail
//!
//! Version: 5.0.0
//!
//! ExtendedContactDetail Type: Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The ExtendedContactDetail datatype specifies contact information for a
/// specific purpose over a period of time, such as an address or telecom
/// details that might be handled or monitored by a specific named person or
/// organization. It extends basic contact details with a purpose, a validity
/// period, and an associated responsible organization. It is commonly used in
/// FHIR R5 resources to convey rich, contextualized contact information.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::extended_contact_detail::ExtendedContactDetail;
///
/// let value = ExtendedContactDetail::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ExtendedContactDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedContactDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// The type of contact
    pub purpose: Option<types::CodeableConcept>,

    /// Name of an individual to contact
    pub name: Option<Vec<types::HumanName>>,

    /// Contact details (e.g.phone/fax/url)
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// Address for the contact
    pub address: Option<types::Address>,

    /// This contact detail is handled/monitored by a specific organization
    pub organization: Option<types::Reference>,

    /// Period that this contact was valid for usage
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExtendedContactDetail;

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
