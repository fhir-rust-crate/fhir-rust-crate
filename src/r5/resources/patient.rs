//! Patient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Patient
//!
//! Version: 5.0.0
//!
//! Patient Resource: Demographics and other administrative information about an individual or animal receiving care or other health-related services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Demographics and other administrative information about an individual or
/// animal receiving care or other health-related services.
///
/// The Patient resource covers data about patients and animals involved in a
/// wide range of health-related activities, including curative activities,
/// psychiatric care, social services, pregnancy care, nursing and assisted
/// living, dietary services, and tracking of personal health and exercise data.
/// It is one of the most frequently referenced resources in FHIR R5 and is used
/// as a subject or focus by most clinical and administrative resources.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::patient::Patient;
///
/// let value = Patient::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Patient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
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

    /// An identifier for this patient
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this patient's record is in active use
    pub active: Option<types::Boolean>,

    /// A name associated with the patient
    pub name: Option<Vec<types::HumanName>>,

    /// A contact detail for the individual
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<types::Code>,

    /// The date of birth for the individual
    pub birth_date: Option<types::Date>,

    /// Indicates if the individual is deceased or not
    pub deceased_boolean: Option<types::Boolean>,

    /// Indicates if the individual is deceased or not
    pub deceased_date_time: Option<types::DateTime>,

    /// An address for the individual
    pub address: Option<Vec<types::Address>>,

    /// Marital (civil) status of a patient
    pub marital_status: Option<types::CodeableConcept>,

    /// Whether patient is part of a multiple birth
    pub multiple_birth_boolean: Option<types::Boolean>,

    /// Whether patient is part of a multiple birth
    pub multiple_birth_integer: Option<types::Integer>,

    /// Image of the patient
    pub photo: Option<Vec<types::Attachment>>,

    /// A contact party (e.g. guardian, partner, friend) for the patient
    pub contact: Option<Vec<PatientContact>>,

    /// A language which may be used to communicate with the patient about his or her health
    pub communication: Option<Vec<PatientCommunication>>,

    /// Patient's nominated primary care provider
    pub general_practitioner: Option<Vec<types::Reference>>,

    /// Organization that is the custodian of the patient record
    pub managing_organization: Option<types::Reference>,

    /// Link to a Patient or RelatedPerson resource that concerns the same actual individual
    pub link: Option<Vec<PatientLink>>,
}

/// A contact party (e.g. guardian, partner, friend) for the patient.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PatientContact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kind of relationship
    pub relationship: Option<Vec<types::CodeableConcept>>,

    /// A name associated with the contact person
    pub name: Option<types::HumanName>,

    /// A contact detail for the person
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// Address for the contact person
    pub address: Option<types::Address>,

    /// male | female | other | unknown
    pub gender: Option<types::Code>,

    /// Organization that is associated with the contact
    pub organization: Option<types::Reference>,

    /// The period during which this contact person or organization is valid to be contacted relating to this patient
    pub period: Option<types::Period>,
}

/// A language which may be used to communicate with the patient about his or
/// her health.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PatientCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The language which can be used to communicate with the patient about his or her health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
}

/// Link to a Patient or RelatedPerson resource that concerns the same actual
/// individual.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PatientLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The other patient or related person resource that the link refers to
    pub other: types::Reference,

    /// replaced-by | replaces | refer | seealso
    pub r#type: types::Code,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Patient;

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
