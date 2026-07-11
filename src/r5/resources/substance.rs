//! Substance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Substance
//!
//! Version: 5.0.0
//!
//! Substance Resource: A homogeneous material with a definite composition.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Substance
///
/// A homogeneous material with a definite composition. This resource captures
/// substances used in healthcare, whether as a specific physical instance (such
/// as a particular batch or package) or as a general kind of substance. It
/// records the substance code, category, quantity, expiry, and its constituent
/// ingredients along with their relative amounts.
///
/// Substances are used across many clinical and administrative workflows: as the
/// active or inactive ingredients that make up a medication, as allergens or
/// irritants referenced from an allergy or intolerance record, as specimens or
/// samples handled by a laboratory, or as raw materials tracked through supply
/// and manufacturing processes. A `Substance` instance may represent either a
/// specific physical package or lot (an "instance") or a general category of
/// material (a "kind"), as indicated by the `instance` field. The `code` field
/// identifies what the substance is, typically using a standard terminology
/// such as SNOMED CT or a national drug/substance coding system, while
/// `ingredient` describes the substance's own composition when it is itself a
/// mixture or compound.
///
/// # See also
///
/// Substances are commonly referenced from medication and clinical resources
/// such as `Medication`, `MedicationKnowledge`, and `AllergyIntolerance`, and
/// use shared data types including [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`CodeableReference`](crate::r5::types::CodeableReference), and
/// [`Quantity`](crate::r5::types::Quantity) to describe classification and amount.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance::Substance;
///
/// let value = Substance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Substance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Substance {
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

    /// Unique identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// True if this represents a specific physical instance/package/lot of the substance rather than a general kind
    pub instance: types::Boolean,

    /// The status of the substance record: active | inactive | entered-in-error
    pub status: Option<types::Code>,

    /// Classification of the substance into one or more categories, e.g. drug, allergen, or biological
    pub category: Option<Vec<types::CodeableConcept>>,

    /// The substance's identity, coded using a terminology such as SNOMED CT, optionally with a direct reference
    pub code: types::CodeableReference,

    /// Textual description of the substance, comments
    pub description: Option<types::Markdown>,

    /// When no longer valid to use
    pub expiry: Option<types::DateTime>,

    /// Amount of substance in the package
    pub quantity: Option<types::Quantity>,

    /// Composition information about the substance, describing its constituent components and their relative amounts
    pub ingredient: Option<Vec<SubstanceIngredient>>,
}

/// SubstanceIngredient
///
/// Composition information about the substance: a component of the substance and
/// its optional relative amount (concentration).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Optional amount (concentration)
    pub quantity: Option<types::Ratio>,

    /// A component of the substance
    pub substance_codeable_concept: Option<types::CodeableConcept>,

    /// A component of the substance
    pub substance_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Substance;

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
