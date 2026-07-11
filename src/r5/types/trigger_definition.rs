//! TriggerDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
//!
//! Version: 5.0.0
//!
//! TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// TriggerDefinition describes a triggering event that can initiate an action,
/// such as executing a knowledge artifact or evaluating a rule. Triggering
/// events can be named events, data events, or periodic events, as determined
/// by the `type` element. It is used throughout FHIR R5 in resources such as
/// PlanDefinition and EventDefinition to specify when and how an activity
/// should be triggered.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::trigger_definition::TriggerDefinition;
///
/// let value = TriggerDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TriggerDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TriggerDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended
    pub r#type: types::Code,

    /// Name or URI that identifies the event
    pub name: Option<types::String>,

    /// Coded definition of the event
    pub code: Option<types::CodeableConcept>,

    /// Reference to a SubscriptionTopic resource that defines this event
    pub subscription_topic: Option<types::Canonical>,

    /// Timing of the event (Timing variant)
    pub timing_timing: Option<types::Timing>,

    /// Timing of the event (Reference variant)
    pub timing_reference: Option<types::Reference>,

    /// Timing of the event (date variant)
    pub timing_date: Option<types::Date>,

    /// Timing of the event (dateTime variant)
    pub timing_date_time: Option<types::DateTime>,

    /// Triggering data of the event (multiple = 'and')
    pub data: Option<Vec<types::DataRequirement>>,

    /// Whether the event triggers (boolean expression)
    pub condition: Option<types::Expression>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TriggerDefinition;

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
