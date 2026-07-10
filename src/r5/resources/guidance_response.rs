//! GuidanceResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
//!
//! Version: 5.0.0
//!
//! GuidanceResponse Resource: A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A guidance response is the formal response to a guidance request,
/// including any output parameters returned by the evaluation, as well as
/// the description of any proposed actions to be taken. It is typically
/// produced by a clinical decision support service after evaluating a
/// knowledge module (such as a rule, ECA rule, or order set) against a
/// patient's data, and may carry proposed actions, data requirements, or
/// evaluation messages.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::guidance_response::GuidanceResponse;
///
/// let value = GuidanceResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: GuidanceResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GuidanceResponse {
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

    /// The identifier of the request associated with this response, if any
    pub request_identifier: Option<types::Identifier>,

    /// Business identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// What guidance was requested
    pub module_uri: Option<types::Uri>,

    /// What guidance was requested
    pub module_canonical: Option<types::Canonical>,

    /// What guidance was requested
    pub module_codeable_concept: Option<types::CodeableConcept>,

    /// success | data-requested | data-required | in-progress | failure | entered-in-error
    pub status: types::Code,

    /// Patient the request was performed for
    pub subject: Option<types::Reference>,

    /// Encounter during which the response was returned
    pub encounter: Option<types::Reference>,

    /// When the guidance response was processed
    pub occurrence_date_time: Option<types::DateTime>,

    /// Device returning the guidance
    pub performer: Option<types::Reference>,

    /// Why guidance is needed
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Additional notes about the response
    pub note: Option<Vec<types::Annotation>>,

    /// Messages resulting from the evaluation of the artifact or artifacts
    pub evaluation_message: Option<types::Reference>,

    /// The output parameters of the evaluation, if any
    pub output_parameters: Option<types::Reference>,

    /// Proposed actions, if any
    pub result: Option<Vec<types::Reference>>,

    /// Additional required data
    pub data_requirement: Option<Vec<types::DataRequirement>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = GuidanceResponse;

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
