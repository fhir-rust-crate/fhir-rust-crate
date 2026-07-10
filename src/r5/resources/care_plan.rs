//! CarePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CarePlan
//!
//! Version: 5.0.0
//!
//! CarePlan Resource: Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Describes the intention of how one or more practitioners intend to deliver
/// care for a particular patient, group or community for a period of time,
/// possibly limited to care for a specific condition or set of conditions.
///
/// The CarePlan resource is used across many care settings to coordinate the
/// planned and ongoing activities directed at addressing a patient's health
/// concerns. It links the subject of care to the goals, addressed conditions,
/// care team, and the set of intended or performed activities that make up the
/// plan. In FHIR R5 it commonly references Goal, CareTeam, and Condition
/// resources to build a comprehensive, longitudinal view of a patient's care.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::care_plan::CarePlan;
///
/// let value = CarePlan::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CarePlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CarePlan {
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

    /// External Ids for this plan
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// Fulfills plan, proposal or order
    pub based_on: Option<Vec<types::Reference>>,

    /// CarePlan replaced by this CarePlan
    pub replaces: Option<Vec<types::Reference>>,

    /// Part of referenced CarePlan
    pub part_of: Option<Vec<types::Reference>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: types::Code,

    /// proposal | plan | order | option | directive
    pub intent: types::Code,

    /// Type of plan
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Human-friendly name for the care plan
    pub title: Option<types::String>,

    /// Summary of nature of plan
    pub description: Option<types::String>,

    /// Who the care plan is for
    pub subject: types::Reference,

    /// The Encounter during which this CarePlan was created
    pub encounter: Option<types::Reference>,

    /// Time period plan covers
    pub period: Option<types::Period>,

    /// Date record was first recorded
    pub created: Option<types::DateTime>,

    /// Who is the designated responsible party
    pub custodian: Option<types::Reference>,

    /// Who provided the content of the care plan
    pub contributor: Option<Vec<types::Reference>>,

    /// Who's involved in plan?
    pub care_team: Option<Vec<types::Reference>>,

    /// Health issues this plan addresses
    pub addresses: Option<Vec<types::CodeableReference>>,

    /// Information considered as part of plan
    pub supporting_info: Option<Vec<types::Reference>>,

    /// Desired outcome of plan
    pub goal: Option<Vec<types::Reference>>,

    /// Action to occur or has occurred as part of plan
    pub activity: Option<Vec<CarePlanActivity>>,

    /// Comments about the plan
    pub note: Option<Vec<types::Annotation>>,
}

/// Action to occur or has occurred as part of plan.
///
/// Identifies an action that has occurred or is a planned action to occur as
/// part of the plan. For example, a medication to be used, lab tests to perform,
/// self-monitoring, or education.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CarePlanActivity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Results of the activity (concept, or Appointment, Encounter, Procedure, etc.)
    pub performed_activity: Option<Vec<types::CodeableReference>>,

    /// Comments about the activity status/progress
    pub progress: Option<Vec<types::Annotation>>,

    /// Activity that is intended to be part of the care plan
    pub planned_activity_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CarePlan;

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
