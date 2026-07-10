//! BiologicallyDerivedProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
//!
//! Version: 5.0.0
//!
//! BiologicallyDerivedProduct Resource: A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// BiologicallyDerivedProduct
///
/// A biological material originating from a biological entity intended to be
/// transplanted or infused into another (possibly the same) biological entity.
/// This resource captures products such as organs, tissues, fluids, cells, and
/// biological agents, along with their provenance, collection, storage, and
/// instance-specific properties. It supports traceability of the material from
/// its biological source event through processing and distribution.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::biologically_derived_product::BiologicallyDerivedProduct;
///
/// let value = BiologicallyDerivedProduct::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: BiologicallyDerivedProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProduct {
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

    /// organ | tissue | fluid | cells | biologicalAgent
    pub product_category: Option<types::Coding>,

    /// A code that identifies the kind of this biologically derived product
    pub product_code: Option<types::CodeableConcept>,

    /// The parent biologically-derived product
    pub parent: Option<Vec<types::Reference>>,

    /// Request to obtain and/or infuse this product
    pub request: Option<Vec<types::Reference>>,

    /// Instance identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// An identifier that supports traceability to the event during which material
    /// in this product from one or more biological entities was obtained or pooled
    pub biological_source_event: Option<types::Identifier>,

    /// Processing facilities responsible for the labeling and distribution of this
    /// biologically derived product
    pub processing_facility: Option<Vec<types::Reference>>,

    /// A unique identifier for an aliquot of a product
    pub division: Option<types::String>,

    /// available | unavailable
    pub product_status: Option<types::Coding>,

    /// Date, and where relevant time, of expiration
    pub expiration_date: Option<types::DateTime>,

    /// How this product was collected
    pub collection: Option<BiologicallyDerivedProductCollection>,

    /// Product storage temperature requirements
    pub storage_temp_requirements: Option<types::Range>,

    /// A property that is specific to this BiologicallyDerviedProduct instance
    pub property: Option<Vec<BiologicallyDerivedProductProperty>>,
}

/// BiologicallyDerivedProductCollection
///
/// How this product was collected, including who performed the collection, the
/// source patient or organization, and the time it took place.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductCollection {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Individual performing collection
    pub collector: Option<types::Reference>,

    /// The patient who underwent the medical procedure to collect the product or
    /// the organization that facilitated the collection
    pub source: Option<types::Reference>,

    /// Time of product collection
    pub collected_date_time: Option<types::DateTime>,

    /// Time of product collection
    pub collected_period: Option<types::Period>,
}

/// BiologicallyDerivedProductProperty
///
/// A property that is specific to this BiologicallyDerivedProduct instance,
/// expressed as a typed code paired with one of several possible value types.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that specifies the property
    pub r#type: types::CodeableConcept,

    /// Property values
    pub value_boolean: Option<types::Boolean>,

    /// Property values
    pub value_integer: Option<types::Integer>,

    /// Property values
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Property values
    pub value_period: Option<types::Period>,

    /// Property values
    pub value_quantity: Option<types::Quantity>,

    /// Property values
    pub value_range: Option<types::Range>,

    /// Property values
    pub value_ratio: Option<types::Ratio>,

    /// Property values
    pub value_string: Option<types::String>,

    /// Property values
    pub value_attachment: Option<types::Attachment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = BiologicallyDerivedProduct;

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
