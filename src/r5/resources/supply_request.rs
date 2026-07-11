//! SupplyRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SupplyRequest
//!
//! Version: 5.0.0
//!
//! SupplyRequest Resource: A record of a non-patient specific request for a medication, substance, device, certain types of biologically derived product, and nutrition product used in the healthcare setting.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a request for a medication, substance, device, or nutrition
/// product used in the healthcare setting.
///
/// The SupplyRequest resource captures a non-patient-specific request to
/// obtain supplies such as medications, substances, devices, biologically
/// derived products, and nutrition products. It records what item is being
/// requested, in what quantity, by whom, and for what reason, along with
/// delivery source and destination information. In FHIR R5 it supports supply
/// chain and inventory workflows that are distinct from patient-specific
/// medication or device requests.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::supply_request::SupplyRequest;
///
/// let value = SupplyRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SupplyRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplyRequest {
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

    /// Business Identifier for SupplyRequest
    pub identifier: Option<Vec<types::Identifier>>,

    /// draft | active | suspended +
    pub status: Option<types::Code>,

    /// What other request is fulfilled by this supply request
    pub based_on: Option<Vec<types::Reference>>,

    /// The kind of supply (central, non-stock, etc.)
    pub category: Option<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// The patient for who the supply request is for
    pub deliver_for: Option<types::Reference>,

    /// Medication, Substance, or Device requested to be supplied
    pub item: types::CodeableReference,

    /// The requested amount of the item indicated
    pub quantity: types::Quantity,

    /// Ordered item details
    pub parameter: Option<Vec<SupplyRequestParameter>>,

    /// When the request should be fulfilled (dateTime)
    pub occurrence_date_time: Option<types::DateTime>,

    /// When the request should be fulfilled (Period)
    pub occurrence_period: Option<types::Period>,

    /// When the request should be fulfilled (Timing)
    pub occurrence_timing: Option<types::Timing>,

    /// When the request was made
    pub authored_on: Option<types::DateTime>,

    /// Individual making the request
    pub requester: Option<types::Reference>,

    /// Who is intended to fulfill the request
    pub supplier: Option<Vec<types::Reference>>,

    /// The reason why the supply item was requested
    pub reason: Option<Vec<types::CodeableReference>>,

    /// The origin of the supply
    pub deliver_from: Option<types::Reference>,

    /// The destination of the supply
    pub deliver_to: Option<types::Reference>,
}

/// Ordered item details.
///
/// The SupplyRequestParameter backbone element specifies additional
/// characteristics of the requested item, such as a coded detail and its
/// associated value expressed as a codeable concept, quantity, range, or
/// boolean.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplyRequestParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Item detail
    pub code: Option<types::CodeableConcept>,

    /// Value of detail (CodeableConcept)
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value of detail (Quantity)
    pub value_quantity: Option<types::Quantity>,

    /// Value of detail (Range)
    pub value_range: Option<types::Range>,

    /// Value of detail (boolean)
    pub value_boolean: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SupplyRequest;

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
