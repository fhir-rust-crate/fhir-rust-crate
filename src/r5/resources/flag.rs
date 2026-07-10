//! Flag
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Flag
//!
//! Version: 5.0.0
//!
//! Flag Resource: Prospective warnings of potential issues when providing care to the patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Prospective warnings of potential issues when providing care to a patient.
/// A Flag is a clinical or administrative alert — such as a special precaution,
/// behavioral concern, or care restriction — that a care provider needs to be
/// aware of when interacting with the subject of the flag. It carries a coded or
/// textual message, a status, an optional active period, and references to the
/// subject, relevant encounter, and author.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::flag::Flag;
///
/// let value = Flag::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Flag = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
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

    /// Business identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | inactive | entered-in-error
    pub status: types::Code,

    /// Clinical, administrative, etc
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Coded or textual message to display to user
    pub code: types::CodeableConcept,

    /// Who/What is flag about?
    pub subject: types::Reference,

    /// Time period when flag is active
    pub period: Option<types::Period>,

    /// Alert relevant during encounter
    pub encounter: Option<types::Reference>,

    /// Flag creator
    pub author: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Flag;

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
