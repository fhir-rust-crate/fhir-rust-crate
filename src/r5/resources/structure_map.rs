//! StructureMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/StructureMap
//!
//! Version: 5.0.0
//!
//! StructureMap Resource: A Map of relationships between 2 structures that can be used to transform data.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A StructureMap describes a map of relationships between two or more
/// structures that can be used to transform instance data from one form to
/// another. It defines the rules, groups, and transforms used by a FHIR
/// mapping engine to convert a source instance into a target instance. This
/// resource is central to the FHIR Mapping Language and is commonly used in
/// implementation guides to specify structural data transformations.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::structure_map::StructureMap;
///
/// let value = StructureMap::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMap = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap {
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

    /// Canonical identifier for this structure map, represented as a URI (globally unique)
    pub url: types::Uri,

    /// Additional identifier for the structure map
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the structure map
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this structure map (computer friendly)
    pub name: types::String,

    /// Name for this structure map (human friendly)
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

    /// Natural language description of the structure map
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for structure map (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this structure map is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// Structure Definition used by this map
    pub structure: Option<Vec<StructureMapStructure>>,

    /// Other maps used by this map (canonical URLs)
    pub import: Option<Vec<types::Canonical>>,

    /// Definition of the constant value used in the map rules
    pub r#const: Option<Vec<StructureMapConst>>,

    /// Named sections for reader convenience
    pub group: Vec<StructureMapGroup>,
}

/// Structure Definition used by this map.
///
/// A structure declares a StructureDefinition that participates in this map,
/// along with the mode in which it is used (source, queried, target, or
/// produced) and an optional alias for referencing the type within map rules.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapStructure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Canonical reference to structure definition
    pub url: types::Canonical,

    /// source | queried | target | produced
    pub mode: types::Code,

    /// Name for type in this map
    pub alias: Option<types::String>,

    /// Documentation on use of structure
    pub documentation: Option<types::String>,
}

/// Definition of the constant value used in the map rules.
///
/// A const provides a named FHIRPath expression whose evaluated value can be
/// reused throughout the map rules, allowing shared literal or computed values
/// to be referenced by name.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapConst {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Constant name
    pub name: Option<types::Id>,

    /// FHIRPath exression - value of the constant
    pub value: Option<types::String>,
}

/// Named sections for reader convenience.
///
/// A group organizes a set of transform rules and their named inputs. Groups
/// may extend other groups and specify a type mode, and they define the entry
/// points invoked when the map is executed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Human-readable label
    pub name: types::Id,

    /// Another group that this group adds rules to
    pub extends: Option<types::Id>,

    /// types | type-and-types
    pub type_mode: Option<types::Code>,

    /// Additional description/explanation for group
    pub documentation: Option<types::String>,

    /// Named instance provided when invoking the map
    pub input: Vec<StructureMapGroupInput>,

    /// Transform Rule from source to target
    pub rule: Option<Vec<StructureMapGroupRule>>,
}

/// Named instance provided when invoking the map.
///
/// An input declares a named data instance passed to the group when the map is
/// invoked, together with its type and its mode (source or target).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name for this instance of data
    pub name: types::Id,

    /// Type for this instance of data
    pub r#type: Option<types::String>,

    /// source | target
    pub mode: types::Code,

    /// Documentation for this instance of data
    pub documentation: Option<types::String>,
}

/// Transform Rule from source to target.
///
/// A rule defines the mapping from one or more sources to one or more targets,
/// and may itself contain nested rules and dependent rule/group invocations.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name of the rule for internal references
    pub name: Option<types::Id>,

    /// Source inputs to the mapping
    pub source: Vec<StructureMapGroupRuleSource>,

    /// Content to create because of this mapping rule
    pub target: Option<Vec<StructureMapGroupRuleTarget>>,

    /// Rules contained in this rule
    pub rule: Option<Vec<StructureMapGroupRule>>,

    /// Which other rules to apply in the context of this rule
    pub dependent: Option<Vec<StructureMapGroupRuleDependent>>,

    /// Documentation for this instance of data
    pub documentation: Option<types::String>,
}

/// Source inputs to the mapping.
///
/// A source identifies the context and optional element that a rule reads
/// from, along with cardinality, type, list-mode, and FHIRPath conditions that
/// determine whether and how the rule applies.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleSource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type or variable this rule applies to
    pub context: types::Id,

    /// Specified minimum cardinality
    pub min: Option<types::Integer>,

    /// Specified maximum cardinality (number or *)
    pub max: Option<types::String>,

    /// Rule only applies if source has this type
    pub r#type: Option<types::String>,

    /// Default value if no value exists
    pub default_value: Option<types::String>,

    /// Optional field for this source
    pub element: Option<types::String>,

    /// first | not_first | last | not_last | only_one
    pub list_mode: Option<types::Code>,

    /// Named context for field, if a field is specified
    pub variable: Option<types::Id>,

    /// FHIRPath expression  - must be true or the rule does not apply
    pub condition: Option<types::String>,

    /// FHIRPath expression  - must be true or the mapping engine throws an error instead of completing
    pub check: Option<types::String>,

    /// Message to put in log if source exists (FHIRPath)
    pub log_message: Option<types::String>,
}

/// Content to create because of this mapping rule.
///
/// A target describes an element to create or populate in the output, the
/// transform to apply, its parameters, and list-handling behavior for the
/// generated content.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Variable this rule applies to
    pub context: Option<types::String>,

    /// Field to create in the context
    pub element: Option<types::String>,

    /// Named context for field, if desired, and a field is specified
    pub variable: Option<types::Id>,

    /// first | share | last | single
    pub list_mode: Option<Vec<types::Code>>,

    /// Internal rule reference for shared list items
    pub list_rule_id: Option<types::Id>,

    /// create | copy +
    pub transform: Option<types::Code>,

    /// Parameters to the transform
    pub parameter: Option<Vec<StructureMapGroupRuleTargetParameter>>,
}

/// Parameters to the transform.
///
/// A parameter supplies a value to a transform, expressed as either a variable
/// reference or a literal of one of several primitive types.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleTargetParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Parameter value - variable or literal
    pub value_id: Option<types::Id>,

    /// Parameter value - variable or literal
    pub value_string: Option<types::String>,

    /// Parameter value - variable or literal
    pub value_boolean: Option<types::Boolean>,

    /// Parameter value - variable or literal
    pub value_integer: Option<types::Integer>,

    /// Parameter value - variable or literal
    pub value_decimal: Option<types::Decimal>,

    /// Parameter value - variable or literal
    pub value_date: Option<types::Date>,

    /// Parameter value - variable or literal
    pub value_time: Option<types::Time>,

    /// Parameter value - variable or literal
    pub value_date_time: Option<types::DateTime>,
}

/// Which other rules to apply in the context of this rule.
///
/// A dependent invokes another named rule or group, passing parameters that
/// bind the current context to the invoked rule or group's inputs.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleDependent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name of a rule or group to apply
    pub name: types::Id,

    /// Parameter to pass to the rule or group
    pub parameter: Vec<StructureMapGroupRuleTargetParameter>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = StructureMap;

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
