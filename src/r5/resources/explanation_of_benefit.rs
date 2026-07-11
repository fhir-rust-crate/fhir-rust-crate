//! ExplanationOfBenefit
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
//!
//! Version: 5.0.0
//!
//! ExplanationOfBenefit Resource: the claim details, adjudication details from the processing of a Claim, and optionally account balance information.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for
/// informing the subscriber of the benefits provided.
///
/// An ExplanationOfBenefit is the response to a Claim and combines information
/// from the original claim request with the payer's adjudication results,
/// making it central to the FHIR R5 financial and billing workflow. It is
/// typically produced by an insurer, health plan, or other adjudicating
/// system after processing a Claim, and it is returned to the subscriber,
/// patient, or provider so they can understand what services or products
/// were billed, how each line item was evaluated, and what amounts (if
/// any) are payable, denied, or the patient's responsibility. Unlike a
/// Claim, which represents a request for payment or authorization, the
/// ExplanationOfBenefit represents the outcome of that request, including
/// per-item and per-category adjudication, totals, payment information,
/// and optional account balance data such as benefit limits already used.
///
/// The resource mirrors much of the structure of the Claim it responds to
/// (items, details, sub-details, diagnoses, procedures, and insurance),
/// while adding adjudication-specific elements such as `adjudication`,
/// `total`, `payment`, and `benefit_balance` that capture the payer's
/// determination for each element of the original request.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject
///   referenced by the `patient` field.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used
///   throughout for coded categories such as status, type, and adjudication
///   categories.
/// - `Claim` and `ClaimResponse` — related resources representing the
///   original request and the underlying adjudication response.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefit;
///
/// let value = ExplanationOfBenefit::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ExplanationOfBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefit {
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
    /// Business Identifier for the resource
    pub identifier: Option<Vec<types::Identifier>>,
    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,
    /// The status of the resource instance: active | cancelled | draft | entered-in-error
    pub status: types::Code,
    /// The category or discipline of the claim, e.g. institutional, oral, pharmacy, professional, or vision
    pub r#type: types::CodeableConcept,
    /// More granular claim type
    pub sub_type: Option<types::CodeableConcept>,
    /// Whether this represents a claim, a preauthorization request, or a predetermination
    pub r#use: types::Code,
    /// Reference to the patient who is the recipient of the products and services
    pub patient: types::Reference,
    /// Relevant time frame for the claim
    pub billable_period: Option<types::Period>,
    /// Response creation date
    pub created: types::DateTime,
    /// Author of the claim
    pub enterer: Option<types::Reference>,
    /// Party responsible for reimbursement
    pub insurer: Option<types::Reference>,
    /// Party responsible for the claim
    pub provider: Option<types::Reference>,
    /// Desired processing urgency
    pub priority: Option<types::CodeableConcept>,
    /// For whom to reserve funds
    pub funds_reserve_requested: Option<types::CodeableConcept>,
    /// Funds reserved status
    pub funds_reserve: Option<types::CodeableConcept>,
    /// Prior or corollary claims
    pub related: Option<Vec<ExplanationOfBenefitRelated>>,
    /// Prescription authorizing services or products
    pub prescription: Option<types::Reference>,
    /// Original prescription if superceded by fulfiller
    pub original_prescription: Option<types::Reference>,
    /// Event information
    pub event: Option<Vec<ExplanationOfBenefitEvent>>,
    /// Recipient of benefits payable
    pub payee: Option<ExplanationOfBenefitPayee>,
    /// Treatment Referral
    pub referral: Option<types::Reference>,
    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<types::Reference>>,
    /// Servicing Facility
    pub facility: Option<types::Reference>,
    /// Reference to the Claim resource that this ExplanationOfBenefit is the outcome of
    pub claim: Option<types::Reference>,
    /// Claim response reference
    pub claim_response: Option<types::Reference>,
    /// The processing outcome of the adjudication: queued | complete | error | partial
    pub outcome: types::Code,
    /// Result of the adjudication
    pub decision: Option<types::CodeableConcept>,
    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Preauthorization reference
    pub pre_auth_ref: Option<Vec<types::String>>,
    /// Preauthorization in-effect period
    pub pre_auth_ref_period: Option<Vec<types::Period>>,
    /// Package billing code
    pub diagnosis_related_group: Option<types::CodeableConcept>,
    /// Care Team members
    pub care_team: Option<Vec<ExplanationOfBenefitCareTeam>>,
    /// Supporting information
    pub supporting_info: Option<Vec<ExplanationOfBenefitSupportingInfo>>,
    /// Pertinent diagnosis information
    pub diagnosis: Option<Vec<ExplanationOfBenefitDiagnosis>>,
    /// Clinical procedures performed
    pub procedure: Option<Vec<ExplanationOfBenefitProcedure>>,
    /// Precedence (primary, secondary, etc.)
    pub precedence: Option<types::PositiveInt>,
    /// Patient insurance information
    pub insurance: Option<Vec<ExplanationOfBenefitInsurance>>,
    /// Details of the event
    pub accident: Option<ExplanationOfBenefitAccident>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Product or service provided
    pub item: Option<Vec<ExplanationOfBenefitItem>>,
    /// Insurer added line items
    pub add_item: Option<Vec<ExplanationOfBenefitAddItem>>,
    /// Header-level adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
    /// Adjudication totals
    pub total: Option<Vec<ExplanationOfBenefitTotal>>,
    /// Payment Details
    pub payment: Option<ExplanationOfBenefitPayment>,
    /// Printed form identifier
    pub form_code: Option<types::CodeableConcept>,
    /// Printed reference or actual form
    pub form: Option<types::Attachment>,
    /// Note concerning adjudication
    pub process_note: Option<Vec<ExplanationOfBenefitProcessNote>>,
    /// When the benefits are applicable
    pub benefit_period: Option<types::Period>,
    /// Balance by Benefit Category
    pub benefit_balance: Option<Vec<ExplanationOfBenefitBenefitBalance>>,
}

/// Prior or corollary claims related to this ExplanationOfBenefit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitRelated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Reference to the related claim
    pub claim: Option<types::Reference>,
    /// How the reference claim is related
    pub relationship: Option<types::CodeableConcept>,
    /// File or case reference
    pub reference: Option<types::Identifier>,
}

/// Event information associated with the ExplanationOfBenefit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitEvent {
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

/// Recipient of the benefits payable.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitPayee {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Category of recipient
    pub r#type: Option<types::CodeableConcept>,
    /// Recipient reference
    pub party: Option<types::Reference>,
}

/// Members of the care team who provided the products and services.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitCareTeam {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Order of care team
    pub sequence: types::PositiveInt,
    /// Practitioner or organization
    pub provider: types::Reference,
    /// Indicator of the lead practitioner
    pub responsible: Option<types::Boolean>,
    /// Function within the team
    pub role: Option<types::CodeableConcept>,
    /// Practitioner or provider specialization
    pub specialty: Option<types::CodeableConcept>,
}

/// Supporting information regarding the claim.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Information instance identifier
    pub sequence: types::PositiveInt,
    /// Classification of the supplied information
    pub category: types::CodeableConcept,
    /// Type of information
    pub code: Option<types::CodeableConcept>,
    /// When it occurred
    pub timing_date: Option<types::Date>,
    /// When it occurred
    pub timing_period: Option<types::Period>,
    /// Data to be provided
    pub value_boolean: Option<types::Boolean>,
    /// Data to be provided
    pub value_string: Option<types::String>,
    /// Data to be provided
    pub value_quantity: Option<types::Quantity>,
    /// Data to be provided
    pub value_attachment: Option<types::Attachment>,
    /// Data to be provided
    pub value_reference: Option<types::Reference>,
    /// Data to be provided
    pub value_identifier: Option<types::Identifier>,
    /// Explanation for the information
    pub reason: Option<types::Coding>,
}

/// Pertinent diagnosis information.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Diagnosis instance identifier
    pub sequence: types::PositiveInt,
    /// Nature of illness or problem
    pub diagnosis_codeable_concept: Option<types::CodeableConcept>,
    /// Nature of illness or problem
    pub diagnosis_reference: Option<types::Reference>,
    /// Timing or nature of the diagnosis
    pub r#type: Option<Vec<types::CodeableConcept>>,
    /// Present on admission
    pub on_admission: Option<types::CodeableConcept>,
}

/// Clinical procedures performed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Procedure instance identifier
    pub sequence: types::PositiveInt,
    /// Category of Procedure
    pub r#type: Option<Vec<types::CodeableConcept>>,
    /// When the procedure was performed
    pub date: Option<types::DateTime>,
    /// Specific clinical procedure
    pub procedure_codeable_concept: Option<types::CodeableConcept>,
    /// Specific clinical procedure
    pub procedure_reference: Option<types::Reference>,
    /// Unique device identifier
    pub udi: Option<Vec<types::Reference>>,
}

/// Patient insurance information for adjudication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Coverage to be used for adjudication
    pub focal: types::Boolean,
    /// Insurance information
    pub coverage: types::Reference,
    /// Prior authorization reference number
    pub pre_auth_ref: Option<Vec<types::String>>,
}

/// Details of an accident event that triggered the claim.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAccident {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// When the incident occurred
    pub date: Option<types::Date>,
    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,
    /// Where the event occurred
    pub location_address: Option<types::Address>,
    /// Where the event occurred
    pub location_reference: Option<types::Reference>,
}

/// Product or service line item provided.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Item instance identifier
    pub sequence: types::PositiveInt,
    /// Applicable care team members
    pub care_team_sequence: Option<Vec<types::PositiveInt>>,
    /// Applicable diagnoses
    pub diagnosis_sequence: Option<Vec<types::PositiveInt>>,
    /// Applicable procedures
    pub procedure_sequence: Option<Vec<types::PositiveInt>>,
    /// Applicable exception and supporting information
    pub information_sequence: Option<Vec<types::PositiveInt>>,
    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,
    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,
    /// Benefit classification
    pub category: Option<types::CodeableConcept>,
    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,
    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,
    /// Request or Referral for Service
    pub request: Option<Vec<types::Reference>>,
    /// Product or service billing modifiers
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
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
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
    /// Unique device identifier
    pub udi: Option<Vec<types::Reference>>,
    /// Anatomical location
    pub body_site: Option<Vec<ExplanationOfBenefitItemBodySite>>,
    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<types::Reference>>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Adjudication details
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
    /// Additional items
    pub detail: Option<Vec<ExplanationOfBenefitItemDetail>>,
}

/// Anatomical location for an item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemBodySite {
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

/// Adjudication review outcome for an item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemReviewOutcome {
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

/// Adjudication details for an item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemAdjudication {
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
    /// Non-monitary value
    pub quantity: Option<types::Quantity>,
}

/// Additional detail items under an item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Product or service provided
    pub sequence: types::PositiveInt,
    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,
    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,
    /// Benefit classification
    pub category: Option<types::CodeableConcept>,
    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,
    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,
    /// Service/Product billing modifiers
    pub modifier: Option<Vec<types::CodeableConcept>>,
    /// Program the product or service is provided under
    pub program_code: Option<Vec<types::CodeableConcept>>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
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
    /// Unique device identifier
    pub udi: Option<Vec<types::Reference>>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Detail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Detail level adjudication details
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
    /// Additional items
    pub sub_detail: Option<Vec<ExplanationOfBenefitItemDetailSubDetail>>,
}

/// Additional sub-detail items under a detail.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Product or service provided
    pub sequence: types::PositiveInt,
    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,
    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,
    /// Benefit classification
    pub category: Option<types::CodeableConcept>,
    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,
    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,
    /// Service/Product billing modifiers
    pub modifier: Option<Vec<types::CodeableConcept>>,
    /// Program the product or service is provided under
    pub program_code: Option<Vec<types::CodeableConcept>>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
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
    /// Unique device identifier
    pub udi: Option<Vec<types::Reference>>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Subdetail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Subdetail level adjudication details
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
}

/// Insurer added line items.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItem {
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
    pub sub_detail_sequence: Option<Vec<types::PositiveInt>>,
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
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
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
    pub body_site: Option<Vec<ExplanationOfBenefitAddItemBodySite>>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Additem level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Added items adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
    /// Insurer added line items
    pub detail: Option<Vec<ExplanationOfBenefitAddItemDetail>>,
}

/// Anatomical location for an insurer-added item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItemBodySite {
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

/// Insurer added detail items under an add item.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItemDetail {
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
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
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
    /// Additem detail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Added items adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
    /// Insurer added line items
    pub sub_detail: Option<Vec<ExplanationOfBenefitAddItemDetailSubDetail>>,
}

/// Insurer added sub-detail items under an add item detail.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
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
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
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
    /// Additem subdetail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Added items adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
}

/// Adjudication totals per category.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitTotal {
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

/// Payment details for the adjudicated claim.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitPayment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Partial or complete payment
    pub r#type: Option<types::CodeableConcept>,
    /// Payment adjustment for non-claim issues
    pub adjustment: Option<types::Money>,
    /// Explanation for the variance
    pub adjustment_reason: Option<types::CodeableConcept>,
    /// Expected date of payment
    pub date: Option<types::Date>,
    /// Payable amount after adjustment
    pub amount: Option<types::Money>,
    /// Business identifier for the payment
    pub identifier: Option<types::Identifier>,
}

/// Note concerning adjudication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitProcessNote {
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
    pub text: Option<types::String>,
    /// Language of the text
    pub language: Option<types::CodeableConcept>,
}

/// Balance by benefit category.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitBenefitBalance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
    /// Benefit classification
    pub category: types::CodeableConcept,
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
    pub financial: Option<Vec<ExplanationOfBenefitBenefitBalanceFinancial>>,
}

/// Benefit summary detail for a benefit balance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
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
    pub used_money: Option<types::Money>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExplanationOfBenefit;

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
