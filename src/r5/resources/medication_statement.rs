//! MedicationStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
//!
//! Version: 5.0.0
//!
//! MedicationStatement Resource: A record of a medication that is being consumed by a patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a medication that is being consumed by a patient.
///
/// In FHIR R5 the MedicationStatement resource captures a point-in-time
/// assertion about a patient's use of a medication. It may indicate that the
/// patient is taking the medication now, has taken it in the past, or will be
/// taking it in the future, and it can also assert that a medication is not
/// being taken. The information may be gathered during history taking,
/// medication reconciliation, or patient reporting, so the source can be the
/// patient, a significant other such as a family member or spouse, or a
/// clinician. It is deliberately a snapshot of medication usage rather than an
/// authorization, and therefore it is not a request, prescription, or order:
/// use MedicationRequest for ordering and MedicationDispense or
/// MedicationAdministration for supply and administration events. The degree of
/// certainty and the reporting perspective are conveyed through the status,
/// information source, and adherence details.
///
/// # See also
///
/// The medication itself is referenced via [`CodeableReference`](crate::r5::types::CodeableReference)
/// (typically to a `Medication` resource or a coded concept), the recipient of
/// the medication is the [`Patient`](crate::r5::resources::patient::Patient)
/// or other subject given as a [`Reference`](crate::r5::types::Reference), and
/// classification uses [`CodeableConcept`](crate::r5::types::CodeableConcept).
/// Related workflow resources include `MedicationRequest`,
/// `MedicationDispense`, and `MedicationAdministration`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_statement::MedicationStatement;
///
/// let value = MedicationStatement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationStatement {
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

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// Status of the assertion as a code such as recorded, entered-in-error, or draft, indicating how the statement should be interpreted.
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type of medication statement
    pub category: Option<Vec<types::CodeableConcept>>,

    /// What medication was taken, given as a coded concept or a reference to a Medication resource.
    pub medication: types::CodeableReference,

    /// Who is or was taking the medication, usually a reference to the Patient who is the subject of the statement.
    pub subject: types::Reference,

    /// Encounter associated with MedicationStatement
    pub encounter: Option<types::Reference>,

    /// The `MedicationStatement.effective[x]` choice element (0..1); see [`MedicationStatementEffective`].
    #[serde(flatten)]
    pub effective: Option<MedicationStatementEffective>,

    /// When the usage was asserted?
    pub date_asserted: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_asserted`](Self::date_asserted) (FHIR `_dateAsserted`).
    #[serde(rename = "_dateAsserted")]
    pub date_asserted_ext: Option<types::Element>,

    /// Person or organization that provided the information about the taking of this medication
    pub information_source: Option<Vec<types::Reference>>,

    /// Link to information used to derive the MedicationStatement
    pub derived_from: Option<Vec<types::Reference>>,

    /// Reason for why the medication is being/was taken
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Further information about the usage
    pub note: Option<Vec<types::Annotation>>,

    /// Link to information relevant to the usage of a medication
    pub related_clinical_information: Option<Vec<types::Reference>>,

    /// Full representation of the dosage instructions
    pub rendered_dosage_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`rendered_dosage_instruction`](Self::rendered_dosage_instruction) (FHIR `_renderedDosageInstruction`).
    #[serde(rename = "_renderedDosageInstruction")]
    pub rendered_dosage_instruction_ext: Option<types::Element>,

    /// Details of how the medication is, was, or should be taken, including dose, route, and timing.
    pub dosage: Option<Vec<types::Dosage>>,

    /// Indicates whether the medication is or is not being consumed or administered
    pub adherence: Option<MedicationStatementAdherence>,
}

/// Indicates whether the medication is or is not being consumed or administered.
///
/// Provides a categorization of whether the patient is following the medication
/// regimen, along with details of the reason for the current adherence state.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationStatementAdherence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of adherence
    pub code: types::CodeableConcept,

    /// Details of the reason for the current use of the medication
    pub reason: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationStatement;

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
/// The `MedicationStatement.effective[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
    /// `effectiveTiming` variant.
    #[fhir("effectiveTiming")]
    Timing(Box<types::Timing>),
}
