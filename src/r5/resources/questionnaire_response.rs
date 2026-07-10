//! QuestionnaireResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
//!
//! Version: 5.0.0
//!
//! QuestionnaireResponse Resource: A structured set of questions and their answers.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A structured set of questions and their answers.
///
/// A QuestionnaireResponse captures the answers gathered when a Questionnaire is
/// completed. The questions are ordered and grouped into coherent subsets that
/// correspond to the grouping structure of the Questionnaire being responded to.
/// Each response records the subject, author, and source of the answers, along
/// with a nested tree of items that hold the individual answers and their child
/// items.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire_response::QuestionnaireResponse;
///
/// let value = QuestionnaireResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: QuestionnaireResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponse {
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

    /// Business identifier for this set of answers
    pub identifier: Option<Vec<types::Identifier>>,

    /// Request fulfilled by this QuestionnaireResponse
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// Canonical URL of Questionnaire being answered
    pub questionnaire: types::Canonical,

    /// in-progress | completed | amended | entered-in-error | stopped
    pub status: types::Code,

    /// The subject of the questions
    pub subject: Option<types::Reference>,

    /// Encounter the questionnaire response is part of
    pub encounter: Option<types::Reference>,

    /// Date the answers were gathered
    pub authored: Option<types::DateTime>,

    /// The individual or device that received and recorded the answers
    pub author: Option<types::Reference>,

    /// The individual or device that answered the questions
    pub source: Option<types::Reference>,

    /// Groups and questions
    pub item: Option<Vec<QuestionnaireResponseItem>>,
}

/// Groups and questions.
///
/// An item within a QuestionnaireResponse. An item may be a group that contains
/// child items, or a question that carries one or more answers. Each item points
/// back to a specific item in the source Questionnaire via its `linkId`.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponseItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pointer to specific item from Questionnaire
    pub link_id: types::String,

    /// ElementDefinition - details for the item
    pub definition: Option<types::Uri>,

    /// Name for group or question text
    pub text: Option<types::String>,

    /// The response(s) to the question
    pub answer: Option<Vec<QuestionnaireResponseItemAnswer>>,

    /// Child items of group item
    pub item: Option<Vec<QuestionnaireResponseItem>>,
}

/// The response(s) to the question.
///
/// A single answer to a question item, expressed as one of the supported value
/// types. An answer may itself contain nested child items, allowing complex,
/// grouped responses beneath a single question.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponseItemAnswer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Single-valued answer to the question (boolean)
    pub value_boolean: Option<types::Boolean>,

    /// Single-valued answer to the question (decimal)
    pub value_decimal: Option<types::Decimal>,

    /// Single-valued answer to the question (integer)
    pub value_integer: Option<types::Integer>,

    /// Single-valued answer to the question (date)
    pub value_date: Option<types::Date>,

    /// Single-valued answer to the question (dateTime)
    pub value_date_time: Option<types::DateTime>,

    /// Single-valued answer to the question (time)
    pub value_time: Option<types::Time>,

    /// Single-valued answer to the question (string)
    pub value_string: Option<types::String>,

    /// Single-valued answer to the question (uri)
    pub value_uri: Option<types::Uri>,

    /// Single-valued answer to the question (Attachment)
    pub value_attachment: Option<types::Attachment>,

    /// Single-valued answer to the question (Coding)
    pub value_coding: Option<types::Coding>,

    /// Single-valued answer to the question (Quantity)
    pub value_quantity: Option<types::Quantity>,

    /// Single-valued answer to the question (Reference)
    pub value_reference: Option<types::Reference>,

    /// Child items of question
    pub item: Option<Vec<QuestionnaireResponseItem>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = QuestionnaireResponse;

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
