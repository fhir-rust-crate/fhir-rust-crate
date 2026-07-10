//! TerminologyCapabilities
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TerminologyCapabilities
//!
//! Version: 5.0.0
//!
//! TerminologyCapabilities Resource: A statement of the capabilities (behaviors) of a FHIR Terminology Server.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A TerminologyCapabilities resource documents a set of capabilities
/// (behaviors) of a FHIR Terminology Server that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation. In FHIR R5 it describes which code systems, value set
/// expansion behaviors, and terminology operations (such as `$validate-code`,
/// `$translate`, and `$closure`) a server supports. It is the terminology
/// analogue of the CapabilityStatement resource and is used for conformance
/// testing and capability negotiation between clients and servers.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilities;
///
/// let value = TerminologyCapabilities::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TerminologyCapabilities = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilities {
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

    /// Canonical identifier for this terminology capabilities, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the terminology capabilities
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the terminology capabilities
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this terminology capabilities (computer friendly)
    pub name: Option<types::String>,

    /// Name for this terminology capabilities (human friendly)
    pub title: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: types::DateTime,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the terminology capabilities
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for terminology capabilities (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this terminology capabilities is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// instance | capability | requirements
    pub kind: types::Code,

    /// Software that is covered by this terminology capability statement
    pub software: Option<TerminologyCapabilitiesSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<TerminologyCapabilitiesImplementation>,

    /// Whether lockedDate is supported
    pub locked_date: Option<types::Boolean>,

    /// A code system supported by the server
    pub code_system: Option<Vec<TerminologyCapabilitiesCodeSystem>>,

    /// Information about the ValueSet/$expand operation
    pub expansion: Option<TerminologyCapabilitiesExpansion>,

    /// in-compose | in-expansion | in-compose-or-expansion
    pub code_search: Option<types::Code>,

    /// Information about the ValueSet/$validate-code operation
    pub validate_code: Option<TerminologyCapabilitiesValidateCode>,

    /// Information about the ConceptMap/$translate operation
    pub translation: Option<TerminologyCapabilitiesTranslation>,

    /// Information about the ConceptMap/$closure operation
    pub closure: Option<TerminologyCapabilitiesClosure>,
}

/// Software that is covered by this terminology capability statement.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesSoftware {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A name the software is known by
    pub name: types::String,

    /// Version covered by this statement
    pub version: Option<types::String>,
}

/// If this describes a specific instance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesImplementation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Describes this specific instance
    pub description: types::String,

    /// Base URL for the implementation
    pub url: Option<types::Url>,
}

/// A code system supported by the server.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Canonical identifier for the code system, represented as a URI
    pub uri: Option<types::Canonical>,

    /// Version of Code System supported
    pub version: Option<Vec<TerminologyCapabilitiesCodeSystemVersion>>,

    /// not-present | example | fragment | complete | supplement
    pub content: types::Code,

    /// Whether subsumption is supported
    pub subsumption: Option<types::Boolean>,
}

/// Version of Code System supported.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystemVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Version identifier for this version
    pub code: Option<types::String>,

    /// If this is the default version for this code system
    pub is_default: Option<types::Boolean>,

    /// If compositional grammar is supported
    pub compositional: Option<types::Boolean>,

    /// Language Displays supported
    pub language: Option<Vec<types::Code>>,

    /// Filter Properties supported
    pub filter: Option<Vec<TerminologyCapabilitiesCodeSystemVersionFilter>>,

    /// Properties supported for $lookup
    pub property: Option<Vec<types::Code>>,
}

/// Filter Properties supported.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystemVersionFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code of the property supported
    pub code: types::Code,

    /// Operations supported for the property
    pub op: Vec<types::Code>,
}

/// Information about the ValueSet/$expand operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesExpansion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether the server can return nested value sets
    pub hierarchical: Option<types::Boolean>,

    /// Whether the server supports paging on expansion
    pub paging: Option<types::Boolean>,

    /// Allow request for incomplete expansions?
    pub incomplete: Option<types::Boolean>,

    /// Supported expansion parameter
    pub parameter: Option<Vec<TerminologyCapabilitiesExpansionParameter>>,

    /// Documentation about text searching works
    pub text_filter: Option<types::Markdown>,
}

/// Supported expansion parameter.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesExpansionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name of the supported expansion parameter
    pub name: types::Code,

    /// Description of support for parameter
    pub documentation: Option<types::String>,
}

/// Information about the ValueSet/$validate-code operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesValidateCode {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether translations are validated
    pub translations: types::Boolean,
}

/// Information about the ConceptMap/$translate operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesTranslation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether the client must identify the map
    pub needs_map: types::Boolean,
}

/// Information about the ConceptMap/$closure operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesClosure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// If cross-system closure is supported
    pub translation: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TerminologyCapabilities;

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
