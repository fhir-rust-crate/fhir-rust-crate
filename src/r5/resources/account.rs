//! Account
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Account
//!
//! Version: 5.0.0
//!
//! Account Resource: A financial tool for tracking value accrued for a particular purpose.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A financial tool for tracking value accrued for a particular purpose. In the
/// healthcare field, an Account is used to track charges for a patient, cost
/// centers, etc. It aggregates the financial context (coverage, guarantors,
/// diagnoses, procedures, and balances) needed to bill and reconcile the value
/// accrued against a subject such as a patient, encounter, or organization.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::account::Account;
///
/// let value = Account::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Account = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Account {
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

    /// Account number
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | inactive | entered-in-error | on-hold | unknown
    pub status: types::Code,

    /// Tracks the lifecycle of the account through the billing process
    pub billing_status: Option<types::CodeableConcept>,

    /// E.g. patient, expense, depreciation
    pub r#type: Option<types::CodeableConcept>,

    /// Human-readable label
    pub name: Option<types::String>,

    /// The entity that caused the expenses
    pub subject: Option<Vec<types::Reference>>,

    /// Transaction window
    pub service_period: Option<types::Period>,

    /// The party(s) that are responsible for covering the payment of this
    /// account, and what order should they be applied to the account
    pub coverage: Option<Vec<AccountCoverage>>,

    /// Entity managing the Account
    pub owner: Option<types::Reference>,

    /// Explanation of purpose/use
    pub description: Option<types::Markdown>,

    /// The parties ultimately responsible for balancing the Account
    pub guarantor: Option<Vec<AccountGuarantor>>,

    /// The list of diagnoses relevant to this account
    pub diagnosis: Option<Vec<AccountDiagnosis>>,

    /// The list of procedures relevant to this account
    pub procedure: Option<Vec<AccountProcedure>>,

    /// Other associated accounts related to this account
    pub related_account: Option<Vec<AccountRelatedAccount>>,

    /// The base or default currency
    pub currency: Option<types::CodeableConcept>,

    /// Calculated account balance(s)
    pub balance: Option<Vec<AccountBalance>>,

    /// Time the balance amount was calculated
    pub calculated_at: Option<types::Instant>,
}

/// The party(s) that are responsible for covering the payment of this account,
/// and what order should they be applied to the account.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountCoverage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The party(s), such as insurances, that may contribute to the payment of
    /// this account
    pub coverage: types::Reference,

    /// The priority of the coverage in the context of this account
    pub priority: Option<types::PositiveInt>,
}

/// The parties ultimately responsible for balancing the Account.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountGuarantor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Responsible entity
    pub party: types::Reference,

    /// Credit or other hold applied
    pub on_hold: Option<types::Boolean>,

    /// Guarantee account during
    pub period: Option<types::Period>,
}

/// The list of diagnoses relevant to this account.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Ranking of the diagnosis (for each type)
    pub sequence: Option<types::PositiveInt>,

    /// The diagnosis relevant to the account
    pub condition: types::CodeableReference,

    /// Date of the diagnosis (when coded diagnosis)
    pub date_of_diagnosis: Option<types::DateTime>,

    /// Type that this diagnosis has relevant to the account (e.g. admission,
    /// billing, discharge …)
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Diagnosis present on Admission
    pub on_admission: Option<types::Boolean>,

    /// Package Code specific for billing
    pub package_code: Option<Vec<types::CodeableConcept>>,
}

/// The list of procedures relevant to this account.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Ranking of the procedure (for each type)
    pub sequence: Option<types::PositiveInt>,

    /// The procedure relevant to the account
    pub code: types::CodeableReference,

    /// Date of the procedure (when coded procedure)
    pub date_of_service: Option<types::DateTime>,

    /// How this procedure value should be used in charging the account
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Package Code specific for billing
    pub package_code: Option<Vec<types::CodeableConcept>>,

    /// Any devices that were associated with the procedure
    pub device: Option<Vec<types::Reference>>,
}

/// Other associated accounts related to this account.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountRelatedAccount {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Relationship of the associated Account
    pub relationship: Option<types::CodeableConcept>,

    /// Reference to an associated Account
    pub account: types::Reference,
}

/// Calculated account balance(s).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Who is expected to pay this part of the balance
    pub aggregate: Option<types::CodeableConcept>,

    /// current | 30 | 60 | 90 | 120
    pub term: Option<types::CodeableConcept>,

    /// Estimated balance
    pub estimate: Option<types::Boolean>,

    /// Calculated amount
    pub amount: types::Money,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Account;

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
