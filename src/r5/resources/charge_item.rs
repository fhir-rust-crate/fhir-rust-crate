//! ChargeItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
//!
//! Version: 5.0.0
//!
//! ChargeItem Resource: The provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The resource ChargeItem describes the provision of healthcare provider
/// products for a certain patient, therefore referring not only to the product,
/// but containing in addition details of the provision, like date, time,
/// amounts and participating organizations and persons. The main usage of the
/// ChargeItem is to enable the billing process and internal cost allocation.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::charge_item::ChargeItem;
///
/// let value = ChargeItem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ChargeItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItem {
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

    /// Business Identifier for item
    pub identifier: Option<Vec<types::Identifier>>,

    /// Defining information about the code of this charge item
    pub definition_uri: Option<Vec<types::Uri>>,

    /// Resource defining the code of this ChargeItem
    pub definition_canonical: Option<Vec<types::Canonical>>,

    /// planned | billable | not-billable | aborted | billed | entered-in-error | unknown
    pub status: types::Code,

    /// Part of referenced ChargeItem
    pub part_of: Option<Vec<types::Reference>>,

    /// A code that identifies the charge, like a billing code
    pub code: types::CodeableConcept,

    /// Individual service was done for/to
    pub subject: types::Reference,

    /// Encounter associated with this ChargeItem
    pub encounter: Option<types::Reference>,

    /// When the charged service was applied
    pub occurrence_date_time: Option<types::DateTime>,

    /// When the charged service was applied
    pub occurrence_period: Option<types::Period>,

    /// When the charged service was applied
    pub occurrence_timing: Option<types::Timing>,

    /// Who performed charged service
    pub performer: Option<Vec<ChargeItemPerformer>>,

    /// Organization providing the charged service
    pub performing_organization: Option<types::Reference>,

    /// Organization requesting the charged service
    pub requesting_organization: Option<types::Reference>,

    /// Organization that has ownership of the (potential, future) revenue
    pub cost_center: Option<types::Reference>,

    /// Quantity of which the charge item has been serviced
    pub quantity: Option<types::Quantity>,

    /// Anatomical location, if relevant
    pub bodysite: Option<Vec<types::CodeableConcept>>,

    /// Unit price overriding the associated rules
    pub unit_price_component: Option<types::MonetaryComponent>,

    /// Total price overriding the associated rules
    pub total_price_component: Option<types::MonetaryComponent>,

    /// Reason for overriding the list price/factor
    pub override_reason: Option<types::CodeableConcept>,

    /// Individual who was entering
    pub enterer: Option<types::Reference>,

    /// Date the charge item was entered
    pub entered_date: Option<types::DateTime>,

    /// Why was the charged  service rendered?
    pub reason: Option<Vec<types::CodeableConcept>>,

    /// Which rendered service is being charged?
    pub service: Option<Vec<types::CodeableReference>>,

    /// Product charged
    pub product: Option<Vec<types::CodeableReference>>,

    /// Account to place this charge
    pub account: Option<Vec<types::Reference>>,

    /// Comments made about the ChargeItem
    pub note: Option<Vec<types::Annotation>>,

    /// Further information supporting this charge
    pub supporting_information: Option<Vec<types::Reference>>,
}

/// Who performed charged service.
///
/// Indicates who or what performed or participated in the charged service.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What type of performance was done
    pub function: Option<types::CodeableConcept>,

    /// Individual who was performing
    pub actor: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ChargeItem;

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
