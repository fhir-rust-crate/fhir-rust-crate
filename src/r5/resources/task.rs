//! Task
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Task
//!
//! Version: 5.0.0
//!
//! Task Resource: A task to be performed.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Task
///
/// A task to be performed, tracked, and managed within a workflow. The Task
/// resource describes an activity that can be performed and tracks its state of
/// completion, capturing who requested it, who is responsible for fulfilling it,
/// what it acts upon, and the inputs and outputs associated with performing it.
/// Tasks are commonly used to coordinate work between systems and participants in
/// clinical and administrative processes in FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::task::Task;
///
/// let value = Task::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Task = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Task {
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

    /// Task Instance Identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Formal definition of task
    pub instantiates_canonical: Option<types::Canonical>,

    /// Formal definition of task
    pub instantiates_uri: Option<types::Uri>,

    /// Request fulfilled by this task
    pub based_on: Option<Vec<types::Reference>>,

    /// Requisition or grouper id
    pub group_identifier: Option<types::Identifier>,

    /// Composite task
    pub part_of: Option<Vec<types::Reference>>,

    /// draft | requested | received | accepted | +
    pub status: types::Code,

    /// Reason for current status
    pub status_reason: Option<types::CodeableReference>,

    /// E.g. "Specimen collected", "IV prepped"
    pub business_status: Option<types::CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order | +
    pub intent: types::Code,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// True if Task is prohibiting action
    pub do_not_perform: Option<types::Boolean>,

    /// Task Type
    pub code: Option<types::CodeableConcept>,

    /// Human-readable explanation of task
    pub description: Option<types::String>,

    /// What task is acting on
    pub focus: Option<types::Reference>,

    /// Beneficiary of the Task
    pub r#for: Option<types::Reference>,

    /// Healthcare event during which this task originated
    pub encounter: Option<types::Reference>,

    /// When the task should be performed
    pub requested_period: Option<types::Period>,

    /// Start and end time of execution
    pub execution_period: Option<types::Period>,

    /// Task Creation Date
    pub authored_on: Option<types::DateTime>,

    /// Task Last Modified Date
    pub last_modified: Option<types::DateTime>,

    /// Who is asking for task to be done
    pub requester: Option<types::Reference>,

    /// Who should perform Task
    pub requested_performer: Option<Vec<types::CodeableReference>>,

    /// Responsible individual
    pub owner: Option<types::Reference>,

    /// Who or what performed the task
    pub performer: Option<Vec<TaskPerformer>>,

    /// Where task occurs
    pub location: Option<types::Reference>,

    /// Why task is needed
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<types::Reference>>,

    /// Comments made about the task
    pub note: Option<Vec<types::Annotation>>,

    /// Key events in history of the Task
    pub relevant_history: Option<Vec<types::Reference>>,

    /// Constraints on fulfillment tasks
    pub restriction: Option<TaskRestriction>,

    /// Information used to perform task
    pub input: Option<Vec<TaskInput>>,

    /// Information produced as part of task
    pub output: Option<Vec<TaskOutput>>,
}

/// Task.performer
///
/// The entity who performed the requested task.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaskPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who performed the task
    pub actor: types::Reference,
}

/// Task.restriction
///
/// If the Task.focus is a request resource and the task is seeking fulfillment,
/// this element identifies any limitations on what parts of the referenced request
/// should be actioned.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaskRestriction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// How many times to repeat
    pub repetitions: Option<types::PositiveInt>,

    /// When fulfillment is sought
    pub period: Option<types::Period>,

    /// For whom is fulfillment sought?
    pub recipient: Option<Vec<types::Reference>>,
}

/// Task.input
///
/// Additional information that may be needed in the execution of the task.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaskInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for the input
    pub r#type: types::CodeableConcept,

    /// Content to use in performing the task
    pub value_base64_binary: Option<types::Base64Binary>,

    /// Content to use in performing the task
    pub value_boolean: Option<types::Boolean>,

    /// Content to use in performing the task
    pub value_canonical: Option<types::Canonical>,

    /// Content to use in performing the task
    pub value_code: Option<types::Code>,

    /// Content to use in performing the task
    pub value_date: Option<types::Date>,

    /// Content to use in performing the task
    pub value_date_time: Option<types::DateTime>,

    /// Content to use in performing the task
    pub value_decimal: Option<types::Decimal>,

    /// Content to use in performing the task
    pub value_id: Option<types::Id>,

    /// Content to use in performing the task
    pub value_instant: Option<types::Instant>,

    /// Content to use in performing the task
    pub value_integer: Option<types::Integer>,

    /// Content to use in performing the task
    pub value_integer64: Option<types::Integer64>,

    /// Content to use in performing the task
    pub value_markdown: Option<types::Markdown>,

    /// Content to use in performing the task
    pub value_oid: Option<types::Oid>,

    /// Content to use in performing the task
    pub value_positive_int: Option<types::PositiveInt>,

    /// Content to use in performing the task
    pub value_string: Option<types::String>,

    /// Content to use in performing the task
    pub value_time: Option<types::Time>,

    /// Content to use in performing the task
    pub value_unsigned_int: Option<types::UnsignedInt>,

    /// Content to use in performing the task
    pub value_uri: Option<types::Uri>,

    /// Content to use in performing the task
    pub value_url: Option<types::Url>,

    /// Content to use in performing the task
    pub value_uuid: Option<types::Uuid>,

    /// Content to use in performing the task
    pub value_address: Option<types::Address>,

    /// Content to use in performing the task
    pub value_age: Option<types::Age>,

    /// Content to use in performing the task
    pub value_annotation: Option<types::Annotation>,

    /// Content to use in performing the task
    pub value_attachment: Option<types::Attachment>,

    /// Content to use in performing the task
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Content to use in performing the task
    pub value_codeable_reference: Option<types::CodeableReference>,

    /// Content to use in performing the task
    pub value_coding: Option<types::Coding>,

    /// Content to use in performing the task
    pub value_contact_point: Option<types::ContactPoint>,

    /// Content to use in performing the task
    pub value_count: Option<types::Count>,

    /// Content to use in performing the task
    pub value_distance: Option<types::Distance>,

    /// Content to use in performing the task
    pub value_duration: Option<types::Duration>,

    /// Content to use in performing the task
    pub value_human_name: Option<types::HumanName>,

    /// Content to use in performing the task
    pub value_identifier: Option<types::Identifier>,

    /// Content to use in performing the task
    pub value_money: Option<types::Money>,

    /// Content to use in performing the task
    pub value_period: Option<types::Period>,

    /// Content to use in performing the task
    pub value_quantity: Option<types::Quantity>,

    /// Content to use in performing the task
    pub value_range: Option<types::Range>,

    /// Content to use in performing the task
    pub value_ratio: Option<types::Ratio>,

    /// Content to use in performing the task
    pub value_ratio_range: Option<types::RatioRange>,

    /// Content to use in performing the task
    pub value_reference: Option<types::Reference>,

    /// Content to use in performing the task
    pub value_sampled_data: Option<types::SampledData>,

    /// Content to use in performing the task
    pub value_signature: Option<types::Signature>,

    /// Content to use in performing the task
    pub value_timing: Option<types::Timing>,

    /// Content to use in performing the task
    pub value_contact_detail: Option<types::ContactDetail>,

    /// Content to use in performing the task
    pub value_data_requirement: Option<types::DataRequirement>,

    /// Content to use in performing the task
    pub value_expression: Option<types::Expression>,

    /// Content to use in performing the task
    pub value_parameter_definition: Option<types::ParameterDefinition>,

    /// Content to use in performing the task
    pub value_related_artifact: Option<types::RelatedArtifact>,

    /// Content to use in performing the task
    pub value_trigger_definition: Option<types::TriggerDefinition>,

    /// Content to use in performing the task
    pub value_usage_context: Option<types::UsageContext>,

    /// Content to use in performing the task
    pub value_availability: Option<types::Availability>,

    /// Content to use in performing the task
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>,

    /// Content to use in performing the task
    pub value_dosage: Option<types::Dosage>,

    /// Content to use in performing the task
    pub value_meta: Option<types::Meta>,
}

/// Task.output
///
/// Outputs produced by the Task.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaskOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for output
    pub r#type: types::CodeableConcept,

    /// Result of output
    pub value_base64_binary: Option<types::Base64Binary>,

    /// Result of output
    pub value_boolean: Option<types::Boolean>,

    /// Result of output
    pub value_canonical: Option<types::Canonical>,

    /// Result of output
    pub value_code: Option<types::Code>,

    /// Result of output
    pub value_date: Option<types::Date>,

    /// Result of output
    pub value_date_time: Option<types::DateTime>,

    /// Result of output
    pub value_decimal: Option<types::Decimal>,

    /// Result of output
    pub value_id: Option<types::Id>,

    /// Result of output
    pub value_instant: Option<types::Instant>,

    /// Result of output
    pub value_integer: Option<types::Integer>,

    /// Result of output
    pub value_integer64: Option<types::Integer64>,

    /// Result of output
    pub value_markdown: Option<types::Markdown>,

    /// Result of output
    pub value_oid: Option<types::Oid>,

    /// Result of output
    pub value_positive_int: Option<types::PositiveInt>,

    /// Result of output
    pub value_string: Option<types::String>,

    /// Result of output
    pub value_time: Option<types::Time>,

    /// Result of output
    pub value_unsigned_int: Option<types::UnsignedInt>,

    /// Result of output
    pub value_uri: Option<types::Uri>,

    /// Result of output
    pub value_url: Option<types::Url>,

    /// Result of output
    pub value_uuid: Option<types::Uuid>,

    /// Result of output
    pub value_address: Option<types::Address>,

    /// Result of output
    pub value_age: Option<types::Age>,

    /// Result of output
    pub value_annotation: Option<types::Annotation>,

    /// Result of output
    pub value_attachment: Option<types::Attachment>,

    /// Result of output
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Result of output
    pub value_codeable_reference: Option<types::CodeableReference>,

    /// Result of output
    pub value_coding: Option<types::Coding>,

    /// Result of output
    pub value_contact_point: Option<types::ContactPoint>,

    /// Result of output
    pub value_count: Option<types::Count>,

    /// Result of output
    pub value_distance: Option<types::Distance>,

    /// Result of output
    pub value_duration: Option<types::Duration>,

    /// Result of output
    pub value_human_name: Option<types::HumanName>,

    /// Result of output
    pub value_identifier: Option<types::Identifier>,

    /// Result of output
    pub value_money: Option<types::Money>,

    /// Result of output
    pub value_period: Option<types::Period>,

    /// Result of output
    pub value_quantity: Option<types::Quantity>,

    /// Result of output
    pub value_range: Option<types::Range>,

    /// Result of output
    pub value_ratio: Option<types::Ratio>,

    /// Result of output
    pub value_ratio_range: Option<types::RatioRange>,

    /// Result of output
    pub value_reference: Option<types::Reference>,

    /// Result of output
    pub value_sampled_data: Option<types::SampledData>,

    /// Result of output
    pub value_signature: Option<types::Signature>,

    /// Result of output
    pub value_timing: Option<types::Timing>,

    /// Result of output
    pub value_contact_detail: Option<types::ContactDetail>,

    /// Result of output
    pub value_data_requirement: Option<types::DataRequirement>,

    /// Result of output
    pub value_expression: Option<types::Expression>,

    /// Result of output
    pub value_parameter_definition: Option<types::ParameterDefinition>,

    /// Result of output
    pub value_related_artifact: Option<types::RelatedArtifact>,

    /// Result of output
    pub value_trigger_definition: Option<types::TriggerDefinition>,

    /// Result of output
    pub value_usage_context: Option<types::UsageContext>,

    /// Result of output
    pub value_availability: Option<types::Availability>,

    /// Result of output
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>,

    /// Result of output
    pub value_dosage: Option<types::Dosage>,

    /// Result of output
    pub value_meta: Option<types::Meta>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Task;

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
