//! NamingSystem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
//!
//! Version: 5.0.0
//!
//! NamingSystem Resource: A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, etc. It represents a "System"
/// used within the Identifier and Coding data types, and captures the metadata
/// about how a namespace is administered and which unique identifiers name it.
/// NamingSystem is used to register the systems that appear in coded and
/// identifier values throughout FHIR resources.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::naming_system::NamingSystem;
///
/// let value = NamingSystem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: NamingSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystem {
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

    /// Canonical identifier for this naming system, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the naming system (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the naming system
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this naming system (computer friendly)
    pub name: types::String,

    /// Title for this naming system (human friendly)
    pub title: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// codesystem | identifier | root
    pub kind: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: types::DateTime,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Who maintains system namespace?
    pub responsible: Option<types::String>,

    /// e.g. driver, provider, patient, bank etc
    pub r#type: Option<types::CodeableConcept>,

    /// Natural language description of the naming system
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for naming system (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this naming system is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the NamingSystem was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the NamingSystem was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the NamingSystem is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the CodeSystem
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the NamingSystem
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the NamingSystem
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the NamingSystem
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// How/where is it used
    pub usage: Option<types::String>,

    /// Unique identifiers used for system
    pub unique_id: Vec<NamingSystemUniqueId>,
}

/// Indicates how the system may be identified when referenced in electronic
/// exchange. Each NamingSystem may have multiple unique identifiers (for example
/// an OID and a URI), each of which names the same underlying system.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystemUniqueId {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// oid | uuid | uri | iri-stem | v2csmnemonic | other
    pub r#type: types::Code,

    /// The unique identifier
    pub value: types::String,

    /// Is this the id that should be used for this type
    pub preferred: Option<types::Boolean>,

    /// Notes about identifier usage
    pub comment: Option<types::String>,

    /// When is identifier valid?
    pub period: Option<types::Period>,

    /// Whether the identifier is authoritative
    pub authoritative: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = NamingSystem;

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
