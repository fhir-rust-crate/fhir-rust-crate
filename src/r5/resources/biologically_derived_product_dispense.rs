//! BiologicallyDerivedProductDispense
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProductDispense
//!
//! Version: 5.0.0
//!
//! BiologicallyDerivedProductDispense Resource: A record of dispensation of a biologically derived product.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A record of dispensation of a biologically derived product.
///
/// This resource represents the dispensing of a biologically derived product,
/// such as blood, cells, or tissue, from a supplier to an intended recipient.
/// It captures the product being dispensed, the patient, the performers, the
/// location, quantities, and timing associated with the dispense event. It is
/// used within FHIR R5 workflows that track the movement and delivery of such
/// products for clinical use.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::biologically_derived_product_dispense::BiologicallyDerivedProductDispense;
///
/// let value = BiologicallyDerivedProductDispense::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: BiologicallyDerivedProductDispense = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductDispense {
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

    /// Business identifier for this dispense
    pub identifier: Option<Vec<types::Identifier>>,

    /// The order or request that this dispense is fulfilling
    pub based_on: Option<Vec<types::Reference>>,

    /// Short description
    pub part_of: Option<Vec<types::Reference>>,

    /// preparation | in-progress | allocated | issued | unfulfilled | returned | entered-in-error | unknown
    pub status: types::Code,

    /// Relationship between the donor and intended recipient
    pub origin_relationship_type: Option<types::CodeableConcept>,

    /// The BiologicallyDerivedProduct that is dispensed
    pub product: types::Reference,

    /// The intended recipient of the dispensed product
    pub patient: types::Reference,

    /// Indicates the type of matching associated with the dispense
    pub match_status: Option<types::CodeableConcept>,

    /// Indicates who or what performed an action
    pub performer: Option<Vec<BiologicallyDerivedProductDispensePerformer>>,

    /// Where the dispense occurred
    pub location: Option<types::Reference>,

    /// Amount dispensed
    pub quantity: Option<types::Quantity>,

    /// When product was selected/matched
    pub prepared_date: Option<types::DateTime>,

    /// When the product was dispatched
    pub when_handed_over: Option<types::DateTime>,

    /// Where the product was dispatched to
    pub destination: Option<types::Reference>,

    /// Additional notes
    pub note: Option<Vec<types::Annotation>>,

    /// Specific instructions for use
    pub usage_instruction: Option<types::String>,
}

/// Indicates who or what performed an action.
///
/// Backbone element for `BiologicallyDerivedProductDispense.performer`,
/// describing an actor involved in the dispense and their function.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifies the function of the performer during the dispense
    pub function: Option<types::CodeableConcept>,

    /// Who performed the action
    pub actor: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = BiologicallyDerivedProductDispense;

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
