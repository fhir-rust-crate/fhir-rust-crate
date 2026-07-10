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
use fhir_derive::Validate;

/// A container for slots of time that may be available for booking appointments.
///
/// In FHIR R5, a Schedule provides a set of dates and times, associated with one
/// or more actors (such as a practitioner, location, device, or patient), during
/// which appointment Slots may be offered for booking. It describes the general
/// availability of the referenced actors and the period of time it covers, and
/// is typically referenced by the more granular Slot resource.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::schedule::Schedule;
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

    /// Whether this schedule is in active use
    pub active: Option<types::Boolean>,

    /// High-level category
    pub service_category: Option<Vec<types::CodeableConcept>>,

    /// Specific service
    pub service_type: Option<Vec<types::CodeableReference>>,

    /// Type of specialty needed
    pub specialty: Option<Vec<types::CodeableConcept>>,

    /// Human-readable label
    pub name: Option<types::String>,

    /// Resource(s) that availability information is being provided for
    pub actor: Vec<types::Reference>,

    /// Period of time covered by schedule
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
