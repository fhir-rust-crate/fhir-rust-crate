//! Practitioner
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Practitioner
//!
//! Version: 5.0.0
//!
//! Practitioner Resource: A person who is directly or indirectly involved in the provisioning of healthcare or related services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A person who is directly or indirectly involved in the provisioning of
/// healthcare or related services. This resource is used to capture information
/// about a person who is involved in the provision of care, such as a physician,
/// nurse, pharmacist, or administrative staff, independent of any particular
/// role or organization they act on behalf of. Roles and organizational
/// affiliations are represented separately via PractitionerRole.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::practitioner::Practitioner;
///
/// let value = Practitioner::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Practitioner = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Practitioner {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content
    pub language: Option<types::Code>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// An identifier for the person as this agent
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this practitioner's record is in active use
    pub active: Option<types::Boolean>,

    /// The name(s) associated with the practitioner
    pub name: Option<Vec<types::HumanName>>,

    /// A contact detail for the practitioner (that apply to all roles)
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<types::Code>,

    /// The date  on which the practitioner was born
    pub birth_date: Option<types::Date>,

    /// Indicates if the practitioner is deceased or not
    pub deceased_boolean: Option<types::Boolean>,

    /// Indicates if the practitioner is deceased or not
    pub deceased_date_time: Option<types::DateTime>,

    /// Address(es) of the practitioner that are not role specific (typically home address)
    pub address: Option<Vec<types::Address>>,

    /// Image of the person
    pub photo: Option<Vec<types::Attachment>>,

    /// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
    pub qualification: Option<Vec<PractitionerQualification>>,

    /// A language which may be used to communicate with the practitioner
    pub communication: Option<Vec<PractitionerCommunication>>,
}

/// Qualifications, certifications, accreditations, licenses, training, etc.
/// pertaining to the provision of care held by the practitioner.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerQualification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// An identifier for this qualification for the practitioner
    pub identifier: Option<Vec<types::Identifier>>,

    /// Coded representation of the qualification
    pub code: types::CodeableConcept,

    /// Period during which the qualification is valid
    pub period: Option<types::Period>,

    /// Organization that regulates and issues the qualification
    pub issuer: Option<types::Reference>,
}

/// A language which may be used to communicate with the practitioner about their
/// care, along with an indication of preference.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The language code used to communicate with the practitioner
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Practitioner;

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
