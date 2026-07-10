//! PlanDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PlanDefinition
//!
//! Version: 5.0.0
//!
//! PlanDefinition Resource: This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// PlanDefinition Resource.
///
/// This resource allows for the definition of various types of plans as a
/// sharable, consumable, and executable artifact. The resource is general
/// enough to support the description of a broad range of clinical and
/// non-clinical artifacts such as clinical decision support rules, order sets,
/// protocols, and drug quality specifications.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::plan_definition::PlanDefinition;
///
/// let value = PlanDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PlanDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinition {
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

    /// Canonical identifier for this plan definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the plan definition
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the plan definition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this plan definition (computer friendly)
    pub name: Option<types::String>,

    /// Name for this plan definition (human friendly)
    pub title: Option<types::String>,

    /// Subordinate title of the plan definition
    pub subtitle: Option<types::String>,

    /// order-set | clinical-protocol | eca-rule | workflow-definition
    pub r#type: Option<types::CodeableConcept>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Type of individual the plan definition is focused on
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// Type of individual the plan definition is focused on
    pub subject_reference: Option<types::Reference>,

    /// Type of individual the plan definition is focused on
    pub subject_canonical: Option<types::Canonical>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the plan definition
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for plan definition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this plan definition is defined
    pub purpose: Option<types::Markdown>,

    /// Describes the clinical usage of the plan
    pub usage: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the plan definition was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the plan definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the plan definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Logic used by the plan definition
    pub library: Option<Vec<types::Canonical>>,

    /// What the plan is trying to accomplish
    pub goal: Option<Vec<PlanDefinitionGoal>>,

    /// Actors within the plan
    pub actor: Option<Vec<PlanDefinitionActor>>,

    /// Action defined by the plan
    pub action: Option<Vec<PlanDefinitionAction>>,

    /// Preconditions for service
    pub as_needed_boolean: Option<types::Boolean>,

    /// Preconditions for service
    pub as_needed_codeable_concept: Option<types::CodeableConcept>,
}

/// What the plan is trying to accomplish.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionGoal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// E.g. Treatment, dietary, behavioral
    pub category: Option<types::CodeableConcept>,

    /// Code or text describing the goal
    pub description: types::CodeableConcept,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<types::CodeableConcept>,

    /// When goal pursuit begins
    pub start: Option<types::CodeableConcept>,

    /// What does the goal address
    pub addresses: Option<Vec<types::CodeableConcept>>,

    /// Supporting documentation for the goal
    pub documentation: Option<Vec<types::RelatedArtifact>>,

    /// Target outcome for the goal
    pub target: Option<Vec<PlanDefinitionGoalTarget>>,
}

/// Target outcome for the goal.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionGoalTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The parameter whose value is to be tracked
    pub measure: Option<types::CodeableConcept>,

    /// The target value to be achieved
    pub detail_quantity: Option<types::Quantity>,

    /// The target value to be achieved
    pub detail_range: Option<types::Range>,

    /// The target value to be achieved
    pub detail_codeable_concept: Option<types::CodeableConcept>,

    /// The target value to be achieved
    pub detail_string: Option<types::String>,

    /// The target value to be achieved
    pub detail_boolean: Option<types::Boolean>,

    /// The target value to be achieved
    pub detail_integer: Option<types::Integer>,

    /// The target value to be achieved
    pub detail_ratio: Option<types::Ratio>,

    /// Reach goal within
    pub due: Option<types::Duration>,
}

/// Actors within the plan.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// User-visible title
    pub title: Option<types::String>,

    /// Describes the actor
    pub description: Option<types::Markdown>,

    /// Who or what can be this actor
    pub option: Vec<PlanDefinitionActorOption>,
}

/// Who or what can be this actor.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActorOption {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<types::Code>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,
}

/// Action defined by the plan.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique id for the action in the PlanDefinition
    pub link_id: Option<types::String>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<types::String>,

    /// User-visible title
    pub title: Option<types::String>,

    /// Brief description of the action
    pub description: Option<types::Markdown>,

    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::Markdown>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// Code representing the meaning of the action or sub-actions
    pub code: Option<types::CodeableConcept>,

    /// Why the action should be performed
    pub reason: Option<Vec<types::CodeableConcept>>,

    /// Supporting documentation for the intended performer of the action
    pub documentation: Option<Vec<types::RelatedArtifact>>,

    /// What goals this action supports
    pub goal_id: Option<Vec<types::Id>>,

    /// Type of individual the action is focused on
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// Type of individual the action is focused on
    pub subject_reference: Option<types::Reference>,

    /// Type of individual the action is focused on
    pub subject_canonical: Option<types::Canonical>,

    /// When the action should be triggered
    pub trigger: Option<Vec<types::TriggerDefinition>>,

    /// Whether or not the action is applicable
    pub condition: Option<Vec<PlanDefinitionActionCondition>>,

    /// Input data requirements
    pub input: Option<Vec<PlanDefinitionActionInput>>,

    /// Output data definition
    pub output: Option<Vec<PlanDefinitionActionOutput>>,

    /// Relationship to another action
    pub related_action: Option<Vec<PlanDefinitionActionRelatedAction>>,

    /// When the action should take place
    pub timing_age: Option<types::Age>,

    /// When the action should take place
    pub timing_duration: Option<types::Duration>,

    /// When the action should take place
    pub timing_range: Option<types::Range>,

    /// When the action should take place
    pub timing_timing: Option<types::Timing>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

    /// Who should participate in the action
    pub participant: Option<Vec<PlanDefinitionActionParticipant>>,

    /// create | update | remove | fire-event
    pub r#type: Option<types::CodeableConcept>,

    /// visual-group | logical-group | sentence-group
    pub grouping_behavior: Option<types::Code>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior: Option<types::Code>,

    /// must | could | must-unless-documented
    pub required_behavior: Option<types::Code>,

    /// yes | no
    pub precheck_behavior: Option<types::Code>,

    /// single | multiple
    pub cardinality_behavior: Option<types::Code>,

    /// Description of the activity to be performed
    pub definition_canonical: Option<types::Canonical>,

    /// Description of the activity to be performed
    pub definition_uri: Option<types::Uri>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,

    /// Dynamic aspects of the definition
    pub dynamic_value: Option<Vec<PlanDefinitionActionDynamicValue>>,

    /// A sub-action
    pub action: Option<Vec<PlanDefinitionAction>>,
}

/// Whether or not the action is applicable.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// applicability | start | stop
    pub kind: types::Code,

    /// Boolean-valued expression
    pub expression: Option<types::Expression>,
}

/// Input data requirements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// User-visible title
    pub title: Option<types::String>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::Id>,
}

/// Output data definition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// User-visible title
    pub title: Option<types::String>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::String>,
}

/// Relationship to another action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What action is this related to
    pub target_id: types::Id,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub relationship: types::Code,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub end_relationship: Option<types::Code>,

    /// Time offset for the relationship
    pub offset_duration: Option<types::Duration>,

    /// Time offset for the relationship
    pub offset_range: Option<types::Range>,
}

/// Who should participate in the action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What actor
    pub actor_id: Option<types::String>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<types::Code>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,
}

/// Dynamic aspects of the definition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionDynamicValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The path to the element to be set dynamically
    pub path: Option<types::String>,

    /// An expression that provides the dynamic value for the customization
    pub expression: Option<types::Expression>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PlanDefinition;

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
