//! Goal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Goal
//!
//! Version: 5.0.0
//!
//! Goal Resource: Describes the intended objective(s) for a patient, group or organization care.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Describes the intended objective(s) for a patient, group or organization
/// care, for example, weight loss, restoring an activity of daily living,
/// obtaining herd immunity via immunization, meeting a process improvement
/// objective, etc. A Goal captures the desired outcome and, optionally, the
/// specific measurable targets that indicate progress toward that outcome.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::goal::Goal;
///
/// let value = Goal::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Goal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
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

    /// External Ids for this goal
    pub identifier: Option<Vec<types::Identifier>>,

    /// proposed | planned | accepted | active | on-hold | completed | cancelled | entered-in-error | rejected
    pub lifecycle_status: types::Code,

    /// in-progress | improving | worsening | no-change | achieved | sustaining | not-achieved | no-progress | not-attainable
    pub achievement_status: Option<types::CodeableConcept>,

    /// E.g. Treatment, dietary, behavioral, etc
    pub category: Option<Vec<types::CodeableConcept>>,

    /// After meeting the goal, ongoing activity is needed to sustain the goal objective
    pub continuous: Option<types::Boolean>,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<types::CodeableConcept>,

    /// Code or text describing goal
    pub description: types::CodeableConcept,

    /// Who this goal is intended for
    pub subject: types::Reference,

    /// When goal pursuit begins (date)
    pub start_date: Option<types::Date>,

    /// When goal pursuit begins (CodeableConcept)
    pub start_codeable_concept: Option<types::CodeableConcept>,

    /// Target outcome for the goal
    pub target: Option<Vec<GoalTarget>>,

    /// When goal status took effect
    pub status_date: Option<types::Date>,

    /// Reason for current status
    pub status_reason: Option<types::String>,

    /// Who's responsible for creating Goal?
    pub source: Option<types::Reference>,

    /// Issues addressed by this goal
    pub addresses: Option<Vec<types::Reference>>,

    /// Comments about the goal
    pub note: Option<Vec<types::Annotation>>,

    /// What result was achieved regarding the goal?
    pub outcome: Option<Vec<types::CodeableReference>>,
}

/// Target outcome for the goal.
///
/// Indicates what should be done and by when to consider progress toward the
/// goal to have been made. A goal may have zero or more targets.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GoalTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The parameter whose value is being tracked
    pub measure: Option<types::CodeableConcept>,

    /// The target value to be achieved (Quantity)
    pub detail_quantity: Option<types::Quantity>,

    /// The target value to be achieved (Range)
    pub detail_range: Option<types::Range>,

    /// The target value to be achieved (CodeableConcept)
    pub detail_codeable_concept: Option<types::CodeableConcept>,

    /// The target value to be achieved (string)
    pub detail_string: Option<types::String>,

    /// The target value to be achieved (boolean)
    pub detail_boolean: Option<types::Boolean>,

    /// The target value to be achieved (integer)
    pub detail_integer: Option<types::Integer>,

    /// The target value to be achieved (Ratio)
    pub detail_ratio: Option<types::Ratio>,

    /// Reach goal on or before (date)
    pub due_date: Option<types::Date>,

    /// Reach goal on or before (Duration)
    pub due_duration: Option<types::Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Goal;

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
