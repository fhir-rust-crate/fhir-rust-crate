//! Evidence
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Evidence
//!
//! Version: 5.0.0
//!
//! Evidence Resource: The Evidence Resource provides a machine-interpretable expression of an evidence concept.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population,
/// exposures/interventions, comparators, outcomes, measured variables,
/// confounding variables), the statistics, and the certainty of this evidence.
/// It is used to summarize research findings and support evidence-based
/// clinical decision making and knowledge management in FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence::Evidence;
///
/// let value = Evidence::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Evidence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    /// Logical id of this artifact.
    pub id: Option<types::String>,

    /// Metadata about the resource.
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created.
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content.
    pub language: Option<types::Code>,

    /// Text summary of the resource, for human interpretation.
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources.
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Canonical identifier for this evidence, represented as a globally unique URI.
    pub url: Option<types::Uri>,

    /// Additional identifier for the summary.
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of this summary.
    pub version: Option<types::String>,

    /// How to compare versions.
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions.
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this summary (machine friendly).
    pub name: Option<types::String>,

    /// Name for this summary (human friendly).
    pub title: Option<types::String>,

    /// Citation for this evidence.
    pub cite_as_reference: Option<types::Reference>,

    /// Citation for this evidence.
    pub cite_as_markdown: Option<types::Markdown>,

    /// draft | active | retired | unknown.
    pub status: types::Code,

    /// For testing purposes, not real usage.
    pub experimental: Option<types::Boolean>,

    /// Date last changed.
    pub date: Option<types::DateTime>,

    /// When the summary was approved by publisher.
    pub approval_date: Option<types::Date>,

    /// When the summary was last reviewed by the publisher.
    pub last_review_date: Option<types::Date>,

    /// Name of the publisher/steward (organization or individual).
    pub publisher: Option<types::String>,

    /// Contact details for the publisher.
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Who authored the content.
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content.
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content.
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content.
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// The context that the content is intended to support.
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Why this Evidence is defined.
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions.
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s).
    pub copyright_label: Option<types::String>,

    /// Link or citation to artifact associated with the summary.
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Description of the particular summary.
    pub description: Option<types::Markdown>,

    /// Declarative description of the Evidence.
    pub assertion: Option<types::Markdown>,

    /// Footnotes and/or explanatory notes.
    pub note: Option<Vec<types::Annotation>>,

    /// Evidence variable such as population, exposure, or outcome.
    pub variable_definition: Vec<EvidenceVariableDefinition>,

    /// The method to combine studies.
    pub synthesis_type: Option<types::CodeableConcept>,

    /// The design of the study that produced this evidence.
    pub study_design: Option<Vec<types::CodeableConcept>>,

    /// Values and parameters for a single statistic.
    pub statistic: Option<Vec<EvidenceStatistic>>,

    /// Certainty or quality of the evidence.
    pub certainty: Option<Vec<EvidenceCertainty>>,
}

/// Evidence variable such as population, exposure, or outcome.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableDefinition {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A text description or summary of the variable.
    pub description: Option<types::Markdown>,

    /// Footnotes and/or explanatory notes.
    pub note: Option<Vec<types::Annotation>>,

    /// population | subpopulation | exposure | referenceExposure | measuredVariable | confounder.
    pub variable_role: types::CodeableConcept,

    /// Definition of the actual variable related to the statistic(s).
    pub observed: Option<types::Reference>,

    /// Definition of the intended variable related to the Evidence.
    pub intended: Option<types::Reference>,

    /// low | moderate | high | exact.
    pub directness_match: Option<types::CodeableConcept>,
}

/// Values and parameters for a single statistic.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatistic {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Description of content.
    pub description: Option<types::Markdown>,

    /// Footnotes and/or explanatory notes.
    pub note: Option<Vec<types::Annotation>>,

    /// Type of statistic, e.g., relative risk.
    pub statistic_type: Option<types::CodeableConcept>,

    /// Associated category for categorical variable.
    pub category: Option<types::CodeableConcept>,

    /// Statistic value.
    pub quantity: Option<types::Quantity>,

    /// The number of events associated with the statistic.
    pub number_of_events: Option<types::UnsignedInt>,

    /// The number of participants affected.
    pub number_affected: Option<types::UnsignedInt>,

    /// Number of samples in the statistic.
    pub sample_size: Option<EvidenceStatisticSampleSize>,

    /// An attribute of the Statistic.
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>>,

    /// An aspect of the statistical model.
    pub model_characteristic: Option<Vec<EvidenceStatisticModelCharacteristic>>,
}

/// Number of samples in the statistic.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticSampleSize {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Textual description of sample size for statistic.
    pub description: Option<types::Markdown>,

    /// Footnote or explanatory note about the sample size.
    pub note: Option<Vec<types::Annotation>>,

    /// Number of contributing studies.
    pub number_of_studies: Option<types::UnsignedInt>,

    /// Cumulative number of participants.
    pub number_of_participants: Option<types::UnsignedInt>,

    /// Number of participants with known results for measured variables.
    pub known_data_count: Option<types::UnsignedInt>,
}

/// An attribute of the Statistic.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticAttributeEstimate {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Textual description of the attribute estimate.
    pub description: Option<types::Markdown>,

    /// Footnote or explanatory note about the estimate.
    pub note: Option<Vec<types::Annotation>>,

    /// The type of attribute estimate, e.g., confidence interval or p value.
    pub r#type: Option<types::CodeableConcept>,

    /// The singular quantity of the attribute estimate, for attribute estimates
    /// represented as single values.
    pub quantity: Option<types::Quantity>,

    /// Level of confidence interval, e.g., 0.95 for 95% confidence interval.
    pub level: Option<types::Decimal>,

    /// Lower and upper bound values of the attribute estimate.
    pub range: Option<types::Range>,

    /// A nested attribute estimate; which is the attribute estimate of an attribute estimate.
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>>,
}

/// An aspect of the statistical model.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticModelCharacteristic {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Model specification.
    pub code: types::CodeableConcept,

    /// Numerical value to complete model specification.
    pub value: Option<types::Quantity>,

    /// A variable adjusted for in the adjusted analysis.
    pub variable: Option<Vec<EvidenceStatisticModelCharacteristicVariable>>,

    /// An attribute of the statistic used as a model characteristic.
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>>,
}

/// A variable adjusted for in the adjusted analysis.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticModelCharacteristicVariable {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Description of the variable.
    pub variable_definition: types::Reference,

    /// continuous | dichotomous | ordinal | polychotomous.
    pub handling: Option<types::Code>,

    /// Description for grouping of ordinal or polychotomous variables.
    pub value_category: Option<Vec<types::CodeableConcept>>,

    /// Discrete value for grouping of ordinal or polychotomous variables.
    pub value_quantity: Option<Vec<types::Quantity>>,

    /// Range of values for grouping of ordinal or polychotomous variables.
    pub value_range: Option<Vec<types::Range>>,
}

/// Certainty or quality of the evidence.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceCertainty {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Textual description of certainty.
    pub description: Option<types::Markdown>,

    /// Footnotes and/or explanatory notes.
    pub note: Option<Vec<types::Annotation>>,

    /// Aspect of certainty being rated.
    pub r#type: Option<types::CodeableConcept>,

    /// Assessment or judgement of the aspect.
    pub rating: Option<types::CodeableConcept>,

    /// Individual or group who did the rating.
    pub rater: Option<types::String>,

    /// A domain or subdomain of certainty.
    pub subcomponent: Option<Vec<EvidenceCertainty>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Evidence;

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
