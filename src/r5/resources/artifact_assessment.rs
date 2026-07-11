//! ArtifactAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
//!
//! Version: 5.0.0
//!
//! ArtifactAssessment Resource: This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ArtifactAssessment provides one or more comments, classifiers, or ratings
/// about a Resource and supports attribution and rights management metadata for
/// the added content. It is used in FHIR R5 to capture reviews, editorial
/// commentary, quality classifiers, and quantitative ratings that are attached
/// to a referenced or canonical artifact. Assessments carry workflow status and
/// disposition to track how comments are processed over their lifecycle.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::artifact_assessment::ArtifactAssessment;
///
/// let value = ArtifactAssessment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ArtifactAssessment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactAssessment {
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

    /// Additional identifier for the artifact assessment
    pub identifier: Option<Vec<types::Identifier>>,

    /// A short title for the assessment for use in displaying and selecting
    pub title: Option<types::String>,

    /// How to cite the comment or rating
    pub cite_as_reference: Option<types::Reference>,

    /// How to cite the comment or rating
    pub cite_as_markdown: Option<types::Markdown>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// When the artifact assessment was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the artifact assessment was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// The artifact assessed, commented upon or rated
    pub artifact_reference: Option<types::Reference>,

    /// The artifact assessed, commented upon or rated
    pub artifact_canonical: Option<types::Canonical>,

    /// The artifact assessed, commented upon or rated
    pub artifact_uri: Option<types::Uri>,

    /// Comment, classifier, or rating content
    pub content: Option<Vec<ArtifactAssessmentContent>>,

    /// submitted | triaged | waiting-for-input | resolved-no-change | resolved-change-required | deferred | duplicate | applied | published | entered-in-error
    pub workflow_status: Option<types::Code>,

    /// unresolved | not-persuasive | persuasive | persuasive-with-modification | not-persuasive-with-modification
    pub disposition: Option<types::Code>,
}

/// Comment, classifier, or rating content of an [`ArtifactAssessment`].
///
/// A content entry captures a single unit of assessment such as a comment, a
/// classifier, or a quantitative rating, along with attribution and rights
/// management metadata. Content entries can be nested via the `component`
/// field to build structured, multi-part assessments.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactAssessmentContent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// comment | classifier | rating | container | response | change-request
    pub information_type: Option<types::Code>,

    /// Brief summary of the content
    pub summary: Option<types::Markdown>,

    /// What type of content
    pub r#type: Option<types::CodeableConcept>,

    /// Rating, classifier, or assessment
    pub classifier: Option<Vec<types::CodeableConcept>>,

    /// Quantitative rating
    pub quantity: Option<types::Quantity>,

    /// Who authored the content
    pub author: Option<types::Reference>,

    /// What the comment is directed to
    pub path: Option<Vec<types::Uri>>,

    /// Additional information
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Acceptable to publicly share the resource content
    pub free_to_share: Option<types::Boolean>,

    /// Contained content
    pub component: Option<Vec<ArtifactAssessmentContent>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ArtifactAssessment;

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
