//! Medication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Medication
//!
//! Version: 5.0.0
//!
//! Medication Resource: This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// This resource is primarily used for the identification and definition of a
/// medication, including its ingredients, for the purposes of prescribing,
/// dispensing, and administering. In FHIR R5 the Medication resource captures
/// the code identifying the drug, its dose form, total volume, active and
/// inactive ingredients with their strengths, and packaging details such as lot
/// number and expiration date. It is referenced by workflow resources like
/// MedicationRequest, MedicationDispense, and MedicationAdministration.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication::Medication;
///
/// let value = Medication::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Medication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Medication {
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

    /// Business identifier for this medication
    pub identifier: Option<Vec<types::Identifier>>,

    /// Codes that identify this medication
    pub code: Option<types::CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: Option<types::Code>,

    /// Organization that has authorization to market medication
    pub marketing_authorization_holder: Option<types::Reference>,

    /// powder | tablets | capsule +
    pub dose_form: Option<types::CodeableConcept>,

    /// When the specified product code does not infer a package size, this is the specific amount of drug in the product
    pub total_volume: Option<types::Quantity>,

    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationIngredient>>,

    /// Details about packaged medications
    pub batch: Option<MedicationBatch>,

    /// Knowledge about this medication
    pub definition: Option<types::Reference>,
}

/// Active or inactive ingredient contained in the medication, identifying the
/// substance or medication and, when known, the strength it contributes.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The ingredient (substance or medication) that the ingredient.strength relates to
    pub item: types::CodeableReference,

    /// Active ingredient indicator
    pub is_active: Option<types::Boolean>,

    /// Quantity of ingredient present, as a Ratio
    pub strength_ratio: Option<types::Ratio>,

    /// Quantity of ingredient present, as a CodeableConcept
    pub strength_codeable_concept: Option<types::CodeableConcept>,

    /// Quantity of ingredient present, as a Quantity
    pub strength_quantity: Option<types::Quantity>,
}

/// Details about the packaged medication, such as the lot number assigned to the
/// batch and the date on which it will expire.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationBatch {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifier assigned to batch
    pub lot_number: Option<types::String>,

    /// When batch will expire
    pub expiration_date: Option<types::DateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Medication;

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
