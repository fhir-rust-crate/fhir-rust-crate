//! Ingredient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Ingredient
//!
//! Version: 5.0.0
//!
//! Ingredient Resource: An ingredient of a manufactured item or pharmaceutical product.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// An ingredient of a manufactured item or pharmaceutical product.
///
/// The Ingredient resource describes a single substance that is a constituent
/// part of a manufactured item or pharmaceutical product. It captures the
/// substance itself, the role it plays within the product (for example active
/// versus inactive, or excipient), any more precise function it performs (such
/// as antioxidant or alkalizing agent), and its strength expressed either as a
/// presentation amount per dosage unit or as a concentration per volume or mass.
/// Strength may additionally be stated in terms of a reference substance, and
/// may vary by country to accommodate differing regulatory expressions.
///
/// In FHIR R5 the Ingredient resource is part of the medication definition
/// domain and is used chiefly for regulated product information and medicinal
/// product authorization. It is typically referenced by definitional resources
/// such as `MedicinalProductDefinition`, `ManufacturedItemDefinition`, and
/// `AdministrableProductDefinition` through its `for` element, rather than
/// describing patient-specific medication use. The substance identity is carried
/// by a [`CodeableReference`](crate::r5::types::CodeableReference), and each
/// quantitative value is modeled with types such as
/// [`Ratio`](crate::r5::types::Ratio), [`Quantity`](crate::r5::types::Quantity),
/// and [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # See also
///
/// The `Substance` and `MedicinalProductDefinition` resources describe the
/// substance and product that an Ingredient relates to, while
/// [`CodeableReference`](crate::r5::types::CodeableReference) and
/// [`Reference`](crate::r5::types::Reference) provide the linking mechanisms.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::ingredient::Ingredient;
///
/// let value = Ingredient::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Ingredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
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

    /// An identifier or code by which the ingredient can be referenced
    pub identifier: Option<types::Identifier>,

    /// Publication lifecycle status of this Ingredient record, one of draft, active, retired, or unknown.
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// References to the product or products that this ingredient is a constituent part of.
    pub r#for: Option<Vec<types::Reference>>,

    /// The purpose the ingredient serves within the product, for example active or inactive.
    pub role: types::CodeableConcept,

    /// Precise action within the drug product, e.g. antioxidant, alkalizing agent
    pub function: Option<Vec<types::CodeableConcept>>,

    /// A classification of the ingredient according to where in the physical item it tends to be used
    pub group: Option<types::CodeableConcept>,

    /// If the ingredient is a known or suspected allergen
    pub allergenic_indicator: Option<types::Boolean>,
    /// Primitive extension sibling for [`allergenic_indicator`](Self::allergenic_indicator) (FHIR `_allergenicIndicator`).
    #[serde(rename = "_allergenicIndicator")]
    pub allergenic_indicator_ext: Option<types::Element>,

    /// A place for providing any notes that are relevant to the component
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// An organization that manufactures this ingredient
    pub manufacturer: Option<Vec<IngredientManufacturer>>,

    /// The substance that comprises this ingredient, including its identity and strength.
    pub substance: IngredientSubstance,
}

/// An organization that manufactures this ingredient.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientManufacturer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// allowed | possible | actual
    pub role: Option<types::Code>,
    /// Primitive extension sibling for [`role`](Self::role) (FHIR `_role`).
    #[serde(rename = "_role")]
    pub role_ext: Option<types::Element>,

    /// An organization that manufactures this ingredient
    pub manufacturer: types::Reference,
}

/// The substance that comprises this ingredient.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A code or full resource that represents the ingredient substance
    pub code: types::CodeableReference,

    /// The quantity of substance, per presentation, or per volume or mass, and type of quantity
    pub strength: Option<Vec<IngredientSubstanceStrength>>,
}

/// The quantity of substance, per presentation, or per volume or mass, and type of quantity.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstanceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The quantity of substance in the unit of presentation
    pub presentation_ratio: Option<types::Ratio>,

    /// The quantity of substance in the unit of presentation
    pub presentation_ratio_range: Option<types::RatioRange>,

    /// The quantity of substance in the unit of presentation
    pub presentation_codeable_concept: Option<types::CodeableConcept>,

    /// The quantity of substance in the unit of presentation
    pub presentation_quantity: Option<types::Quantity>,

    /// Text of either the whole presentation strength or a part of it
    pub text_presentation: Option<types::String>,
    /// Primitive extension sibling for [`text_presentation`](Self::text_presentation) (FHIR `_textPresentation`).
    #[serde(rename = "_textPresentation")]
    pub text_presentation_ext: Option<types::Element>,

    /// The strength per unitary volume (or mass)
    pub concentration_ratio: Option<types::Ratio>,

    /// The strength per unitary volume (or mass)
    pub concentration_ratio_range: Option<types::RatioRange>,

    /// The strength per unitary volume (or mass)
    pub concentration_codeable_concept: Option<types::CodeableConcept>,

    /// The strength per unitary volume (or mass)
    pub concentration_quantity: Option<types::Quantity>,

    /// Text of either the whole concentration strength or a part of it
    pub text_concentration: Option<types::String>,
    /// Primitive extension sibling for [`text_concentration`](Self::text_concentration) (FHIR `_textConcentration`).
    #[serde(rename = "_textConcentration")]
    pub text_concentration_ext: Option<types::Element>,

    /// A code that indicates if the strength is based on the ingredient substance as stated or on the substance base
    pub basis: Option<types::CodeableConcept>,

    /// When strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`).
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// Where the strength range applies
    pub country: Option<Vec<types::CodeableConcept>>,

    /// Strength expressed in terms of a reference substance
    pub reference_strength: Option<Vec<IngredientSubstanceStrengthReferenceStrength>>,
}

/// Strength expressed in terms of a reference substance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstanceStrengthReferenceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Relevant reference substance
    pub substance: types::CodeableReference,

    /// Strength expressed in terms of a reference substance
    pub strength_ratio: Option<types::Ratio>,

    /// Strength expressed in terms of a reference substance
    pub strength_ratio_range: Option<types::RatioRange>,

    /// Strength expressed in terms of a reference substance
    pub strength_quantity: Option<types::Quantity>,

    /// When strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`).
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// Where the strength range applies
    pub country: Option<Vec<types::CodeableConcept>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Ingredient;

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
