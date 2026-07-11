//! Timing
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Timing
//!
//! Version: 5.0.0
//!
//! Timing Type: Specifies an event that may occur multiple times.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Specifies an event that may occur multiple times.
///
/// Timing schedules are used to record when things are planned, expected or
/// requested to occur. The most common usage is in dosage instructions for
/// medications. They are also used when planning care of various kinds, and
/// may be used for reporting the schedule to which past regular activities
/// were carried out.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::timing::Timing;
///
/// let value = Timing::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Timing = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// When the event occurs
    pub event: Option<Vec<types::DateTime>>,

    /// When the event is to occur
    pub repeat: Option<TimingRepeat>,

    /// C | BID | TID | QID | AM | PM | QD | QOD | +
    pub code: Option<types::CodeableConcept>,
}

/// When the event is to occur.
///
/// A set of rules that describe when the event is scheduled, nested within a
/// [`Timing`] value.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::timing::TimingRepeat;
///
/// let value = TimingRepeat::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TimingRepeat = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TimingRepeat {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Length/Range of lengths, or (Start and/or end) limits
    pub bounds_duration: Option<types::Duration>,

    /// Length/Range of lengths, or (Start and/or end) limits
    pub bounds_range: Option<types::Range>,

    /// Length/Range of lengths, or (Start and/or end) limits
    pub bounds_period: Option<types::Period>,

    /// Number of times to repeat
    pub count: Option<types::PositiveInt>,

    /// Maximum number of times to repeat
    pub count_max: Option<types::PositiveInt>,

    /// How long when it happens
    pub duration: Option<types::Decimal>,

    /// How long when it happens (Max)
    pub duration_max: Option<types::Decimal>,

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    pub duration_unit: Option<types::Code>,

    /// Indicates the number of repetitions that should occur within a period
    pub frequency: Option<types::PositiveInt>,

    /// Event occurs up to frequencyMax times per period
    pub frequency_max: Option<types::PositiveInt>,

    /// The duration to which the frequency applies
    pub period: Option<types::Decimal>,

    /// Upper limit of period (3-4 hours)
    pub period_max: Option<types::Decimal>,

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    pub period_unit: Option<types::Code>,

    /// mon | tue | wed | thu | fri | sat | sun
    pub day_of_week: Option<Vec<types::Code>>,

    /// Time of day for action
    pub time_of_day: Option<Vec<types::Time>>,

    /// Code for time period of occurrence
    pub when: Option<Vec<types::Code>>,

    /// Minutes from event (before or after)
    pub offset: Option<types::UnsignedInt>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Timing;

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
