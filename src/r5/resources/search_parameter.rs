//! SearchParameter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
//!
//! Version: 5.0.0
//!
//! SearchParameter Resource: A search parameter that defines a named search item that can be used to search/filter on a resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A search parameter that defines a named search item that can be used to
/// search or filter on a resource. SearchParameter resources describe how a
/// FHIR server exposes queryable elements of a resource, mapping a search code
/// to a FHIRPath expression that extracts the matchable values. They are a
/// foundational part of the FHIR RESTful search framework and are commonly
/// published within implementation guides and capability statements.
///
/// Conceptually, a SearchParameter binds a short `code` (the token that
/// appears in a search URL, for example `?status=active`) to the underlying
/// element(s) of a resource that the code queries against, along with the
/// data `type` (such as token, date, reference, or composite) that governs
/// how the value is parsed and compared. Servers advertise the search
/// parameters they support for each resource type in their CapabilityStatement,
/// and clients use that information to construct valid search requests.
/// Composite search parameters combine two or more component parameters,
/// each with its own subexpression, to allow matching on the relationship
/// between values (for example, an observation's code and its value together).
///
/// # See also
///
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used for the jurisdiction of a search parameter.
/// - `CapabilityStatement`, which typically enumerates the search parameters a server supports.
/// - `Observation` and other resource types, which are the common subjects of search parameter definitions.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::search_parameter::SearchParameter;
///
/// let value = SearchParameter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SearchParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchParameter {
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

    /// Canonical identifier for this search parameter, represented as a globally unique URI used to reference it from other artifacts
    pub url: types::Uri,

    /// Additional identifier for the search parameter (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the search parameter
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Computer-friendly name for this search parameter, used as the base for the machine-readable name
    pub name: types::String,

    /// Name for this search parameter (human friendly)
    pub title: Option<types::String>,

    /// Original definition for the search parameter
    pub derived_from: Option<types::Canonical>,

    /// Publication status of the search parameter definition: draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the search parameter, explaining what it matches and how it is intended to be used
    pub description: types::Markdown,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for search parameter (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this search parameter is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// Recommended name for the parameter as it appears in a search URL, for example `?code=...`
    pub code: types::Code,

    /// The resource type(s) this search parameter applies to, such as `Patient` or `Observation`
    pub base: Vec<types::Code>,

    /// Data type of the search parameter's value: number | date | string | token | reference | composite | quantity | uri | special
    pub r#type: types::Code,

    /// FHIRPath expression, relative to the resource, that extracts the values matched by this search parameter
    pub expression: Option<types::String>,

    /// normal | phonetic | other
    pub processing_mode: Option<types::Code>,

    /// FHIRPath expression that constraints the usage of this SearchParamete
    pub constraint: Option<types::String>,

    /// Types of resource (if a resource reference)
    pub target: Option<Vec<types::Code>>,

    /// Allow multiple values per parameter (or)
    pub multiple_or: Option<types::Boolean>,

    /// Allow multiple parameters (and)
    pub multiple_and: Option<types::Boolean>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<Vec<types::Code>>,

    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<Vec<types::Code>>,

    /// Chained names supported
    pub chain: Option<Vec<types::String>>,

    /// For Composite resources to define the parts
    pub component: Option<Vec<SearchParameterComponent>>,
}

/// For Composite resources to define the parts. Each component points to a
/// sub search parameter definition and provides a subexpression, relative to
/// the main expression, that yields the value for that part of the composite.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchParameterComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Defines how the part works
    pub definition: types::Canonical,

    /// Subexpression relative to main expression
    pub expression: types::String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SearchParameter;

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
