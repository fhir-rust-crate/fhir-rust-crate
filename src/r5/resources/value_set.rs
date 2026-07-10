//! ValueSet
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ValueSet
//!
//! Version: 5.0.0
//!
//! ValueSet Resource: A ValueSet resource instance specifies a set of codes drawn from one or more code systems, intended for use in a particular context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link
/// between CodeSystem definitions and their use in coded elements, providing the
/// content logical definition (compose) that selects codes and an optional
/// pre-computed expansion listing the resulting concepts. In FHIR R5 it is a
/// canonical, publishable terminology artifact used throughout the specification
/// to constrain the allowed values of coded elements.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSet;
///
/// let value = ValueSet::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ValueSet = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet {
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

    /// Canonical identifier for this value set, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the value set (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the value set
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this value set (computer friendly)
    pub name: Option<types::String>,

    /// Name for this value set (human friendly)
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

    /// Natural language description of the value set
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for value set (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Indicates whether or not any change to the content logical definition may occur
    pub immutable: Option<types::Boolean>,

    /// Why this value set is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the ValueSet was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the ValueSet was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the ValueSet is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the ValueSet
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the ValueSet
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the ValueSet
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the ValueSet
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Content logical definition of the value set (CLD)
    pub compose: Option<ValueSetCompose>,

    /// Used when the value set is "expanded"
    pub expansion: Option<ValueSetExpansion>,

    /// Description of the semantic space the Value Set Expansion is intended to cover and should further clarify the text in ValueSet.description
    pub scope: Option<ValueSetScope>,
}

/// Content logical definition of the value set (CLD).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetCompose {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Fixed date for references with no specified version (transitive)
    pub locked_date: Option<types::Date>,

    /// Whether inactive codes are in the value set
    pub inactive: Option<types::Boolean>,

    /// Include one or more codes from a code system or other value set(s)
    pub include: Vec<ValueSetComposeInclude>,

    /// Explicitly exclude codes from a code system or other value sets
    pub exclude: Option<Vec<ValueSetComposeInclude>>,

    /// Property to return if client doesn't override
    pub property: Option<Vec<types::String>>,
}

/// Include one or more codes from a code system or other value set(s).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeInclude {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The system the codes come from
    pub system: Option<types::Uri>,

    /// Specific version of the code system referred to
    pub version: Option<types::String>,

    /// A concept defined in the system
    pub concept: Option<Vec<ValueSetComposeIncludeConcept>>,

    /// Select codes/concepts by their properties (including relationships)
    pub filter: Option<Vec<ValueSetComposeIncludeFilter>>,

    /// Select the contents included in this value set
    pub value_set: Option<Vec<types::Canonical>>,

    /// A copyright statement for the specific code system included in the value set
    pub copyright: Option<types::String>,
}

/// A concept defined in the system.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeConcept {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code or expression from system
    pub code: types::Code,

    /// Text to display for this code for this value set in this valueset
    pub display: Option<types::String>,

    /// Additional representations for this concept
    pub designation: Option<Vec<ValueSetComposeIncludeConceptDesignation>>,
}

/// Additional representations for this concept.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeConceptDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Human language of the designation
    pub language: Option<types::Code>,

    /// Types of uses of designations
    pub r#use: Option<types::Coding>,

    /// Additional ways how this designation would be used
    pub additional_use: Option<Vec<types::Coding>>,

    /// The text value for this designation
    pub value: types::String,
}

/// Select codes/concepts by their properties (including relationships).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A property/filter defined by the code system
    pub property: types::Code,

    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | child-of | descendent-leaf | exists
    pub op: types::Code,

    /// Code from the system, or regex criteria, or boolean value for exists
    pub value: types::String,
}

/// Used when the value set is "expanded".
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifies the value set expansion (business identifier)
    pub identifier: Option<types::Uri>,

    /// Opaque urls for paging through expansion results
    pub next: Option<types::Uri>,

    /// Time ValueSet expansion happened
    pub timestamp: types::DateTime,

    /// Total number of codes in the expansion
    pub total: Option<types::Integer>,

    /// Offset at which this resource starts
    pub offset: Option<types::Integer>,

    /// Parameter that controlled the expansion process
    pub parameter: Option<Vec<ValueSetExpansionParameter>>,

    /// Additional information supplied about each concept
    pub property: Option<Vec<ValueSetExpansionProperty>>,

    /// Codes in the value set
    pub contains: Option<Vec<ValueSetExpansionContains>>,
}

/// Parameter that controlled the expansion process.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name as assigned by the client or server
    pub name: types::String,

    /// Value of the named parameter
    pub value_string: Option<types::String>,

    /// Value of the named parameter
    pub value_boolean: Option<types::Boolean>,

    /// Value of the named parameter
    pub value_integer: Option<types::Integer>,

    /// Value of the named parameter
    pub value_decimal: Option<types::Decimal>,

    /// Value of the named parameter
    pub value_uri: Option<types::Uri>,

    /// Value of the named parameter
    pub value_code: Option<types::Code>,

    /// Value of the named parameter
    pub value_date_time: Option<types::DateTime>,
}

/// Additional information supplied about each concept.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifies the property on the concepts, and when referred to in operations
    pub code: types::Code,

    /// Formal identifier for the property
    pub uri: Option<types::Uri>,
}

/// Codes in the value set.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContains {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// System value for the code
    pub system: Option<types::Uri>,

    /// If user cannot select this entry
    pub r#abstract: Option<types::Boolean>,

    /// If concept is inactive in the code system
    pub inactive: Option<types::Boolean>,

    /// Version in which this code/display is defined
    pub version: Option<types::String>,

    /// Code - if blank, this is not a selectable code
    pub code: Option<types::Code>,

    /// User display for the concept
    pub display: Option<types::String>,

    /// Additional representations for this item
    pub designation: Option<Vec<ValueSetComposeIncludeConceptDesignation>>,

    /// Property value for the concept
    pub property: Option<Vec<ValueSetExpansionContainsProperty>>,

    /// Codes contained under this entry
    pub contains: Option<Vec<ValueSetExpansionContains>>,
}

/// Property value for the concept.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContainsProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to ValueSet.expansion.property.code
    pub code: types::Code,

    /// Value of the property for this concept
    pub value_code: Option<types::Code>,

    /// Value of the property for this concept
    pub value_coding: Option<types::Coding>,

    /// Value of the property for this concept
    pub value_string: Option<types::String>,

    /// Value of the property for this concept
    pub value_integer: Option<types::Integer>,

    /// Value of the property for this concept
    pub value_boolean: Option<types::Boolean>,

    /// Value of the property for this concept
    pub value_date_time: Option<types::DateTime>,

    /// Value of the property for this concept
    pub value_decimal: Option<types::Decimal>,

    /// SubProperty value for the concept
    pub sub_property: Option<Vec<ValueSetExpansionContainsPropertySubProperty>>,
}

/// SubProperty value for the concept.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContainsPropertySubProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to ValueSet.expansion.property.code
    pub code: types::Code,

    /// Value of the subproperty for this concept
    pub value_code: Option<types::Code>,

    /// Value of the subproperty for this concept
    pub value_coding: Option<types::Coding>,

    /// Value of the subproperty for this concept
    pub value_string: Option<types::String>,

    /// Value of the subproperty for this concept
    pub value_integer: Option<types::Integer>,

    /// Value of the subproperty for this concept
    pub value_boolean: Option<types::Boolean>,

    /// Value of the subproperty for this concept
    pub value_date_time: Option<types::DateTime>,

    /// Value of the subproperty for this concept
    pub value_decimal: Option<types::Decimal>,
}

/// Description of the semantic space the Value Set Expansion is intended to cover.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetScope {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Criteria describing which concepts or codes should be included and why
    pub inclusion_criteria: Option<types::String>,

    /// Criteria describing which concepts or codes should be excluded and why
    pub exclusion_criteria: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ValueSet;

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
