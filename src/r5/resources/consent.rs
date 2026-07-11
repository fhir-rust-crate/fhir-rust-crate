//! Consent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Consent
//!
//! Version: 5.0.0
//!
//! Consent Resource: A record of a healthcare consumer's choices or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a healthcare consumer's choices, or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or
/// recipient role(s) to perform one or more actions within a given policy
/// context, for specific purposes and periods of time.
///
/// In FHIR R5, the Consent resource is used to capture privacy, treatment,
/// research, and advance-care directives, along with the provisions that
/// constrain how data and actions are governed by the consent.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::consent::Consent;
///
/// let value = Consent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Consent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Consent {
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

    /// Identifier for this record (external references)
    pub identifier: Option<Vec<types::Identifier>>,

    /// draft | active | inactive | not-done | entered-in-error | unknown
    pub status: types::Code,

    /// Classification of the consent statement - for indexing/retrieval
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Who the consent applies to
    pub subject: Option<types::Reference>,

    /// Fully executed date of the consent
    pub date: Option<types::Date>,

    /// Effective period for this Consent
    pub period: Option<types::Period>,

    /// Who is granting rights according to the policy and rules
    pub grantor: Option<Vec<types::Reference>>,

    /// Who is agreeing to the policy and rules
    pub grantee: Option<Vec<types::Reference>>,

    /// Consent workflow management
    pub manager: Option<Vec<types::Reference>>,

    /// Consent Enforcer
    pub controller: Option<Vec<types::Reference>>,

    /// Source from which this consent is taken
    pub source_attachment: Option<Vec<types::Attachment>>,

    /// Source from which this consent is taken
    pub source_reference: Option<Vec<types::Reference>>,

    /// Regulations establishing base Consent
    pub regulatory_basis: Option<Vec<types::CodeableConcept>>,

    /// Computable version of the backing policy
    pub policy_basis: Option<ConsentPolicyBasis>,

    /// Human Readable Policy
    pub policy_text: Option<Vec<types::Reference>>,

    /// Consent Verified by patient or family
    pub verification: Option<Vec<ConsentVerification>>,

    /// deny | permit
    pub decision: Option<types::Code>,

    /// Constraints to the base Consent.policyRule/Consent.policy
    pub provision: Option<Vec<ConsentProvision>>,
}

/// Computable version of the backing policy.
///
/// A backing policy, referenced either as a FHIR resource or an external
/// computable URL, that this consent is derived from.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConsentPolicyBasis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference backing policy resource
    pub reference: Option<types::Reference>,

    /// URL to a computable backing policy
    pub url: Option<types::Url>,
}

/// Consent Verified by patient or family.
///
/// Whether a treatment instruction (e.g. artificial respiration: yes or no)
/// was verified with the patient, his/her family or another authorized person.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConsentVerification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Has been verified
    pub verified: types::Boolean,

    /// Business case of verification
    pub verification_type: Option<types::CodeableConcept>,

    /// Person conducting verification
    pub verified_by: Option<types::Reference>,

    /// Person who verified
    pub verified_with: Option<types::Reference>,

    /// When consent verified
    pub verification_date: Option<Vec<types::DateTime>>,
}

/// Constraints to the base Consent.policyRule/Consent.policy.
///
/// An exception to the base policy of this consent. An exception can be an
/// addition or removal of access permissions. Provisions may be nested to any
/// depth to express complex constraints.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConsentProvision {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Timeframe for this provision
    pub period: Option<types::Period>,

    /// Who|what controlled by this provision (or group, by role)
    pub actor: Option<Vec<ConsentProvisionActor>>,

    /// Actions controlled by this provision
    pub action: Option<Vec<types::CodeableConcept>>,

    /// Security Labels that define affected resources
    pub security_label: Option<Vec<types::Coding>>,

    /// Context of activities covered by this provision
    pub purpose: Option<Vec<types::Coding>>,

    /// e.g. Resource Type, Profile, CDA, etc
    pub document_type: Option<Vec<types::Coding>>,

    /// e.g. Resource Type, Profile, etc
    pub resource_type: Option<Vec<types::Coding>>,

    /// e.g. LOINC or SNOMED CT code, etc. in the content
    pub code: Option<Vec<types::CodeableConcept>>,

    /// Timeframe for data controlled by this provision
    pub data_period: Option<types::Period>,

    /// Data controlled by this provision
    pub data: Option<Vec<ConsentProvisionData>>,

    /// A computable expression of the consent
    pub expression: Option<types::Expression>,

    /// Nested Exception Provisions
    pub provision: Option<Vec<ConsentProvision>>,
}

/// Who|what controlled by this provision (or group, by role).
///
/// Who or what is controlled by this provision. Use group to identify a set of
/// actors by some property they share (e.g. 'admitting officers').
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConsentProvisionActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// How the actor is involved
    pub role: Option<types::CodeableConcept>,

    /// Resource for the actor (or group, by role)
    pub reference: Option<types::Reference>,
}

/// Data controlled by this provision.
///
/// The resources controlled by this provision if specific resources are
/// referenced.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConsentProvisionData {
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

#[cfg(test)]
mod tests {
    use super::*;
    type T = Consent;

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
