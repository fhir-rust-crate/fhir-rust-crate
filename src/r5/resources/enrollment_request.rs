//! EnrollmentRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EnrollmentRequest
//!
//! Version: 5.0.0
//!
//! EnrollmentRequest Resource: This resource provides the insurance enrollment details to the insurer regarding a specified coverage.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// EnrollmentRequest resource.
///
/// This resource provides the insurance enrollment details to the insurer
/// regarding a specified coverage. It is used to request that an insurer enroll
/// a candidate subject under a particular coverage, and is typically exchanged
/// between a provider and an insurer as part of the eligibility and enrollment
/// workflow in FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::enrollment_request::EnrollmentRequest;
///
/// let value = EnrollmentRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EnrollmentRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentRequest {
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

    /// Creation date
    pub created: Option<types::DateTime>,

    /// Target
    pub insurer: Option<types::Reference>,

    /// Responsible practitioner
    pub provider: Option<types::Reference>,

    /// The subject to be enrolled
    pub candidate: Option<types::Reference>,

    /// Insurance information
    pub coverage: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EnrollmentRequest;

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
