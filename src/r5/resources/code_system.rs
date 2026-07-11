//! CodeSystem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CodeSystem
//!
//! Version: 5.0.0
//!
//! CodeSystem Resource: The CodeSystem resource is used to declare the existence of and describe a code system or code system supplement and its key properties, and optionally define a part or all of its content.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The CodeSystem resource is used to declare the existence of and describe a
/// code system or code system supplement and its key properties, and optionally
/// define a part or all of its content. In FHIR R5 it is the canonical way to
/// publish terminologies, whether small local code lists or large external
/// systems, and it underpins ValueSet expansion and terminology operations. It
/// captures metadata such as the canonical URL, versioning, filters, and
/// properties, along with an optional hierarchy of concepts and their
/// designations.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::code_system::CodeSystem;
///
/// let value = CodeSystem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CodeSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem {
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

    /// Canonical identifier for this code system, represented as a URI (globally unique) (Coding.system)
    pub url: Option<types::Uri>,

    /// Additional identifier for the code system (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the code system (Coding.version)
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this code system (computer friendly)
    pub name: Option<types::String>,

    /// Name for this code system (human friendly)
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

    /// Natural language description of the code system
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for code system (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this code system is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the CodeSystem was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the CodeSystem was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the CodeSystem is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the CodeSystem
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the CodeSystem
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the CodeSystem
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the CodeSystem
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// If code comparison is case sensitive
    pub case_sensitive: Option<types::Boolean>,

    /// Canonical reference to the value set with entire code system
    pub value_set: Option<types::Canonical>,

    /// grouped-by | is-a | part-of | classified-with
    pub hierarchy_meaning: Option<types::Code>,

    /// If code system defines a compositional grammar
    pub compositional: Option<types::Boolean>,

    /// If definitions are not stable
    pub version_needed: Option<types::Boolean>,

    /// not-present | example | fragment | complete | supplement
    pub content: types::Code,

    /// Canonical URL of Code System this adds designations and properties to
    pub supplements: Option<types::Canonical>,

    /// Total concepts in the code system
    pub count: Option<types::UnsignedInt>,

    /// Filter that can be used in a value set
    pub filter: Option<Vec<CodeSystemFilter>>,

    /// Additional information supplied about each concept
    pub property: Option<Vec<CodeSystemProperty>>,

    /// Concepts in the code system
    pub concept: Option<Vec<CodeSystemConcept>>,
}

/// Filter that can be used in a value set
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that identifies the filter
    pub code: types::Code,

    /// How or why the filter is used
    pub description: Option<types::String>,

    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | child-of | descendent-leaf | exists
    pub operator: Vec<types::Code>,

    /// What to use for the value
    pub value: types::String,
}

/// Additional information supplied about each concept
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemProperty {
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

    /// Why the property is defined, and/or what it conveys
    pub description: Option<types::String>,

    /// code | Coding | string | integer | boolean | dateTime | decimal
    pub r#type: types::Code,
}

/// Concepts in the code system
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemConcept {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that identifies concept
    pub code: types::Code,

    /// Text to display to the user
    pub display: Option<types::String>,

    /// Formal definition
    pub definition: Option<types::String>,

    /// Additional representations for the concept
    pub designation: Option<Vec<CodeSystemConceptDesignation>>,

    /// Property value for the concept
    pub property: Option<Vec<CodeSystemConceptProperty>>,

    /// Child Concepts (is-a/contains/categorizes)
    pub concept: Option<Vec<CodeSystemConcept>>,
}

/// Additional representations for the concept
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemConceptDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Human language of the designation
    pub language: Option<types::Code>,

    /// Details how this designation would be used
    pub r#use: Option<types::Coding>,

    /// Additional ways how this designation would be used
    pub additional_use: Option<Vec<types::Coding>>,

    /// The text value for this designation
    pub value: types::String,
}

/// Property value for the concept
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemConceptProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to CodeSystem.property.code
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
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CodeSystem;

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
