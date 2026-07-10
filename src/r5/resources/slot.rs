//! Slot
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Slot
//!
//! Version: 5.0.0
//!
//! Slot Resource: A slot of time on a schedule that may be available for booking appointments.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Slot represents a single interval of time on a Schedule that may be available
/// for booking appointments. Each slot references the Schedule it belongs to,
/// carries a start and end instant, and has a status such as `busy`, `free`, or
/// `busy-unavailable`. Slots can further describe the categories, service types,
/// specialties, and appointment types that may be booked, and may be flagged as
/// overbooked. They are typically generated from a Schedule's planning horizon
/// and consumed by scheduling systems to determine availability.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::slot::Slot;
///
/// let value = Slot::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Slot = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
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

    /// External Ids for this item
    pub identifier: Option<Vec<types::Identifier>>,

    /// A broad categorization of the service that is to be performed during this appointment
    pub service_category: Option<Vec<types::CodeableConcept>>,

    /// The type of appointments that can be booked into this slot (ideally this would be an identifiable service - which is at a location, rather than the location itself). If provided then this overrides the value provided on the Schedule resource
    pub service_type: Option<Vec<types::CodeableReference>>,

    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    pub specialty: Option<Vec<types::CodeableConcept>>,

    /// The style of appointment or patient that may be booked in the slot (not service type)
    pub appointment_type: Option<Vec<types::CodeableConcept>>,

    /// The schedule resource that this slot defines an interval of status information
    pub schedule: types::Reference,

    /// busy | free | busy-unavailable | busy-tentative | entered-in-error
    pub status: types::Code,

    /// Date/Time that the slot is to begin
    pub start: types::Instant,

    /// Date/Time that the slot is to conclude
    pub end: types::Instant,

    /// This slot has already been overbooked, appointments are unlikely to be accepted for this time
    pub overbooked: Option<types::Boolean>,

    /// Comments on the slot to describe any extended information. Such as custom constraints on the slot
    pub comment: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Slot;

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
