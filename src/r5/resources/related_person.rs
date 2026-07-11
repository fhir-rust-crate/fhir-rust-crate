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
use fhir_derive_macros::Validate;

/// Information about a person that is involved in a patient's health or the care
/// for a patient, but who is not the target of healthcare, nor has a formal
/// responsibility in the care process.
///
/// A RelatedPerson resource captures a person such as a parent, spouse, guardian,
/// neighbour, friend, or informal caregiver who has a personal or non-professional
/// relationship to a patient. Unlike a practitioner, this person is not a member of
/// the care team and has no formal clinical or legal responsibility for the care
/// process, yet is often the party who accompanies the patient, provides history,
/// acts as an emergency contact, or serves as an interpreter. The resource records
/// the person's identity, demographics, contact details, addresses, and the nature
/// and validity period of their relationship to the patient, so that a system can
/// represent and reference them without conflating them with a patient or a
/// clinician.
///
/// In FHIR R5 a RelatedPerson is anchored to a single patient through its required
/// `patient` reference, and is commonly referenced from workflow resources such as
/// Encounter, Appointment, CarePlan, and Communication wherever a non-clinician
/// party participates in or is contacted about the patient's care. The relationship
/// type and any preferred languages are conveyed with coded concepts.
///
/// # Related resources
///
/// See also [`Patient`](crate::r5::resources::patient::Patient) for the individual
/// this person is related to, and [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// for how the relationship and communication language are coded. For members of the
/// care team who do have a formal clinical role, use the `Practitioner` resource
/// instead.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::related_person::RelatedPerson;
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
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Business identifier(s) assigned to this related person, distinct from any identifiers held for the patient
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this related person's record is currently in active use for care coordination
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Required reference to the patient this person is related to, anchoring the record to a single patient
    pub patient: types::Reference,

    /// Coded nature of the relationship to the patient, such as parent, spouse, guardian, or emergency contact
    pub relationship: Option<Vec<types::CodeableConcept>>,

    /// Name(s) by which this related person is known, using the same `HumanName` structure as on `Patient`
    pub name: Option<Vec<types::HumanName>>,

    /// A contact detail for the person
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<types::Code>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`).
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// The date on which the related person was born
    pub birth_date: Option<types::Date>,
    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR `_birthDate`).
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

    /// Address where the related person can be contacted or visited
    pub address: Option<Vec<types::Address>>,

    /// Image of the person
    pub photo: Option<Vec<types::Attachment>>,

    /// Period of time during which this relationship to the patient is considered valid and in effect
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
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`).
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,
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
