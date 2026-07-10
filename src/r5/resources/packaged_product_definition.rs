//! PackagedProductDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PackagedProductDefinition
//!
//! Version: 5.0.0
//!
//! PackagedProductDefinition Resource: A medically related item or items, in a container or package.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A medically related item or items, in a container or package.
///
/// PackagedProductDefinition describes a packaged product, such as a box of
/// tablets, a bottle of liquid, or a multi-component kit, together with its
/// packaging hierarchy, legal status of supply, marketing status, and the
/// medically related items contained within. It is used in regulated medicinal
/// product definitions to capture how a product is presented and supplied.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::packaged_product_definition::PackagedProductDefinition;
///
/// let value = PackagedProductDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PackagedProductDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinition {
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

    /// A unique identifier for this package as whole - not for the content of the package
    pub identifier: Option<Vec<types::Identifier>>,

    /// A name for this package. Typically as listed in a drug formulary, catalogue, inventory etc
    pub name: Option<types::String>,

    /// A high level category e.g. medicinal product, raw material, shipping container etc
    pub r#type: Option<types::CodeableConcept>,

    /// The product that this is a pack for
    pub package_for: Option<Vec<types::Reference>>,

    /// The status within the lifecycle of this item. High level - not intended to duplicate details elsewhere e.g. legal status, or authorization/marketing status
    pub status: Option<types::CodeableConcept>,

    /// The date at which the given status became applicable
    pub status_date: Option<types::DateTime>,

    /// A total of the complete count of contained items of a particular type/form, independent of sub-packaging or organization. This can be considered as the pack size
    pub contained_item_quantity: Option<Vec<types::Quantity>>,

    /// Textual description. Note that this is not the name of the package or product
    pub description: Option<types::Markdown>,

    /// The legal status of supply of the packaged item as classified by the regulator
    pub legal_status_of_supply: Option<Vec<PackagedProductDefinitionLegalStatusOfSupply>>,

    /// Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated
    pub marketing_status: Option<Vec<types::MarketingStatus>>,

    /// Identifies if the drug product is supplied with another item such as a diluent or adjuvant
    pub copackaged_indicator: Option<types::Boolean>,

    /// Manufacturer of this package type (multiple means these are all possible manufacturers)
    pub manufacturer: Option<Vec<types::Reference>>,

    /// Additional information or supporting documentation about the packaged product
    pub attached_document: Option<Vec<types::Reference>>,

    /// A packaging item, as a container for medically related items, possibly with other packaging items within, or a packaging component, such as bottle cap
    pub packaging: Option<PackagedProductDefinitionPackaging>,

    /// Allows the key features to be recorded, such as "hospital pack", "nurse prescribable"
    pub characteristic: Option<Vec<::serde_json::Value>>,
}

/// The legal status of supply of the packaged item as classified by the regulator.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionLegalStatusOfSupply {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The actual status of supply. In what situation this package type may be supplied for use
    pub code: Option<types::CodeableConcept>,

    /// The place where the legal status of supply applies
    pub jurisdiction: Option<types::CodeableConcept>,
}

/// A packaging item, as a container for medically related items, possibly with
/// other packaging items within, or a packaging component, such as bottle cap.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionPackaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// An identifier that is specific to this particular part of the packaging. Including possibly a Data Carrier Identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// The physical type of the container of the items
    pub r#type: Option<types::CodeableConcept>,

    /// Is this a part of the packaging (e.g. a cap or bottle stopper), rather than the packaging itself (e.g. a bottle or vial)
    pub component_part: Option<types::Boolean>,

    /// The quantity of this level of packaging in the package that contains it (with the outermost level being 1)
    pub quantity: Option<types::Integer>,

    /// Material type of the package item
    pub material: Option<Vec<types::CodeableConcept>>,

    /// A possible alternate material for this part of the packaging, that is allowed to be used instead of the usual material
    pub alternate_material: Option<Vec<types::CodeableConcept>>,

    /// Shelf Life and storage information
    pub shelf_life_storage: Option<Vec<types::ProductShelfLife>>,

    /// Manufacturer of this packaging item (multiple means these are all potential manufacturers)
    pub manufacturer: Option<Vec<types::Reference>>,

    /// General characteristics of this item
    pub property: Option<Vec<PackagedProductDefinitionPackagingProperty>>,

    /// The item(s) within the packaging
    pub contained_item: Option<Vec<PackagedProductDefinitionPackagingContainedItem>>,

    /// Allows containers (and parts of containers) within containers, still as a part of single packaged product
    pub packaging: Option<Vec<PackagedProductDefinitionPackaging>>,
}

/// General characteristics of a packaging item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionPackagingProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A code expressing the type of characteristic
    pub r#type: types::CodeableConcept,

    /// A value for the characteristic (CodeableConcept)
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// A value for the characteristic (Quantity)
    pub value_quantity: Option<types::Quantity>,

    /// A value for the characteristic (date)
    pub value_date: Option<types::Date>,

    /// A value for the characteristic (boolean)
    pub value_boolean: Option<types::Boolean>,

    /// A value for the characteristic (Attachment)
    pub value_attachment: Option<types::Attachment>,
}

/// The item(s) within the packaging.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionPackagingContainedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The actual item(s) of medication, as manufactured, or a device, or other medically related item (food, biologicals, raw materials, medical fluids, gases etc.), as contained in the package
    pub item: types::CodeableReference,

    /// The number of this type of item within this packaging or for continuous items such as liquids it is the quantity (for example 25ml)
    pub amount: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PackagedProductDefinition;

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
