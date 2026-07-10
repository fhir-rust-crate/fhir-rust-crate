//! RelatedPerson
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RelatedPerson
//!
//! Version: 5.0.0
//!
//! RelatedPerson Resource: Information about a person that is involved in a patient's health or the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Information about a person that is involved in a patient's health or the care
/// for a patient, but who is not the target of healthcare, nor has a formal
/// responsibility in the care process.
///
/// A RelatedPerson resource captures a person such as a parent, spouse, guardian,
/// neighbour, or caregiver who has a personal or non-professional relationship to
/// a patient. It records their identity, demographics, contact details, and the
/// nature and validity period of their relationship to the patient. In FHIR R5 it
/// is commonly referenced from resources like Encounter, Appointment, and
/// Communication where a non-clinician party participates in the patient's care.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::related_person::RelatedPerson;
///
/// let value = RelatedPerson::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: RelatedPerson = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPerson {
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

    /// A human identifier for this person
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this related person's record is in active use
    pub active: Option<types::Boolean>,

    /// The patient this person is related to
    pub patient: types::Reference,

    /// The relationship of the related person to the patient
    pub relationship: Option<Vec<types::CodeableConcept>>,

    /// A name associated with the person
    pub name: Option<Vec<types::HumanName>>,

    /// A contact detail for the person
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<types::Code>,

    /// The date on which the related person was born
    pub birth_date: Option<types::Date>,

    /// Address where the related person can be contacted or visited
    pub address: Option<Vec<types::Address>>,

    /// Image of the person
    pub photo: Option<Vec<types::Attachment>>,

    /// Period of time that this relationship is considered valid
    pub period: Option<types::Period>,

    /// A language which may be used to communicate with the related person about the patient's health
    pub communication: Option<Vec<RelatedPersonCommunication>>,
}

/// A language which may be used to communicate with the related person about the
/// patient's health.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPersonCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The language which can be used to communicate with the related person about the patient's health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RelatedPerson;

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
