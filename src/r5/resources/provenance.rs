//! Provenance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Provenance
//!
//! Version: 5.0.0
//!
//! Provenance Resource: record describing entities and processes involved in producing and delivering or otherwise influencing a resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Provenance of a resource is a record that describes entities and processes
/// involved in producing and delivering or otherwise influencing that resource.
///
/// Provenance provides a critical foundation for assessing authenticity,
/// enabling trust, and allowing reproducibility. Provenance assertions are a
/// form of contextual metadata and can themselves become important records
/// with their own provenance. In FHIR R5 this resource is commonly used to
/// track who did what, when, and why for the resources it targets.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::provenance::Provenance;
///
/// let value = Provenance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Provenance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Provenance {
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

    /// Target Reference(s) (usually version specific)
    pub target: Vec<types::Reference>,

    /// When the activity occurred
    pub occurred_period: Option<types::Period>,

    /// When the activity occurred
    pub occurred_date_time: Option<types::DateTime>,

    /// When the activity was recorded / updated
    pub recorded: Option<types::Instant>,

    /// Policy or plan the activity was defined by
    pub policy: Option<Vec<types::Uri>>,

    /// Where the activity occurred, if relevant
    pub location: Option<types::Reference>,

    /// Authorization (purposeOfUse) related to the event
    pub authorization: Option<Vec<types::CodeableReference>>,

    /// Activity that occurred
    pub activity: Option<types::CodeableConcept>,

    /// Workflow authorization within which this event occurred
    pub based_on: Option<Vec<types::Reference>>,

    /// The patient is the subject of the data created/updated (.target) by the activity
    pub patient: Option<types::Reference>,

    /// Encounter within which this event occurred or which the event is tightly associated
    pub encounter: Option<types::Reference>,

    /// Actor involved
    pub agent: Vec<ProvenanceAgent>,

    /// An entity used in this activity
    pub entity: Option<Vec<ProvenanceEntity>>,

    /// Signature on target
    pub signature: Option<Vec<types::Signature>>,
}

/// Actor involved in the activity described by the Provenance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceAgent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// How the agent participated
    pub r#type: Option<types::CodeableConcept>,

    /// What the agents role was
    pub role: Option<Vec<types::CodeableConcept>>,

    /// The agent that participated in the event
    pub who: types::Reference,

    /// The agent that delegated
    pub on_behalf_of: Option<types::Reference>,
}

/// An entity used in the activity described by the Provenance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// revision | quotation | source | instantiates | removal
    pub role: types::Code,

    /// Identity of entity
    pub what: types::Reference,

    /// Entity is attributed to this agent
    pub agent: Option<Vec<ProvenanceAgent>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Provenance;

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
