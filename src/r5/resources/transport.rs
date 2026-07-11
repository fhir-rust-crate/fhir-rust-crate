//! Transport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Transport
//!
//! Version: 5.0.0
//!
//! Transport Resource: Record of transport.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Transport
///
/// Record of transport of item. A Transport resource tracks the movement of an
/// item (such as a specimen, a device, or a patient) from one location to
/// another, capturing who requested it, its current status, the desired and
/// current locations, and any inputs and outputs associated with performing the
/// transport. It is used in FHIR R5 to coordinate and audit logistics workflows,
/// such as moving a specimen from a collection site to a laboratory, relocating
/// equipment between departments, or transporting a patient between care areas.
/// Transport is a "worker" resource similar in spirit to `Task`: it is created,
/// assigned to an owner, tracked through its lifecycle (from `in-progress` to
/// `completed`, `abandoned`, or `cancelled`), and may reference the request it
/// fulfills via `basedOn`, as well as capture the requester, the responsible
/// performer, and the reason the transport is needed.
///
/// # Related resources
///
/// The entity being moved, or the beneficiary of the transport, is commonly
/// referenced through `focus` or `for`, which may point at a
/// [`Patient`](crate::r5::resources::patient::Patient), a `Specimen`, or a
/// `Device`. Classification and status details are typically expressed using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values such as
/// `code`, `statusReason`, and `performerType`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::transport::Transport;
///
/// let value = Transport::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Transport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
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

    /// External identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Formal definition of transport
    pub instantiates_canonical: Option<types::Canonical>,

    /// Formal definition of transport
    pub instantiates_uri: Option<types::Uri>,

    /// Request fulfilled by this transport
    pub based_on: Option<Vec<types::Reference>>,

    /// Requisition or grouper id
    pub group_identifier: Option<types::Identifier>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// Current lifecycle state: in-progress | completed | abandoned | cancelled | planned | entered-in-error
    pub status: Option<types::Code>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: types::Code,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// Transport Type
    pub code: Option<types::CodeableConcept>,

    /// Human-readable explanation of transport
    pub description: Option<types::String>,

    /// The item, resource, or entity that the transport is moving or acting on
    pub focus: Option<types::Reference>,

    /// The individual for whose benefit the transport is being performed, such as a patient
    pub r#for: Option<types::Reference>,

    /// Healthcare event during which this transport originated
    pub encounter: Option<types::Reference>,

    /// Completion time of the event (the occurrence)
    pub completion_time: Option<types::DateTime>,

    /// Transport Creation Date
    pub authored_on: Option<types::DateTime>,

    /// Transport Last Modified Date
    pub last_modified: Option<types::DateTime>,

    /// Who is asking for transport to be done
    pub requester: Option<types::Reference>,

    /// The kind of participant expected to carry out the transport
    pub performer_type: Option<Vec<types::CodeableConcept>>,

    /// The individual or organization responsible for carrying out the transport
    pub owner: Option<types::Reference>,

    /// The principal physical location where the transport is performed
    pub location: Option<types::Reference>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<types::Reference>>,

    /// Comments made about the transport
    pub note: Option<Vec<types::Annotation>>,

    /// Key events in history of the Transport
    pub relevant_history: Option<Vec<types::Reference>>,

    /// Constraints on fulfillment transports
    pub restriction: Option<TransportRestriction>,

    /// Information used to perform transport
    pub input: Option<Vec<TransportInput>>,

    /// Information produced as part of transport
    pub output: Option<Vec<TransportOutput>>,

    /// The location to which the focus of the transport should be moved
    pub requested_location: types::Reference,

    /// The current physical location of the focus of the transport
    pub current_location: types::Reference,

    /// The clinical or administrative reason the transport is needed
    pub reason: Option<types::CodeableReference>,

    /// Parent (or preceding) transport
    pub history: Option<types::Reference>,
}

/// Transport.restriction
///
/// Constraints on fulfillment transports. If the Transport.focus is a request
/// resource and the transport is seeking fulfillment, this element identifies any
/// limitations on what parts of the referenced request should be actioned.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransportRestriction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// How many times to repeat
    pub repetitions: Option<types::PositiveInt>,

    /// When fulfillment sought
    pub period: Option<types::Period>,

    /// For whom is fulfillment sought?
    pub recipient: Option<Vec<types::Reference>>,
}

/// Transport.input
///
/// Information used to perform transport. Additional information that may be
/// needed in the execution of the transport, expressed as a labelled, typed value.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransportInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for the input
    pub r#type: types::CodeableConcept,

    /// Content to use in performing the transport
    pub value_base64_binary: Option<types::Base64Binary>,

    /// Content to use in performing the transport
    pub value_boolean: Option<types::Boolean>,

    /// Content to use in performing the transport
    pub value_canonical: Option<types::Canonical>,

    /// Content to use in performing the transport
    pub value_code: Option<types::Code>,

    /// Content to use in performing the transport
    pub value_date: Option<types::Date>,

    /// Content to use in performing the transport
    pub value_date_time: Option<types::DateTime>,

    /// Content to use in performing the transport
    pub value_decimal: Option<types::Decimal>,

    /// Content to use in performing the transport
    pub value_id: Option<types::Id>,

    /// Content to use in performing the transport
    pub value_instant: Option<types::Instant>,

    /// Content to use in performing the transport
    pub value_integer: Option<types::Integer>,

    /// Content to use in performing the transport
    pub value_integer64: Option<types::Integer64>,

    /// Content to use in performing the transport
    pub value_markdown: Option<types::Markdown>,

    /// Content to use in performing the transport
    pub value_oid: Option<types::Oid>,

    /// Content to use in performing the transport
    pub value_positive_int: Option<types::PositiveInt>,

    /// Content to use in performing the transport
    pub value_string: Option<types::String>,

    /// Content to use in performing the transport
    pub value_time: Option<types::Time>,

    /// Content to use in performing the transport
    pub value_unsigned_int: Option<types::UnsignedInt>,

    /// Content to use in performing the transport
    pub value_uri: Option<types::Uri>,

    /// Content to use in performing the transport
    pub value_url: Option<types::Url>,

    /// Content to use in performing the transport
    pub value_uuid: Option<types::Uuid>,

    /// Content to use in performing the transport
    pub value_address: Option<types::Address>,

    /// Content to use in performing the transport
    pub value_age: Option<types::Age>,

    /// Content to use in performing the transport
    pub value_annotation: Option<types::Annotation>,

    /// Content to use in performing the transport
    pub value_attachment: Option<types::Attachment>,

    /// Content to use in performing the transport
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Content to use in performing the transport
    pub value_codeable_reference: Option<types::CodeableReference>,

    /// Content to use in performing the transport
    pub value_coding: Option<types::Coding>,

    /// Content to use in performing the transport
    pub value_contact_point: Option<types::ContactPoint>,

    /// Content to use in performing the transport
    pub value_count: Option<types::Count>,

    /// Content to use in performing the transport
    pub value_distance: Option<types::Distance>,

    /// Content to use in performing the transport
    pub value_duration: Option<types::Duration>,

    /// Content to use in performing the transport
    pub value_human_name: Option<types::HumanName>,

    /// Content to use in performing the transport
    pub value_identifier: Option<types::Identifier>,

    /// Content to use in performing the transport
    pub value_money: Option<types::Money>,

    /// Content to use in performing the transport
    pub value_period: Option<types::Period>,

    /// Content to use in performing the transport
    pub value_quantity: Option<types::Quantity>,

    /// Content to use in performing the transport
    pub value_range: Option<types::Range>,

    /// Content to use in performing the transport
    pub value_ratio: Option<types::Ratio>,

    /// Content to use in performing the transport
    pub value_ratio_range: Option<types::RatioRange>,

    /// Content to use in performing the transport
    pub value_reference: Option<types::Reference>,

    /// Content to use in performing the transport
    pub value_sampled_data: Option<types::SampledData>,

    /// Content to use in performing the transport
    pub value_signature: Option<types::Signature>,

    /// Content to use in performing the transport
    pub value_timing: Option<types::Timing>,

    /// Content to use in performing the transport
    pub value_contact_detail: Option<types::ContactDetail>,

    /// Content to use in performing the transport
    pub value_data_requirement: Option<types::DataRequirement>,

    /// Content to use in performing the transport
    pub value_expression: Option<types::Expression>,

    /// Content to use in performing the transport
    pub value_parameter_definition: Option<types::ParameterDefinition>,

    /// Content to use in performing the transport
    pub value_related_artifact: Option<types::RelatedArtifact>,

    /// Content to use in performing the transport
    pub value_trigger_definition: Option<types::TriggerDefinition>,

    /// Content to use in performing the transport
    pub value_usage_context: Option<types::UsageContext>,

    /// Content to use in performing the transport
    pub value_availability: Option<types::Availability>,

    /// Content to use in performing the transport
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>,

    /// Content to use in performing the transport
    pub value_dosage: Option<types::Dosage>,

    /// Content to use in performing the transport
    pub value_meta: Option<types::Meta>,
}

/// Transport.output
///
/// Information produced as part of transport. Outputs produced by the Transport,
/// expressed as labelled, typed values.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransportOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for output
    pub r#type: types::CodeableConcept,

    /// Result of output
    pub value_base64_binary: Option<types::Base64Binary>,

    /// Result of output
    pub value_boolean: Option<types::Boolean>,

    /// Result of output
    pub value_canonical: Option<types::Canonical>,

    /// Result of output
    pub value_code: Option<types::Code>,

    /// Result of output
    pub value_date: Option<types::Date>,

    /// Result of output
    pub value_date_time: Option<types::DateTime>,

    /// Result of output
    pub value_decimal: Option<types::Decimal>,

    /// Result of output
    pub value_id: Option<types::Id>,

    /// Result of output
    pub value_instant: Option<types::Instant>,

    /// Result of output
    pub value_integer: Option<types::Integer>,

    /// Result of output
    pub value_integer64: Option<types::Integer64>,

    /// Result of output
    pub value_markdown: Option<types::Markdown>,

    /// Result of output
    pub value_oid: Option<types::Oid>,

    /// Result of output
    pub value_positive_int: Option<types::PositiveInt>,

    /// Result of output
    pub value_string: Option<types::String>,

    /// Result of output
    pub value_time: Option<types::Time>,

    /// Result of output
    pub value_unsigned_int: Option<types::UnsignedInt>,

    /// Result of output
    pub value_uri: Option<types::Uri>,

    /// Result of output
    pub value_url: Option<types::Url>,

    /// Result of output
    pub value_uuid: Option<types::Uuid>,

    /// Result of output
    pub value_address: Option<types::Address>,

    /// Result of output
    pub value_age: Option<types::Age>,

    /// Result of output
    pub value_annotation: Option<types::Annotation>,

    /// Result of output
    pub value_attachment: Option<types::Attachment>,

    /// Result of output
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Result of output
    pub value_codeable_reference: Option<types::CodeableReference>,

    /// Result of output
    pub value_coding: Option<types::Coding>,

    /// Result of output
    pub value_contact_point: Option<types::ContactPoint>,

    /// Result of output
    pub value_count: Option<types::Count>,

    /// Result of output
    pub value_distance: Option<types::Distance>,

    /// Result of output
    pub value_duration: Option<types::Duration>,

    /// Result of output
    pub value_human_name: Option<types::HumanName>,

    /// Result of output
    pub value_identifier: Option<types::Identifier>,

    /// Result of output
    pub value_money: Option<types::Money>,

    /// Result of output
    pub value_period: Option<types::Period>,

    /// Result of output
    pub value_quantity: Option<types::Quantity>,

    /// Result of output
    pub value_range: Option<types::Range>,

    /// Result of output
    pub value_ratio: Option<types::Ratio>,

    /// Result of output
    pub value_ratio_range: Option<types::RatioRange>,

    /// Result of output
    pub value_reference: Option<types::Reference>,

    /// Result of output
    pub value_sampled_data: Option<types::SampledData>,

    /// Result of output
    pub value_signature: Option<types::Signature>,

    /// Result of output
    pub value_timing: Option<types::Timing>,

    /// Result of output
    pub value_contact_detail: Option<types::ContactDetail>,

    /// Result of output
    pub value_data_requirement: Option<types::DataRequirement>,

    /// Result of output
    pub value_expression: Option<types::Expression>,

    /// Result of output
    pub value_parameter_definition: Option<types::ParameterDefinition>,

    /// Result of output
    pub value_related_artifact: Option<types::RelatedArtifact>,

    /// Result of output
    pub value_trigger_definition: Option<types::TriggerDefinition>,

    /// Result of output
    pub value_usage_context: Option<types::UsageContext>,

    /// Result of output
    pub value_availability: Option<types::Availability>,

    /// Result of output
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>,

    /// Result of output
    pub value_dosage: Option<types::Dosage>,

    /// Result of output
    pub value_meta: Option<types::Meta>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Transport;

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
