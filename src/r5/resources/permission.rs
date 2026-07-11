//! Permission
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Permission
//!
//! Version: 5.0.0
//!
//! Permission Resource: Permission resource holds access rules for a given data and context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Permission resource holds access rules for a given data and context.
///
/// In FHIR R5 the Permission resource captures a machine-processable authorization
/// policy: it expresses the set of constraints under which specific data may be
/// accessed or acted upon. Each Permission carries one or more rules that either
/// permit or deny particular activities, scoped by the actors involved, the
/// purposes of use, the actions performed, and the data selected by explicit
/// references, security labels, time periods, or FHIRPath expressions. A Permission
/// is asserted by a person or organization, may be constrained to a validity
/// period, and specifies a combining algorithm (for example deny-overrides or
/// permit-overrides) that determines how its rules are reconciled when more than
/// one applies. This makes it well suited to modeling fine-grained access-control
/// decisions, security policies, and the enforceable representation of a patient's
/// or organization's data-sharing directives.
///
/// # Related resources
///
/// A Permission frequently complements a broader
/// [`Consent`](crate::r5::resources::consent::Consent), which records a subject's
/// wishes, while the Permission expresses the enforceable rules derived from them.
/// Rules commonly reference actors and data such as
/// [`Patient`](crate::r5::resources::patient::Patient) records and audit trails
/// like [`Provenance`](crate::r5::resources::provenance::Provenance), and they
/// classify activities and limits using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::permission::Permission;
///
/// let value = Permission::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Permission = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
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

    /// Lifecycle state of the permission: active, entered-in-error, draft, or rejected.
    pub status: types::Code,

    /// Reference to the person or entity that asserts this permission and its rules.
    pub asserter: Option<types::Reference>,

    /// The date(s) on which the permission was asserted by the asserter.
    pub date: Option<Vec<types::DateTime>>,

    /// The period during which this permission's rules are in effect.
    pub validity: Option<types::Period>,

    /// The legal or regulatory basis and supporting evidence justifying the use of the data.
    pub justification: Option<PermissionJustification>,

    /// Combining algorithm that reconciles conflicting rules: deny-overrides, permit-overrides, ordered-deny-overrides, ordered-permit-overrides, deny-unless-permit, or permit-unless-deny.
    pub combining: types::Code,

    /// The ordered set of rules that constrain access under this permission.
    pub rule: Option<Vec<PermissionRule>>,
}

/// The asserted justification for using the data.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionJustification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The regulatory grounds upon which this Permission builds
    pub basis: Option<Vec<types::CodeableConcept>>,

    /// Justifing rational
    pub evidence: Option<Vec<types::Reference>>,
}

/// Constraints to the Permission.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// deny | permit
    pub r#type: Option<types::Code>,

    /// The selection criteria to identify data that is within scope of this provision
    pub data: Option<Vec<PermissionRuleData>>,

    /// A description or definition of which activities are allowed to be done on the data
    pub activity: Option<Vec<PermissionRuleActivity>>,

    /// What limits apply to the use of the data
    pub limit: Option<Vec<types::CodeableConcept>>,
}

/// The selection criteria to identify data that is within scope of this provision.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Explicit FHIR Resource references
    pub resource: Option<Vec<PermissionRuleDataResource>>,

    /// Security tag code on .meta.security
    pub security: Option<Vec<types::Coding>>,

    /// Timeframe encompasing data create/update
    pub period: Option<Vec<types::Period>>,

    /// Expression identifying the data
    pub expression: Option<types::Expression>,
}

/// Explicit FHIR Resource references.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleDataResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// instance | related | dependents | authoredby
    pub meaning: types::Code,

    /// The actual data reference
    pub reference: types::Reference,
}

/// A description or definition of which activities are allowed to be done on the data.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleActivity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Authorized actor(s)
    pub actor: Option<Vec<types::Reference>>,

    /// Actions controlled by this rule
    pub action: Option<Vec<types::CodeableConcept>>,

    /// The purpose for which the permission is given
    pub purpose: Option<Vec<types::CodeableConcept>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Permission;

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
