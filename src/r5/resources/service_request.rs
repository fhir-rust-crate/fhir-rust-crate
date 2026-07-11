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
use fhir_derive_macros::Validate;

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
/// Clinically, a ServiceRequest represents the intent to have a service
/// performed and progresses through a lifecycle described by its `status`
/// (draft, active, on-hold, revoked, completed, entered-in-error, or unknown)
/// and `intent` (proposal, plan, directive, order, and related values). It is
/// the FHIR workflow analogue of a paper or electronic order and is commonly
/// used to drive downstream workflows such as scheduling, specimen collection,
/// imaging acquisition, and result reporting, with the fulfilling actor
/// typically producing a DiagnosticReport, Procedure, or Observation that
/// references the originating request via `basedOn`.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — often the `subject` of the request.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for `category`, `body_site`, and other coded fields.
/// - `DiagnosticReport`, `Procedure`, and `Observation` — typical resources that fulfill a ServiceRequest.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::service_request::ServiceRequest;
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

    /// Business identifiers assigned to this order by the requester, performer, or other systems
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

    /// What request fulfills
    pub based_on: Option<Vec<types::Reference>>,

    /// What request replaces
    pub replaces: Option<Vec<types::Reference>>,

    /// Composite Request ID
    pub requisition: Option<types::Identifier>,

    /// The current lifecycle status of the order: draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Whether the request is a proposal, plan, directive, order, or similar (proposal | plan | directive | order +)
    pub intent: crate::r5::coded::Coded<crate::r5::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Broad categorization of the type of service requested, e.g. imaging, laboratory, or counseling
    pub category: Option<Vec<types::CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if service/procedure should not be performed
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`).
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// The specific service, procedure, or product being requested/ordered, coded or referenced
    pub code: Option<types::CodeableReference>,

    /// Additional order information
    pub order_detail: Option<Vec<ServiceRequestOrderDetail>>,

    /// The `ServiceRequest.quantity[x]` choice element (0..1); see [`ServiceRequestQuantity`].
    #[serde(flatten)]
    pub quantity: Option<ServiceRequestQuantity>,

    /// The individual, group, device, or location the service is ordered for, most often a [`Patient`](crate::r5::resources::patient::Patient)
    pub subject: types::Reference,

    /// What the service request is about, when it is not about the subject of record
    pub focus: Option<Vec<types::Reference>>,

    /// Encounter in which the request was created
    pub encounter: Option<types::Reference>,

    /// The `ServiceRequest.occurrence[x]` choice element (0..1); see [`ServiceRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ServiceRequestOccurrence>,

    /// The `ServiceRequest.asNeeded[x]` choice element (0..1); see [`ServiceRequestAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<ServiceRequestAsNeeded>,

    /// Date request signed
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

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

    /// The `ServiceRequest.orderDetail.parameter.value[x]` choice element (0..1); see [`ServiceRequestOrderDetailParameterValue`].
    #[serde(flatten)]
    pub value: Option<ServiceRequestOrderDetailParameterValue>,
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

    /// The `ServiceRequest.patientInstruction.instruction[x]` choice element (0..1); see [`ServiceRequestPatientInstructionInstruction`].
    #[serde(flatten)]
    pub instruction: Option<ServiceRequestPatientInstructionInstruction>,
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
/// The `ServiceRequest.asNeeded[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `ServiceRequest.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `ServiceRequest.orderDetail.parameter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestOrderDetailParameterValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
}

/// The `ServiceRequest.patientInstruction.instruction[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestPatientInstructionInstruction {
    /// `instructionMarkdown` variant.
    #[fhir("instructionMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
    /// `instructionReference` variant.
    #[fhir("instructionReference")]
    Reference(Box<types::Reference>),
}

/// The `ServiceRequest.quantity[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestQuantity {
    /// `quantityQuantity` variant.
    #[fhir("quantityQuantity")]
    Quantity(Box<types::Quantity>),
    /// `quantityRatio` variant.
    #[fhir("quantityRatio")]
    Ratio(Box<types::Ratio>),
    /// `quantityRange` variant.
    #[fhir("quantityRange")]
    Range(Box<types::Range>),
}
