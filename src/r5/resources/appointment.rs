//! Appointment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Appointment
//!
//! Version: 5.0.0
//!
//! Appointment Resource: A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one
/// or more Encounter(s).
///
/// In FHIR R5, an Appointment records the scheduling of a healthcare event and
/// the participants (patients, practitioners, related persons, locations, and
/// devices) that are expected to be involved. It captures the administrative
/// coordination of care rather than the clinical care itself: the proposed or
/// agreed date and time, the service category and service type to be performed,
/// the specialty required, the reason for the visit, and the acceptance status
/// of each participant. Its status moves through a defined lifecycle (proposed,
/// pending, booked, arrived, fulfilled, cancelled, noshow, entered-in-error,
/// checked-in, and waitlist) that drives scheduling workflows. An Appointment
/// may fill one or more Slots exposed by a Schedule, may request a specific
/// period when an exact time is not yet fixed, and may define a recurrence
/// template so that a repeating series of visits is generated from a single
/// booking. When a booked Appointment actually takes place it typically gives
/// rise to one or more Encounters that document the care delivered, and it can
/// also carry virtual service connection details for telehealth visits.
///
/// # Related resources
///
/// See also [`Patient`](crate::r5::resources::patient::Patient) and
/// [`Practitioner`](crate::r5::resources::practitioner::Practitioner) as typical
/// participants, [`Slot`](crate::r5::resources::slot::Slot) and
/// [`Schedule`](crate::r5::resources::schedule::Schedule) for the availability
/// this booking consumes, and
/// [`Encounter`](crate::r5::resources::encounter::Encounter) for the clinical
/// event that may result. Coded fields use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and links to other
/// resources use [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::appointment::Appointment;
///
/// let value = Appointment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Appointment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Appointment {
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

    /// Lifecycle state of the appointment that drives scheduling workflow: proposed | pending | booked | arrived | fulfilled | cancelled | noshow | entered-in-error | checked-in | waitlist
    pub status: types::Code,

    /// The coded reason for the appointment being cancelled
    pub cancellation_reason: Option<types::CodeableConcept>,

    /// Classification when becoming an encounter
    pub class: Option<Vec<types::CodeableConcept>>,

    /// A broad categorization of the service that is to be performed during this appointment
    pub service_category: Option<Vec<types::CodeableConcept>>,

    /// The specific service that is to be performed during this appointment
    pub service_type: Option<Vec<types::CodeableReference>>,

    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    pub specialty: Option<Vec<types::CodeableConcept>>,

    /// The style of appointment or patient that has been booked in the slot (not service type)
    pub appointment_type: Option<types::CodeableConcept>,

    /// Reason this appointment is scheduled
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Used to make informed decisions if needing to re-prioritize
    pub priority: Option<types::CodeableConcept>,

    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<types::String>,

    /// Appointment replaced by this Appointment
    pub replaces: Option<Vec<types::Reference>>,

    /// Connection details of a virtual service (e.g. conference call)
    pub virtual_service: Option<Vec<types::VirtualServiceDetail>>,

    /// Additional information to support the appointment
    pub supporting_information: Option<Vec<types::Reference>>,

    /// The previous appointment in a series
    pub previous_appointment: Option<types::Reference>,

    /// The originating appointment in a recurring set of appointments
    pub originating_appointment: Option<types::Reference>,

    /// Date and time when the appointment is scheduled to begin, once a fixed time has been set
    pub start: Option<types::Instant>,

    /// Date and time when the appointment is scheduled to conclude, once a fixed time has been set
    pub end: Option<types::Instant>,

    /// Can be less than start/end (e.g. estimate)
    pub minutes_duration: Option<types::PositiveInt>,

    /// Potential date/time interval(s) requested to allocate the appointment within
    pub requested_period: Option<Vec<types::Period>>,

    /// The Slot resource(s) that this appointment is filling, drawn from a Schedule's published availability
    pub slot: Option<Vec<types::Reference>>,

    /// The set of accounts that may be used for billing for this Appointment
    pub account: Option<Vec<types::Reference>>,

    /// The date that this appointment was initially created
    pub created: Option<types::DateTime>,

    /// When the appointment was cancelled
    pub cancellation_date: Option<types::DateTime>,

    /// Additional comments
    pub note: Option<Vec<types::Annotation>>,

    /// Detailed information and instructions for the patient
    pub patient_instruction: Option<Vec<types::CodeableReference>>,

    /// The request this appointment is allocated to assess
    pub based_on: Option<Vec<types::Reference>>,

    /// The patient or group of patients that this appointment primarily concerns
    pub subject: Option<types::Reference>,

    /// The actors expected to attend, each with a role, required flag, and acceptance status
    pub participant: Vec<AppointmentParticipant>,

    /// The sequence number in the recurrence
    pub recurrence_id: Option<types::PositiveInt>,

    /// Indicates that this appointment varies from a recurrence pattern
    pub occurrence_changed: Option<types::Boolean>,

    /// Details of the recurrence pattern/template used to generate occurrences
    pub recurrence_template: Option<Vec<AppointmentRecurrenceTemplate>>,
}

/// Participants involved in appointment.
///
/// Lists an actor (individual, device, location, or service) expected to take
/// part in the appointment, along with their role, participation period, whether
/// they are required, and their acceptance status.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Role of participant in the appointment
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Participation period of the actor
    pub period: Option<types::Period>,

    /// The individual, device, location, or service participating in the appointment
    pub actor: Option<types::Reference>,

    /// The participant is required to attend (optional when false)
    pub required: Option<types::Boolean>,

    /// accepted | declined | tentative | needs-action
    pub status: types::Code,
}

/// Details of the recurrence pattern/template used to generate occurrences.
///
/// Describes how a recurring set of appointments repeats: the timezone,
/// recurrence type, end conditions, and the weekly, monthly, or yearly template
/// that governs occurrence generation, plus any excluded dates or recurrence ids.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The timezone of the occurrences
    pub timezone: Option<types::CodeableConcept>,

    /// The frequency of the recurrence
    pub recurrence_type: types::CodeableConcept,

    /// The date when the recurrence should end
    pub last_occurrence_date: Option<types::Date>,

    /// The number of planned occurrences
    pub occurrence_count: Option<types::PositiveInt>,

    /// Specific dates for a recurring set of appointments (no template)
    pub occurrence_date: Option<Vec<types::Date>>,

    /// Information about weekly recurring appointments
    pub weekly_template: Option<AppointmentRecurrenceTemplateWeeklyTemplate>,

    /// Information about monthly recurring appointments
    pub monthly_template: Option<AppointmentRecurrenceTemplateMonthlyTemplate>,

    /// Information about yearly recurring appointments
    pub yearly_template: Option<AppointmentRecurrenceTemplateYearlyTemplate>,

    /// Any dates that should be excluded from the series
    pub excluding_date: Option<Vec<types::Date>>,

    /// Any recurrence IDs that should be excluded from the recurrence
    pub excluding_recurrence_id: Option<Vec<types::PositiveInt>>,
}

/// Information about weekly recurring appointments.
///
/// Specifies which days of the week the appointment recurs on and how many
/// weeks apart each recurrence is spaced.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateWeeklyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Recurs on Mondays
    pub monday: Option<types::Boolean>,

    /// Recurs on Tuesday
    pub tuesday: Option<types::Boolean>,

    /// Recurs on Wednesday
    pub wednesday: Option<types::Boolean>,

    /// Recurs on Thursday
    pub thursday: Option<types::Boolean>,

    /// Recurs on Friday
    pub friday: Option<types::Boolean>,

    /// Recurs on Saturday
    pub saturday: Option<types::Boolean>,

    /// Recurs on Sunday
    pub sunday: Option<types::Boolean>,

    /// Recurs every nth week
    pub week_interval: Option<types::PositiveInt>,
}

/// Information about monthly recurring appointments.
///
/// Specifies whether the appointment recurs on a particular day of the month or
/// on a particular weekday within a particular week of the month, and how many
/// months apart each recurrence is spaced.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateMonthlyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Recurs on a specific day of the month
    pub day_of_month: Option<types::PositiveInt>,

    /// Indicates which week of the month the appointment should occur
    pub nth_week_of_month: Option<types::Coding>,

    /// Indicates which day of the week the appointment should occur
    pub day_of_week: Option<types::Coding>,

    /// Recurs every nth month
    pub month_interval: types::PositiveInt,
}

/// Information about yearly recurring appointments.
///
/// Specifies how many years apart each recurrence of the appointment is spaced.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateYearlyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Recurs every nth year
    pub year_interval: types::PositiveInt,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Appointment;

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
