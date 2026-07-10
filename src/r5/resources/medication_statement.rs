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
use fhir_derive::Validate;

/// A record of a medication that is being consumed by a patient.
///
/// A MedicationStatement may indicate that the patient may be taking the
/// medication now, or has taken the medication in the past, or will be taking
/// the medication in the future. The source of this information can be the
/// patient, a significant other (such as a family member or spouse), or a
/// clinician. It is used to record a snapshot of medication use, and is not a
/// request or order.
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

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// recorded | entered-in-error | draft
    pub status: types::Code,

    /// Type of medication statement
    pub category: Option<Vec<types::CodeableConcept>>,

    /// What medication was taken
    pub medication: types::CodeableReference,

    /// Who is/was taking the medication
    pub subject: types::Reference,

    /// Encounter associated with MedicationStatement
    pub encounter: Option<types::Reference>,

    /// The date/time or interval when the medication is/was/will be taken
    pub effective_date_time: Option<types::DateTime>,

    /// The date/time or interval when the medication is/was/will be taken
    pub effective_period: Option<types::Period>,

    /// The date/time or interval when the medication is/was/will be taken
    pub effective_timing: Option<types::Timing>,

    /// When the usage was asserted?
    pub date_asserted: Option<types::DateTime>,

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

    /// Details of how medication is/was taken or should be taken
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
