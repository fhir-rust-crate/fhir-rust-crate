//! Encounter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Encounter
//!
//! Version: 5.0.0
//!
//! Encounter Resource: An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// An interaction between healthcare provider(s), and/or patient(s) for the
/// purpose of providing healthcare service(s) or assessing the health status of
/// patient(s).
///
/// In FHIR R5 the Encounter resource records the context of a healthcare
/// interaction such as an inpatient admission, an outpatient visit, or a virtual
/// consultation. It ties together the subject, participants, locations, reasons,
/// diagnoses, and administrative details (admission and discharge) of the
/// interaction.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter::Encounter;
///
/// let value = Encounter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Encounter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
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

    /// Identifier(s) by which this encounter is known
    pub identifier: Option<Vec<types::Identifier>>,

    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    pub status: types::Code,

    /// Classification of patient encounter context - e.g. Inpatient, outpatient
    pub class: Option<Vec<types::CodeableConcept>>,

    /// Indicates the urgency of the encounter
    pub priority: Option<types::CodeableConcept>,

    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, ...)
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Specific type of service
    pub service_type: Option<Vec<types::CodeableReference>>,

    /// The patient or group related to this encounter
    pub subject: Option<types::Reference>,

    /// The current status of the subject in relation to the Encounter
    pub subject_status: Option<types::CodeableConcept>,

    /// Episode(s) of care that this encounter should be recorded against
    pub episode_of_care: Option<Vec<types::Reference>>,

    /// The request that initiated this encounter
    pub based_on: Option<Vec<types::Reference>>,

    /// The group(s) that are allocated to participate in this encounter
    pub care_team: Option<Vec<types::Reference>>,

    /// Another Encounter this encounter is part of
    pub part_of: Option<types::Reference>,

    /// The organization (facility) responsible for this encounter
    pub service_provider: Option<types::Reference>,

    /// List of participants involved in the encounter
    pub participant: Option<Vec<EncounterParticipant>>,

    /// The appointment that scheduled this encounter
    pub appointment: Option<Vec<types::Reference>>,

    /// Connection details of a virtual service (e.g. conference call)
    pub virtual_service: Option<Vec<types::VirtualServiceDetail>>,

    /// The actual start and end time of the encounter
    pub actual_period: Option<types::Period>,

    /// The planned start date/time (or admission date) of the encounter
    pub planned_start_date: Option<types::DateTime>,

    /// The planned end date/time (or discharge date) of the encounter
    pub planned_end_date: Option<types::DateTime>,

    /// Actual quantity of time the encounter lasted (less time absent)
    pub length: Option<types::Duration>,

    /// The list of medical reasons that are expected to be addressed during the episode of care
    pub reason: Option<Vec<EncounterReason>>,

    /// The list of diagnosis relevant to this encounter
    pub diagnosis: Option<Vec<EncounterDiagnosis>>,

    /// The set of accounts that may be used for billing for this Encounter
    pub account: Option<Vec<types::Reference>>,

    /// Diet preferences reported by the patient
    pub diet_preference: Option<Vec<types::CodeableConcept>>,

    /// Wheelchair, translator, stretcher, etc
    pub special_arrangement: Option<Vec<types::CodeableConcept>>,

    /// Special courtesies (VIP, board member)
    pub special_courtesy: Option<Vec<types::CodeableConcept>>,

    /// Details about the admission to a healthcare service
    pub admission: Option<EncounterAdmission>,

    /// List of locations where the patient has been
    pub location: Option<Vec<EncounterLocation>>,
}

/// List of participants involved in the encounter.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Role of participant in encounter
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Period of time during the encounter that the participant participated
    pub period: Option<types::Period>,

    /// The individual, device, or service participating in the encounter
    pub actor: Option<types::Reference>,
}

/// The list of medical reasons that are expected to be addressed during the
/// episode of care.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterReason {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What the reason value should be used for/as
    pub r#use: Option<Vec<types::CodeableConcept>>,

    /// Reason the encounter takes place (core or reference)
    pub value: Option<Vec<types::CodeableReference>>,
}

/// The list of diagnosis relevant to this encounter.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The diagnosis relevant to the encounter
    pub condition: Option<Vec<types::CodeableReference>>,

    /// Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)
    pub r#use: Option<Vec<types::CodeableConcept>>,
}

/// Details about the admission to a healthcare service.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterAdmission {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pre-admission identifier
    pub pre_admission_identifier: Option<types::Identifier>,

    /// The location/organization from which the patient came before admission
    pub origin: Option<types::Reference>,

    /// From where patient was admitted (physician referral, transfer)
    pub admit_source: Option<types::CodeableConcept>,

    /// Indicates that the patient is being re-admitted
    pub re_admission: Option<types::CodeableConcept>,

    /// Location/organization to which the patient is discharged
    pub destination: Option<types::Reference>,

    /// Category or kind of location after discharge
    pub discharge_disposition: Option<types::CodeableConcept>,
}

/// List of locations where the patient has been.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Location the encounter takes place
    pub location: types::Reference,

    /// planned | active | reserved | completed
    pub status: Option<types::Code>,

    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<types::CodeableConcept>,

    /// Time period during which the patient was present at the location
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Encounter;

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
