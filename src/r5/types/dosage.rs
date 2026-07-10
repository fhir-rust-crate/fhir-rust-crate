//! Dosage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Dosage
//!
//! Version: 5.0.0
//!
//! Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Dosage indicates how a medication is, was, or should be taken by the patient.
///
/// It captures the full instruction set for administering a medication,
/// including timing, route, site, method, dose amounts and rates, and upper
/// limits on how much may be taken per period, per administration, or over a
/// lifetime. In FHIR R5 it is a complex datatype used throughout medication
/// resources such as MedicationRequest, MedicationStatement, and
/// MedicationDispense.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::dosage::Dosage;
///
/// let value = Dosage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Dosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Dosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The order of the dosage instructions
    pub sequence: Option<types::Integer>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<types::String>,

    /// Supplemental instruction or warnings to the patient - e.g. "with meals", "may cause drowsiness"
    pub additional_instruction: Option<Vec<types::CodeableConcept>>,

    /// Patient or consumer oriented instructions
    pub patient_instruction: Option<types::String>,

    /// When medication should be administered
    pub timing: Option<types::Timing>,

    /// Take "as needed"
    pub as_needed: Option<types::Boolean>,

    /// Take "as needed" (for x)
    pub as_needed_for: Option<Vec<types::CodeableConcept>>,

    /// Body site to administer to
    pub site: Option<types::CodeableConcept>,

    /// How drug should enter body
    pub route: Option<types::CodeableConcept>,

    /// Technique for administering medication
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication administered, to be administered or typical amount to be administered
    pub dose_and_rate: Option<Vec<DosageDoseAndRate>>,

    /// Upper limit on medication per unit of time
    pub max_dose_per_period: Option<Vec<types::Ratio>>,

    /// Upper limit on medication per administration
    pub max_dose_per_administration: Option<types::Quantity>,

    /// Upper limit on medication per lifetime of the patient
    pub max_dose_per_lifetime: Option<types::Quantity>,
}

/// The amount of medication administered, to be administered, or the typical
/// amount to be administered, optionally paired with the kind of dose or rate.
///
/// This nested backbone element expresses a single dose-and-rate pairing within
/// a [`Dosage`], covering the dose amount (as a range or quantity) and the rate
/// at which it is administered (as a ratio, range, or quantity).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DosageDoseAndRate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// The kind of dose or rate specified
    pub r#type: Option<types::CodeableConcept>,

    /// Amount of medication per dose (Range variant)
    pub dose_range: Option<types::Range>,

    /// Amount of medication per dose (Quantity variant)
    pub dose_quantity: Option<types::Quantity>,

    /// Amount of medication per unit of time (Ratio variant)
    pub rate_ratio: Option<types::Ratio>,

    /// Amount of medication per unit of time (Range variant)
    pub rate_range: Option<types::Range>,

    /// Amount of medication per unit of time (Quantity variant)
    pub rate_quantity: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Dosage;

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
