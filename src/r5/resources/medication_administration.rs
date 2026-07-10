//! MedicationAdministration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
//!
//! Version: 5.0.0
//!
//! MedicationAdministration Resource: Describes the event of a patient consuming or otherwise being administered a medication.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// MedicationAdministration describes the event of a patient consuming or
/// otherwise being administered a medication. This may be as simple as
/// swallowing a tablet or it may be a long running infusion. Related resources
/// tie this event to the authorizing prescription, and the specific encounter
/// between patient and health care practitioner. This event can also be used to
/// record waste using a status of not-done and the appropriate statusReason.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_administration::MedicationAdministration;
///
/// let value = MedicationAdministration::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministration {
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

    /// Plan this is fulfilled by this administration
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// in-progress | not-done | on-hold | completed | entered-in-error | stopped | unknown
    pub status: types::Code,

    /// Reason administration not performed
    pub status_reason: Option<Vec<types::CodeableConcept>>,

    /// Type of medication administration
    pub category: Option<Vec<types::CodeableConcept>>,

    /// What was administered
    pub medication: types::CodeableReference,

    /// Who received medication
    pub subject: types::Reference,

    /// Encounter administered as part of
    pub encounter: Option<types::Reference>,

    /// Additional information to support administration
    pub supporting_information: Option<Vec<types::Reference>>,

    /// Specific date/time or interval of time during which the administration took place (dateTime)
    pub occurence_date_time: Option<types::DateTime>,

    /// Specific date/time or interval of time during which the administration took place (Period)
    pub occurence_period: Option<types::Period>,

    /// Specific date/time or interval of time during which the administration took place (Timing)
    pub occurence_timing: Option<types::Timing>,

    /// When the MedicationAdministration was first captured in the subject's record
    pub recorded: Option<types::DateTime>,

    /// Full dose was not administered
    pub is_sub_potent: Option<types::Boolean>,

    /// Reason full dose was not administered
    pub sub_potent_reason: Option<Vec<types::CodeableConcept>>,

    /// Who or what performed the medication administration and what type of performance they did
    pub performer: Option<Vec<MedicationAdministrationPerformer>>,

    /// Concept, condition or observation that supports why the medication was administered
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Request administration performed against
    pub request: Option<types::Reference>,

    /// Device used to administer
    pub device: Option<Vec<types::CodeableReference>>,

    /// Information about the administration
    pub note: Option<Vec<types::Annotation>>,

    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,

    /// A list of events of interest in the lifecycle
    pub event_history: Option<Vec<types::Reference>>,
}

/// Who or what performed the medication administration and what type of
/// performance they did.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministrationPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who or what performed the medication administration
    pub actor: types::CodeableReference,
}

/// Details of how medication was taken.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministrationDosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<types::String>,

    /// Body site administered to
    pub site: Option<types::CodeableConcept>,

    /// Path of substance into body
    pub route: Option<types::CodeableConcept>,

    /// How drug was administered
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    pub dose: Option<types::Quantity>,

    /// Dose quantity per unit of time (Ratio)
    pub rate_ratio: Option<types::Ratio>,

    /// Dose quantity per unit of time (Quantity)
    pub rate_quantity: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationAdministration;

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
