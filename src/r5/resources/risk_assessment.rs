//! RiskAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
//!
//! Version: 5.0.0
//!
//! RiskAssessment Resource: An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// RiskAssessment
///
/// An assessment of the likely outcome(s) for a patient or other subject as
/// well as the likelihood of each outcome. It captures the method used to make
/// the assessment, the condition or context that prompted it, and one or more
/// predictions describing the probability, qualitative risk, and timeframe of
/// each possible outcome. RiskAssessment resources are commonly used to record
/// clinical decision support results and probabilistic prognoses, such as the
/// likelihood of a disease occurring, a treatment succeeding, or a future
/// clinical event taking place. Assessments may be generated manually by a
/// clinician, derived from a decision-support tool or predictive algorithm, or
/// produced by a risk-scoring calculation engine, and can be linked back to the
/// observations, conditions, or other evidence (`basis`) that informed the
/// prediction.
///
/// Related resources: the subject of a RiskAssessment is typically a
/// [`Patient`](crate::r5::resources::patient::Patient), the evaluation method and
/// predicted outcomes are represented using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and a RiskAssessment
/// may reference a `Condition` being assessed or an `Encounter` during which the
/// assessment was performed.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::risk_assessment::RiskAssessment;
///
/// let value = RiskAssessment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: RiskAssessment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessment {
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

    /// Unique identifier for the assessment
    pub identifier: Option<Vec<types::Identifier>>,

    /// Request fulfilled by this assessment
    pub based_on: Option<types::Reference>,

    /// Part of this occurrence
    pub parent: Option<types::Reference>,

    /// The status of the RiskAssessment, using the codes registered | preliminary | final | amended +
    pub status: types::Code,

    /// The algorithm, risk-scoring tool, or evaluation mechanism used to generate the assessment
    pub method: Option<types::CodeableConcept>,

    /// The type of the risk assessment being performed, for example a general clinical risk assessment or a specific screening tool
    pub code: Option<types::CodeableConcept>,

    /// The patient or group the risk assessment applies to
    pub subject: types::Reference,

    /// Where was assessment performed?
    pub encounter: Option<types::Reference>,

    /// When was assessment made?
    pub occurrence_date_time: Option<types::DateTime>,

    /// When was assessment made?
    pub occurrence_period: Option<types::Period>,

    /// Condition assessed
    pub condition: Option<types::Reference>,

    /// Who did assessment?
    pub performer: Option<types::Reference>,

    /// Why the assessment was necessary?
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Information used in assessment
    pub basis: Option<Vec<types::Reference>>,

    /// One or more predicted outcomes for the subject, each with its own probability, qualitative risk, and timeframe
    pub prediction: Option<Vec<RiskAssessmentPrediction>>,

    /// Recommended steps to reduce the predicted risk, or an indication that no mitigation is available
    pub mitigation: Option<types::String>,

    /// Comments on the risk assessment
    pub note: Option<Vec<types::Annotation>>,
}

/// RiskAssessmentPrediction
///
/// Describes the expected outcome for the subject, including the likelihood of
/// that outcome and the timeframe over which the risk applies.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessmentPrediction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Possible outcome for the subject
    pub outcome: Option<types::CodeableConcept>,

    /// Likelihood of specified outcome
    pub probability_decimal: Option<types::Decimal>,

    /// Likelihood of specified outcome
    pub probability_range: Option<types::Range>,

    /// Likelihood of specified outcome as a qualitative value
    pub qualitative_risk: Option<types::CodeableConcept>,

    /// Relative likelihood
    pub relative_risk: Option<types::Decimal>,

    /// Timeframe or age range
    pub when_period: Option<types::Period>,

    /// Timeframe or age range
    pub when_range: Option<types::Range>,

    /// Explanation of prediction
    pub rationale: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RiskAssessment;

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
