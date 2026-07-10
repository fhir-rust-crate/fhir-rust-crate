//! EnrollmentResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EnrollmentResponse
//!
//! Version: 5.0.0
//!
//! EnrollmentResponse Resource: This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// This resource provides enrollment and plan details from the processing of an
/// EnrollmentRequest resource.
///
/// EnrollmentResponse is the payer's reply to an `EnrollmentRequest`. It
/// communicates the outcome of processing a request to enroll a patient into a
/// coverage or plan, along with any disposition message describing the result.
/// It references the originating request, the responsible insurer organization,
/// and the provider that submitted the request. This resource is part of the
/// FHIR financial module.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::enrollment_response::EnrollmentResponse;
///
/// let value = EnrollmentResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EnrollmentResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentResponse {
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

    /// Business Identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<types::Code>,

    /// Claim reference
    pub request: Option<types::Reference>,

    /// queued | complete | error | partial
    pub outcome: Option<types::Code>,

    /// Disposition Message
    pub disposition: Option<types::String>,

    /// Creation date
    pub created: Option<types::DateTime>,

    /// Insurer
    pub organization: Option<types::Reference>,

    /// Responsible practitioner
    pub request_provider: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EnrollmentResponse;

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
