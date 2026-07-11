//! EventDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EventDefinition
//!
//! Version: 5.0.0
//!
//! EventDefinition Resource: The EventDefinition resource provides a reusable description of when a particular event can occur.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The EventDefinition resource provides a reusable description of when a
/// particular event can occur, defined in terms of one or more triggers.
/// It is used to describe events that may be referenced by other knowledge
/// artifacts such as plan definitions and clinical guidelines. Each event
/// definition captures the conditions under which the event fires along with
/// the metadata needed to publish and maintain it as a reusable resource.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::event_definition::EventDefinition;
///
/// let value = EventDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EventDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
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

    /// Canonical identifier for this event definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the event definition
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the event definition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this event definition (computer friendly)
    pub name: Option<types::String>,

    /// Name for this event definition (human friendly)
    pub title: Option<types::String>,

    /// Subordinate title of the event definition
    pub subtitle: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Type of individual the event definition is focused on
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// Type of individual the event definition is focused on
    pub subject_reference: Option<types::Reference>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the event definition
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for event definition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this event definition is defined
    pub purpose: Option<types::Markdown>,

    /// Describes the clinical usage of the event definition
    pub usage: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the event definition was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the event definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the event definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// "when" the event occurs (multiple = 'or')
    pub trigger: Vec<types::TriggerDefinition>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EventDefinition;

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
