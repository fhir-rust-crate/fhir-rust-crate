//! Requirements
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Requirements
//!
//! Version: 5.0.0
//!
//! Requirements Resource: The Requirements resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The Requirements resource is used to gather a set of requirement statements
/// that describe what a system, actor, or specification is expected to do. Each
/// statement carries a conformance verb (SHALL, SHOULD, MAY, SHOULD-NOT) and a
/// markdown requirement, and may be derived from or refine other statements.
/// It is commonly used within implementation guides to formally capture and
/// trace expectations, linking them to actors and to the design artifacts that
/// satisfy them.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::requirements::Requirements;
///
/// let value = Requirements::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Requirements = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
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

    /// Canonical identifier for this Requirements, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the Requirements (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the Requirements
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this Requirements (computer friendly)
    pub name: Option<types::String>,

    /// Name for this Requirements (human friendly)
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

    /// Natural language description of the requirements
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for Requirements (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this Requirements is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// Other set of Requirements this builds on
    pub derived_from: Option<Vec<types::Canonical>>,

    /// External artifact (rule/document etc. that) created this set of requirements
    pub reference: Option<Vec<types::Url>>,

    /// Actor for these requirements
    pub actor: Option<Vec<types::Canonical>>,

    /// Actual statement as markdown
    pub statement: Option<Vec<RequirementsStatement>>,
}

/// A single requirement statement within a Requirements resource, carrying a
/// unique key, an optional human label, one or more conformance verbs, and the
/// actual requirement expressed as markdown. Statements may be derived from or
/// refine other statements and can reference the design artifacts that satisfy
/// them.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequirementsStatement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Key that identifies this statement
    pub key: types::Id,

    /// Short Human label for this statement
    pub label: Option<types::String>,

    /// SHALL | SHOULD | MAY | SHOULD-NOT
    pub conformance: Option<Vec<types::Code>>,

    /// Set to true if requirements statement is conditional
    pub conditionality: Option<types::Boolean>,

    /// The actual requirement
    pub requirement: types::Markdown,

    /// Another statement this clarifies/restricts ([url#]key)
    pub derived_from: Option<types::String>,

    /// A larger requirement that this requirement helps to refine and enable
    pub parent: Option<types::String>,

    /// Design artifact that satisfies this requirement
    pub satisfied_by: Option<Vec<types::Url>>,

    /// External artifact (rule/document etc. that) created this requirement
    pub reference: Option<Vec<types::Url>>,

    /// Who asked for this statement
    pub source: Option<Vec<types::Reference>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Requirements;

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
