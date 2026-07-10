//! Questionnaire
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
//!
//! Version: 5.0.0
//!
//! Questionnaire Resource: A structured set of questions intended to guide the collection of answers from end-users.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A structured set of questions intended to guide the collection of answers
/// from end-users.
///
/// Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection. The
/// Questionnaire resource defines the questions and structure, while the
/// answers themselves are captured in a QuestionnaireResponse. In FHIR R5 it is
/// a canonical, versionable resource commonly used for forms such as patient
/// intake, surveys, and clinical assessments, with items that can nest to any
/// depth and support conditional display via enableWhen logic.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire::Questionnaire;
///
/// let value = Questionnaire::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Questionnaire = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Questionnaire {
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

    /// Canonical identifier for this questionnaire, represented as an absolute URI (globally unique)
    pub url: Option<types::Uri>,

    /// Business identifier for questionnaire
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the questionnaire
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this questionnaire (computer friendly)
    pub name: Option<types::String>,

    /// Name for this questionnaire (human friendly)
    pub title: Option<types::String>,

    /// Based on Questionnaire
    pub derived_from: Option<Vec<types::Canonical>>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Resource that can be subject of QuestionnaireResponse
    pub subject_type: Option<Vec<types::Code>>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the questionnaire
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for questionnaire (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this questionnaire is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the questionnaire was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the questionnaire was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the questionnaire is expected to be used
    pub effective_period: Option<types::Period>,

    /// Concept that represents the overall questionnaire
    pub code: Option<Vec<types::Coding>>,

    /// Questions and sections within the Questionnaire
    pub item: Option<Vec<QuestionnaireItem>>,
}

/// Questions and sections within the Questionnaire.
///
/// Each item is either a group (containing nested items), a display element, or
/// a question that collects an answer of a particular type.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique id for item in questionnaire
    pub link_id: types::String,

    /// ElementDefinition - details for the item
    pub definition: Option<types::Uri>,

    /// Corresponding concept for this item in a terminology
    pub code: Option<Vec<types::Coding>>,

    /// E.g. "1(a)", "2.5.3"
    pub prefix: Option<types::String>,

    /// Primary text for the item
    pub text: Option<types::String>,

    /// group | display | boolean | decimal | integer | date | dateTime +
    pub r#type: types::Code,

    /// Only allow data when
    pub enable_when: Option<Vec<QuestionnaireItemEnableWhen>>,

    /// all | any
    pub enable_behavior: Option<types::Code>,

    /// hidden | protected
    pub disabled_display: Option<types::Code>,

    /// Whether the item must be included in data results
    pub required: Option<types::Boolean>,

    /// Whether the item may repeat
    pub repeats: Option<types::Boolean>,

    /// Don't allow human editing
    pub read_only: Option<types::Boolean>,

    /// No more than these many characters
    pub max_length: Option<types::Integer>,

    /// optionsOnly | optionsOrType | optionsOrString
    pub answer_constraint: Option<types::Code>,

    /// ValueSet containing permitted answers
    pub answer_value_set: Option<types::Canonical>,

    /// Permitted answer
    pub answer_option: Option<Vec<QuestionnaireItemAnswerOption>>,

    /// Initial value(s) when item is first rendered
    pub initial: Option<Vec<QuestionnaireItemInitial>>,

    /// Nested questionnaire items
    pub item: Option<Vec<QuestionnaireItem>>,
}

/// Only allow data when a specified condition is met.
///
/// Defines a question/answer pair that must hold for the enclosing item to be
/// enabled, combined via the item's enableBehavior when multiple are present.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItemEnableWhen {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The linkId of question that determines whether item is enabled/disabled
    pub question: types::String,

    /// exists | = | != | > | < | >= | <=
    pub operator: types::Code,

    /// Value for question comparison based on operator
    pub answer_boolean: Option<types::Boolean>,

    /// Value for question comparison based on operator
    pub answer_decimal: Option<types::Decimal>,

    /// Value for question comparison based on operator
    pub answer_integer: Option<types::Integer>,

    /// Value for question comparison based on operator
    pub answer_date: Option<types::Date>,

    /// Value for question comparison based on operator
    pub answer_date_time: Option<types::DateTime>,

    /// Value for question comparison based on operator
    pub answer_time: Option<types::Time>,

    /// Value for question comparison based on operator
    pub answer_string: Option<types::String>,

    /// Value for question comparison based on operator
    pub answer_coding: Option<types::Coding>,

    /// Value for question comparison based on operator
    pub answer_quantity: Option<types::Quantity>,

    /// Value for question comparison based on operator
    pub answer_reference: Option<types::Reference>,
}

/// Permitted answer.
///
/// One of the discrete answer options offered for a question item, optionally
/// marked as selected by default.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItemAnswerOption {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Answer value
    pub value_integer: Option<types::Integer>,

    /// Answer value
    pub value_date: Option<types::Date>,

    /// Answer value
    pub value_time: Option<types::Time>,

    /// Answer value
    pub value_string: Option<types::String>,

    /// Answer value
    pub value_coding: Option<types::Coding>,

    /// Answer value
    pub value_reference: Option<types::Reference>,

    /// Whether option is selected by default
    pub initial_selected: Option<types::Boolean>,
}

/// Initial value(s) when item is first rendered.
///
/// Supplies the default value for a question item using the appropriate typed
/// value for the item's answer type.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItemInitial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Actual value for initializing the question
    pub value_boolean: Option<types::Boolean>,

    /// Actual value for initializing the question
    pub value_decimal: Option<types::Decimal>,

    /// Actual value for initializing the question
    pub value_integer: Option<types::Integer>,

    /// Actual value for initializing the question
    pub value_date: Option<types::Date>,

    /// Actual value for initializing the question
    pub value_date_time: Option<types::DateTime>,

    /// Actual value for initializing the question
    pub value_time: Option<types::Time>,

    /// Actual value for initializing the question
    pub value_string: Option<types::String>,

    /// Actual value for initializing the question
    pub value_uri: Option<types::Uri>,

    /// Actual value for initializing the question
    pub value_attachment: Option<types::Attachment>,

    /// Actual value for initializing the question
    pub value_coding: Option<types::Coding>,

    /// Actual value for initializing the question
    pub value_quantity: Option<types::Quantity>,

    /// Actual value for initializing the question
    pub value_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Questionnaire;

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
