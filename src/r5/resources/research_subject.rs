//! ResearchSubject
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
//!
//! Version: 5.0.0
//!
//! ResearchSubject Resource: A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A ResearchSubject is a participant or object which is the recipient of
/// investigative activities in a research study.
///
/// A ResearchSubject links a subject (such as a patient, group, specimen, or
/// device) to a specific ResearchStudy and records the details of that subject's
/// participation. It tracks the subject's status over time through a series of
/// progress states and milestones, the period of participation, and the
/// comparison group (arm) the subject was assigned to and actually followed. In
/// FHIR R5 it is commonly used to manage enrollment and consent within clinical
/// trials and other research activities.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::research_subject::ResearchSubject;
///
/// let value = ResearchSubject::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ResearchSubject = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubject {
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

    /// Business Identifier for research subject in a study
    pub identifier: Option<Vec<types::Identifier>>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// Subject status
    pub progress: Option<Vec<ResearchSubjectProgress>>,

    /// Start and end of participation
    pub period: Option<types::Period>,

    /// Study subject is part of
    pub study: types::Reference,

    /// Who or what is part of study
    pub subject: types::Reference,

    /// What path should be followed
    pub assigned_comparison_group: Option<types::Id>,

    /// What path was followed
    pub actual_comparison_group: Option<types::Id>,

    /// Agreement to participate in study
    pub consent: Option<Vec<types::Reference>>,
}

/// Subject status.
///
/// Records the current and historical states and milestones a research subject
/// has passed through during participation in a study, along with the reason and
/// dates for each state change.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubjectProgress {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// state | milestone
    pub r#type: Option<types::CodeableConcept>,

    /// candidate | eligible | follow-up | ineligible | not-registered |
    /// off-study | on-study | on-study-intervention | on-study-observation |
    /// pending-on-study | potential-candidate | screening | withdrawn
    pub subject_state: Option<types::CodeableConcept>,

    /// SignedUp | Screened | Randomized
    pub milestone: Option<types::CodeableConcept>,

    /// State change reason
    pub reason: Option<types::CodeableConcept>,

    /// State change date
    pub start_date: Option<types::DateTime>,

    /// State change date
    pub end_date: Option<types::DateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ResearchSubject;

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
