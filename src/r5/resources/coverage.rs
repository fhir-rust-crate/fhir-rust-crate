//! Coverage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coverage
//!
//! Version: 5.0.0
//!
//! Coverage Resource: Financial instrument which may be used to reimburse or pay for health care products and services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. This includes both insurance and self-payment. In
/// FHIR R5 the Coverage resource records the identifiers and relationships of a
/// payment source (such as an insurance policy, a government program, or a
/// self-pay arrangement) and links a beneficiary to the party responsible for
/// payment. It is referenced by billing and adjudication resources such as
/// Claim, ExplanationOfBenefit, and CoverageEligibilityRequest.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage::Coverage;
///
/// let value = Coverage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Coverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
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

    /// Business identifier(s) for this coverage
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | cancelled | draft | entered-in-error
    pub status: types::Code,

    /// insurance | self-pay | other
    pub kind: types::Code,

    /// Self-pay parties and responsibility
    pub payment_by: Option<Vec<CoveragePaymentBy>>,

    /// Coverage category such as medical or accident
    pub r#type: Option<types::CodeableConcept>,

    /// Owner of the policy
    pub policy_holder: Option<types::Reference>,

    /// Subscriber to the policy
    pub subscriber: Option<types::Reference>,

    /// ID assigned to the subscriber
    pub subscriber_id: Option<Vec<types::Identifier>>,

    /// Plan beneficiary
    pub beneficiary: types::Reference,

    /// Dependent number
    pub dependent: Option<types::String>,

    /// Beneficiary relationship to the subscriber
    pub relationship: Option<types::CodeableConcept>,

    /// Coverage start and end dates
    pub period: Option<types::Period>,

    /// Issuer of the policy
    pub insurer: Option<types::Reference>,

    /// Additional coverage classifications
    pub class: Option<Vec<CoverageClass>>,

    /// Relative order of the coverage
    pub order: Option<types::PositiveInt>,

    /// Insurer network
    pub network: Option<types::String>,

    /// Patient payments for services/products
    pub cost_to_beneficiary: Option<Vec<CoverageCostToBeneficiary>>,

    /// Reimbursement to insurer
    pub subrogation: Option<types::Boolean>,

    /// Contract details
    pub contract: Option<Vec<types::Reference>>,

    /// Insurance plan details
    pub insurance_plan: Option<types::Reference>,
}

/// Self-pay parties and responsibility. Identifies parties that are responsible
/// for self-payment of a portion of the beneficiary's costs and the extent of
/// their responsibility.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoveragePaymentBy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Parties performing self-payment
    pub party: types::Reference,

    /// Party's responsibility
    pub responsibility: Option<types::String>,
}

/// Additional coverage classifications. A suite of underwriter-specific
/// classifiers, such as group, plan, or subgroup, used to categorize the
/// coverage.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageClass {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of class such as 'group' or 'plan'
    pub r#type: types::CodeableConcept,

    /// Value associated with the type
    pub value: types::Identifier,

    /// Human readable description of the type and value
    pub name: Option<types::String>,
}

/// Patient payments for services/products. A suite of codes indicating the cost
/// category and associated amount which have been detailed in the policy and
/// may have been included on the health card.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageCostToBeneficiary {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Cost category
    pub r#type: Option<types::CodeableConcept>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// In or out of network
    pub network: Option<types::CodeableConcept>,

    /// Individual or family
    pub unit: Option<types::CodeableConcept>,

    /// Annual or lifetime
    pub term: Option<types::CodeableConcept>,

    /// The amount or percentage due from the beneficiary
    pub value_quantity: Option<types::Quantity>,

    /// The amount or percentage due from the beneficiary
    pub value_money: Option<types::Money>,

    /// Exceptions for patient payments
    pub exception: Option<Vec<CoverageCostToBeneficiaryException>>,
}

/// Exceptions for patient payments. A suite of codes indicating exceptions or
/// reductions to patient costs and their effective periods.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageCostToBeneficiaryException {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Exception category
    pub r#type: types::CodeableConcept,

    /// The effective period of the exception
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coverage;

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
