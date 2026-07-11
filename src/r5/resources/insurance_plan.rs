//! InsurancePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
//!
//! Version: 5.0.0
//!
//! InsurancePlan Resource: Details of a Health Insurance product/plan provided by an organization.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Details of a Health Insurance product/plan provided by an organization. An
/// InsurancePlan describes a health insurance offering comprised of a list of
/// covered benefits (i.e. the product), costs associated with those benefits
/// (i.e. the plan), and additional information about the offering, such as who
/// it is owned and administered by, a coverage area, contact information, etc.
/// It is distinct from a member's actual Coverage of a given product.
///
/// InsurancePlan is used by payers, health plan directories, and provider
/// network publications to describe the products and plans an insurer
/// offers, including the networks that participate in them, the geographic
/// areas they serve, and the general and specific costs (premiums,
/// copayments, deductibles, and similar) associated with each plan. It
/// supports use cases such as plan comparison shopping, network adequacy
/// reporting, and payer-to-payer or payer-to-provider directory exchange.
///
/// # Related resources
///
/// - [`Organization`](crate::r5::resources::organization::Organization) is
///   typically referenced as the product issuer (`owned_by`) and
///   administrator (`administered_by`).
/// - `Coverage` represents a member's actual enrollment in a product
///   described by an InsurancePlan.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used
///   throughout to code plan types, coverage types, and benefit categories.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlan;
///
/// let value = InsurancePlan::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: InsurancePlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlan {
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

    /// Business Identifier for Product
    pub identifier: Option<Vec<types::Identifier>>,

    /// The lifecycle status of this plan: draft | active | retired | unknown.
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of product, such as medical or dental, coded as a
    /// [`CodeableConcept`](crate::r5::types::CodeableConcept).
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Official name of the health insurance product or plan.
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Alternate names
    pub alias: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`).
    #[serde(rename = "_alias")]
    pub alias_ext: Option<Vec<Option<types::Element>>>,

    /// When the product is available
    pub period: Option<types::Period>,

    /// Reference to the [`Organization`](crate::r5::resources::organization::Organization)
    /// that is the product issuer.
    pub owned_by: Option<types::Reference>,

    /// Reference to the [`Organization`](crate::r5::resources::organization::Organization)
    /// that administers the product.
    pub administered_by: Option<types::Reference>,

    /// Geographic area(s), typically Location resources, where the product applies.
    pub coverage_area: Option<Vec<types::Reference>>,

    /// Official contact details relevant to the health insurance plan/product
    pub contact: Option<Vec<types::ExtendedContactDetail>>,

    /// Technical endpoint
    pub endpoint: Option<Vec<types::Reference>>,

    /// What networks are Included
    pub network: Option<Vec<types::Reference>>,

    /// Coverage details
    pub coverage: Option<Vec<InsurancePlanCoverage>>,

    /// Plan details
    pub plan: Option<Vec<InsurancePlanPlan>>,
}

/// Coverage details: the details of a coverage offered by the insurance product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of coverage
    pub r#type: types::CodeableConcept,

    /// What networks provide coverage
    pub network: Option<Vec<types::Reference>>,

    /// List of benefits
    pub benefit: Vec<InsurancePlanCoverageBenefit>,
}

/// List of benefits: specific benefits under this type of coverage.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverageBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of benefit
    pub r#type: types::CodeableConcept,

    /// Referral requirements
    pub requirement: Option<types::String>,
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`).
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// Benefit limits
    pub limit: Option<Vec<InsurancePlanCoverageBenefitLimit>>,
}

/// Benefit limits: the specific limits on the benefit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverageBenefitLimit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Maximum value allowed
    pub value: Option<types::Quantity>,

    /// Benefit limit details
    pub code: Option<types::CodeableConcept>,
}

/// Plan details: details about an insurance plan.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlan {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Business Identifier for Product
    pub identifier: Option<Vec<types::Identifier>>,

    /// Type of plan
    pub r#type: Option<types::CodeableConcept>,

    /// Where product applies
    pub coverage_area: Option<Vec<types::Reference>>,

    /// What networks provide coverage
    pub network: Option<Vec<types::Reference>>,

    /// Overall costs
    pub general_cost: Option<Vec<InsurancePlanPlanGeneralCost>>,

    /// Specific costs
    pub specific_cost: Option<Vec<InsurancePlanPlanSpecificCost>>,
}

/// Overall costs: overall costs associated with the plan.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanGeneralCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of cost
    pub r#type: Option<types::CodeableConcept>,

    /// Number of enrollees
    pub group_size: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`group_size`](Self::group_size) (FHIR `_groupSize`).
    #[serde(rename = "_groupSize")]
    pub group_size_ext: Option<types::Element>,

    /// Cost value
    pub cost: Option<types::Money>,

    /// Additional cost information
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// Specific costs: costs associated with the coverage provided by the product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// General category of benefit
    pub category: types::CodeableConcept,

    /// Benefits list
    pub benefit: Option<Vec<InsurancePlanPlanSpecificCostBenefit>>,
}

/// Benefits list: list of the specific benefits under this category of benefit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCostBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of specific benefit
    pub r#type: types::CodeableConcept,

    /// List of the costs
    pub cost: Option<Vec<InsurancePlanPlanSpecificCostBenefitCost>>,
}

/// List of the costs: list of the costs associated with a specific benefit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCostBenefitCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of cost
    pub r#type: types::CodeableConcept,

    /// in-network | out-of-network | other
    pub applicability: Option<types::CodeableConcept>,

    /// Additional information about the cost
    pub qualifiers: Option<Vec<types::CodeableConcept>>,

    /// The actual cost value
    pub value: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = InsurancePlan;

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
