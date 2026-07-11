//! NutritionProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionProduct
//!
//! Version: 5.0.0
//!
//! NutritionProduct Resource: A food or supplement that is consumed by patients.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A food or supplement that is consumed by patients.
///
/// NutritionProduct is a definitional resource in FHIR R5 that describes a
/// food, enteral or oral nutritional supplement, infant formula, thickener, or
/// similar consumable product. It captures the product's classification, its
/// nutritional composition expressed as nutrients, the ingredients it contains,
/// any known or suspected allergens, descriptive characteristics such as color,
/// texture, or preparation, and one or more physical instances identified by
/// lot number, expiry date, or serial identifier. The resource lets systems
/// represent products consistently across nutrition ordering, dispensing,
/// inventory, and product-catalog scenarios so that clinical, dietary, and
/// supply workflows can reference a single well-defined product definition.
///
/// In typical use, a NutritionProduct is referenced by ordering and
/// administration resources rather than describing a specific patient event on
/// its own. The product's status indicates whether the definition is active,
/// inactive, or entered in error, and its manufacturer and category support
/// catalog lookup and reporting.
///
/// # See also
///
/// Related resources and data types include the `NutritionOrder` and
/// `NutritionIntake` resources that reference nutrition products,
/// [`Patient`](crate::r5::resources::patient::Patient) as the consumer of the
/// product, [`CodeableConcept`](crate::r5::types::CodeableConcept) used for
/// coded classification, and
/// [`CodeableReference`](crate::r5::types::CodeableReference) used to point to
/// nutrients, ingredients, and allergens.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_product::NutritionProduct;
///
/// let value = NutritionProduct::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: NutritionProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProduct {
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

    /// Coded identifier for the product that conveys its detailed nutrients and ingredients, drawn from food or supplement terminologies
    pub code: Option<types::CodeableConcept>,

    /// Lifecycle state of the product definition: active, inactive, or entered-in-error
    pub status: types::Code,

    /// Broad product groups or categories used to classify the product, such as Legume and Legume Products, Beverages, or Beef Products
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Manufacturer, representative or officially responsible for the product
    pub manufacturer: Option<Vec<types::Reference>>,

    /// The product's nutritional composition, expressed as nutrient amounts such as per pack, per serving, or per dose
    pub nutrient: Option<Vec<NutritionProductNutrient>>,

    /// Ingredients contained in this product
    pub ingredient: Option<Vec<NutritionProductIngredient>>,

    /// Known or suspected allergens present in the product, supporting allergy screening and safe consumption
    pub known_allergen: Option<Vec<types::CodeableReference>>,

    /// Specifies descriptive properties of the nutrition product
    pub characteristic: Option<Vec<NutritionProductCharacteristic>>,

    /// One or several physical instances or occurrences of the nutrition product
    pub instance: Option<Vec<NutritionProductInstance>>,

    /// Comments made about the product
    pub note: Option<Vec<types::Annotation>>,
}

/// The product's nutritional information expressed by the nutrients.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductNutrient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The (relevant) nutrients in the product
    pub item: Option<types::CodeableReference>,

    /// The amount of nutrient expressed in one or more units: X per pack / per serving / per dose
    pub amount: Option<Vec<types::Ratio>>,
}

/// Ingredients contained in this product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The ingredient contained in the product
    pub item: types::CodeableReference,

    /// The amount of ingredient that is in the product
    pub amount: Option<Vec<types::Ratio>>,
}

/// Specifies descriptive properties of the nutrition product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code specifying the type of characteristic
    pub r#type: types::CodeableConcept,

    /// The value of the characteristic (CodeableConcept)
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// The value of the characteristic (string)
    pub value_string: Option<types::String>,

    /// The value of the characteristic (Quantity)
    pub value_quantity: Option<types::Quantity>,

    /// The value of the characteristic (base64Binary)
    pub value_base64_binary: Option<types::Base64Binary>,

    /// The value of the characteristic (Attachment)
    pub value_attachment: Option<types::Attachment>,

    /// The value of the characteristic (boolean)
    pub value_boolean: Option<types::Boolean>,
}

/// One or several physical instances or occurrences of the nutrition product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The amount of items or instances
    pub quantity: Option<types::Quantity>,

    /// The identifier for the physical instance, typically a serial number or manufacturer number
    pub identifier: Option<Vec<types::Identifier>>,

    /// The name for the specific product
    pub name: Option<types::String>,

    /// The identification of the batch or lot of the product
    pub lot_number: Option<types::String>,

    /// The expiry date or date and time for the product
    pub expiry: Option<types::DateTime>,

    /// The date until which the product is expected to be good for consumption
    pub use_by: Option<types::DateTime>,

    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    pub biological_source_event: Option<types::Identifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = NutritionProduct;

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
