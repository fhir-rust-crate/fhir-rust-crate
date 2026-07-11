//! RequestOrchestration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RequestOrchestration
//!
//! Version: 5.0.0
//!
//! RequestOrchestration Resource: A set of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A set of related requests that can be used to capture intended activities
/// that have inter-dependencies such as "give this medication after that one".
///
/// RequestOrchestration lets systems represent a group of coordinated requests
/// together with the actions, conditions, timing, and relationships that tie
/// them together. It is commonly produced by applying a PlanDefinition and is
/// used to drive clinical decision support and workflow orchestration in FHIR
/// R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::request_orchestration::RequestOrchestration;
///
/// let value = RequestOrchestration::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: RequestOrchestration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestration {
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

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// Fulfills plan, proposal, or order
    pub based_on: Option<Vec<types::Reference>>,

    /// Request(s) replaced by this request
    pub replaces: Option<Vec<types::Reference>>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: types::Code,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: types::Code,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// What's being requested/ordered
    pub code: Option<types::CodeableConcept>,

    /// Who the request orchestration is about
    pub subject: Option<types::Reference>,

    /// Created as part of
    pub encounter: Option<types::Reference>,

    /// When the request orchestration was authored
    pub authored_on: Option<types::DateTime>,

    /// Device or practitioner that authored the request orchestration
    pub author: Option<types::Reference>,

    /// Why the request orchestration is needed
    pub reason: Option<Vec<types::CodeableReference>>,

    /// What goals
    pub goal: Option<Vec<types::Reference>>,

    /// Additional notes about the response
    pub note: Option<Vec<types::Annotation>>,

    /// Proposed actions, if any
    pub action: Option<Vec<RequestOrchestrationAction>>,
}

/// Proposed actions, if any.
///
/// An action to be taken as part of the request orchestration, potentially
/// containing nested sub-actions.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pointer to specific item from the PlanDefinition
    pub link_id: Option<types::String>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<types::String>,

    /// User-visible title
    pub title: Option<types::String>,

    /// Short description of the action
    pub description: Option<types::Markdown>,

    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::Markdown>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// Code representing the meaning of the action or sub-actions
    pub code: Option<Vec<types::CodeableConcept>>,

    /// Supporting documentation for the intended performer of the action
    pub documentation: Option<Vec<types::RelatedArtifact>>,

    /// What goals
    pub goal: Option<Vec<types::Reference>>,

    /// Whether or not the action is applicable
    pub condition: Option<Vec<RequestOrchestrationActionCondition>>,

    /// Input data requirements
    pub input: Option<Vec<RequestOrchestrationActionInput>>,

    /// Output data definition
    pub output: Option<Vec<RequestOrchestrationActionOutput>>,

    /// Relationship to another action
    pub related_action: Option<Vec<RequestOrchestrationActionRelatedAction>>,

    /// When the action should take place
    pub timing_date_time: Option<types::DateTime>,

    /// When the action should take place
    pub timing_age: Option<types::Age>,

    /// When the action should take place
    pub timing_period: Option<types::Period>,

    /// When the action should take place
    pub timing_duration: Option<types::Duration>,

    /// When the action should take place
    pub timing_range: Option<types::Range>,

    /// When the action should take place
    pub timing_timing: Option<types::Timing>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

    /// Who should perform the action
    pub participant: Option<Vec<RequestOrchestrationActionParticipant>>,

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

    /// The target of the action
    pub resource: Option<types::Reference>,

    /// Description of the activity to be performed
    pub definition_canonical: Option<types::Canonical>,

    /// Description of the activity to be performed
    pub definition_uri: Option<types::Uri>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,

    /// Dynamic aspects of the definition
    pub dynamic_value: Option<Vec<RequestOrchestrationActionDynamicValue>>,

    /// Sub action
    pub action: Option<Vec<RequestOrchestrationAction>>,
}

/// Whether or not the action is applicable.
///
/// An expression that describes applicability criteria, or start/stop
/// conditions for the action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionCondition {
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
///
/// Defines input data requirements for the action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionInput {
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
///
/// Defines the outputs of the action, if any.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionOutput {
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
///
/// A relationship to another action such as "before" or "after".
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What action this is related to
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

/// Who should perform the action.
///
/// The participant that should perform or be responsible for the action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionParticipant {
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

    /// E.g. Nurse, Surgeon, Parent, etc
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,

    /// Who/what is participating?
    pub actor_canonical: Option<types::Canonical>,

    /// Who/what is participating?
    pub actor_reference: Option<types::Reference>,
}

/// Dynamic aspects of the definition.
///
/// Customizations that should be applied to the statically defined resource,
/// expressed as a path and an expression.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionDynamicValue {
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
    type T = RequestOrchestration;

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
