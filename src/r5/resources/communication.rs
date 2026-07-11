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
use fhir_derive_macros::Validate;

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
/// Unlike `CommunicationRequest`, which represents a request or order for a
/// communication to occur, `Communication` is a record that a communication
/// event has actually happened (or is happening), whether or not it was
/// triggered by an order. It is often used for auditing, tracking, and
/// reporting on information exchange, and it may reference the order that
/// prompted it via `based_on`.
///
/// # See also
///
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for
///   category, medium, priority, and topic coding.
/// - [`Reference`](crate::r5::types::Reference) — used to link the subject,
///   encounter, sender, and recipients, which are commonly resources such as
///   `Patient`, `Practitioner`, or `Organization`.
/// - [`Annotation`](crate::r5::types::Annotation) — used for the `note`
///   field's free-text comments.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::communication::Communication;
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

    /// Unique identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    pub instantiates_canonical_ext: Option<Vec<Option<types::Element>>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<Vec<Option<types::Element>>>,

    /// Request fulfilled by this communication
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event (e.g. Communication, Procedure)
    pub part_of: Option<Vec<types::Reference>>,

    /// Reply to
    pub in_response_to: Option<Vec<types::Reference>>,

    /// The lifecycle status of this communication: preparation | in-progress
    /// | not-done | on-hold | stopped | completed | entered-in-error | unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Message category
    pub category: Option<Vec<types::CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// A channel of communication
    pub medium: Option<Vec<types::CodeableConcept>>,

    /// Focus of message, typically a reference to a `Patient` or `Group`
    /// about whom the communication is concerned.
    pub subject: Option<types::Reference>,

    /// Description of the purpose/content
    pub topic: Option<types::CodeableConcept>,

    /// Resources that pertain to this communication
    pub about: Option<Vec<types::Reference>>,

    /// The Encounter during which this Communication was created
    pub encounter: Option<types::Reference>,

    /// When sent
    pub sent: Option<types::DateTime>,
    /// Primitive extension sibling for [`sent`](Self::sent) (FHIR `_sent`).
    #[serde(rename = "_sent")]
    pub sent_ext: Option<types::Element>,

    /// When received
    pub received: Option<types::DateTime>,
    /// Primitive extension sibling for [`received`](Self::received) (FHIR `_received`).
    #[serde(rename = "_received")]
    pub received_ext: Option<types::Element>,

    /// Who the information is shared with, such as one or more patients,
    /// practitioners, or other care team members.
    pub recipient: Option<Vec<types::Reference>>,

    /// Who shares the information, i.e. the originator of the communication.
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

    /// The `Communication.payload.content[x]` choice element (0..1); see [`CommunicationPayloadContent`].
    #[serde(flatten)]
    pub content: Option<CommunicationPayloadContent>,
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
/// The `Communication.payload.content[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CommunicationPayloadContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
    /// `contentCodeableConcept` variant.
    #[fhir("contentCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}
