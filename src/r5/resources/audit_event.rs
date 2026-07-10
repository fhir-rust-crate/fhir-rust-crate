//! AuditEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
//!
//! Version: 5.0.0
//!
//! AuditEvent Resource: A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// AuditEvent records an event that is relevant for purposes such as
/// operations, privacy, security, maintenance, and performance analysis.
///
/// A typical AuditEvent captures who (agents) did what (code/action) to which
/// data (entities), when it occurred, where it was reported from (source), and
/// whether it succeeded (outcome). It is the FHIR R5 representation of a
/// security or activity audit log entry, commonly used to satisfy regulatory
/// accountability and traceability requirements.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::audit_event::AuditEvent;
///
/// let value = AuditEvent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AuditEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent {
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

    /// Type/identifier of event
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Specific type of event
    pub code: types::CodeableConcept,

    /// Type of action performed during the event
    pub action: Option<types::Code>,

    /// emergency | alert | critical | error | warning | notice | informational | debug
    pub severity: Option<types::Code>,

    /// When the activity occurred (Period)
    pub occurred_period: Option<types::Period>,

    /// When the activity occurred (dateTime)
    pub occurred_date_time: Option<types::DateTime>,

    /// Time when the event was recorded
    pub recorded: types::Instant,

    /// Whether the event succeeded or failed
    pub outcome: Option<AuditEventOutcome>,

    /// Authorization related to the event
    pub authorization: Option<Vec<types::CodeableConcept>>,

    /// Workflow authorization within which this event occurred
    pub based_on: Option<Vec<types::Reference>>,

    /// The patient is the subject of the data used/created/updated/deleted during the activity
    pub patient: Option<types::Reference>,

    /// Encounter within which this event occurred or which the event is tightly associated
    pub encounter: Option<types::Reference>,

    /// Actor involved in the event
    pub agent: Vec<AuditEventAgent>,

    /// Audit Event Reporter
    pub source: AuditEventSource,

    /// Data or objects used
    pub entity: Option<Vec<AuditEventEntity>>,
}

/// Whether the event succeeded or failed.
///
/// Indicates the outcome of the audited event, using a coded value plus
/// optional additional detail describing the result.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventOutcome {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether the event succeeded or failed
    pub code: types::Coding,

    /// Additional outcome detail
    pub detail: Option<Vec<types::CodeableConcept>>,
}

/// Actor involved in the event.
///
/// An agent is a person, organization, device, or software that participated in
/// the audited activity, described by its role, identity, location, and network
/// access point.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventAgent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// How agent participated
    pub r#type: Option<types::CodeableConcept>,

    /// Agent role in the event
    pub role: Option<Vec<types::CodeableConcept>>,

    /// Identifier of who
    pub who: types::Reference,

    /// Whether user is initiator
    pub requestor: Option<types::Boolean>,

    /// The agent location when the event occurred
    pub location: Option<types::Reference>,

    /// Policy that authorized the agent participation in the event
    pub policy: Option<Vec<types::Uri>>,

    /// This agent network location for the activity (Reference)
    pub network_reference: Option<types::Reference>,

    /// This agent network location for the activity (uri)
    pub network_uri: Option<types::Uri>,

    /// This agent network location for the activity (string)
    pub network_string: Option<types::String>,

    /// Allowable authorization for this agent
    pub authorization: Option<Vec<types::CodeableConcept>>,
}

/// Audit Event Reporter.
///
/// Identifies the system or application that detected and reported the audited
/// event, including its logical site, the observing entity, and the type of
/// source.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventSource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Logical source location within the enterprise
    pub site: Option<types::Reference>,

    /// The identity of source detecting the event
    pub observer: types::Reference,

    /// The type of source where event originated
    pub r#type: Option<Vec<types::CodeableConcept>>,
}

/// Data or objects used.
///
/// An entity describes a specific piece of data or an object that was involved
/// in the audited activity, including its role, security labels, query
/// parameters, and additional key/value details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Specific instance of resource
    pub what: Option<types::Reference>,

    /// What role the entity played
    pub role: Option<types::CodeableConcept>,

    /// Security labels on the entity
    pub security_label: Option<Vec<types::CodeableConcept>>,

    /// Query parameters
    pub query: Option<types::Base64Binary>,

    /// Additional Information about the entity
    pub detail: Option<Vec<AuditEventEntityDetail>>,

    /// Entity is attributed to this agent
    pub agent: Option<Vec<AuditEventAgent>>,
}

/// Additional Information about the entity.
///
/// A name/value pair providing extra descriptive information about the entity,
/// where the value may be one of several data types.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventEntityDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name of the property
    pub r#type: types::CodeableConcept,

    /// Property value (Quantity)
    pub value_quantity: Option<types::Quantity>,

    /// Property value (CodeableConcept)
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Property value (string)
    pub value_string: Option<types::String>,

    /// Property value (boolean)
    pub value_boolean: Option<types::Boolean>,

    /// Property value (integer)
    pub value_integer: Option<types::Integer>,

    /// Property value (Range)
    pub value_range: Option<types::Range>,

    /// Property value (Ratio)
    pub value_ratio: Option<types::Ratio>,

    /// Property value (time)
    pub value_time: Option<types::Time>,

    /// Property value (dateTime)
    pub value_date_time: Option<types::DateTime>,

    /// Property value (Period)
    pub value_period: Option<types::Period>,

    /// Property value (base64Binary)
    pub value_base64_binary: Option<types::Base64Binary>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AuditEvent;

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
