//! EpisodeOfCare
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
//!
//! Version: 5.0.0
//!
//! EpisodeOfCare Resource: An association between a patient and an organization / healthcare provider(s) during which time encounters may occur.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// An association between a patient and an organization / healthcare
/// provider(s) during which time encounters may occur.
///
/// EpisodeOfCare represents a period during which a managing organization
/// assumes a level of responsibility for a patient's care, potentially
/// spanning many encounters. It groups together the type of care, the medical
/// reasons and diagnoses being addressed, the care manager and care team, and
/// the accounts used for billing. In FHIR R5 it is commonly used to model
/// longitudinal programs such as disease management, specialist referrals, or
/// ongoing treatment relationships.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::episode_of_care::EpisodeOfCare;
///
/// let value = EpisodeOfCare::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EpisodeOfCare = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCare {
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

    /// Business Identifier(s) relevant for this EpisodeOfCare
    pub identifier: Option<Vec<types::Identifier>>,

    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: types::Code,

    /// Past list of status codes (the current status may be included to cover the start date of the status)
    pub status_history: Option<Vec<EpisodeOfCareStatusHistory>>,

    /// Type/class  - e.g. specialist referral, disease management
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// The list of medical reasons that are expected to be addressed during the episode of care
    pub reason: Option<Vec<EpisodeOfCareReason>>,

    /// The list of medical conditions that were addressed during the episode of care
    pub diagnosis: Option<Vec<EpisodeOfCareDiagnosis>>,

    /// The patient who is the focus of this episode of care
    pub patient: types::Reference,

    /// Organization that assumes responsibility for care coordination
    pub managing_organization: Option<types::Reference>,

    /// Interval during responsibility is assumed
    pub period: Option<types::Period>,

    /// Originating Referral Request(s)
    pub referral_request: Option<Vec<types::Reference>>,

    /// Care manager/care coordinator for the patient
    pub care_manager: Option<types::Reference>,

    /// Other practitioners facilitating this episode of care
    pub care_team: Option<Vec<types::Reference>>,

    /// The set of accounts that may be used for billing for this EpisodeOfCare
    pub account: Option<Vec<types::Reference>>,
}

/// Past list of status codes (the current status may be included to cover the
/// start date of the status).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareStatusHistory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: types::Code,

    /// Duration the EpisodeOfCare was in the specified status
    pub period: types::Period,
}

/// The list of medical reasons that are expected to be addressed during the
/// episode of care.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareReason {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What the reason value should be used for/as
    pub r#use: Option<types::CodeableConcept>,

    /// Medical reason to be addressed
    pub value: Option<Vec<types::CodeableReference>>,
}

/// The list of medical conditions that were addressed during the episode of
/// care.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The medical condition that was addressed during the episode of care
    pub condition: Option<Vec<types::CodeableReference>>,

    /// Role that this diagnosis has within the episode of care (e.g. admission, billing, discharge …)
    pub r#use: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EpisodeOfCare;

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
