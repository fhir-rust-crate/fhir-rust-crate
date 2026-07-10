//! SupplyDelivery
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
//!
//! Version: 5.0.0
//!
//! SupplyDelivery Resource: Record of delivery of what is supplied.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Record of delivery of what is supplied.
///
/// The SupplyDelivery resource documents the supply and delivery of items such
/// as medication, devices, biologically derived products, and other substances.
/// It captures what was delivered, the quantity, when it occurred, who supplied
/// it, and where it was sent. It is commonly used in supply chain and inventory
/// workflows to record fulfillment of a supply request or order.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::supply_delivery::SupplyDelivery;
///
/// let value = SupplyDelivery::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SupplyDelivery = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplyDelivery {
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

    /// External identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Fulfills plan, proposal or order
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// in-progress | completed | abandoned | entered-in-error
    pub status: Option<types::Code>,

    /// Patient for whom the item is supplied
    pub patient: Option<types::Reference>,

    /// Category of supply event
    pub r#type: Option<types::CodeableConcept>,

    /// The item that is delivered or supplied
    pub supplied_item: Option<Vec<SupplyDeliverySuppliedItem>>,

    /// When event occurred
    pub occurrence_date_time: Option<types::DateTime>,

    /// When event occurred
    pub occurrence_period: Option<types::Period>,

    /// When event occurred
    pub occurrence_timing: Option<types::Timing>,

    /// The item supplier
    pub supplier: Option<types::Reference>,

    /// Where the delivery was sent
    pub destination: Option<types::Reference>,

    /// Who received the delivery
    pub receiver: Option<Vec<types::Reference>>,
}

/// The item that is delivered or supplied.
///
/// Describes a single item that is delivered as part of the supply delivery,
/// including the amount supplied and a reference or coded value identifying the
/// medication, substance, device, or biologically derived product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplyDeliverySuppliedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Amount supplied
    pub quantity: Option<types::Quantity>,

    /// Medication, Substance, Device or Biologically Derived Product supplied
    pub item_codeable_concept: Option<types::CodeableConcept>,

    /// Medication, Substance, Device or Biologically Derived Product supplied
    pub item_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SupplyDelivery;

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
