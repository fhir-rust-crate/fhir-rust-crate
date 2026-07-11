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
use fhir_derive_macros::Validate;

/// A record of dispensation of a biologically derived product.
///
/// This resource represents the dispensing of a biologically derived product,
/// such as blood, cells, or tissue, from a supplier to an intended recipient.
/// It captures the product being dispensed, the patient, the performers, the
/// location, quantities, and timing associated with the dispense event. It is
/// used within FHIR R5 workflows that track the movement and delivery of such
/// products for clinical use.
///
/// Dispensation is a discrete administrative and clinical event that occurs
/// after a `BiologicallyDerivedProduct` has been requested, matched, and
/// prepared for a specific recipient. The resource records who requested the
/// dispense (`based_on`), what was dispensed (`product`), for whom
/// (`patient`), who carried out the dispense (`performer`), where it took
/// place (`location`), and the key timestamps for preparation and hand-over.
/// It also tracks lifecycle status (for example `preparation`,
/// `in-progress`, or `issued`) so that downstream systems, such as blood
/// bank or transfusion services, can monitor the product from allocation
/// through delivery to the point of use.
///
/// # Related resources
///
/// This resource commonly references a `BiologicallyDerivedProduct` (the
/// item dispensed), a [`Patient`](crate::r5::resources::patient::Patient) as
/// the intended recipient, and uses
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values to describe
/// donor/recipient relationships and match status.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::biologically_derived_product_dispense::BiologicallyDerivedProductDispense;
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

    /// Business identifier for this dispense
    pub identifier: Option<Vec<types::Identifier>>,

    /// The order or request that this dispense is fulfilling
    pub based_on: Option<Vec<types::Reference>>,

    /// Short description
    pub part_of: Option<Vec<types::Reference>>,

    /// Status of the dispense: preparation | in-progress | allocated | issued | unfulfilled | returned | entered-in-error | unknown
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Relationship between the donor and intended recipient
    pub origin_relationship_type: Option<types::CodeableConcept>,

    /// Reference to the specific BiologicallyDerivedProduct being dispensed
    pub product: types::Reference,

    /// Reference to the patient who is the intended recipient of the product
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
    /// Primitive extension sibling for [`prepared_date`](Self::prepared_date) (FHIR `_preparedDate`).
    #[serde(rename = "_preparedDate")]
    pub prepared_date_ext: Option<types::Element>,

    /// When the product was dispatched
    pub when_handed_over: Option<types::DateTime>,
    /// Primitive extension sibling for [`when_handed_over`](Self::when_handed_over) (FHIR `_whenHandedOver`).
    #[serde(rename = "_whenHandedOver")]
    pub when_handed_over_ext: Option<types::Element>,

    /// Where the product was dispatched to
    pub destination: Option<types::Reference>,

    /// Additional notes
    pub note: Option<Vec<types::Annotation>>,

    /// Specific instructions for use
    pub usage_instruction: Option<types::String>,
    /// Primitive extension sibling for [`usage_instruction`](Self::usage_instruction) (FHIR `_usageInstruction`).
    #[serde(rename = "_usageInstruction")]
    pub usage_instruction_ext: Option<types::Element>,
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
