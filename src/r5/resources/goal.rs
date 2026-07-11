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
use fhir_derive_macros::Validate;

/// Describes the intended objective(s) for a patient, group or organization
/// care, for example, weight loss, restoring an activity of daily living,
/// obtaining herd immunity via immunization, meeting a process improvement
/// objective, etc. A Goal captures the desired outcome and, optionally, the
/// specific measurable targets that indicate progress toward that outcome.
///
/// Goals are used throughout clinical and administrative workflows to record
/// what a care team, patient, or organization is trying to achieve, and to
/// track progress over time. A Goal is often referenced from a care plan and
/// may be associated with one or more conditions, observations, or other
/// clinical findings that motivated it. Each Goal has a lifecycle status
/// (such as `proposed`, `active`, or `completed`) and, separately, an
/// optional achievement status describing how well the goal is being met.
/// The `target` element allows one or more measurable outcomes to be
/// specified, each expressed using a quantity, range, ratio, or other value
/// type, optionally with a due date or duration by which it should be met.
///
/// # Related resources
///
/// A Goal's `subject` is commonly a [`Patient`](crate::r5::resources::patient::Patient),
/// though it may also reference a group or organization. Goals are typically
/// linked from a care plan and may reference conditions or observations as
/// the basis for the goal; codes describing categories, priority, and
/// achievement status use [`CodeableConcept`](crate::r5::types::CodeableConcept).
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

    /// External Ids for this goal
    pub identifier: Option<Vec<types::Identifier>>,

    /// The current lifecycle status of this goal: proposed | planned | accepted | active | on-hold | completed | cancelled | entered-in-error | rejected
    pub lifecycle_status: types::Code,
    /// Primitive extension sibling for [`lifecycle_status`](Self::lifecycle_status) (FHIR `_lifecycleStatus`).
    #[serde(rename = "_lifecycleStatus")]
    pub lifecycle_status_ext: Option<types::Element>,

    /// Describes progress toward meeting the goal: in-progress | improving | worsening | no-change | achieved | sustaining | not-achieved | no-progress | not-attainable
    pub achievement_status: Option<types::CodeableConcept>,

    /// E.g. Treatment, dietary, behavioral, etc
    pub category: Option<Vec<types::CodeableConcept>>,

    /// After meeting the goal, ongoing activity is needed to sustain the goal objective
    pub continuous: Option<types::Boolean>,
    /// Primitive extension sibling for [`continuous`](Self::continuous) (FHIR `_continuous`).
    #[serde(rename = "_continuous")]
    pub continuous_ext: Option<types::Element>,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<types::CodeableConcept>,

    /// Human-readable code or text describing the goal to be achieved
    pub description: types::CodeableConcept,

    /// The patient, group, or organization for whom this goal is intended
    pub subject: types::Reference,

    /// When goal pursuit begins (date)
    pub start_date: Option<types::Date>,

    /// When goal pursuit begins (CodeableConcept)
    pub start_codeable_concept: Option<types::CodeableConcept>,

    /// One or more measurable target outcomes that define what should be achieved
    pub target: Option<Vec<GoalTarget>>,

    /// When goal status took effect
    pub status_date: Option<types::Date>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`).
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::String>,
    /// Primitive extension sibling for [`status_reason`](Self::status_reason) (FHIR `_statusReason`).
    #[serde(rename = "_statusReason")]
    pub status_reason_ext: Option<types::Element>,

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
