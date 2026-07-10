//! NutritionOrder
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
//!
//! Version: 5.0.0
//!
//! NutritionOrder Resource: A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
///
/// A NutritionOrder captures the details of what a patient should be fed,
/// covering oral diets, oral nutritional supplements, and enteral (tube)
/// feeding formulas. It records ordering context, the subject and encounter,
/// scheduling, texture and nutrient modifications, and any related allergies,
/// intolerances, and preferences. It is typically used within clinical
/// workflows to communicate nutrition requirements to dietary services.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::nutrition_order::NutritionOrder;
///
/// let value = NutritionOrder::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: NutritionOrder = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrder {
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

    /// Identifiers assigned to this order
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// Instantiates protocol or definition
    pub instantiates: Option<Vec<types::Uri>>,

    /// What this order fulfills
    pub based_on: Option<Vec<types::Reference>>,

    /// Composite Request ID
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: types::Code,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: types::Code,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// Who requires the diet, formula or nutritional supplement
    pub subject: types::Reference,

    /// The encounter associated with this nutrition order
    pub encounter: Option<types::Reference>,

    /// Information to support fulfilling of the nutrition order
    pub supporting_information: Option<Vec<types::Reference>>,

    /// Date and time the nutrition order was requested
    pub date_time: types::DateTime,

    /// Who ordered the diet, formula or nutritional supplement
    pub orderer: Option<types::Reference>,

    /// Who is desired to perform the administration of what is being ordered
    pub performer: Option<Vec<types::CodeableReference>>,

    /// List of the patient's food and nutrition-related allergies and intolerances
    pub allergy_intolerance: Option<Vec<types::Reference>>,

    /// Order-specific modifier about the type of food that should be given
    pub food_preference_modifier: Option<Vec<types::CodeableConcept>>,

    /// Order-specific modifier about the type of food that should not be given
    pub exclude_food_modifier: Option<Vec<types::CodeableConcept>>,

    /// Capture when a food item is brought in by the patient and/or family
    pub outside_food_allowed: Option<types::Boolean>,

    /// Oral diet components
    pub oral_diet: Option<NutritionOrderOralDiet>,

    /// Supplement components
    pub supplement: Option<Vec<NutritionOrderSupplement>>,

    /// Enteral formula components
    pub enteral_formula: Option<NutritionOrderEnteralFormula>,

    /// Comments
    pub note: Option<Vec<types::Annotation>>,
}

/// Oral diet components.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDiet {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of oral diet or diet restrictions that describe what can be consumed orally
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Scheduling information for oral diets
    pub schedule: Option<NutritionOrderOralDietSchedule>,

    /// Required nutrient modifications
    pub nutrient: Option<Vec<NutritionOrderOralDietNutrient>>,

    /// Required texture modifications
    pub texture: Option<Vec<NutritionOrderOralDietTexture>>,

    /// The required consistency of fluids and liquids provided to the patient
    pub fluid_consistency_type: Option<Vec<types::CodeableConcept>>,

    /// Instructions or additional information about the oral diet
    pub instruction: Option<types::String>,
}

/// Scheduling information for oral diets.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDietSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Scheduled frequency of diet
    pub timing: Option<Vec<types::Timing>>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

/// Required nutrient modifications.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDietNutrient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of nutrient that is being modified
    pub modifier: Option<types::CodeableConcept>,

    /// Quantity of the specified nutrient
    pub amount: Option<types::Quantity>,
}

/// Required texture modifications.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDietTexture {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code to indicate how to alter the texture of the foods, e.g. pureed
    pub modifier: Option<types::CodeableConcept>,

    /// Concepts that are used to identify an entity that is ingested for nutritional purposes
    pub food_type: Option<types::CodeableConcept>,
}

/// Supplement components.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderSupplement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of supplement product requested
    pub r#type: Option<types::CodeableReference>,

    /// Product or brand name of the nutritional supplement
    pub product_name: Option<types::String>,

    /// Scheduling information for supplements
    pub schedule: Option<NutritionOrderSupplementSchedule>,

    /// Amount of the nutritional supplement
    pub quantity: Option<types::Quantity>,

    /// Instructions or additional information about the oral supplement
    pub instruction: Option<types::String>,
}

/// Scheduling information for supplements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderSupplementSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Scheduled frequency of diet
    pub timing: Option<Vec<types::Timing>>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

/// Enteral formula components.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormula {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of enteral or infant formula
    pub base_formula_type: Option<types::CodeableReference>,

    /// Product or brand name of the enteral or infant formula
    pub base_formula_product_name: Option<types::String>,

    /// Intended type of device for the administration
    pub delivery_device: Option<Vec<types::CodeableReference>>,

    /// Components to add to the feeding
    pub additive: Option<Vec<NutritionOrderEnteralFormulaAdditive>>,

    /// Amount of energy per specified volume that is required
    pub caloric_density: Option<types::Quantity>,

    /// How the formula should enter the patient's gastrointestinal tract
    pub route_of_administration: Option<types::CodeableConcept>,

    /// Formula feeding instruction as structured data
    pub administration: Option<Vec<NutritionOrderEnteralFormulaAdministration>>,

    /// Upper limit on formula volume per unit of time
    pub max_volume_to_deliver: Option<types::Quantity>,

    /// Formula feeding instructions expressed as text
    pub administration_instruction: Option<types::Markdown>,
}

/// Components to add to the feeding.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormulaAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of modular component to add to the feeding
    pub r#type: Option<types::CodeableReference>,

    /// Product or brand name of the modular additive
    pub product_name: Option<types::String>,

    /// Amount of additive to be given or mixed in
    pub quantity: Option<types::Quantity>,
}

/// Formula feeding instruction as structured data.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormulaAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Scheduling information for enteral formula products
    pub schedule: Option<NutritionOrderEnteralFormulaAdministrationSchedule>,

    /// The volume of formula to provide
    pub quantity: Option<types::Quantity>,

    /// Speed with which the formula is provided per period of time
    pub rate_quantity: Option<types::Quantity>,

    /// Speed with which the formula is provided per period of time
    pub rate_ratio: Option<types::Ratio>,
}

/// Scheduling information for enteral formula products.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormulaAdministrationSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Scheduled frequency of enteral formula
    pub timing: Option<Vec<types::Timing>>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = NutritionOrder;

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
