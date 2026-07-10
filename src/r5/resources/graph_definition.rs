//! GraphDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GraphDefinition
//!
//! Version: 5.0.0
//!
//! GraphDefinition Resource: A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A formal computable definition of a graph of resources.
///
/// A GraphDefinition describes a coherent set of resources that form a graph by
/// following references from a starting node. It defines the set of nodes
/// (potential targets) and the links (rules) between them, including cardinality
/// and compartment consistency constraints. In FHIR R5 it is used by servers and
/// tooling to drive operations such as `$graph` and `$graphql`, and to document
/// and validate the shape of interconnected resource collections.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::graph_definition::GraphDefinition;
///
/// let value = GraphDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: GraphDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinition {
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

    /// Canonical identifier for this graph definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the GraphDefinition (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the graph definition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this graph definition (computer friendly)
    pub name: types::String,

    /// Name for this graph definition (human friendly)
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

    /// Natural language description of the graph definition
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for graph definition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this graph definition is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// Starting Node
    pub start: Option<types::Id>,

    /// Potential target for the link
    pub node: Option<Vec<GraphDefinitionNode>>,

    /// Links this graph makes rules about
    pub link: Option<Vec<GraphDefinitionLink>>,
}

/// Potential target for the link.
///
/// A node identifies a resource type (and optional profile) that may be a target
/// within the graph, referenced by links via its internal `node_id`.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionNode {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Internal ID - target for link references
    pub node_id: types::Id,

    /// Why this node is specified
    pub description: Option<types::String>,

    /// Type of resource this link refers to
    pub r#type: types::Code,

    /// Profile for the target resource
    pub profile: Option<types::Canonical>,
}

/// Links this graph makes rules about.
///
/// A link connects a source node to a target node, optionally along a path in the
/// resource, and expresses cardinality and compartment consistency rules between
/// the connected resources.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Why this link is specified
    pub description: Option<types::String>,

    /// Minimum occurrences for this link
    pub min: Option<types::Integer>,

    /// Maximum occurrences for this link
    pub max: Option<types::String>,

    /// Source Node for this link
    pub source_id: types::Id,

    /// Path in the resource that contains the link
    pub path: Option<types::String>,

    /// Which slice (if profiled)
    pub slice_name: Option<types::String>,

    /// Target Node for this link
    pub target_id: types::Id,

    /// Criteria for reverse lookup
    pub params: Option<types::String>,

    /// Compartment Consistency Rules
    pub compartment: Option<Vec<GraphDefinitionLinkCompartment>>,
}

/// Compartment Consistency Rules.
///
/// A compartment rule constrains how a compartment (such as Patient or Encounter)
/// must relate across the linked resources, using a rule and optional FHIRPath
/// expression.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLinkCompartment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// where | requires
    pub r#use: types::Code,

    /// identical | matching | different | custom
    pub rule: types::Code,

    /// Patient | Encounter | RelatedPerson | Practitioner | Device | EpisodeOfCare
    pub code: types::Code,

    /// Custom rule, as a FHIRPath expression
    pub expression: Option<types::String>,

    /// Documentation for FHIRPath expression
    pub description: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = GraphDefinition;

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
