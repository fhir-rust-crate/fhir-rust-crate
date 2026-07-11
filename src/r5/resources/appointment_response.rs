//! AppointmentResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AppointmentResponse
//!
//! Version: 5.0.0
//!
//! AppointmentResponse Resource: A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A reply to an appointment request for a patient and/or practitioner(s),
/// such as a confirmation or rejection.
///
/// AppointmentResponse resources are used to communicate the acceptance or
/// declination of a proposed appointment by one or more participants. Each
/// response references the `Appointment` it relates to and reports the
/// participation status of a given actor. Responses may also propose a new
/// time or apply to all occurrences of a recurring appointment request.
///
/// This resource supports the administrative workflow of scheduling: when an
/// `Appointment` is created with participants whose required status is not
/// yet confirmed, each invited participant (or a system acting on their
/// behalf) returns an `AppointmentResponse` indicating whether they accept,
/// decline, or remain tentative about the proposed time. A response can also
/// act as a counter-proposal by suggesting an alternate `start`/`end` window
/// via `proposed_new_time`, and it can indicate whether the acceptance or
/// decline applies to every occurrence of a recurring appointment series.
///
/// Related resources / See also: the parent
/// [`Appointment`](crate::r5::types::Reference) referenced by `appointment`,
/// the responding [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// participant type, and the `actor` [`Reference`](crate::r5::types::Reference)
/// which typically points to a `Patient`, `Practitioner`, `RelatedPerson`,
/// `Device`, `HealthcareService`, or `Location`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::appointment_response::AppointmentResponse;
///
/// let value = AppointmentResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AppointmentResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentResponse {
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

    /// Reference to the parent `Appointment` this response relates to.
    pub appointment: types::Reference,

    /// Set when this response proposes a different time than the one requested.
    pub proposed_new_time: Option<types::Boolean>,

    /// Time from appointment, or requested new start time
    pub start: Option<types::Instant>,

    /// Time from appointment, or requested new end time
    pub end: Option<types::Instant>,

    /// Role of participant in the appointment
    pub participant_type: Option<Vec<types::CodeableConcept>>,

    /// The participant this response is on behalf of; typically a
    /// [`Patient`](crate::r5::resources::patient::Patient), Practitioner,
    /// RelatedPerson, Location, HealthcareService, or Device.
    pub actor: Option<types::Reference>,

    /// The participation status code: accepted | declined | tentative |
    /// needs-action | entered-in-error.
    pub participant_status: types::Code,

    /// Additional free-text comments from the participant about the appointment.
    pub comment: Option<types::Markdown>,

    /// Whether this response is for all occurrences in a recurring request.
    pub recurring: Option<types::Boolean>,

    /// Original date within a recurring request
    pub occurrence_date: Option<types::Date>,

    /// The recurrence ID of the specific recurring request
    pub recurrence_id: Option<types::PositiveInt>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AppointmentResponse;

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
