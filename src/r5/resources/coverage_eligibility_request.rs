//! CoverageEligibilityRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityRequest
//!
//! Version: 5.0.0
//!
//! CoverageEligibilityRequest Resource: Provides patient and insurance coverage information to an insurer to determine coverage validity and benefits.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of a
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance
/// details of the policy. It is used to inquire about auth requirements,
/// benefits, coverage discovery, or validation prior to delivering products
/// and services.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_request::CoverageEligibilityRequest;
///
/// let value = CoverageEligibilityRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CoverageEligibilityRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequest {
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

    /// Business Identifier for coverage eligiblity request
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | cancelled | draft | entered-in-error
    pub status: types::Code,

    /// Desired processing priority
    pub priority: Option<types::CodeableConcept>,

    /// auth-requirements | benefits | discovery | validation
    pub purpose: Vec<types::Code>,

    /// Intended recipient of products and services
    pub patient: types::Reference,

    /// Event information
    pub event: Option<Vec<CoverageEligibilityRequestEvent>>,

    /// Estimated date or dates of service
    pub serviced_date: Option<types::Date>,

    /// Estimated date or dates of service
    pub serviced_period: Option<types::Period>,

    /// Creation date
    pub created: types::DateTime,

    /// Author
    pub enterer: Option<types::Reference>,

    /// Party responsible for the request
    pub provider: Option<types::Reference>,

    /// Coverage issuer
    pub insurer: types::Reference,

    /// Servicing facility
    pub facility: Option<types::Reference>,

    /// Supporting information
    pub supporting_info: Option<Vec<CoverageEligibilityRequestSupportingInfo>>,

    /// Patient insurance information
    pub insurance: Option<Vec<CoverageEligibilityRequestInsurance>>,

    /// Item to be evaluated for eligibiity
    pub item: Option<Vec<CoverageEligibilityRequestItem>>,
}

/// Additional information codes regarding exceptions, special considerations,
/// the condition, situation, prior or concurrent issues.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Specific event
    pub r#type: types::CodeableConcept,

    /// Occurance date or period
    pub when_date_time: Option<types::DateTime>,

    /// Occurance date or period
    pub when_period: Option<types::Period>,
}

/// Additional information codes regarding exceptions, special considerations,
/// the condition, situation, prior or concurrent issues supplied in support of
/// the coverage eligibility request.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Information instance identifier
    pub sequence: types::PositiveInt,

    /// Data to be provided
    pub information: types::Reference,

    /// Applies to all items
    pub applies_to_all: Option<types::Boolean>,
}

/// Financial instruments for reimbursement for the health care products and
/// services identified in the request.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Applicable coverage
    pub focal: Option<types::Boolean>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,
}

/// Service categories or billable services for which benefit details and/or an
/// authorization prior to service delivery may be required by the payor.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Applicable exception or supporting information
    pub supporting_info_sequence: Option<Vec<types::PositiveInt>>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// Product or service billing modifiers
    pub modifier: Option<Vec<types::CodeableConcept>>,

    /// Perfoming practitioner
    pub provider: Option<types::Reference>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Servicing facility
    pub facility: Option<types::Reference>,

    /// Applicable diagnosis
    pub diagnosis: Option<Vec<CoverageEligibilityRequestItemDiagnosis>>,

    /// Product or service details
    pub detail: Option<Vec<types::Reference>>,
}

/// Patient diagnosis for which care is sought, expressed as a code or a
/// reference to a Condition resource.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestItemDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Nature of illness or problem
    pub diagnosis_codeable_concept: Option<types::CodeableConcept>,

    /// Nature of illness or problem
    pub diagnosis_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CoverageEligibilityRequest;

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
