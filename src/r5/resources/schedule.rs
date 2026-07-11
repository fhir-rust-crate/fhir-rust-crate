//! Schedule
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Schedule
//!
//! Version: 5.0.0
//!
//! Schedule Resource: A container for slots of time that may be available for booking appointments.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A container for slots of time that may be available for booking appointments.
///
/// In FHIR R5, a Schedule is an administrative resource that describes the
/// general availability of one or more actors, such as a practitioner, a
/// location, a piece of equipment, or a patient, over a defined period of time.
/// It groups together the bookable time belonging to those actors and, together
/// with the planning horizon, bounds the window during which appointments may be
/// offered. A Schedule does not itself hold individual bookable intervals;
/// instead it acts as the organizing container that the finer-grained
/// [`Slot`](crate::r5::resources::slot::Slot) resources reference, and each Slot
/// then carries the concrete free or busy status for a specific interval.
/// Scheduling systems typically use the service category, service type, and
/// specialty to help clients find the right Schedule when searching for
/// availability, before an [`Appointment`](crate::r5::resources::appointment::Appointment)
/// is created against one of its slots.
///
/// # Related resources
///
/// See also [`Slot`](crate::r5::resources::slot::Slot),
/// [`Appointment`](crate::r5::resources::appointment::Appointment),
/// [`Practitioner`](crate::r5::resources::practitioner::Practitioner),
/// [`Location`](crate::r5::resources::location::Location), and
/// [`Patient`](crate::r5::resources::patient::Patient), any of which may appear
/// as an actor for a Schedule.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::schedule::Schedule;
///
/// let value = Schedule::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Schedule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
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

    /// Whether this schedule is in active use; inactive schedules should not be offered for booking
    pub active: Option<types::Boolean>,

    /// High-level category of the service or resource this schedule provides, such as general practice or dental
    pub service_category: Option<Vec<types::CodeableConcept>>,

    /// Specific type of service that may be booked against this schedule
    pub service_type: Option<Vec<types::CodeableReference>>,

    /// Type of specialty needed
    pub specialty: Option<Vec<types::CodeableConcept>>,

    /// Human-readable label
    pub name: Option<types::String>,

    /// Resource(s), such as a practitioner, location, device, or patient, whose availability this schedule describes
    pub actor: Vec<types::Reference>,

    /// Period of time covered by the schedule, bounding when slots and appointments may be offered
    pub planning_horizon: Option<types::Period>,

    /// Comments on availability
    pub comment: Option<types::Markdown>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Schedule;

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
