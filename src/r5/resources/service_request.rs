//! ServiceRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
//!
//! Version: 5.0.0
//!
//! ServiceRequest Resource: A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A record of a request for service such as diagnostic investigations,
/// treatments, or operations to be performed.
///
/// The ServiceRequest resource is used to place an order or request for a
/// service to be performed on or for a patient, group, device, or location.
/// Typical services include diagnostic tests, imaging studies, laboratory work,
/// referrals, counseling, and procedures. In FHIR R5 it carries the requested
/// code, subject, timing, requester, performer, and supporting clinical context,
/// and can be linked to fulfilling results and provenance.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::service_request::ServiceRequest;
///
/// let value = ServiceRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ServiceRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequest {
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

    /// Identifiers assigned to this order
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// What request fulfills
    pub based_on: Option<Vec<types::Reference>>,

    /// What request replaces
    pub replaces: Option<Vec<types::Reference>>,

    /// Composite Request ID
    pub requisition: Option<types::Identifier>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: types::Code,

    /// proposal | plan | directive | order +
    pub intent: types::Code,

    /// Classification of service
    pub category: Option<Vec<types::CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// True if service/procedure should not be performed
    pub do_not_perform: Option<types::Boolean>,

    /// What is being requested/ordered
    pub code: Option<types::CodeableReference>,

    /// Additional order information
    pub order_detail: Option<Vec<ServiceRequestOrderDetail>>,

    /// Service amount
    pub quantity_quantity: Option<types::Quantity>,

    /// Service amount
    pub quantity_ratio: Option<types::Ratio>,

    /// Service amount
    pub quantity_range: Option<types::Range>,

    /// Individual or Entity the service is ordered for
    pub subject: types::Reference,

    /// What the service request is about, when it is not about the subject of record
    pub focus: Option<Vec<types::Reference>>,

    /// Encounter in which the request was created
    pub encounter: Option<types::Reference>,

    /// When service should occur
    pub occurrence_date_time: Option<types::DateTime>,

    /// When service should occur
    pub occurrence_period: Option<types::Period>,

    /// When service should occur
    pub occurrence_timing: Option<types::Timing>,

    /// Preconditions for service
    pub as_needed_boolean: Option<types::Boolean>,

    /// Preconditions for service
    pub as_needed_codeable_concept: Option<types::CodeableConcept>,

    /// Date request signed
    pub authored_on: Option<types::DateTime>,

    /// Who/what is requesting service
    pub requester: Option<types::Reference>,

    /// Performer role
    pub performer_type: Option<types::CodeableConcept>,

    /// Requested performer
    pub performer: Option<Vec<types::Reference>>,

    /// Requested location
    pub location: Option<Vec<types::CodeableReference>>,

    /// Explanation/Justification for procedure or service
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<types::Reference>>,

    /// Additional clinical information
    pub supporting_info: Option<Vec<types::CodeableReference>>,

    /// Procedure Samples
    pub specimen: Option<Vec<types::Reference>>,

    /// Coded location on Body
    pub body_site: Option<Vec<types::CodeableConcept>>,

    /// BodyStructure-based location on the body
    pub body_structure: Option<types::Reference>,

    /// Comments
    pub note: Option<Vec<types::Annotation>>,

    /// Patient or consumer-oriented instructions
    pub patient_instruction: Option<Vec<ServiceRequestPatientInstruction>>,

    /// Request provenance
    pub relevant_history: Option<Vec<types::Reference>>,
}

/// Additional order information for a ServiceRequest.
///
/// Captures supplementary details about the order beyond the primary requested
/// code, optionally referencing an external context and carrying a set of coded
/// parameters that further specify the service being requested.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequestOrderDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The context of the order details by reference
    pub parameter_focus: Option<types::CodeableReference>,

    /// The parameter details for the service being requested
    pub parameter: Vec<ServiceRequestOrderDetailParameter>,
}

/// The parameter details for the service being requested.
///
/// Each parameter pairs a coded detail with a typed value, allowing the order to
/// carry structured, machine-processable specifications such as quantities,
/// ranges, or coded qualifiers for the requested service.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequestOrderDetailParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The detail of the order being requested
    pub code: types::CodeableConcept,

    /// The value for the order detail
    pub value_quantity: Option<types::Quantity>,

    /// The value for the order detail
    pub value_ratio: Option<types::Ratio>,

    /// The value for the order detail
    pub value_range: Option<types::Range>,

    /// The value for the order detail
    pub value_boolean: Option<types::Boolean>,

    /// The value for the order detail
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// The value for the order detail
    pub value_string: Option<types::String>,

    /// The value for the order detail
    pub value_period: Option<types::Period>,
}

/// Patient or consumer-oriented instructions for a ServiceRequest.
///
/// Provides guidance intended for the patient or consumer, expressed either as
/// inline markdown text or as a reference to a document resource carrying the
/// instructional content.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequestPatientInstruction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Patient or consumer-oriented instructions
    pub instruction_markdown: Option<types::Markdown>,

    /// Patient or consumer-oriented instructions
    pub instruction_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ServiceRequest;

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
