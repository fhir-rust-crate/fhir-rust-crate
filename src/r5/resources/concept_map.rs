//! ConceptMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ConceptMap
//!
//! Version: 5.0.0
//!
//! ConceptMap Resource: A statement of relationships from one set of concepts to one or more other concepts.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
///
/// A ConceptMap describes how the concepts of one system relate to the
/// concepts of another. It is commonly used to translate codes between code
/// systems or value sets during data exchange and terminology operations such
/// as the `$translate` operation in FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMap;
///
/// let value = ConceptMap::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConceptMap = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMap {
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

    /// Canonical identifier for this concept map, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the concept map
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the concept map
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this concept map (computer friendly)
    pub name: Option<types::String>,

    /// Name for this concept map (human friendly)
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

    /// Natural language description of the concept map
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for concept map (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this concept map is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the ConceptMap was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the ConceptMap was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the ConceptMap is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the ConceptMap
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the ConceptMap
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the ConceptMap
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the ConceptMap
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Additional properties of the mapping
    pub property: Option<Vec<ConceptMapProperty>>,

    /// Definition of an additional attribute to act as a data source or target
    pub additional_attribute: Option<Vec<ConceptMapAdditionalAttribute>>,

    /// The source value set that contains the concepts that are being mapped
    pub source_scope_uri: Option<types::Uri>,

    /// The source value set that contains the concepts that are being mapped
    pub source_scope_canonical: Option<types::Canonical>,

    /// The target value set which provides context for the mappings
    pub target_scope_uri: Option<types::Uri>,

    /// The target value set which provides context for the mappings
    pub target_scope_canonical: Option<types::Canonical>,

    /// Same source and target systems
    pub group: Option<Vec<ConceptMapGroup>>,
}

/// Additional properties of the mapping.
///
/// A property defines an attribute that can be attached to the mappings within
/// this ConceptMap, and that may be referred to in the `$translate` operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifies the property on the mappings, and when referred to in the $translate operation
    pub code: types::Code,

    /// Formal identifier for the property
    pub uri: Option<types::Uri>,

    /// Why the property is defined, and/or what it conveys
    pub description: Option<types::String>,

    /// Coding | string | integer | boolean | dateTime | decimal | code
    pub r#type: types::Code,

    /// The CodeSystem from which code values come
    pub system: Option<types::Canonical>,
}

/// Definition of an additional attribute to act as a data source or target.
///
/// Additional attributes describe data elements that can be used as sources or
/// targets for dependsOn and product relationships within group element targets.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapAdditionalAttribute {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifies this additional attribute through this resource
    pub code: types::Code,

    /// Formal identifier for the data element referred to in this attribte
    pub uri: Option<types::Uri>,

    /// Why the additional attribute is defined, and/or what the data element it refers to is
    pub description: Option<types::String>,

    /// code | Coding | string | boolean | Quantity
    pub r#type: types::Code,
}

/// Same source and target systems.
///
/// A group organizes mappings that share the same source and target code
/// systems, containing the individual element-to-target mappings.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Source system where concepts to be mapped are defined
    pub source: Option<types::Canonical>,

    /// Target system that the concepts are to be mapped to
    pub target: Option<types::Canonical>,

    /// Mappings for a concept from the source set
    pub element: Vec<ConceptMapGroupElement>,

    /// What to do when there is no mapping target for the source concept
    pub unmapped: Option<ConceptMapGroupUnmapped>,
}

/// Mappings for a concept from the source set.
///
/// Each element identifies a source concept and its associated target concepts
/// within the enclosing group.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifies element being mapped
    pub code: Option<types::Code>,

    /// Display for the code
    pub display: Option<types::String>,

    /// Identifies the set of concepts being mapped
    pub value_set: Option<types::Canonical>,

    /// No mapping to a target concept for this source concept
    pub no_map: Option<types::Boolean>,

    /// Concept in target system for element
    pub target: Option<Vec<ConceptMapGroupElementTarget>>,
}

/// Concept in target system for element.
///
/// Describes a concept in the target system that the source element maps to,
/// including the relationship and any dependent or produced data elements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that identifies the target element
    pub code: Option<types::Code>,

    /// Display for the code
    pub display: Option<types::String>,

    /// Identifies the set of target concepts
    pub value_set: Option<types::Canonical>,

    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: types::Code,

    /// Description of status/issues in mapping
    pub comment: Option<types::String>,

    /// Property value for the source -> target mapping
    pub property: Option<Vec<ConceptMapGroupElementTargetProperty>>,

    /// Other properties required for this mapping
    pub depends_on: Option<Vec<ConceptMapGroupElementTargetDependsOn>>,

    /// Other data elements that this mapping also produces
    pub product: Option<Vec<types::Element>>,
}

/// Property value for the source -> target mapping.
///
/// Attaches a value for a property (declared in `ConceptMap.property`) to a
/// specific source-to-target mapping.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTargetProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to ConceptMap.property.code
    pub code: types::Code,

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

    /// Value of the property for this concept
    pub value_code: Option<types::Code>,
}

/// Other properties required for this mapping.
///
/// Indicates that a mapping is only valid when a referenced data element has a
/// particular value, allowing conditional or context-dependent mappings.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTargetDependsOn {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A reference to a mapping attribute defined in ConceptMap.additionalAttribute
    pub attribute: types::Code,

    /// Value of the referenced data element
    pub value_code: Option<types::Code>,

    /// Value of the referenced data element
    pub value_coding: Option<types::Coding>,

    /// Value of the referenced data element
    pub value_string: Option<types::String>,

    /// Value of the referenced data element
    pub value_boolean: Option<types::Boolean>,

    /// Value of the referenced data element
    pub value_quantity: Option<types::Quantity>,

    /// The mapping depends on a data element with a value from this value set
    pub value_set: Option<types::Canonical>,
}

/// What to do when there is no mapping target for the source concept and
/// `ConceptMap.group.element.noMap` is not true.
///
/// Describes fallback behavior for unmapped source concepts, such as reusing
/// the source code, using a fixed code, or delegating to another ConceptMap.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupUnmapped {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// use-source-code | fixed | other-map
    pub mode: types::Code,

    /// Fixed code when mode = fixed
    pub code: Option<types::Code>,

    /// Display for the code
    pub display: Option<types::String>,

    /// Fixed code set when mode = fixed
    pub value_set: Option<types::Canonical>,

    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: Option<types::Code>,

    /// canonical reference to an additional ConceptMap to use for mapping if the source concept is unmapped
    pub other_map: Option<types::Canonical>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ConceptMap;

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
