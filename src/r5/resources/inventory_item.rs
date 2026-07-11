//! InventoryItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InventoryItem
//!
//! Version: 5.0.0
//!
//! InventoryItem Resource: functional description of an inventory item used in inventory and supply-related workflows.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A functional description of an inventory item used in inventory and
/// supply-related workflows.
///
/// The InventoryItem resource represents a class of item that is stocked,
/// counted, ordered, or otherwise tracked within an inventory or supply chain.
/// It is a lightweight, inventory-oriented description rather than a full
/// clinical definition: it captures the item's business identifiers, status,
/// category, and type codes, one or more names (brand, common, functional, or
/// generic), the organizations responsible for it, human-readable descriptive
/// characteristics, associations to related products, the base unit of measure
/// and net content, and physical instance details such as serial number, lot or
/// batch number, and expiry. In FHIR R5 it is intended for materials
/// management, warehousing, stock counting, ordering, and related administrative
/// and supply-chain workflows, and may point to a more detailed clinical product
/// resource when one exists.
///
/// # Related resources
///
/// An inventory item frequently references organizations via
/// [`Organization`](crate::r5::resources::organization::Organization), and its
/// physical instances may be associated with a
/// [`Location`](crate::r5::resources::location::Location). When the tracked item
/// is a clinical product, the product reference commonly points to a
/// [`Medication`](crate::r5::resources::medication::Medication) or
/// [`Device`](crate::r5::resources::device::Device). Coded fields use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and quantities use
/// [`Quantity`](crate::r5::types::Quantity).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItem;
///
/// let value = InventoryItem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: InventoryItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
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

    /// Business identifier for the inventory item
    pub identifier: Option<Vec<types::Identifier>>,

    /// Lifecycle status of this inventory record: active, inactive, entered-in-error, or unknown.
    pub status: types::Code,

    /// Category or class of the item
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Coded designation of the specific type of item being inventoried, drawn from a code system.
    pub code: Option<Vec<types::CodeableConcept>>,

    /// The item name(s) - the brand name, or common name, functional name, generic name or others
    pub name: Option<Vec<InventoryItemName>>,

    /// Organization(s) responsible for the product
    pub responsible_organization: Option<Vec<InventoryItemResponsibleOrganization>>,

    /// Descriptive characteristics of the item
    pub description: Option<InventoryItemDescription>,

    /// The usage status like recalled, in use, discarded
    pub inventory_status: Option<Vec<types::CodeableConcept>>,

    /// The base unit of measure - the unit in which the product is used or counted
    pub base_unit: Option<types::CodeableConcept>,

    /// Net content or amount present in the item
    pub net_content: Option<types::Quantity>,

    /// Association with other items or products
    pub association: Option<Vec<InventoryItemAssociation>>,

    /// Characteristic of the item
    pub characteristic: Option<Vec<InventoryItemCharacteristic>>,

    /// Instances or occurrences of the product
    pub instance: Option<InventoryItemInstance>,

    /// Reference to a more detailed clinical product resource, such as a Medication or Device, that this inventory item represents.
    pub product_reference: Option<types::Reference>,
}

/// The item name(s) - the brand name, or common name, functional name, generic
/// name or others.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of name e.g. 'brand-name', 'functional-name', 'common-name'
    pub name_type: types::Coding,

    /// The language used to express the item name
    pub language: types::Code,

    /// The name or designation of the item
    pub name: types::String,
}

/// Organization(s) responsible for the product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemResponsibleOrganization {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The role of the organization e.g. manufacturer, distributor, or other
    pub role: types::CodeableConcept,

    /// An organization that is associated with the item
    pub organization: types::Reference,
}

/// Descriptive characteristics of the item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemDescription {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The language that is used in the item description
    pub language: Option<types::Code>,

    /// Textual description of the item
    pub description: Option<types::String>,
}

/// Association with other items or products.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemAssociation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of association between the device and the other item
    pub association_type: types::CodeableConcept,

    /// The related item or product
    pub related_item: types::Reference,

    /// The quantity of the product in this product
    pub quantity: types::Ratio,
}

/// Characteristic of the item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The characteristic that is being defined
    pub characteristic_type: types::CodeableConcept,

    /// The value of the attribute
    pub value_string: Option<types::String>,

    /// The value of the attribute
    pub value_integer: Option<types::Integer>,

    /// The value of the attribute
    pub value_decimal: Option<types::Decimal>,

    /// The value of the attribute
    pub value_boolean: Option<types::Boolean>,

    /// The value of the attribute
    pub value_url: Option<types::Url>,

    /// The value of the attribute
    pub value_date_time: Option<types::DateTime>,

    /// The value of the attribute
    pub value_quantity: Option<types::Quantity>,

    /// The value of the attribute
    pub value_range: Option<types::Range>,

    /// The value of the attribute
    pub value_ratio: Option<types::Ratio>,

    /// The value of the attribute
    pub value_annotation: Option<types::Annotation>,

    /// The value of the attribute
    pub value_address: Option<types::Address>,

    /// The value of the attribute
    pub value_duration: Option<types::Duration>,

    /// The value of the attribute
    pub value_codeable_concept: Option<types::CodeableConcept>,
}

/// Instances or occurrences of the product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The identifier for the physical instance, typically a serial number
    pub identifier: Option<Vec<types::Identifier>>,

    /// The lot or batch number of the item
    pub lot_number: Option<types::String>,

    /// The expiry date or date and time for the product
    pub expiry: Option<types::DateTime>,

    /// The subject that the item is associated with
    pub subject: Option<types::Reference>,

    /// The location that the item is associated with
    pub location: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = InventoryItem;

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
