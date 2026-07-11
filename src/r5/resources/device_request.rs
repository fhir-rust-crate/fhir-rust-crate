//! DeviceRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
//!
//! Version: 5.0.0
//!
//! DeviceRequest Resource: Represents a request a device to be provided to a specific patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Represents a request for a device to be provided to a specific patient.
///
/// The device may be an implantable device to be subsequently implanted, or an
/// external assistive device, such as a walker, to be delivered and subsequently
/// used. A DeviceRequest captures the intent, priority, requested device, subject,
/// timing, and supporting clinical context for the order. In FHIR R5 it is a
/// request workflow resource frequently used to drive device supply and fulfillment.
///
/// Clinically, a DeviceRequest is created by an ordering clinician or system to
/// communicate that a device is needed for a particular patient, and to track that
/// order through its lifecycle (from draft, through active fulfillment, to
/// completion or revocation). It participates in the FHIR request/event workflow
/// pattern alongside resources that report on the resulting event, and it may
/// reference prior requests it fulfills or replaces via `based_on` and `replaces`.
///
/// Related resources: the `subject` is typically a
/// [`Patient`](crate::r5::resources::patient::Patient); the requested item is
/// described via [`CodeableReference`](crate::r5::types::CodeableReference), which
/// may point to a `Device` or `DeviceDefinition`; and detailed dosing/usage-style
/// parameters use [`CodeableConcept`](crate::r5::types::CodeableConcept) and
/// [`Quantity`](crate::r5::types::Quantity) values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_request::DeviceRequest;
///
/// let value = DeviceRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DeviceRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
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

    /// External Request identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// What request fulfills
    pub based_on: Option<Vec<types::Reference>>,

    /// What request replaces
    pub replaces: Option<Vec<types::Reference>>,

    /// Identifier of composite request
    pub group_identifier: Option<types::Identifier>,

    /// Status of the request in its lifecycle: draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: Option<types::Code>,

    /// Indicates the level of authority of the request, e.g. proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: types::Code,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// True if the request is to stop or not to start using the device
    pub do_not_perform: Option<types::Boolean>,

    /// The device being requested, as a coded concept and/or reference to a Device or DeviceDefinition
    pub code: types::CodeableReference,

    /// Quantity of devices to supply
    pub quantity: Option<types::Integer>,

    /// Device details
    pub parameter: Option<Vec<DeviceRequestParameter>>,

    /// The patient (or group/location/device) for whom the device is being requested
    pub subject: types::Reference,

    /// Encounter motivating request
    pub encounter: Option<types::Reference>,

    /// Desired time or schedule for use
    pub occurrence_date_time: Option<types::DateTime>,

    /// Desired time or schedule for use
    pub occurrence_period: Option<types::Period>,

    /// Desired time or schedule for use
    pub occurrence_timing: Option<types::Timing>,

    /// When recorded
    pub authored_on: Option<types::DateTime>,

    /// The individual or entity who initiated the request and is responsible for its activation
    pub requester: Option<types::Reference>,

    /// The desired individual or entity to perform the fulfillment of the request
    pub performer: Option<types::CodeableReference>,

    /// Coded/Linked Reason for request
    pub reason: Option<Vec<types::CodeableReference>>,

    /// PRN status of request
    pub as_needed: Option<types::Boolean>,

    /// Device usage reason
    pub as_needed_for: Option<types::CodeableConcept>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<types::Reference>>,

    /// Additional clinical information
    pub supporting_info: Option<Vec<types::Reference>>,

    /// Notes or comments
    pub note: Option<Vec<types::Annotation>>,

    /// Request provenance
    pub relevant_history: Option<Vec<types::Reference>>,
}

/// Device details.
///
/// Specific parameters for the ordered item, expressed as a coded detail and an
/// associated value.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequestParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Device detail
    pub code: Option<types::CodeableConcept>,

    /// Value of detail
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value of detail
    pub value_quantity: Option<types::Quantity>,

    /// Value of detail
    pub value_range: Option<types::Range>,

    /// Value of detail
    pub value_boolean: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceRequest;

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
