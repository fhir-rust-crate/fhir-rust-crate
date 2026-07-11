//! ConditionDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ConditionDefinition
//!
//! Version: 5.0.0
//!
//! ConditionDefinition Resource: A definition of a condition and information relevant to managing it.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A definition of a condition and information relevant to managing it.
///
/// ConditionDefinition is a canonical, knowledge-artifact style resource that
/// describes a clinical condition, problem, or diagnosis independent of any
/// single patient or encounter. Rather than recording that a particular person
/// currently has a condition, it captures reusable knowledge about the condition
/// itself: the identifying code, the subjective severity, the anatomical body
/// site, and the stage or grade, together with metadata flags indicating whether
/// severity, body site, and stage are appropriate to record for this condition.
/// It also links to the observations and preconditions that are diagnostically
/// relevant, the medications commonly used, the questionnaires used for
/// pre-admission, differential diagnosis, or outcome assessment, the care teams
/// suited to managing it, and the care or treatment plans that apply.
///
/// In FHIR R5 a ConditionDefinition acts as a definitional resource that drives
/// clinical decision support, standardized data capture, and consistent
/// recording of patient-specific condition instances. Authoring systems and
/// registries publish these definitions so that downstream applications can
/// prompt for the right data and validate recorded conditions against agreed
/// guidance.
///
/// See also the patient-specific [`Condition`](crate::r5::resources::condition::Condition)
/// resource that a definition informs, the [`Patient`](crate::r5::resources::patient::Patient)
/// it is ultimately recorded against, and related definitional and supporting
/// resources such as [`Observation`](crate::r5::resources::observation::Observation),
/// [`Questionnaire`](crate::r5::resources::questionnaire::Questionnaire),
/// [`CareTeam`](crate::r5::resources::care_team::CareTeam), and
/// [`PlanDefinition`](crate::r5::resources::plan_definition::PlanDefinition).
/// Many of its fields use the shared [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// data type.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::condition_definition::ConditionDefinition;
///
/// let value = ConditionDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConditionDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinition {
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

    /// Canonical identifier for this condition definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the condition definition
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the condition definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this condition definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this condition definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the event definition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`).
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// Publication lifecycle status of this definition: draft, active, retired, or unknown.
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the condition definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for condition definition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Required CodeableConcept identifying the condition, problem, or diagnosis this definition describes.
    pub code: types::CodeableConcept,

    /// Subjective severity that is appropriate to record for this condition, such as mild, moderate, or severe.
    pub severity: Option<types::CodeableConcept>,

    /// Anatomical location associated with the condition, when a body site is clinically relevant.
    pub body_site: Option<types::CodeableConcept>,

    /// Stage or grade of the condition, typically assessed through a formal clinical staging system.
    pub stage: Option<types::CodeableConcept>,

    /// Whether Severity is appropriate
    pub has_severity: Option<types::Boolean>,
    /// Primitive extension sibling for [`has_severity`](Self::has_severity) (FHIR `_hasSeverity`).
    #[serde(rename = "_hasSeverity")]
    pub has_severity_ext: Option<types::Element>,

    /// Whether bodySite is appropriate
    pub has_body_site: Option<types::Boolean>,
    /// Primitive extension sibling for [`has_body_site`](Self::has_body_site) (FHIR `_hasBodySite`).
    #[serde(rename = "_hasBodySite")]
    pub has_body_site_ext: Option<types::Element>,

    /// Whether stage is appropriate
    pub has_stage: Option<types::Boolean>,
    /// Primitive extension sibling for [`has_stage`](Self::has_stage) (FHIR `_hasStage`).
    #[serde(rename = "_hasStage")]
    pub has_stage_ext: Option<types::Element>,

    /// Formal Definition for the condition
    pub definition: Option<Vec<types::Uri>>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<Vec<Option<types::Element>>>,

    /// Observations particularly relevant to this condition
    pub observation: Option<Vec<ConditionDefinitionObservation>>,

    /// Medications particularly relevant for this condition
    pub medication: Option<Vec<ConditionDefinitionMedication>>,

    /// Observation that suggets this condition
    pub precondition: Option<Vec<ConditionDefinitionPrecondition>>,

    /// Appropriate team for this condition
    pub team: Option<Vec<types::Reference>>,

    /// Questionnaire for this condition
    pub questionnaire: Option<Vec<ConditionDefinitionQuestionnaire>>,

    /// Plan that is appropriate
    pub plan: Option<Vec<ConditionDefinitionPlan>>,
}

/// Observations particularly relevant to this condition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionObservation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Category that is relevant
    pub category: Option<types::CodeableConcept>,

    /// Code for relevant Observation
    pub code: Option<types::CodeableConcept>,
}

/// Medications particularly relevant for this condition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionMedication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Category that is relevant
    pub category: Option<types::CodeableConcept>,

    /// Code for relevant Medication
    pub code: Option<types::CodeableConcept>,
}

/// Observation that suggets this condition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionPrecondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// sensitive | specific
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Code for relevant Observation
    pub code: types::CodeableConcept,

    /// Value of Observation
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value of Observation
    pub value_quantity: Option<types::Quantity>,
}

/// Questionnaire for this condition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionQuestionnaire {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// preadmit | diff-diagnosis | outcome
    pub purpose: types::Code,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Specific Questionnaire
    pub reference: types::Reference,
}

/// Plan that is appropriate.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionPlan {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Use for the plan
    pub role: Option<types::CodeableConcept>,

    /// The actual plan
    pub reference: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ConditionDefinition;

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
