//! ActorDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ActorDefinition
//!
//! Version: 5.0.0
//!
//! ActorDefinition Resource: The ActorDefinition resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The ActorDefinition resource is used to describe an actor - a human or an
/// application that plays a role in data exchange, and that may have
/// obligations associated with the role the actor plays. Actors are the
/// participants in an exchange of information, and an ActorDefinition provides
/// a canonical, referenceable description of one such participant. It is
/// commonly used within implementation guides to formally define the systems
/// and people that a specification expects to interact.
///
/// An ActorDefinition does not describe a specific individual, organization,
/// or software product; rather it defines a role or category of participant
/// (for example a "client" or "server" system, or a "patient" or "provider"
/// person) that other conformance resources, such as CapabilityStatement or
/// StructureDefinition-based profiles, can reference. Implementation guide
/// authors use ActorDefinition to give each participant in an interoperability
/// scenario a stable, citable identity, and to document the expected
/// capabilities, behaviors, and constraints associated with fulfilling that
/// role. The `type` field distinguishes whether the actor is a `person` or a
/// `system`, and narrative fields such as `documentation` and `reference`
/// describe the actor's responsibilities and point implementers to further
/// guidance.
///
/// # Related resources
///
/// ActorDefinition is a conformance/terminology-infrastructure resource. It is
/// commonly referenced alongside `CapabilityStatement` (via `capabilities`)
/// and other canonical definitional resources within an implementation guide,
/// and it uses shared types such as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Identifier`](crate::r5::types::Identifier), and
/// [`ContactDetail`](crate::r5::types::ContactDetail) for metadata. It is
/// distinct from resources that describe real-world participants, such as
/// [`Patient`](crate::r5::resources::patient::Patient) or `Practitioner`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::actor_definition::ActorDefinition;
///
/// let value = ActorDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ActorDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActorDefinition {
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

    /// Canonical identifier for this actor definition, represented as a URI (globally unique), used to reference this actor from other resources
    pub url: Option<types::Uri>,

    /// Additional identifier for the actor definition (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the actor definition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this actor definition (computer friendly)
    pub name: Option<types::String>,

    /// Name for this actor definition (human friendly)
    pub title: Option<types::String>,

    /// The publication status of this actor definition: draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the actor
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for actor definition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this actor definition is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// Whether the actor represents a human participant or a software system: person | system
    pub r#type: types::Code,

    /// Functionality associated with the actor
    pub documentation: Option<types::Markdown>,

    /// Reference to more information about the actor
    pub reference: Option<Vec<types::Url>>,

    /// Canonical reference to a CapabilityStatement describing the actor's expected behavior (if applicable)
    pub capabilities: Option<types::Canonical>,

    /// Definition of this actor in another context / IG
    pub derived_from: Option<Vec<types::Canonical>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ActorDefinition;

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
