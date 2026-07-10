//! MedicationRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
//!
//! Version: 5.0.0
//!
//! MedicationRequest Resource: An order or request for both supply of the medication and the instructions for administration of the medication to a patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or
/// "MedicationOrder" to generalize the use across inpatient and outpatient
/// settings, including care plans, etc., and to harmonize with workflow
/// patterns.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::medication_request::MedicationRequest;
///
/// let value = MedicationRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequest {
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

    /// External ids for this request
    pub identifier: Option<Vec<types::Identifier>>,

    /// A plan or request that is fulfilled in whole or in part by this medication request
    pub based_on: Option<Vec<types::Reference>>,

    /// Reference to an order/prescription that is being replaced by this MedicationRequest
    pub prior_prescription: Option<types::Reference>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// active | on-hold | ended | stopped | completed | cancelled | entered-in-error | draft | unknown
    pub status: types::Code,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// When the status was changed
    pub status_changed: Option<types::DateTime>,

    /// proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: types::Code,

    /// Grouping or category of medication request
    pub category: Option<Vec<types::CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// True if patient is to stop taking or not to start taking the medication
    pub do_not_perform: Option<types::Boolean>,

    /// Medication to be taken
    pub medication: types::CodeableReference,

    /// Individual or group for whom the medication has been requested
    pub subject: types::Reference,

    /// The person or organization who provided the information about this request, if the source is someone other than the requestor
    pub information_source: Option<Vec<types::Reference>>,

    /// Encounter created as part of encounter/admission/stay
    pub encounter: Option<types::Reference>,

    /// Information to support fulfilling of the medication
    pub supporting_information: Option<Vec<types::Reference>>,

    /// When request was initially authored
    pub authored_on: Option<types::DateTime>,

    /// Who/What requested the Request
    pub requester: Option<types::Reference>,

    /// Reported rather than primary record
    pub reported: Option<types::Boolean>,

    /// Desired kind of performer of the medication administration
    pub performer_type: Option<types::CodeableConcept>,

    /// Intended performer of administration
    pub performer: Option<Vec<types::Reference>>,

    /// Intended type of device for the administration
    pub device: Option<Vec<types::CodeableReference>>,

    /// Person who entered the request
    pub recorder: Option<types::Reference>,

    /// Reason or indication for ordering or not ordering the medication
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Overall pattern of medication administration
    pub course_of_therapy_type: Option<types::CodeableConcept>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<types::Reference>>,

    /// Information about the prescription
    pub note: Option<Vec<types::Annotation>>,

    /// Full representation of the dosage instructions
    pub rendered_dosage_instruction: Option<types::Markdown>,

    /// Period over which the medication is to be taken
    pub effective_dose_period: Option<types::Period>,

    /// Specific instructions for how the medication should be taken
    pub dosage_instruction: Option<Vec<types::Dosage>>,

    /// Medication supply authorization
    pub dispense_request: Option<MedicationRequestDispenseRequest>,

    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationRequestSubstitution>,

    /// A list of events of interest in the lifecycle
    pub event_history: Option<Vec<types::Reference>>,
}

/// Medication supply authorization.
///
/// Indicates the specific details for the dispense or medication supply part
/// of a medication request (also known as a Medication Prescription or
/// Medication Order).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestDispenseRequest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// First fill details
    pub initial_fill: Option<MedicationRequestDispenseRequestInitialFill>,

    /// Minimum period of time between dispenses
    pub dispense_interval: Option<types::Duration>,

    /// Time period supply is authorized for
    pub validity_period: Option<types::Period>,

    /// Number of refills authorized
    pub number_of_repeats_allowed: Option<types::UnsignedInt>,

    /// Amount of medication to supply per dispense
    pub quantity: Option<types::Quantity>,

    /// Number of days supply per dispense
    pub expected_supply_duration: Option<types::Duration>,

    /// Intended performer of dispense
    pub dispenser: Option<types::Reference>,

    /// Additional information for the dispenser
    pub dispenser_instruction: Option<Vec<types::Annotation>>,

    /// Type of adherence packaging to use for the dispense
    pub dose_administration_aid: Option<types::CodeableConcept>,
}

/// First fill details.
///
/// Indicates the quantity or duration for the first dispense of the
/// medication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestDispenseRequestInitialFill {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// First fill quantity
    pub quantity: Option<types::Quantity>,

    /// First fill duration
    pub duration: Option<types::Duration>,
}

/// Any restrictions on medication substitution.
///
/// Indicates whether or not substitution can or should be part of the dispense.
/// In some cases, substitution must happen, in other cases substitution must
/// not happen. This block explains the prescriber's intent.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestSubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether substitution is allowed or not
    pub allowed_boolean: Option<types::Boolean>,

    /// Whether substitution is allowed or not
    pub allowed_codeable_concept: Option<types::CodeableConcept>,

    /// Why should (not) substitution be made
    pub reason: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationRequest;

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
