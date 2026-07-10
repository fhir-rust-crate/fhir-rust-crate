//! EvidenceVariable
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
//!
//! Version: 5.0.0
//!
//! EvidenceVariable Resource: The EvidenceVariable resource describes an element that knowledge (Evidence) is about.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about. It represents a "population", "exposure", or "outcome" concept that
/// defines characteristics used in evidence-based statistics. EvidenceVariables
/// are the building blocks referenced by Evidence and related resources to
/// describe what was studied and how it was measured.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::evidence_variable::EvidenceVariable;
///
/// let value = EvidenceVariable::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EvidenceVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariable {
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

    /// Canonical identifier for this evidence variable, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the evidence variable
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the evidence variable
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this evidence variable (computer friendly)
    pub name: Option<types::String>,

    /// Name for this evidence variable (human friendly)
    pub title: Option<types::String>,

    /// Title for use in informal contexts
    pub short_title: Option<types::String>,

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

    /// Natural language description of the evidence variable
    pub description: Option<types::Markdown>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<types::Annotation>>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Why this EvidenceVariable is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the resource was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the resource was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the resource is expected to be used
    pub effective_period: Option<types::Period>,

    /// Who authored the content
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Actual or conceptual
    pub actual: Option<types::Boolean>,

    /// A defining factor of the EvidenceVariable
    pub characteristic: Option<Vec<EvidenceVariableCharacteristic>>,

    /// continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<types::Code>,

    /// A grouping for ordinal or polychotomous variables
    pub category: Option<Vec<EvidenceVariableCategory>>,
}

/// A defining factor of the EvidenceVariable.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for internal linking
    pub link_id: Option<types::Id>,

    /// Natural language description of the characteristic
    pub description: Option<types::Markdown>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<types::Annotation>>,

    /// Whether the characteristic is an inclusion criterion or exclusion criterion
    pub exclude: Option<types::Boolean>,

    /// Defines the characteristic (without using type and value) by a Reference
    pub definition_reference: Option<types::Reference>,

    /// Defines the characteristic (without using type and value) by a Canonical
    pub definition_canonical: Option<types::Canonical>,

    /// Defines the characteristic (without using type and value) by a CodeableConcept
    pub definition_codeable_concept: Option<types::CodeableConcept>,

    /// Defines the characteristic (without using type and value) by an expression
    pub definition_expression: Option<types::Expression>,

    /// Defines the characteristic (without using type and value) by an id
    pub definition_id: Option<types::Id>,

    /// Defines the characteristic using type and value
    pub definition_by_type_and_value: Option<EvidenceVariableCharacteristicDefinitionByTypeAndValue>,

    /// Used to specify how two or more characteristics are combined
    pub definition_by_combination: Option<EvidenceVariableCharacteristicDefinitionByCombination>,

    /// Number of occurrences meeting the characteristic
    pub instances_quantity: Option<types::Quantity>,

    /// Number of occurrences meeting the characteristic
    pub instances_range: Option<types::Range>,

    /// Length of time in which the characteristic is met
    pub duration_quantity: Option<types::Quantity>,

    /// Length of time in which the characteristic is met
    pub duration_range: Option<types::Range>,

    /// Timing in which the characteristic is determined
    pub time_from_event: Option<Vec<EvidenceVariableCharacteristicTimeFromEvent>>,
}

/// Defines the characteristic using type and value.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicDefinitionByTypeAndValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Expresses the type of characteristic
    pub r#type: types::CodeableConcept,

    /// Method for how the characteristic value was determined
    pub method: Option<Vec<types::CodeableConcept>>,

    /// Device used for determining characteristic
    pub device: Option<types::Reference>,

    /// Defines the characteristic when coupled with characteristic.type
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Defines the characteristic when coupled with characteristic.type
    pub value_boolean: Option<types::Boolean>,

    /// Defines the characteristic when coupled with characteristic.type
    pub value_quantity: Option<types::Quantity>,

    /// Defines the characteristic when coupled with characteristic.type
    pub value_range: Option<types::Range>,

    /// Defines the characteristic when coupled with characteristic.type
    pub value_reference: Option<types::Reference>,

    /// Defines the characteristic when coupled with characteristic.type
    pub value_id: Option<types::Id>,

    /// Reference point for valueQuantity or valueRange
    pub offset: Option<types::CodeableConcept>,
}

/// Used to specify how two or more characteristics are combined.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicDefinitionByCombination {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// all-of | any-of | at-least | at-most | statistical | net-effect | dataset
    pub code: types::Code,

    /// Provides the value of "n" when "at-least" or "at-most" codes are used
    pub threshold: Option<types::PositiveInt>,

    /// A defining factor of the characteristic
    pub characteristic: Vec<EvidenceVariableCharacteristic>,
}

/// Timing in which the characteristic is determined.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicTimeFromEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Human readable description
    pub description: Option<types::Markdown>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<types::Annotation>>,

    /// The event used as a base point (reference point) in time
    pub event_codeable_concept: Option<types::CodeableConcept>,

    /// The event used as a base point (reference point) in time
    pub event_reference: Option<types::Reference>,

    /// The event used as a base point (reference point) in time
    pub event_date_time: Option<types::DateTime>,

    /// The event used as a base point (reference point) in time
    pub event_id: Option<types::Id>,

    /// Used to express the observation at a defined amount of time before or after the event
    pub quantity: Option<types::Quantity>,

    /// Used to express the observation within a period before and/or after the event
    pub range: Option<types::Range>,
}

/// A grouping for ordinal or polychotomous variables.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCategory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Description of the grouping
    pub name: Option<types::String>,

    /// Definition of the grouping
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Definition of the grouping
    pub value_quantity: Option<types::Quantity>,

    /// Definition of the grouping
    pub value_range: Option<types::Range>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EvidenceVariable;

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
