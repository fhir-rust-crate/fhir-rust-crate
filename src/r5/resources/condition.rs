//! Condition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Condition
//!
//! Version: 5.0.0
//!
//! Condition Resource: A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A clinical condition, problem, diagnosis, or other event, situation, issue,
/// or clinical concept that has risen to a level of concern.
///
/// The Condition resource is used to record detailed information about a
/// condition, problem, diagnosis, or other health matter that is of concern.
/// It can be used to record conditions that a patient has, is at risk of, or is
/// being managed for, and is central to problem lists, encounter diagnoses, and
/// clinical assessments in FHIR R5.
///
/// Typical uses include capturing a patient's active and historical problem
/// list, documenting a diagnosis reached during an encounter, and tracking the
/// clinical course of a condition over time through fields such as
/// `clinical_status`, `verification_status`, `onset`, `abatement`, and `stage`.
/// A `Condition` always refers back to the `subject` (usually a patient) for
/// whom the condition is asserted, and it may be linked to the `encounter`
/// during which it was identified or recorded.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the individual who is typically the subject of a condition.
/// - `Encounter` — the clinical encounter during which the condition was noted or assessed.
/// - `Observation` — clinical findings that may support or evidence a condition.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used throughout this resource for coded status, category, severity, and body site values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::condition::Condition;
///
/// let value = Condition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Condition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// External Ids for this condition
    pub identifier: Option<Vec<types::Identifier>>,

    /// The clinical status of the condition, e.g. active | recurrence | relapse | inactive | remission | resolved | unknown.
    pub clinical_status: types::CodeableConcept,

    /// The degree of confidence in the assertion, e.g. unconfirmed | provisional | differential | confirmed | refuted | entered-in-error.
    pub verification_status: Option<types::CodeableConcept>,

    /// A categorization of the condition, e.g. problem-list-item | encounter-diagnosis.
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Subjective severity of condition
    pub severity: Option<types::CodeableConcept>,

    /// Identification of the condition, problem, or diagnosis, typically drawn from a terminology such as ICD-10 or SNOMED CT.
    pub code: Option<types::CodeableConcept>,

    /// Anatomical location, if relevant
    pub body_site: Option<Vec<types::CodeableConcept>>,

    /// The patient or group who has the condition; a reference to the relevant [`Patient`](crate::r5::resources::patient::Patient) (or group).
    pub subject: types::Reference,

    /// The encounter during which this condition was created or asserted
    pub encounter: Option<types::Reference>,

    /// Estimated or actual date, date-time, or age
    pub onset_date_time: Option<types::DateTime>,

    /// Estimated or actual date, date-time, or age
    pub onset_age: Option<types::Age>,

    /// Estimated or actual date, date-time, or age
    pub onset_period: Option<types::Period>,

    /// Estimated or actual date, date-time, or age
    pub onset_range: Option<types::Range>,

    /// Estimated or actual date, date-time, or age
    pub onset_string: Option<types::String>,

    /// When in resolution/remission
    pub abatement_date_time: Option<types::DateTime>,

    /// When in resolution/remission
    pub abatement_age: Option<types::Age>,

    /// When in resolution/remission
    pub abatement_period: Option<types::Period>,

    /// When in resolution/remission
    pub abatement_range: Option<types::Range>,

    /// When in resolution/remission
    pub abatement_string: Option<types::String>,

    /// Date condition was first recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`).
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Who or what participated in the activities related to the condition and how they were involved
    pub participant: Option<Vec<ConditionParticipant>>,

    /// Stage/grade, usually assessed formally
    pub stage: Option<Vec<ConditionStage>>,

    /// Supporting evidence for the verification status
    pub evidence: Option<Vec<types::CodeableReference>>,

    /// Additional information about the Condition
    pub note: Option<Vec<types::Annotation>>,
}

/// Who or what participated in the activities related to the condition and how
/// they were involved.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who or what participated in the activities related to the condition
    pub actor: types::Reference,
}

/// Clinical stage or grade of a condition, usually assessed formally.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionStage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Simple summary (disease specific)
    pub summary: Option<types::CodeableConcept>,

    /// Formal record of assessment
    pub assessment: Option<Vec<types::Reference>>,

    /// Kind of staging
    pub r#type: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Condition;

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
