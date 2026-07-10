//! Person
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Person
//!
//! Version: 5.0.0
//!
//! Person Resource: Demographics and administrative information about a person independent of a specific health-related context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Demographics and administrative information about a person independent of a
/// specific health-related context.
///
/// A Person resource captures the identity and demographic details of an
/// individual who may play one or more roles in the healthcare system, such as
/// a patient, practitioner, or related person. It provides a way to link the
/// several records that concern the same actual human being across different
/// contexts. In FHIR R5 it commonly carries names, identifiers, contact
/// details, and links to Patient, Practitioner, or RelatedPerson resources.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::person::Person;
///
/// let value = Person::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Person = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Person {
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

    /// This person's record is in active use
    pub active: Option<types::Boolean>,

    /// A name associated with the person
    pub name: Option<Vec<types::HumanName>>,

    /// A contact detail for the person
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<types::Code>,

    /// The date on which the person was born
    pub birth_date: Option<types::Date>,

    /// Indicates if the individual is deceased or not
    pub deceased_boolean: Option<types::Boolean>,

    /// Indicates if the individual is deceased or not
    pub deceased_date_time: Option<types::DateTime>,

    /// One or more addresses for the person
    pub address: Option<Vec<types::Address>>,

    /// Marital (civil) status of a person
    pub marital_status: Option<types::CodeableConcept>,

    /// Image of the person
    pub photo: Option<Vec<types::Attachment>>,

    /// A language which may be used to communicate with the person about his or her health
    pub communication: Option<Vec<PersonCommunication>>,

    /// The organization that is the custodian of the person record
    pub managing_organization: Option<types::Reference>,

    /// Link to a resource that concerns the same actual person
    pub link: Option<Vec<PersonLink>>,
}

/// A language which may be used to communicate with the person about his or her
/// health.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PersonCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The language which can be used to communicate with the person about his or her health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
}

/// Link to a resource that concerns the same actual person.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PersonLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The resource to which this actual person is associated
    pub target: types::Reference,

    /// level1 | level2 | level3 | level4
    pub assurance: Option<types::Code>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Person;

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
