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
use fhir_derive::Validate;

/// The Permission resource holds access rules for a given data and context.
///
/// It expresses a set of constraints under which data may be accessed, combining
/// one or more rules that permit or deny particular activities on selected data.
/// A Permission may be asserted by a person or entity, may be valid for a given
/// period, and carries a combining algorithm that determines how its rules
/// interact. It is used in FHIR R5 to model consent-adjacent authorization
/// policies and fine-grained access control decisions.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::permission::Permission;
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

    /// active | entered-in-error | draft | rejected
    pub status: types::Code,

    /// The person or entity that asserts the permission
    pub asserter: Option<types::Reference>,

    /// The date that permission was asserted
    pub date: Option<Vec<types::DateTime>>,

    /// The period in which the permission is active
    pub validity: Option<types::Period>,

    /// The asserted justification for using the data
    pub justification: Option<PermissionJustification>,

    /// deny-overrides | permit-overrides | ordered-deny-overrides | ordered-permit-overrides | deny-unless-permit | permit-unless-deny
    pub combining: types::Code,

    /// Constraints to the Permission
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
