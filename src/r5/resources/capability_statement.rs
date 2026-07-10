//! CapabilityStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
//!
//! Version: 5.0.0
//!
//! CapabilityStatement Resource: A set of capabilities (behaviors) of a FHIR Server or Client for a particular version of FHIR.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server or Client for a particular version of FHIR that may be used as a
/// statement of actual server functionality or a statement of required or
/// desired server implementation. It provides for a degree of automatic
/// negotiation of features and interoperability between FHIR systems.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::capability_statement::CapabilityStatement;
///
/// let value = CapabilityStatement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CapabilityStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement {
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

    /// Canonical identifier for this capability statement, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the CapabilityStatement (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the capability statement
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this capability statement (computer friendly)
    pub name: Option<types::String>,

    /// Name for this capability statement (human friendly)
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

    /// Natural language description of the capability statement
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for capability statement (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this capability statement is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// instance | capability | requirements
    pub kind: types::Code,

    /// Canonical URL of another capability statement this implements
    pub instantiates: Option<Vec<types::Canonical>>,

    /// Canonical URL of another capability statement this adds to
    pub imports: Option<Vec<types::Canonical>>,

    /// Software that is covered by this capability statement
    pub software: Option<CapabilityStatementSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<CapabilityStatementImplementation>,

    /// FHIR Version the system supports
    pub fhir_version: types::Code,

    /// formats supported (xml | json | ttl | mime type)
    pub format: Vec<types::Code>,

    /// Patch formats supported
    pub patch_format: Option<Vec<types::Code>>,

    /// Languages supported
    pub accept_language: Option<Vec<types::Code>>,

    /// Implementation guides supported
    pub implementation_guide: Option<Vec<types::Canonical>>,

    /// If the endpoint is a RESTful one
    pub rest: Option<Vec<CapabilityStatementRest>>,

    /// If messaging is supported
    pub messaging: Option<Vec<CapabilityStatementMessaging>>,

    /// Document definition
    pub document: Option<Vec<CapabilityStatementDocument>>,
}

/// Software that is covered by this capability statement.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementSoftware {
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

    /// Date this version was released
    pub release_date: Option<types::DateTime>,
}

/// If this describes a specific instance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementImplementation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Describes this specific instance
    pub description: types::Markdown,

    /// Base URL for the installation
    pub url: Option<types::Url>,

    /// Organization that manages the data
    pub custodian: Option<types::Reference>,
}

/// If the endpoint is a RESTful one.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// client | server
    pub mode: types::Code,

    /// General description of implementation
    pub documentation: Option<types::Markdown>,

    /// Information about security of implementation
    pub security: Option<CapabilityStatementRestSecurity>,

    /// Resource served on the REST interface
    pub resource: Option<Vec<CapabilityStatementRestResource>>,

    /// What operations are supported?
    pub interaction: Option<Vec<CapabilityStatementRestInteraction>>,

    /// Search parameters for searching all resources
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>>,

    /// Definition of a system level operation
    pub operation: Option<Vec<CapabilityStatementRestResourceOperation>>,

    /// Compartments served/used by system
    pub compartment: Option<Vec<types::Canonical>>,
}

/// Information about security of implementation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestSecurity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Adds CORS Headers (http://enable-cors.org/)
    pub cors: Option<types::Boolean>,

    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    pub service: Option<Vec<types::CodeableConcept>>,

    /// General description of how security works
    pub description: Option<types::Markdown>,
}

/// Resource served on the REST interface.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A resource type that is supported
    pub r#type: types::Code,

    /// System-wide profile
    pub profile: Option<types::Canonical>,

    /// Use-case specific profiles
    pub supported_profile: Option<Vec<types::Canonical>>,

    /// Additional information about the use of the resource type
    pub documentation: Option<types::Markdown>,

    /// What operations are supported?
    pub interaction: Option<Vec<CapabilityStatementRestResourceInteraction>>,

    /// no-version | versioned | versioned-update
    pub versioning: Option<types::Code>,

    /// Whether vRead can return past versions
    pub read_history: Option<types::Boolean>,

    /// If update can commit to a new identity
    pub update_create: Option<types::Boolean>,

    /// If allows/uses conditional create
    pub conditional_create: Option<types::Boolean>,

    /// not-supported | modified-since | not-match | full-support
    pub conditional_read: Option<types::Code>,

    /// If allows/uses conditional update
    pub conditional_update: Option<types::Boolean>,

    /// If allows/uses conditional patch
    pub conditional_patch: Option<types::Boolean>,

    /// not-supported | single | multiple - how conditional delete is supported
    pub conditional_delete: Option<types::Code>,

    /// literal | logical | resolves | enforced | local
    pub reference_policy: Option<Vec<types::Code>>,

    /// _include values supported by the server
    pub search_include: Option<Vec<types::String>>,

    /// _revinclude values supported by the server
    pub search_rev_include: Option<Vec<types::String>>,

    /// Search parameters supported by implementation
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>>,

    /// Definition of a resource operation
    pub operation: Option<Vec<CapabilityStatementRestResourceOperation>>,
}

/// What operations are supported on a resource type?
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// read | vread | update | patch | delete | history-instance | history-type | create | search-type
    pub code: types::Code,

    /// Anything special about operation behavior
    pub documentation: Option<types::Markdown>,
}

/// Search parameters supported by implementation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceSearchParam {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name for parameter in search url
    pub name: types::String,

    /// Source of definition for parameter
    pub definition: Option<types::Canonical>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    pub r#type: types::Code,

    /// Server-specific usage
    pub documentation: Option<types::Markdown>,
}

/// Definition of a resource operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name by which the operation/query is invoked
    pub name: types::String,

    /// The defined operation/query
    pub definition: types::Canonical,

    /// Specific details about operation behavior
    pub documentation: Option<types::Markdown>,
}

/// What operations are supported at the system (all-resources) level?
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// transaction | batch | search-system | history-system
    pub code: types::Code,

    /// Anything special about operation behavior
    pub documentation: Option<types::Markdown>,
}

/// If messaging is supported.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Where messages should be sent
    pub endpoint: Option<Vec<CapabilityStatementMessagingEndpoint>>,

    /// Reliable Message Cache Length (min)
    pub reliable_cache: Option<types::UnsignedInt>,

    /// Messaging interface behavior details
    pub documentation: Option<types::Markdown>,

    /// Messages supported by this system
    pub supported_message: Option<Vec<CapabilityStatementMessagingSupportedMessage>>,
}

/// Where messages should be sent.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingEndpoint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// http | ftp | mllp +
    pub protocol: types::Coding,

    /// Network address or identifier of the end-point
    pub address: types::Url,
}

/// Messages supported by this system.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingSupportedMessage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// sender | receiver
    pub mode: types::Code,

    /// Message supported by this system
    pub definition: types::Canonical,
}

/// Document definition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementDocument {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// producer | consumer
    pub mode: types::Code,

    /// Description of document support
    pub documentation: Option<types::Markdown>,

    /// Constraint on the resources used in the document
    pub profile: types::Canonical,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CapabilityStatement;

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
