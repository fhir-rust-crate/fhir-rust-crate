//! Library
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Library
//!
//! Version: 5.0.0
//!
//! Library Resource: A general-purpose container for knowledge asset definitions.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Library resource is a general-purpose container for knowledge asset
/// definitions. It is used to describe, package, and expose knowledge assets
/// such as logic libraries, information model descriptions, and curated
/// collections of related assets, giving them consistent metadata, versioning,
/// and lifecycle governance. In FHIR R5 a Library most often carries clinical
/// decision support logic, typically expressed in Clinical Quality Language
/// (CQL) and its compiled ELM representation, as content that is either embedded
/// inline or referenced through an attachment. Alongside that content it
/// declares the input parameters the logic accepts and the data requirements it
/// consumes, so that engines can evaluate the logic and callers can understand
/// what data must be supplied. Libraries are commonly authored and shared as
/// part of a broader knowledge artifact ecosystem and are referenced by
/// measures, decision support rules, and other definitional resources.
///
/// # See also
///
/// A Library carries a resource type and topic classification using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), declares its inputs
/// with `ParameterDefinition`, describes the data it consumes with
/// `DataRequirement`, and holds its embedded or referenced payload in an
/// `Attachment`. Publication metadata is captured with `ContactDetail`,
/// `UsageContext`, and `RelatedArtifact`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::library::Library;
///
/// let value = Library::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Library = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Library {
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

    /// Canonical identifier for this library, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the library
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the library
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this library (computer friendly)
    pub name: Option<types::String>,

    /// Name for this library (human friendly)
    pub title: Option<types::String>,

    /// Subordinate title of the library
    pub subtitle: Option<types::String>,

    /// Publication lifecycle state of the library: draft, active, retired, or unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Kind of library, such as logic-library, model-definition, asset-collection, or module-definition
    pub r#type: types::CodeableConcept,

    /// Type of individual the library content is focused on
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// Type of individual the library content is focused on
    pub subject_reference: Option<types::Reference>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the library
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for library (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this library is defined
    pub purpose: Option<types::Markdown>,

    /// Describes the clinical usage of the library
    pub usage: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the library was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the library was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the library is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Input and output parameters that the library logic accepts or produces
    pub parameter: Option<Vec<types::ParameterDefinition>>,

    /// Data the library consumes when evaluated, describing required resources and elements
    pub data_requirement: Option<Vec<types::DataRequirement>>,

    /// Payload of the library, such as CQL or ELM logic, embedded inline or referenced by URL
    pub content: Option<Vec<types::Attachment>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Library;

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
