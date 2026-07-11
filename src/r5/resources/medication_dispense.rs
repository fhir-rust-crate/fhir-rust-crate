//! MedicationDispense
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
//!
//! Version: 5.0.0
//!
//! MedicationDispense Resource: Indicates that a medication product is to be or has been dispensed for a named person/patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Indicates that a medication product is to be or has been dispensed for a
/// named person/patient.
///
/// In FHIR R5, MedicationDispense records the provision of a medication supply,
/// together with the instructions for administering it. It is typically the
/// result of a pharmacy or dispensing system responding to a medication order,
/// and it documents the fulfillment step of the broader medication process that
/// spans requesting, dispensing, and administration. The resource captures who
/// the medication is for, what product was supplied, who performed the dispense
/// and in what role, the quantity and days supply, the timing of preparation
/// and hand-over, any substitution that occurred relative to the prescription,
/// and the dosage instructions. It is used for pharmacy operations, medication
/// reconciliation, billing, and clinical record keeping.
///
/// The dispensed product is described by the `medication` field as a
/// [`CodeableReference`](crate::r5::types::CodeableReference), and the
/// `subject` identifies the person receiving the medication, usually a
/// [`Patient`](crate::r5::resources::patient::Patient). The dispense is
/// commonly linked back to the authorizing order through
/// `authorizing_prescription`. Related resources in the medication workflow
/// include `MedicationRequest`, `MedicationAdministration`, `Medication`, and
/// `MedicationStatement`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_dispense::MedicationDispense;
///
/// let value = MedicationDispense::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationDispense = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationDispense {
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

    /// Plan that is fulfilled by this dispense
    pub based_on: Option<Vec<types::Reference>>,

    /// Event that dispense is part of
    pub part_of: Option<Vec<types::Reference>>,

    /// Current lifecycle state of the dispense, such as preparation, in-progress, cancelled, on-hold, completed, entered-in-error, stopped, declined, or unknown.
    pub status: types::Code,

    /// Why a dispense was not performed
    pub not_performed_reason: Option<types::CodeableReference>,

    /// When the status changed
    pub status_changed: Option<types::DateTime>,

    /// Type of medication dispense
    pub category: Option<Vec<types::CodeableConcept>>,

    /// The medication product that was supplied, given either as a coded concept or a reference to a Medication resource.
    pub medication: types::CodeableReference,

    /// The person or group the dispense is for, most often a reference to a Patient.
    pub subject: types::Reference,

    /// Encounter associated with event
    pub encounter: Option<types::Reference>,

    /// Information that supports the dispensing of the medication
    pub supporting_information: Option<Vec<types::Reference>>,

    /// Who performed event
    pub performer: Option<Vec<MedicationDispensePerformer>>,

    /// Where the dispense occurred
    pub location: Option<types::Reference>,

    /// Medication order that authorizes the dispense
    pub authorizing_prescription: Option<Vec<types::Reference>>,

    /// Trial fill, partial fill, emergency fill, etc
    pub r#type: Option<types::CodeableConcept>,

    /// Amount dispensed
    pub quantity: Option<types::Quantity>,

    /// Amount of medication expressed as a timing amount
    pub days_supply: Option<types::Quantity>,

    /// When the recording of the dispense started
    pub recorded: Option<types::DateTime>,

    /// When product was packaged and reviewed
    pub when_prepared: Option<types::DateTime>,

    /// When product was given out
    pub when_handed_over: Option<types::DateTime>,

    /// Where the medication was/will be sent
    pub destination: Option<types::Reference>,

    /// Who collected the medication or where the medication was delivered
    pub receiver: Option<Vec<types::Reference>>,

    /// Information about the dispense
    pub note: Option<Vec<types::Annotation>>,

    /// Full representation of the dosage instructions
    pub rendered_dosage_instruction: Option<types::Markdown>,

    /// How the medication is to be used by the patient or administered by the caregiver
    pub dosage_instruction: Option<Vec<types::Dosage>>,

    /// Whether a substitution was performed on the dispense
    pub substitution: Option<MedicationDispenseSubstitution>,

    /// A list of relevant lifecycle events
    pub event_history: Option<Vec<types::Reference>>,
}

/// Who performed event.
///
/// Indicates who or what performed the medication dispense event and what role
/// they played in the process.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Who performed the dispense and what they did
    pub function: Option<types::CodeableConcept>,

    /// Individual who was performing
    pub actor: types::Reference,
}

/// Whether a substitution was performed on the dispense.
///
/// Indicates whether or not a substitution was made as part of the dispense,
/// including the type of substitution, the reasons for it, and who was
/// responsible.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationDispenseSubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether a substitution was or was not performed on the dispense
    pub was_substituted: types::Boolean,

    /// Code signifying whether a different drug was dispensed from what was prescribed
    pub r#type: Option<types::CodeableConcept>,

    /// Why was substitution made
    pub reason: Option<Vec<types::CodeableConcept>>,

    /// Who is responsible for the substitution
    pub responsible_party: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationDispense;

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
