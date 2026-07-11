//! EncounterHistory
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EncounterHistory
//!
//! Version: 5.0.0
//!
//! EncounterHistory Resource: A record of significant events/milestones key data throughout the history of an Encounter
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of significant events/milestones and key data throughout the
/// history of an Encounter.
///
/// EncounterHistory captures a snapshot of an Encounter's status and related
/// values at a particular point in time, allowing systems to preserve the
/// evolution of an encounter without repeatedly updating and versioning the
/// full Encounter resource. Each instance references the associated Encounter
/// and records the status, class, timing, and location details as they applied
/// at that moment. In FHIR R5 this supports auditing and reconstructing the
/// progression of a patient's encounter over time.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter_history::EncounterHistory;
///
/// let value = EncounterHistory::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EncounterHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterHistory {
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

    /// The Encounter associated with this set of historic values
    pub encounter: Option<types::Reference>,

    /// Identifier(s) by which this encounter is known
    pub identifier: Option<Vec<types::Identifier>>,

    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    pub status: types::Code,

    /// Classification of patient encounter
    pub class: types::CodeableConcept,

    /// Specific type of encounter
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Specific type of service
    pub service_type: Option<Vec<types::CodeableReference>>,

    /// The patient or group related to this encounter
    pub subject: Option<types::Reference>,

    /// The current status of the subject in relation to the Encounter
    pub subject_status: Option<types::CodeableConcept>,

    /// The actual start and end time associated with this set of values associated with the encounter
    pub actual_period: Option<types::Period>,

    /// The planned start date/time (or admission date) of the encounter
    pub planned_start_date: Option<types::DateTime>,

    /// The planned end date/time (or discharge date) of the encounter
    pub planned_end_date: Option<types::DateTime>,

    /// Actual quantity of time the encounter lasted (less time absent)
    pub length: Option<types::Duration>,

    /// Location of the patient at this point in the encounter
    pub location: Option<Vec<EncounterHistoryLocation>>,
}

/// Location of the patient at this point in the encounter.
///
/// Records where the patient was located during this segment of the encounter,
/// including a reference to the Location resource and an optional description of
/// the physical type of that location (such as bed, room, ward, or virtual).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterHistoryLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Location the encounter takes place
    pub location: types::Reference,

    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EncounterHistory;

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
