//! ImplementationGuide
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImplementationGuide
//!
//! Version: 5.0.0
//!
//! ImplementationGuide Resource: A set of rules of how a particular interoperability or standards problem is solved.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and
/// to publish a computable definition of all the parts. An ImplementationGuide
/// bundles together conformance resources, narrative pages, and build
/// parameters so that a specification can be rendered, validated, and
/// distributed as an NPM package.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::implementation_guide::ImplementationGuide;
///
/// let value = ImplementationGuide::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImplementationGuide = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuide {
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

    /// Canonical identifier for this implementation guide, represented as a URI (globally unique)
    pub url: types::Uri,

    /// Additional identifier for the implementation guide (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the implementation guide
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this implementation guide (computer friendly)
    pub name: types::String,

    /// Name for this implementation guide (human friendly)
    pub title: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the implementation guide
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for implementation guide (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this implementation guide is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// NPM Package name for IG
    pub package_id: types::Id,

    /// SPDX license code for this IG (or not-open-source)
    pub license: Option<types::Code>,

    /// FHIR Version(s) this Implementation Guide targets
    pub fhir_version: Vec<types::Code>,

    /// Another Implementation guide this depends on
    pub depends_on: Option<Vec<ImplementationGuideDependsOn>>,

    /// Profiles that apply globally
    pub global: Option<Vec<ImplementationGuideGlobal>>,

    /// Information needed to build the IG
    pub definition: Option<ImplementationGuideDefinition>,

    /// Information about an assembled IG
    pub manifest: Option<ImplementationGuideManifest>,
}

/// Another Implementation guide that this implementation guide depends on.
/// Dependencies are typically expressed against other published IGs whose
/// profiles, value sets, or examples are reused.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDependsOn {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identity of the IG that this depends on
    pub uri: types::Canonical,

    /// NPM Package name for IG this depends on
    pub package_id: Option<types::Id>,

    /// Version of the IG
    pub version: Option<types::String>,

    /// Why dependency exists
    pub reason: Option<types::Markdown>,
}

/// A profile that applies globally to all resources of a given type within the
/// implementation guide.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideGlobal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type this profile applies to
    pub r#type: types::Code,

    /// Profile that all resources must conform to
    pub profile: types::Canonical,
}

/// The information needed to build the implementation guide, including the
/// groupings, resources, pages, parameters, and templates that make up the IG.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Grouping used to present related resources in the IG
    pub grouping: Option<Vec<ImplementationGuideDefinitionGrouping>>,

    /// Resource in the implementation guide
    pub resource: Option<Vec<ImplementationGuideDefinitionResource>>,

    /// Page/Section in the Guide
    pub page: Option<ImplementationGuideDefinitionPage>,

    /// Defines how IG is built by tools
    pub parameter: Option<Vec<ImplementationGuideDefinitionParameter>>,

    /// A template for building resources
    pub template: Option<Vec<ImplementationGuideDefinitionTemplate>>,
}

/// A logical group used to present related resources together in the rendered
/// implementation guide.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionGrouping {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Descriptive name for the package
    pub name: types::String,

    /// Human readable text describing the package
    pub description: Option<types::Markdown>,
}

/// A resource that is part of the implementation guide, with metadata about how
/// it is included and rendered.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Location of the resource
    pub reference: types::Reference,

    /// Versions this applies to (if different to IG)
    pub fhir_version: Option<Vec<types::Code>>,

    /// Human readable name for the resource
    pub name: Option<types::String>,

    /// Reason why included in guide
    pub description: Option<types::Markdown>,

    /// Is this an example
    pub is_example: Option<types::Boolean>,

    /// Profile(s) this is an example of
    pub profile: Option<Vec<types::Canonical>>,

    /// Grouping this is part of
    pub grouping_id: Option<types::Id>,
}

/// A page or section within the implementation guide. Pages nest recursively to
/// form the navigation tree of the rendered guide.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionPage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Source for page
    pub source_url: Option<types::Url>,

    /// Source for page
    pub source_string: Option<types::String>,

    /// Source for page
    pub source_markdown: Option<types::Markdown>,

    /// Name of the page when published
    pub name: types::Url,

    /// Short title shown for navigational assistance
    pub title: types::String,

    /// html | markdown | xml | generated
    pub generation: types::Code,

    /// Nested Pages / Sections
    pub page: Option<Vec<ImplementationGuideDefinitionPage>>,
}

/// A parameter that defines how the implementation guide is built by tooling.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that identifies parameter
    pub code: types::Coding,

    /// Value for named type
    pub value: types::String,
}

/// A template used for building resources within the implementation guide.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of template specified
    pub code: types::Code,

    /// The source location for the template
    pub source: types::String,

    /// The scope in which the template applies
    pub scope: Option<types::String>,
}

/// Information about an assembled implementation guide, describing where the
/// rendered output and its constituent resources and pages can be found.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Location of rendered implementation guide
    pub rendering: Option<types::Url>,

    /// Resource in the implementation guide
    pub resource: Vec<ImplementationGuideManifestResource>,

    /// HTML page within the parent IG
    pub page: Option<Vec<ImplementationGuideManifestPage>>,

    /// Image within the IG
    pub image: Option<Vec<types::String>>,

    /// Additional linkable file in IG
    pub other: Option<Vec<types::String>>,
}

/// A resource in the assembled implementation guide, with its rendered location.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifestResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Location of the resource
    pub reference: types::Reference,

    /// Is this an example
    pub is_example: Option<types::Boolean>,

    /// Profile(s) this is an example of
    pub profile: Option<Vec<types::Canonical>>,

    /// Relative path for page in IG
    pub relative_path: Option<types::Url>,
}

/// An HTML page within the assembled implementation guide.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifestPage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// HTML page name
    pub name: types::String,

    /// Title of the page, for references
    pub title: Option<types::String>,

    /// Anchor available on the page
    pub anchor: Option<Vec<types::String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImplementationGuide;

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
