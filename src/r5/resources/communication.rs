//! Communication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Communication
//!
//! Version: 5.0.0
//!
//! Communication Resource: A clinical or business level record of information being transmitted or shared.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A clinical or business level record of information being transmitted or
/// shared.
///
/// The Communication resource captures a discrete unit of information that was
/// conveyed between a sender and one or more recipients, such as an alert sent
/// to a responsible provider or a public health agency communication issued in
/// response to a reportable condition. It records the status, category,
/// medium, timing, and payload of the exchange, along with the parties
/// involved and the subject or encounter it concerns. In FHIR R5 it is
/// commonly used to document notifications, messages, and other information
/// transfers that are relevant to patient care or business workflow.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::communication::Communication;
///
/// let value = Communication::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Communication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Communication {
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

    /// Unique identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// Request fulfilled by this communication
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event (e.g. Communication, Procedure)
    pub part_of: Option<Vec<types::Reference>>,

    /// Reply to
    pub in_response_to: Option<Vec<types::Reference>>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: types::Code,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Message category
    pub category: Option<Vec<types::CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// A channel of communication
    pub medium: Option<Vec<types::CodeableConcept>>,

    /// Focus of message
    pub subject: Option<types::Reference>,

    /// Description of the purpose/content
    pub topic: Option<types::CodeableConcept>,

    /// Resources that pertain to this communication
    pub about: Option<Vec<types::Reference>>,

    /// The Encounter during which this Communication was created
    pub encounter: Option<types::Reference>,

    /// When sent
    pub sent: Option<types::DateTime>,

    /// When received
    pub received: Option<types::DateTime>,

    /// Who the information is shared with
    pub recipient: Option<Vec<types::Reference>>,

    /// Who shares the information
    pub sender: Option<types::Reference>,

    /// Indication for message
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Message payload
    pub payload: Option<Vec<CommunicationPayload>>,

    /// Comments made about the communication
    pub note: Option<Vec<types::Annotation>>,
}

/// Message payload.
///
/// Text, attachment(s), or resource(s) that was communicated to the recipient
/// as part of the Communication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationPayload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Message part content
    pub content_attachment: Option<types::Attachment>,

    /// Message part content
    pub content_reference: Option<types::Reference>,

    /// Message part content
    pub content_codeable_concept: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Communication;

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
