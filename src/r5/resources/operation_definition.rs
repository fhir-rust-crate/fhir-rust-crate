//! OperationDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OperationDefinition
//!
//! Version: 5.0.0
//!
//! OperationDefinition Resource: A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A formal computable definition of an operation (on the RESTful interface) or
/// a named query (using the search interaction).
///
/// OperationDefinition describes the inputs, outputs, and behavior of an
/// operation or named query that a FHIR server can support. It is used to
/// declare custom operations invoked via `$operation` endpoints, specifying
/// their parameters, cardinalities, and the levels (system, type, instance) at
/// which they may be invoked.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::operation_definition::OperationDefinition;
///
/// let value = OperationDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: OperationDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinition {
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

    /// Canonical identifier for this operation definition, represented as an absolute URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the implementation guide (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the operation definition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this operation definition (computer friendly)
    pub name: types::String,

    /// Name for this operation definition (human friendly)
    pub title: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// operation | query
    pub kind: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the operation definition
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for operation definition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this operation definition is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// Whether content is changed by the operation
    pub affects_state: Option<types::Boolean>,

    /// Recommended name for operation in search url
    pub code: types::Code,

    /// Additional information about use
    pub comment: Option<types::Markdown>,

    /// Marks this as a profile of the base
    pub base: Option<types::Canonical>,

    /// Types this operation applies to
    pub resource: Option<Vec<types::Code>>,

    /// Invoke at the system level?
    pub system: types::Boolean,

    /// Invoke at the type level?
    pub r#type: types::Boolean,

    /// Invoke on an instance?
    pub instance: types::Boolean,

    /// Validation information for in parameters
    pub input_profile: Option<types::Canonical>,

    /// Validation information for out parameters
    pub output_profile: Option<types::Canonical>,

    /// Parameters for the operation/query
    pub parameter: Option<Vec<OperationDefinitionParameter>>,

    /// Define overloaded variants for when  generating code
    pub overload: Option<Vec<OperationDefinitionOverload>>,
}

/// Parameters for the operation/query.
///
/// Each parameter describes an input or output of the operation, including its
/// name, use (in or out), cardinality, type, and any value set binding.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name in Parameters.parameter.name or in URL
    pub name: types::Code,

    /// in | out
    pub r#use: types::Code,

    /// instance | type | system
    pub scope: Option<Vec<types::Code>>,

    /// Minimum Cardinality
    pub min: types::Integer,

    /// Maximum Cardinality (a number or *)
    pub max: types::String,

    /// Description of meaning/use
    pub documentation: Option<types::Markdown>,

    /// What type this parameter has
    pub r#type: Option<types::Code>,

    /// Allowed sub-type this parameter can have (if type is abstract)
    pub allowed_type: Option<Vec<types::Code>>,

    /// If type is Reference | canonical, allowed targets. If type is 'Resource', then this constrains the allowed resource types
    pub target_profile: Option<Vec<types::Canonical>>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    pub search_type: Option<types::Code>,

    /// ValueSet details if this is coded
    pub binding: Option<OperationDefinitionParameterBinding>,

    /// References to this parameter
    pub referenced_from: Option<Vec<OperationDefinitionParameterReferencedFrom>>,

    /// Parts of a nested Parameter
    pub part: Option<Vec<OperationDefinitionParameter>>,
}

/// ValueSet details if this is coded.
///
/// Binds the parameter to a value set with a given strength, applicable when
/// the parameter is a coded data type.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionParameterBinding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// required | extensible | preferred | example
    pub strength: types::Code,

    /// Source of value set
    pub value_set: types::Canonical,
}

/// References to this parameter.
///
/// Identifies other parameters that reference this parameter, used to describe
/// relationships between parameters (for example, a search parameter that
/// references a target parameter).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionParameterReferencedFrom {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Referencing parameter
    pub source: types::String,

    /// Element id of reference
    pub source_id: Option<types::String>,
}

/// Define overloaded variants for when generating code.
///
/// Describes a named combination of parameters that together form a valid
/// invocation, used primarily to guide code generation for client libraries.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionOverload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name of parameter to include in overload
    pub parameter_name: Option<Vec<types::String>>,

    /// Comments to go on overload
    pub comment: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = OperationDefinition;

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
