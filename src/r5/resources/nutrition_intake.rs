//! NutritionIntake
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionIntake
//!
//! Version: 5.0.0
//!
//! NutritionIntake Resource: A record of food or fluid that is being consumed by a patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A record of food or fluid that is being consumed by a patient. A
/// NutritionIntake may indicate that the patient may be consuming the food or
/// fluid now or has consumed the food or fluid in the past. The source of this
/// information can be the patient, a significant other (such as a family member
/// or spouse), or a clinician. A common scenario where this information is
/// captured is during the history taking process at an encounter or visit.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::nutrition_intake::NutritionIntake;
///
/// let value = NutritionIntake::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: NutritionIntake = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntake {
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

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// Fulfils plan, proposal or order
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: types::Code,

    /// Reason for current status
    pub status_reason: Option<Vec<types::CodeableConcept>>,

    /// Code representing an overall type of nutrition intake
    pub code: Option<types::CodeableConcept>,

    /// Who is/was consuming the food or fluid
    pub subject: types::Reference,

    /// Encounter associated with NutritionIntake
    pub encounter: Option<types::Reference>,

    /// The date/time or interval when the food or fluid is/was consumed
    pub occurrence_date_time: Option<types::DateTime>,

    /// The date/time or interval when the food or fluid is/was consumed
    pub occurrence_period: Option<types::Period>,

    /// When the intake was recorded
    pub recorded: Option<types::DateTime>,

    /// Person or organization that provided the information about the consumption of this food or fluid
    pub reported_boolean: Option<types::Boolean>,

    /// Person or organization that provided the information about the consumption of this food or fluid
    pub reported_reference: Option<types::Reference>,

    /// What food or fluid product or item was consumed
    pub consumed_item: Vec<NutritionIntakeConsumedItem>,

    /// Total nutrient for the whole meal, product, serving
    pub ingredient_label: Option<Vec<NutritionIntakeIngredientLabel>>,

    /// Who was performed in the intake
    pub performer: Option<Vec<NutritionIntakePerformer>>,

    /// Where the intake occurred
    pub location: Option<types::Reference>,

    /// Additional supporting information
    pub derived_from: Option<Vec<types::Reference>>,

    /// Reason for why the food or fluid is /was consumed
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Further information about the consumption
    pub note: Option<Vec<types::Annotation>>,
}

/// What food or fluid product or item was consumed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakeConsumedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of food or fluid product
    pub r#type: types::CodeableConcept,

    /// Code that identifies the food or fluid product that was consumed
    pub nutrition_product: types::CodeableReference,

    /// Scheduled frequency of consumption
    pub schedule: Option<types::Timing>,

    /// Quantity of the specified food
    pub amount: Option<types::Quantity>,

    /// Rate at which enteral feeding was administered
    pub rate: Option<types::Quantity>,

    /// Flag to indicate if the food or fluid item was refused or otherwise not consumed
    pub not_consumed: Option<types::Boolean>,

    /// Reason food or fluid was not consumed
    pub not_consumed_reason: Option<types::CodeableConcept>,
}

/// Total nutrient for the whole meal, product, serving.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakeIngredientLabel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Total nutrient consumed
    pub nutrient: types::CodeableReference,

    /// Total amount of nutrient consumed
    pub amount: types::Quantity,
}

/// Who was performed in the intake.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of performer
    pub function: Option<types::CodeableConcept>,

    /// Who performed the intake
    pub actor: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = NutritionIntake;

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
