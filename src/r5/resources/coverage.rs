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
use fhir_derive_macros::Validate;

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. This includes both insurance and self-payment. In
/// FHIR R5 the Coverage resource records the identifiers and relationships of a
/// payment source (such as an insurance policy, a government program, or a
/// self-pay arrangement) and links a beneficiary to the party responsible for
/// payment. It is referenced by billing and adjudication resources such as
/// Claim, ExplanationOfBenefit, and CoverageEligibilityRequest.
///
/// Coverage captures the administrative and financial details needed to
/// determine who will pay for a beneficiary's health care: the subscriber and
/// beneficiary, the insurer or other responsible party, the coverage period,
/// classification information such as group or plan identifiers, and any
/// patient cost-sharing rules (for example copays, deductibles, or
/// exceptions). Systems typically use it to check eligibility, to populate
/// claims and pre-authorization requests, and to reconcile payments during
/// adjudication.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) is commonly the
///   `beneficiary` or `subscriber` referenced by this resource.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used for coded
///   fields such as `type`, `relationship`, and cost category classifications.
/// - `Claim`, `ExplanationOfBenefit`, and `CoverageEligibilityRequest` are
///   related billing and adjudication resources that reference a Coverage.
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

    /// Business identifier(s) for this coverage, such as a member or policy number
    pub identifier: Option<Vec<types::Identifier>>,

    /// The status of the resource instance: active | cancelled | draft | entered-in-error
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The nature of the coverage: insurance | self-pay | other
    pub kind: types::Code,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

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

    /// Plan beneficiary, typically a reference to a Patient
    pub beneficiary: types::Reference,

    /// Dependent number
    pub dependent: Option<types::String>,
    /// Primitive extension sibling for [`dependent`](Self::dependent) (FHIR `_dependent`).
    #[serde(rename = "_dependent")]
    pub dependent_ext: Option<types::Element>,

    /// Beneficiary relationship to the subscriber
    pub relationship: Option<types::CodeableConcept>,

    /// Coverage start and end dates
    pub period: Option<types::Period>,

    /// Issuer of the policy, such as the insurance organization or program
    pub insurer: Option<types::Reference>,

    /// Additional coverage classifications
    pub class: Option<Vec<CoverageClass>>,

    /// Relative order of the coverage
    pub order: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`order`](Self::order) (FHIR `_order`).
    #[serde(rename = "_order")]
    pub order_ext: Option<types::Element>,

    /// Insurer network
    pub network: Option<types::String>,
    /// Primitive extension sibling for [`network`](Self::network) (FHIR `_network`).
    #[serde(rename = "_network")]
    pub network_ext: Option<types::Element>,

    /// Patient payments for services/products
    pub cost_to_beneficiary: Option<Vec<CoverageCostToBeneficiary>>,

    /// Reimbursement to insurer
    pub subrogation: Option<types::Boolean>,
    /// Primitive extension sibling for [`subrogation`](Self::subrogation) (FHIR `_subrogation`).
    #[serde(rename = "_subrogation")]
    pub subrogation_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`responsibility`](Self::responsibility) (FHIR `_responsibility`).
    #[serde(rename = "_responsibility")]
    pub responsibility_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
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
