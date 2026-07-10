//! AdverseEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
//!
//! Version: 5.0.0
//!
//! AdverseEvent Resource: An event that may be related to unintended effects on a patient or research participant.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// An event (i.e. any change to current patient status) that may be related to
/// unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects, and is used in both
/// clinical care and research safety domains.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEvent;
///
/// let value = AdverseEvent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AdverseEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEvent {
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

    /// Business identifier for the event
    pub identifier: Option<Vec<types::Identifier>>,

    /// in-progress | completed | entered-in-error | unknown
    pub status: types::Code,

    /// actual | potential
    pub actuality: types::Code,

    /// wrong-patient | procedure-mishap | medication-mishap | device |
    /// unsafe-physical-environment | hospital-aquired-infection | wrong-body-site
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Event or incident that occurred or was averted
    pub code: Option<types::CodeableConcept>,

    /// Subject impacted by event
    pub subject: types::Reference,

    /// The Encounter associated with the start of the AdverseEvent
    pub encounter: Option<types::Reference>,

    /// When the event occurred
    pub occurrence_date_time: Option<types::DateTime>,

    /// When the event occurred
    pub occurrence_period: Option<types::Period>,

    /// When the event occurred
    pub occurrence_timing: Option<types::Timing>,

    /// When the event was detected
    pub detected: Option<types::DateTime>,

    /// When the event was recorded
    pub recorded_date: Option<types::DateTime>,

    /// Effect on the subject due to this event
    pub resulting_effect: Option<Vec<types::Reference>>,

    /// Location where adverse event occurred
    pub location: Option<types::Reference>,

    /// Seriousness or gravity of the event
    pub seriousness: Option<types::CodeableConcept>,

    /// Type of outcome from the adverse event
    pub outcome: Option<Vec<types::CodeableConcept>>,

    /// Who recorded the adverse event
    pub recorder: Option<types::Reference>,

    /// Who was involved in the adverse event or the potential adverse event
    /// and what they did
    pub participant: Option<Vec<AdverseEventParticipant>>,

    /// Research study that the subject is enrolled in
    pub study: Option<Vec<types::Reference>>,

    /// Considered likely or probable or anticipated in the research study
    pub expected_in_research_study: Option<types::Boolean>,

    /// The suspected agent causing the adverse event
    pub suspect_entity: Option<Vec<AdverseEventSuspectEntity>>,

    /// Contributing factors suspected to have increased the probability or
    /// severity of the adverse event
    pub contributing_factor: Option<Vec<AdverseEventContributingFactor>>,

    /// Preventive actions that contributed to avoiding the adverse event
    pub preventive_action: Option<Vec<AdverseEventPreventiveAction>>,

    /// Ameliorating actions taken after the adverse event occured in order to
    /// reduce the extent of harm
    pub mitigating_action: Option<Vec<AdverseEventMitigatingAction>>,

    /// Supporting information relevant to the event
    pub supporting_info: Option<Vec<AdverseEventSupportingInfo>>,

    /// Comment on adverse event
    pub note: Option<Vec<types::Annotation>>,
}

/// Who was involved in the adverse event or the potential adverse event and
/// what they did.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who was involved in the adverse event or the potential adverse event
    pub actor: types::Reference,
}

/// The suspected agent causing the adverse event.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Refers to the specific entity that caused the adverse event
    pub instance_codeable_concept: Option<types::CodeableConcept>,

    /// Refers to the specific entity that caused the adverse event
    pub instance_reference: Option<types::Reference>,

    /// Information on the possible cause of the event
    pub causality: Option<AdverseEventSuspectEntityCausality>,
}

/// Information on the possible cause of the event.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntityCausality {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Method of evaluating the relatedness of the suspected entity to the event
    pub assessment_method: Option<types::CodeableConcept>,

    /// Result of the assessment regarding the relatedness of the suspected
    /// entity to the event
    pub entity_relatedness: Option<types::CodeableConcept>,

    /// Author of the information on the possible cause of the event
    pub author: Option<types::Reference>,
}

/// Contributing factors suspected to have increased the probability or severity
/// of the adverse event.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventContributingFactor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Item suspected to have increased the probability or severity of the
    /// adverse event
    pub item_reference: Option<types::Reference>,

    /// Item suspected to have increased the probability or severity of the
    /// adverse event
    pub item_codeable_concept: Option<types::CodeableConcept>,
}

/// Preventive actions that contributed to avoiding the adverse event.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventPreventiveAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Action that contributed to avoiding the adverse event
    pub item_reference: Option<types::Reference>,

    /// Action that contributed to avoiding the adverse event
    pub item_codeable_concept: Option<types::CodeableConcept>,
}

/// Ameliorating actions taken after the adverse event occured in order to
/// reduce the extent of harm.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventMitigatingAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Ameliorating action taken after the adverse event occured in order to
    /// reduce the extent of harm
    pub item_reference: Option<types::Reference>,

    /// Ameliorating action taken after the adverse event occured in order to
    /// reduce the extent of harm
    pub item_codeable_concept: Option<types::CodeableConcept>,
}

/// Supporting information relevant to the event.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Subject medical history or document relevant to this adverse event
    pub item_reference: Option<types::Reference>,

    /// Subject medical history or document relevant to this adverse event
    pub item_codeable_concept: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AdverseEvent;

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
