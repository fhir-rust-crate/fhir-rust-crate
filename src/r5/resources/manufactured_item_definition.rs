//! ManufacturedItemDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition
//!
//! Version: 5.0.0
//!
//! ManufacturedItemDefinition Resource: The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The definition and characteristics of a medicinal manufactured item, such as
/// a tablet, capsule, patch, or ampoule, as contained in a packaged medicinal
/// product. It captures the item exactly as it is manufactured, before any
/// transformation that may be needed for administration, and records its
/// manufactured dose form, unit of presentation, manufacturers, marketing
/// status, ingredients, physical components, and general characteristics.
///
/// In FHIR R5 this resource belongs to the medication definition family and is
/// used chiefly in medicinal product regulatory submissions, such as those
/// following the ISO IDMP standards, as well as in packaging and supply-chain
/// descriptions. A ManufacturedItemDefinition is typically referenced from a
/// packaged product to say what physical items a pack contains, and its
/// ingredients and constituents may point to more detailed substance and
/// ingredient definitions. Coded fields such as the manufactured dose form use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and availability is
/// expressed through [`MarketingStatus`](crate::r5::types::MarketingStatus).
///
/// # Related resources
///
/// See also the `PackagedProductDefinition`, `MedicinalProductDefinition`, and
/// `Ingredient` resources, which together describe a medicinal product and how
/// its manufactured items are packaged, marketed, and composed.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::manufactured_item_definition::ManufacturedItemDefinition;
///
/// let value = ManufacturedItemDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ManufacturedItemDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinition {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Business identifier for this manufactured item, distinct from the resource's logical id
    pub identifier: Option<Vec<types::Identifier>>,

    /// Publication status of this definition, one of draft, active, retired, or unknown
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// A descriptive name applied to this item, suitable for labeling or catalog display
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Dose form of the item as manufactured, such as tablet or capsule, before any transformation needed for administration
    pub manufactured_dose_form: types::CodeableConcept,

    /// The "real-world" units in which the quantity of the item is described
    pub unit_of_presentation: Option<types::CodeableConcept>,

    /// Manufacturer of the item, one of several possible, referencing an Organization
    pub manufacturer: Option<Vec<types::Reference>>,

    /// Allows specifying that an item is on the market for sale, or that it is
    /// not available, and the dates and locations associated
    pub marketing_status: Option<Vec<types::MarketingStatus>>,

    /// The ingredients of this manufactured item. Only needed if these are not
    /// specified by incoming references from the Ingredient resource
    pub ingredient: Option<Vec<types::CodeableConcept>>,

    /// General characteristics of this item
    pub property: Option<Vec<ManufacturedItemDefinitionProperty>>,

    /// Physical parts of the manufactured item, that it is intrisically made from
    pub component: Option<Vec<ManufacturedItemDefinitionComponent>>,
}

/// General characteristics of this item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A code expressing the type of characteristic
    pub r#type: types::CodeableConcept,

    /// A value for the characteristic
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// A value for the characteristic
    pub value_quantity: Option<types::Quantity>,

    /// A value for the characteristic
    pub value_date: Option<types::Date>,

    /// A value for the characteristic
    pub value_boolean: Option<types::Boolean>,

    /// A value for the characteristic
    pub value_markdown: Option<types::Markdown>,

    /// A value for the characteristic
    pub value_attachment: Option<types::Attachment>,

    /// A value for the characteristic
    pub value_reference: Option<types::Reference>,
}

/// Physical parts of the manufactured item, that it is intrisically made from.
/// This is distinct from the ingredients that are part of its chemical makeup.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Defining type of the component e.g. shell, layer, ink
    pub r#type: types::CodeableConcept,

    /// The function of this component within the item e.g. delivers active
    /// ingredient, masks taste
    pub function: Option<Vec<types::CodeableConcept>>,

    /// The measurable amount of total quantity of all substances in the
    /// component, expressable in different ways (e.g. by mass or volume)
    pub amount: Option<Vec<types::Quantity>>,

    /// A reference to a constituent of the manufactured item as a whole, linked
    /// here so that its component location within the item can be indicated
    pub constituent: Option<Vec<ManufacturedItemDefinitionComponentConstituent>>,

    /// General characteristics of this component
    pub property: Option<Vec<ManufacturedItemDefinitionProperty>>,

    /// A component that this component contains or is made from
    pub component: Option<Vec<ManufacturedItemDefinitionComponent>>,
}

/// A reference to a constituent of the manufactured item as a whole, linked here
/// so that its component location within the item can be indicated.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionComponentConstituent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The measurable amount of the substance, expressable in different ways
    /// (e.g. by mass or volume)
    pub amount: Option<Vec<types::Quantity>>,

    /// The physical location of the constituent/ingredient within the component
    pub location: Option<Vec<types::CodeableConcept>>,

    /// The function of this constituent within the component e.g. binder
    pub function: Option<Vec<types::CodeableConcept>>,

    /// The ingredient that is the constituent of the given component
    pub has_ingredient: Option<Vec<types::CodeableReference>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ManufacturedItemDefinition;

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
