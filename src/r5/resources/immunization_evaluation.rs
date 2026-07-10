//! ImmunizationEvaluation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImmunizationEvaluation
//!
//! Version: 5.0.0
//!
//! ImmunizationEvaluation Resource: Describes a comparison of an immunization event against published recommendations to determine if the administration is "valid" in relation to those recommendations.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// ImmunizationEvaluation describes a comparison of an immunization event
/// against published recommendations to determine if the administration is
/// "valid" in relation to those recommendations. It captures whether a given
/// dose counts toward a vaccine series and, if not, the reason why. This
/// resource is typically produced by clinical decision support systems that
/// assess a patient's immunization history against a schedule.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::immunization_evaluation::ImmunizationEvaluation;
///
/// let value = ImmunizationEvaluation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImmunizationEvaluation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationEvaluation {
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

    /// Business identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// completed | entered-in-error
    pub status: types::Code,

    /// Who this evaluation is for
    pub patient: types::Reference,

    /// Date evaluation was performed
    pub date: Option<types::DateTime>,

    /// Who is responsible for publishing the recommendations
    pub authority: Option<types::Reference>,

    /// The vaccine preventable disease schedule being evaluated
    pub target_disease: types::CodeableConcept,

    /// Immunization being evaluated
    pub immunization_event: types::Reference,

    /// Status of the dose relative to published recommendations
    pub dose_status: types::CodeableConcept,

    /// Reason why the doese is considered valid, invalid or some other status
    pub dose_status_reason: Option<Vec<types::CodeableConcept>>,

    /// Evaluation notes
    pub description: Option<types::Markdown>,

    /// Name of vaccine series
    pub series: Option<types::String>,

    /// Dose number within series
    pub dose_number: Option<types::String>,

    /// Recommended number of doses for immunity
    pub series_doses: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImmunizationEvaluation;

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
