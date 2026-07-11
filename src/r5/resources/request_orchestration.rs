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
/// Clinically, a RequestOrchestration captures the output of applying a care
/// pathway, protocol, or order set (typically expressed as a PlanDefinition)
/// to a particular patient's circumstances. Rather than issuing each order,
/// referral, or task as an unrelated resource, the orchestration groups them
/// as nested actions with explicit conditions (such as an applicability
/// expression that must evaluate true), timing (a fixed date/time, age,
/// period, duration, range, or full Timing schedule), and relationships to
/// other actions (for example "start this after that one completes").
/// Consumers such as order entry or clinical decision support systems walk
/// the action tree to determine what to propose, schedule, or execute, and
/// downstream systems create the concrete request resources (for example
/// MedicationRequest or ServiceRequest) once an action is accepted.
///
/// # See also
///
/// - The subject of a RequestOrchestration is typically a
///   [`Patient`](crate::r5::resources::patient::Patient), referenced via the
///   `subject` field.
/// - Coded values such as `code`, `priority`, and action `code` use
///   [`CodeableConcept`](crate::r5::types::CodeableConcept).
/// - Individual proposed activities are represented by nested
///   `RequestOrchestrationAction` entries, which may themselves be produced
///   from a `PlanDefinition`.
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

    /// Business identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    pub instantiates_canonical_ext: Option<Vec<Option<types::Element>>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<Vec<Option<types::Element>>>,

    /// Fulfills plan, proposal, or order
    pub based_on: Option<Vec<types::Reference>>,

    /// Request(s) replaced by this request
    pub replaces: Option<Vec<types::Reference>>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// The lifecycle status of this request orchestration: draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The degree of authority/intentionality of the orchestration: proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: types::Code,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Indicates how quickly the orchestration should be addressed: routine | urgent | asap | stat
    pub priority: Option<types::Code>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// A code that identifies what the overall orchestration is meant to accomplish, using a [`CodeableConcept`](crate::r5::types::CodeableConcept)
    pub code: Option<types::CodeableConcept>,

    /// The [`Patient`](crate::r5::resources::patient::Patient) or other subject that the request orchestration is about
    pub subject: Option<types::Reference>,

    /// Created as part of
    pub encounter: Option<types::Reference>,

    /// When the request orchestration was authored
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`).
    #[serde(rename = "_prefix")]
    pub prefix_ext: Option<types::Element>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Short description of the action
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::Markdown>,
    /// Primitive extension sibling for [`text_equivalent`](Self::text_equivalent) (FHIR `_textEquivalent`).
    #[serde(rename = "_textEquivalent")]
    pub text_equivalent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`grouping_behavior`](Self::grouping_behavior) (FHIR `_groupingBehavior`).
    #[serde(rename = "_groupingBehavior")]
    pub grouping_behavior_ext: Option<types::Element>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior: Option<types::Code>,
    /// Primitive extension sibling for [`selection_behavior`](Self::selection_behavior) (FHIR `_selectionBehavior`).
    #[serde(rename = "_selectionBehavior")]
    pub selection_behavior_ext: Option<types::Element>,

    /// must | could | must-unless-documented
    pub required_behavior: Option<types::Code>,
    /// Primitive extension sibling for [`required_behavior`](Self::required_behavior) (FHIR `_requiredBehavior`).
    #[serde(rename = "_requiredBehavior")]
    pub required_behavior_ext: Option<types::Element>,

    /// yes | no
    pub precheck_behavior: Option<types::Code>,
    /// Primitive extension sibling for [`precheck_behavior`](Self::precheck_behavior) (FHIR `_precheckBehavior`).
    #[serde(rename = "_precheckBehavior")]
    pub precheck_behavior_ext: Option<types::Element>,

    /// single | multiple
    pub cardinality_behavior: Option<types::Code>,
    /// Primitive extension sibling for [`cardinality_behavior`](Self::cardinality_behavior) (FHIR `_cardinalityBehavior`).
    #[serde(rename = "_cardinalityBehavior")]
    pub cardinality_behavior_ext: Option<types::Element>,

    /// The target of the action
    pub resource: Option<types::Reference>,

    /// Description of the activity to be performed
    pub definition_canonical: Option<types::Canonical>,

    /// Description of the activity to be performed
    pub definition_uri: Option<types::Uri>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`).
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::Id>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`).
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::String>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`).
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`).
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub relationship: types::Code,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`).
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub end_relationship: Option<types::Code>,
    /// Primitive extension sibling for [`end_relationship`](Self::end_relationship) (FHIR `_endRelationship`).
    #[serde(rename = "_endRelationship")]
    pub end_relationship_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`).
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

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
