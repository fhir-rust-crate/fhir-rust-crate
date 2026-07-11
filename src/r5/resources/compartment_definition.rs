//! CompartmentDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CompartmentDefinition
//!
//! Version: 5.0.0
//!
//! CompartmentDefinition Resource: A compartment definition that defines how resources are accessed on a server.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A compartment definition that defines how resources are accessed on a
/// server.
///
/// CompartmentDefinition describes a logical grouping of resources (a
/// compartment) such as Patient, Encounter, or Practitioner, and specifies
/// which search parameters relate each resource type to that compartment. It
/// is used in FHIR R5 to drive access, filtering, and `$everything`-style
/// operations by identifying the resources that belong together.
///
/// A server declares its supported compartments as a set of
/// CompartmentDefinition resources so that clients and other systems can
/// discover, for a given compartment code (for example `Patient`), which
/// resource types participate in that compartment and which search
/// parameter on each resource type links it back to the compartment's
/// membership resource. This underpins compartment-based search (such as
/// `GET /Patient/{id}/*`) and access-control policies that scope visibility
/// to the resources belonging to a particular patient, practitioner, or
/// other compartment-defining resource.
///
/// See also: the compartment's membership resource is typically
/// [`Patient`](crate::r5::resources::patient::Patient) or a similar resource
/// such as `Encounter`, `Practitioner`, `RelatedPerson`, `Device`, or
/// `EpisodeOfCare`; compartment membership rules are commonly consumed
/// alongside `SearchParameter` definitions and access-control mechanisms
/// such as `CapabilityStatement`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::compartment_definition::CompartmentDefinition;
///
/// let value = CompartmentDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CompartmentDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompartmentDefinition {
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

    /// Canonical identifier for this compartment definition, represented as a
    /// URI (globally unique); used to reference this definition from other
    /// artifacts such as a `CapabilityStatement`
    pub url: types::Uri,

    /// Business version of the compartment definition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this compartment definition (computer friendly)
    pub name: types::String,

    /// Name for this compartment definition (human friendly)
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

    /// Natural language description of the compartment definition
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Why this compartment definition is defined
    pub purpose: Option<types::Markdown>,

    /// Patient | Encounter | RelatedPerson | Practitioner | Device |
    /// EpisodeOfCare; identifies which resource type defines membership in
    /// this compartment
    pub code: types::Code,

    /// Whether the compartment search syntax (`GET [base]/[type]/{id}/*`) is
    /// supported for this compartment
    pub search: types::Boolean,

    /// How each resource type is related to the compartment, including the
    /// search parameter that links it to the membership resource
    pub resource: Option<Vec<CompartmentDefinitionResource>>,
}

/// How a resource is related to the compartment.
///
/// Information about how a resource is related to the compartment.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompartmentDefinitionResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name of resource type
    pub code: types::Code,

    /// Search Parameter Name, or chained parameters
    pub param: Option<Vec<types::String>>,

    /// Additional documentation about the resource and compartment
    pub documentation: Option<types::String>,

    /// Search Param for interpreting $everything.start
    pub start_param: Option<types::Uri>,

    /// Search Param for interpreting $everything.end
    pub end_param: Option<types::Uri>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CompartmentDefinition;

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
