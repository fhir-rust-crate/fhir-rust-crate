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
use fhir_derive_macros::Validate;

/// Provenance of a resource is a record that describes the entities and
/// processes involved in producing and delivering, or otherwise influencing,
/// that resource. It answers the essential questions of who did what, when,
/// where, why, and on whose authority, thereby documenting the origin and
/// custody chain of clinical and administrative information.
///
/// Provenance provides a critical foundation for assessing authenticity,
/// enabling trust, and allowing reproducibility. Provenance assertions are a
/// form of contextual metadata and can themselves become important records
/// with their own provenance. In FHIR R5, a Provenance resource points to one
/// or more target resources it describes, records the activity that occurred
/// and when, names the participating agents and their roles, references any
/// source entities that were revised or quoted, and may carry digital
/// signatures that attest to the target's integrity. Provenance is typically
/// created automatically by systems as resources are authored, amended, or
/// exchanged, and is used to support auditing, medico-legal accountability,
/// data quality assessment, and regulatory compliance.
///
/// Related resources: the [`AuditEvent`](crate::r5::resources::audit_event::AuditEvent)
/// resource records the technical details of security-relevant events and is
/// often used alongside Provenance, which emphasizes the meaning and
/// authorship of a resource's content. Agents and targets are expressed with
/// [`Reference`](crate::r5::types::Reference), activities and roles with
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and timing with
/// [`Period`](crate::r5::types::Period) and [`DateTime`](crate::r5::types::DateTime).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::provenance::Provenance;
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

    /// The resource(s) whose provenance is being described, usually referenced version-specifically so the assertion applies to an exact state.
    pub target: Vec<types::Reference>,

    /// The `Provenance.occurred[x]` choice element (0..1); see [`ProvenanceOccurred`].
    #[serde(flatten)]
    pub occurred: Option<ProvenanceOccurred>,

    /// The instant the provenance assertion was recorded or last updated, which may differ from when the activity itself occurred.
    pub recorded: Option<types::Instant>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`).
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Policy or plan the activity was defined by
    pub policy: Option<Vec<types::Uri>>,
    /// Primitive extension sibling for [`policy`](Self::policy) (FHIR `_policy`).
    #[serde(rename = "_policy")]
    pub policy_ext: Option<Vec<Option<types::Element>>>,

    /// Where the activity occurred, if relevant
    pub location: Option<types::Reference>,

    /// Authorization (purposeOfUse) related to the event
    pub authorization: Option<Vec<types::CodeableReference>>,

    /// The activity that is involved in producing the target, e.g. assemble, copy, transmit, or sign.
    pub activity: Option<types::CodeableConcept>,

    /// Workflow authorization within which this event occurred
    pub based_on: Option<Vec<types::Reference>>,

    /// The patient is the subject of the data created/updated (.target) by the activity
    pub patient: Option<types::Reference>,

    /// Encounter within which this event occurred or which the event is tightly associated
    pub encounter: Option<types::Reference>,

    /// The actors, human or system, that participated in the activity, each described by a ProvenanceAgent with its role; at least one is required.
    pub agent: Vec<ProvenanceAgent>,

    /// The source entities that were used, revised, quoted, or otherwise consumed by this activity, each described by a ProvenanceEntity.
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

    /// The individual, device, or organization that participated in the event, referenced e.g. via [`Patient`](crate::r5::resources::patient::Patient) or Practitioner.
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
    pub role: crate::r5::coded::Coded<crate::r5::codes::ProvenanceEntityRole>,
    /// Primitive extension sibling for [`role`](Self::role) (FHIR `_role`).
    #[serde(rename = "_role")]
    pub role_ext: Option<types::Element>,

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
/// The `Provenance.occurred[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ProvenanceOccurred {
    /// `occurredPeriod` variant.
    #[fhir("occurredPeriod")]
    Period(Box<types::Period>),
    /// `occurredDateTime` variant.
    #[fhir("occurredDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
