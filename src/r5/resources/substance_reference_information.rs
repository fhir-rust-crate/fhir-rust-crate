//! SubstanceReferenceInformation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation
//!
//! Version: 5.0.0
//!
//! SubstanceReferenceInformation Resource: Todo.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// SubstanceReferenceInformation resource.
///
/// Todo. This resource is used to capture reference information for a substance,
/// including related genes, gene elements, and biological targets. It is part of
/// the FHIR R5 substance definition family used in medicinal product regulation
/// and pharmaceutical data exchange.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::substance_reference_information::SubstanceReferenceInformation;
///
/// let value = SubstanceReferenceInformation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SubstanceReferenceInformation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformation {
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

    /// Todo
    pub comment: Option<types::String>,

    /// Todo
    pub gene: Option<Vec<SubstanceReferenceInformationGene>>,

    /// Todo
    pub gene_element: Option<Vec<SubstanceReferenceInformationGeneElement>>,

    /// Todo
    pub target: Option<Vec<SubstanceReferenceInformationTarget>>,
}

/// Todo
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationGene {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Todo
    pub gene_sequence_origin: Option<types::CodeableConcept>,

    /// Todo
    pub gene: Option<types::CodeableConcept>,

    /// Todo
    pub source: Option<Vec<types::Reference>>,
}

/// Todo
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationGeneElement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Todo
    pub r#type: Option<types::CodeableConcept>,

    /// Todo
    pub element: Option<types::Identifier>,

    /// Todo
    pub source: Option<Vec<types::Reference>>,
}

/// Todo
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Todo
    pub target: Option<types::Identifier>,

    /// Todo
    pub r#type: Option<types::CodeableConcept>,

    /// Todo
    pub interaction: Option<types::CodeableConcept>,

    /// Todo
    pub organism: Option<types::CodeableConcept>,

    /// Todo
    pub organism_type: Option<types::CodeableConcept>,

    /// Todo
    pub amount_quantity: Option<types::Quantity>,

    /// Todo
    pub amount_range: Option<types::Range>,

    /// Todo
    pub amount_string: Option<types::String>,

    /// Todo
    pub amount_type: Option<types::CodeableConcept>,

    /// Todo
    pub source: Option<Vec<types::Reference>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceReferenceInformation;

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
