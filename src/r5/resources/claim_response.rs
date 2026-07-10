//! ClaimResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
//!
//! Version: 5.0.0
//!
//! ClaimResponse Resource: This resource provides the adjudication details from the processing of a Claim resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// This resource provides the adjudication details from the processing of a
/// Claim resource. It conveys the outcome of an insurer's or payer's processing
/// of a submitted claim, preauthorization, or predetermination, including
/// adjudicated amounts, notes, payment details, and any processing errors.
/// A ClaimResponse is typically returned in reply to a Claim request.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::claim_response::ClaimResponse;
///
/// let value = ClaimResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClaimResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
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

    /// Business Identifier for a claim response
    pub identifier: Option<Vec<types::Identifier>>,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// active | cancelled | draft | entered-in-error
    pub status: types::Code,

    /// More granular claim type
    pub r#type: types::CodeableConcept,

    /// More granular claim type
    pub sub_type: Option<types::CodeableConcept>,

    /// claim | preauthorization | predetermination
    pub r#use: types::Code,

    /// The recipient of the products and services
    pub patient: types::Reference,

    /// Response creation date
    pub created: types::DateTime,

    /// Party responsible for reimbursement
    pub insurer: Option<types::Reference>,

    /// Party responsible for the claim
    pub requestor: Option<types::Reference>,

    /// Id of resource triggering adjudication
    pub request: Option<types::Reference>,

    /// queued | complete | error | partial
    pub outcome: types::Code,

    /// Result of the adjudication
    pub decision: Option<types::CodeableConcept>,

    /// Disposition Message
    pub disposition: Option<types::String>,

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,

    /// Preauthorization reference effective period
    pub pre_auth_period: Option<types::Period>,

    /// Event information
    pub event: Option<Vec<ClaimResponseEvent>>,

    /// Party to be paid any benefits payable
    pub payee_type: Option<types::CodeableConcept>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<types::Reference>>,

    /// Package billing code
    pub diagnosis_related_group: Option<types::CodeableConcept>,

    /// Adjudication for claim line items
    pub item: Option<Vec<ClaimResponseItem>>,

    /// Insurer added line items
    pub add_item: Option<Vec<ClaimResponseAddItem>>,

    /// Header-level adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Adjudication totals
    pub total: Option<Vec<ClaimResponseTotal>>,

    /// Payment Details
    pub payment: Option<ClaimResponsePayment>,

    /// Funds reserved status
    pub funds_reserve: Option<types::CodeableConcept>,

    /// Printed form identifier
    pub form_code: Option<types::CodeableConcept>,

    /// Printed reference or actual form
    pub form: Option<types::Attachment>,

    /// Note concerning adjudication
    pub process_note: Option<Vec<ClaimResponseProcessNote>>,

    /// Request for additional information
    pub communication_request: Option<Vec<types::Reference>>,

    /// Patient insurance information
    pub insurance: Option<Vec<ClaimResponseInsurance>>,

    /// Processing errors
    pub error: Option<Vec<ClaimResponseError>>,
}

/// Event information for the ClaimResponse.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseEvent {
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

/// Adjudication for claim line items.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Claim item instance identifier
    pub item_sequence: types::PositiveInt,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,

    /// Adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Adjudication details
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Adjudication for claim details
    pub detail: Option<Vec<ClaimResponseItemDetail>>,
}

/// Adjudication results for a claim item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemReviewOutcome {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Result of the adjudication
    pub decision: Option<types::CodeableConcept>,

    /// Reason for result of the adjudication
    pub reason: Option<Vec<types::CodeableConcept>>,

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,

    /// Preauthorization reference effective period
    pub pre_auth_period: Option<types::Period>,
}

/// Adjudication details for a claim item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemAdjudication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of adjudication information
    pub category: types::CodeableConcept,

    /// Explanation of adjudication outcome
    pub reason: Option<types::CodeableConcept>,

    /// Monetary amount
    pub amount: Option<types::Money>,

    /// Non-monetary value
    pub quantity: Option<types::Quantity>,
}

/// Adjudication for claim details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Claim detail instance identifier
    pub detail_sequence: types::PositiveInt,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,

    /// Detail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Detail level adjudication details
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Adjudication for claim sub-details
    pub sub_detail: Option<Vec<ClaimResponseItemDetailSubDetail>>,
}

/// Adjudication for claim sub-details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Claim sub-detail instance identifier
    pub sub_detail_sequence: types::PositiveInt,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,

    /// Subdetail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Subdetail level adjudication details
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,
}

/// Insurer added line items.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Item sequence number
    pub item_sequence: Option<Vec<types::PositiveInt>>,

    /// Detail sequence number
    pub detail_sequence: Option<Vec<types::PositiveInt>>,

    /// Subdetail sequence number
    pub subdetail_sequence: Option<Vec<types::PositiveInt>>,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// Authorized providers
    pub provider: Option<Vec<types::Reference>>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,

    /// Request or Referral for Service
    pub request: Option<Vec<types::Reference>>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<types::CodeableConcept>>,

    /// Program the product or service is provided under
    pub program_code: Option<Vec<types::CodeableConcept>>,

    /// Date or dates of service or product delivery
    pub serviced_date: Option<types::Date>,

    /// Date or dates of service or product delivery
    pub serviced_period: Option<types::Period>,

    /// Place of service or where product was supplied
    pub location_codeable_concept: Option<types::CodeableConcept>,

    /// Place of service or where product was supplied
    pub location_address: Option<types::Address>,

    /// Place of service or where product was supplied
    pub location_reference: Option<types::Reference>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,

    /// Total tax
    pub tax: Option<types::Money>,

    /// Total item cost
    pub net: Option<types::Money>,

    /// Anatomical location
    pub body_site: Option<Vec<ClaimResponseAddItemBodySite>>,

    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,

    /// Added items adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Insurer added line details
    pub detail: Option<Vec<ClaimResponseAddItemDetail>>,
}

/// Anatomical location for an insurer added line item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItemBodySite {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Location
    pub site: Vec<types::CodeableReference>,

    /// Sub-location
    pub sub_site: Option<Vec<types::CodeableConcept>>,
}

/// Insurer added line details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<types::CodeableConcept>>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,

    /// Total tax
    pub tax: Option<types::Money>,

    /// Total item cost
    pub net: Option<types::Money>,

    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,

    /// Added items detail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items detail adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Insurer added line items
    pub sub_detail: Option<Vec<ClaimResponseAddItemDetailSubDetail>>,
}

/// Insurer added line sub-details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<types::CodeableConcept>>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,

    /// Total tax
    pub tax: Option<types::Money>,

    /// Total item cost
    pub net: Option<types::Money>,

    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,

    /// Added items subdetail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items subdetail adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,
}

/// Adjudication totals.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseTotal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of adjudication information
    pub category: types::CodeableConcept,

    /// Financial total for the category
    pub amount: types::Money,
}

/// Payment Details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponsePayment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Partial or complete payment
    pub r#type: types::CodeableConcept,

    /// Payment adjustment for non-claim issues
    pub adjustment: Option<types::Money>,

    /// Explanation for the adjustment
    pub adjustment_reason: Option<types::CodeableConcept>,

    /// Expected date of payment
    pub date: Option<types::Date>,

    /// Payable amount after adjustment
    pub amount: types::Money,

    /// Business identifier for the payment
    pub identifier: Option<types::Identifier>,
}

/// Note concerning adjudication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseProcessNote {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Note instance identifier
    pub number: Option<types::PositiveInt>,

    /// Note purpose
    pub r#type: Option<types::CodeableConcept>,

    /// Note explanatory text
    pub text: types::String,

    /// Language of the text
    pub language: Option<types::CodeableConcept>,
}

/// Patient insurance information.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Insurance instance identifier
    pub sequence: types::PositiveInt,

    /// Coverage to be used for adjudication
    pub focal: types::Boolean,

    /// Insurance information
    pub coverage: types::Reference,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,

    /// Adjudication results
    pub claim_response: Option<types::Reference>,
}

/// Processing errors.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseError {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Item sequence number
    pub item_sequence: Option<types::PositiveInt>,

    /// Detail sequence number
    pub detail_sequence: Option<types::PositiveInt>,

    /// Subdetail sequence number
    pub sub_detail_sequence: Option<types::PositiveInt>,

    /// Error code detailing processing issues
    pub code: types::CodeableConcept,

    /// FHIRPath of element(s) related to issue
    pub expression: Option<Vec<types::String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ClaimResponse;

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
