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
use fhir_derive_macros::Validate;

/// MedicationAdministration records the event of a patient actually consuming or
/// otherwise being administered a medication. In FHIR R5 it represents the point
/// in the medication process where a dose is given, as opposed to being ordered
/// or dispensed, and it captures who received the medication, what was given,
/// when and where it occurred, the dose and route, and who performed the act.
/// An administration can be as simple as swallowing a single tablet or as
/// involved as a long running intravenous infusion recorded over a period of
/// time. Clinically and administratively it supports medication reconciliation,
/// adherence monitoring, billing, safety surveillance, and audit of the
/// medication administration record (MAR). The event ties back to the
/// authorizing request or prescription and to the specific encounter between the
/// patient and the health care practitioner. It can also record that a planned
/// dose was intentionally withheld or wasted by using a status of not-done
/// together with an appropriate statusReason.
///
/// Related resources: the recipient is referenced through [`subject`], typically
/// a [`Patient`](crate::r5::resources::patient::Patient); the context is the
/// [`Encounter`](crate::r5::resources::encounter::Encounter); the substance is
/// described via a [`CodeableReference`](crate::r5::types::CodeableReference) to
/// a [`Medication`](crate::r5::resources::medication::Medication); and the
/// authorization is the associated
/// [`MedicationRequest`](crate::r5::resources::medication_request::MedicationRequest).
/// See also `MedicationDispense` and `MedicationStatement` for other stages of
/// the medication process.
///
/// [`subject`]: MedicationAdministration::subject
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

    /// External identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Plan this is fulfilled by this administration
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// Current state of the administration event, drawn from the required status value set: in-progress, not-done, on-hold, completed, entered-in-error, stopped, or unknown.
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason administration not performed
    pub status_reason: Option<Vec<types::CodeableConcept>>,

    /// Type of medication administration
    pub category: Option<Vec<types::CodeableConcept>>,

    /// What was administered, given either as a coded medication or as a reference to a Medication resource via a CodeableReference.
    pub medication: types::CodeableReference,

    /// Who received the medication, most commonly a reference to the Patient who was administered the dose.
    pub subject: types::Reference,

    /// Encounter administered as part of
    pub encounter: Option<types::Reference>,

    /// Additional information to support administration
    pub supporting_information: Option<Vec<types::Reference>>,

    /// The `MedicationAdministration.occurence[x]` choice element (0..1); see [`MedicationAdministrationOccurence`].
    #[serde(flatten)]
    pub occurence: Option<MedicationAdministrationOccurence>,

    /// When the MedicationAdministration was first captured in the subject's record
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`).
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Full dose was not administered
    pub is_sub_potent: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_sub_potent`](Self::is_sub_potent) (FHIR `_isSubPotent`).
    #[serde(rename = "_isSubPotent")]
    pub is_sub_potent_ext: Option<types::Element>,

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

    /// Details of how the medication was taken, including dose amount, site, route, method, and rate of administration.
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
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Body site administered to
    pub site: Option<types::CodeableConcept>,

    /// Path of substance into body
    pub route: Option<types::CodeableConcept>,

    /// How drug was administered
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    pub dose: Option<types::Quantity>,

    /// The `MedicationAdministration.dosage.rate[x]` choice element (0..1); see [`MedicationAdministrationDosageRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationAdministrationDosageRate>,
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
/// The `MedicationAdministration.dosage.rate[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationDosageRate {
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
    /// `rateQuantity` variant.
    #[fhir("rateQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `MedicationAdministration.occurence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationOccurence {
    /// `occurenceDateTime` variant.
    #[fhir("occurenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurencePeriod` variant.
    #[fhir("occurencePeriod")]
    Period(Box<types::Period>),
    /// `occurenceTiming` variant.
    #[fhir("occurenceTiming")]
    Timing(Box<types::Timing>),
}
