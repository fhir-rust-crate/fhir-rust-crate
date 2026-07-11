//! CoverageEligibilityResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse
//!
//! Version: 5.0.0
//!
//! CoverageEligibilityResponse Resource: This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource provides eligibility and plan details from the processing of a
/// CoverageEligibilityRequest resource. It conveys the insurer's response about
/// whether coverage is in force for a patient, along with the benefits, limits,
/// authorization requirements, and any processing errors. A
/// CoverageEligibilityResponse is typically returned in reply to a
/// CoverageEligibilityRequest.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_response::CoverageEligibilityResponse;
///
/// let value = CoverageEligibilityResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CoverageEligibilityResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponse {
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

    /// auth-requirements | benefits | discovery | validation
    pub purpose: Vec<types::Code>,

    /// Intended recipient of products and services
    pub patient: types::Reference,

    /// Event information
    pub event: Option<Vec<CoverageEligibilityResponseEvent>>,

    /// Estimated date or dates of service
    pub serviced_date: Option<types::Date>,

    /// Estimated date or dates of service
    pub serviced_period: Option<types::Period>,

    /// Response creation date
    pub created: types::DateTime,

    /// Party responsible for the request
    pub requestor: Option<types::Reference>,

    /// Eligibility request reference
    pub request: types::Reference,

    /// queued | complete | error | partial
    pub outcome: types::Code,

    /// Disposition Message
    pub disposition: Option<types::String>,

    /// Coverage issuer
    pub insurer: types::Reference,

    /// Patient insurance information
    pub insurance: Option<Vec<CoverageEligibilityResponseInsurance>>,

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,

    /// Printed form identifier
    pub form: Option<types::CodeableConcept>,

    /// Processing errors
    pub error: Option<Vec<CoverageEligibilityResponseError>>,
}

/// Event information for a CoverageEligibilityResponse.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseEvent {
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

/// Patient insurance information within a CoverageEligibilityResponse.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Coverage inforce indicator
    pub inforce: Option<types::Boolean>,

    /// When the benefits are applicable
    pub benefit_period: Option<types::Period>,

    /// Benefits and authorization details
    pub item: Option<Vec<CoverageEligibilityResponseInsuranceItem>>,
}

/// Benefits and authorization details for an insurance entry.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseInsuranceItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// Product or service billing modifiers
    pub modifier: Option<Vec<types::CodeableConcept>>,

    /// Performing practitioner
    pub provider: Option<types::Reference>,

    /// Excluded from the plan
    pub excluded: Option<types::Boolean>,

    /// Short name for the benefit
    pub name: Option<types::String>,

    /// Description of the benefit or services covered
    pub description: Option<types::String>,

    /// In or out of network
    pub network: Option<types::CodeableConcept>,

    /// Individual or family
    pub unit: Option<types::CodeableConcept>,

    /// Annual or lifetime
    pub term: Option<types::CodeableConcept>,

    /// Benefit Summary
    pub benefit: Option<Vec<CoverageEligibilityResponseInsuranceItemBenefit>>,

    /// Authorization required flag
    pub authorization_required: Option<types::Boolean>,

    /// Type of required supporting materials
    pub authorization_supporting: Option<Vec<types::CodeableConcept>>,

    /// Preauthorization requirements endpoint
    pub authorization_url: Option<types::Uri>,
}

/// Benefit Summary detailing allowed and used amounts.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Benefit classification
    pub r#type: types::CodeableConcept,

    /// Benefits allowed
    pub allowed_unsigned_int: Option<types::UnsignedInt>,

    /// Benefits allowed
    pub allowed_string: Option<types::String>,

    /// Benefits allowed
    pub allowed_money: Option<types::Money>,

    /// Benefits used
    pub used_unsigned_int: Option<types::UnsignedInt>,

    /// Benefits used
    pub used_string: Option<types::String>,

    /// Benefits used
    pub used_money: Option<types::Money>,
}

/// Processing errors reported in a CoverageEligibilityResponse.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseError {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Error code detailing processing issues
    pub code: types::CodeableConcept,

    /// FHIRPath of element(s) related to issue
    pub expression: Option<Vec<types::String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CoverageEligibilityResponse;

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
