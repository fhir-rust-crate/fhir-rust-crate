//! ImmunizationRecommendation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
//!
//! Version: 5.0.0
//!
//! ImmunizationRecommendation Resource: A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// ImmunizationRecommendation resource.
///
/// A patient's point-in-time set of recommendations (i.e. forecasting)
/// according to a published schedule with optional supporting justification.
/// It captures which vaccines a patient should receive, the forecast status
/// of each recommendation, and the immunizations and observations that support
/// the reasoning. It is commonly produced by immunization forecasting engines.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::immunization_recommendation::ImmunizationRecommendation;
///
/// let value = ImmunizationRecommendation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImmunizationRecommendation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendation {
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

    /// Who this profile is for
    pub patient: types::Reference,

    /// Date recommendation(s) created
    pub date: types::DateTime,

    /// Who is responsible for protocol
    pub authority: Option<types::Reference>,

    /// Vaccine administration recommendations
    pub recommendation: Vec<ImmunizationRecommendationRecommendation>,
}

/// Vaccine administration recommendations.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendationRecommendation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Vaccine  or vaccine group recommendation applies to
    pub vaccine_code: Option<Vec<types::CodeableConcept>>,

    /// Disease to be immunized against
    pub target_disease: Option<Vec<types::CodeableConcept>>,

    /// Vaccine which is contraindicated to fulfill the recommendation
    pub contraindicated_vaccine_code: Option<Vec<types::CodeableConcept>>,

    /// Vaccine recommendation status
    pub forecast_status: types::CodeableConcept,

    /// Vaccine administration status reason
    pub forecast_reason: Option<Vec<types::CodeableConcept>>,

    /// Dates governing proposed immunization
    pub date_criterion: Option<Vec<ImmunizationRecommendationRecommendationDateCriterion>>,

    /// Protocol details
    pub description: Option<types::Markdown>,

    /// Name of vaccination series
    pub series: Option<types::String>,

    /// Recommended dose number within series
    pub dose_number: Option<types::String>,

    /// Recommended number of doses for immunity
    pub series_doses: Option<types::String>,

    /// Past immunizations supporting recommendation
    pub supporting_immunization: Option<Vec<types::Reference>>,

    /// Patient observations supporting recommendation
    pub supporting_patient_information: Option<Vec<types::Reference>>,
}

/// Dates governing proposed immunization.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of date
    pub code: types::CodeableConcept,

    /// Recommended date
    pub value: types::DateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImmunizationRecommendation;

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
