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
    /// Business Identifier for the resource
    pub identifier: Option<Vec<types::Identifier>>,
    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,
    /// The status of the resource instance: active | cancelled | draft | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::ExplanationofbenefitStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,
    /// The category or discipline of the claim, e.g. institutional, oral, pharmacy, professional, or vision
    pub r#type: types::CodeableConcept,
    /// More granular claim type
    pub sub_type: Option<types::CodeableConcept>,
    /// Whether this represents a claim, a preauthorization request, or a predetermination
    pub r#use: crate::r5::coded::Coded<crate::r5::codes::ClaimUse>,
    /// Primitive extension sibling for [`use`](Self::r#use) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,
    /// Reference to the patient who is the recipient of the products and services
    pub patient: types::Reference,
    /// Relevant time frame for the claim
    pub billable_period: Option<types::Period>,
    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,
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
    pub outcome: crate::r5::coded::Coded<crate::r5::codes::ClaimOutcome>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`).
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,
    /// Result of the adjudication
    pub decision: Option<types::CodeableConcept>,
    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`).
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,
    /// Preauthorization reference
    pub pre_auth_ref: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`precedence`](Self::precedence) (FHIR `_precedence`).
    #[serde(rename = "_precedence")]
    pub precedence_ext: Option<types::Element>,
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
    /// The `ExplanationOfBenefit.event.when[x]` choice element (0..1); see [`ExplanationOfBenefitEventWhen`].
    #[serde(flatten)]
    pub when: Option<ExplanationOfBenefitEventWhen>,
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Practitioner or organization
    pub provider: types::Reference,
    /// Indicator of the lead practitioner
    pub responsible: Option<types::Boolean>,
    /// Primitive extension sibling for [`responsible`](Self::responsible) (FHIR `_responsible`).
    #[serde(rename = "_responsible")]
    pub responsible_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Classification of the supplied information
    pub category: types::CodeableConcept,
    /// Type of information
    pub code: Option<types::CodeableConcept>,
    /// The `ExplanationOfBenefit.supportingInfo.timing[x]` choice element (0..1); see [`ExplanationOfBenefitSupportingInfoTiming`].
    #[serde(flatten)]
    pub timing: Option<ExplanationOfBenefitSupportingInfoTiming>,
    /// The `ExplanationOfBenefit.supportingInfo.value[x]` choice element (0..1); see [`ExplanationOfBenefitSupportingInfoValue`].
    #[serde(flatten)]
    pub value: Option<ExplanationOfBenefitSupportingInfoValue>,
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (0..1); see [`ExplanationOfBenefitDiagnosisDiagnosis`].
    #[serde(flatten)]
    pub diagnosis: Option<ExplanationOfBenefitDiagnosisDiagnosis>,
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Category of Procedure
    pub r#type: Option<Vec<types::CodeableConcept>>,
    /// When the procedure was performed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
    /// The `ExplanationOfBenefit.procedure.procedure[x]` choice element (0..1); see [`ExplanationOfBenefitProcedureProcedure`].
    #[serde(flatten)]
    pub procedure: Option<ExplanationOfBenefitProcedureProcedure>,
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
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`).
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,
    /// Insurance information
    pub coverage: types::Reference,
    /// Prior authorization reference number
    pub pre_auth_ref: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,
    /// The `ExplanationOfBenefit.accident.location[x]` choice element (0..1); see [`ExplanationOfBenefitAccidentLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitAccidentLocation>,
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Applicable care team members
    pub care_team_sequence: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`care_team_sequence`](Self::care_team_sequence) (FHIR `_careTeamSequence`).
    #[serde(rename = "_careTeamSequence")]
    pub care_team_sequence_ext: Option<Vec<Option<types::Element>>>,
    /// Applicable diagnoses
    pub diagnosis_sequence: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`diagnosis_sequence`](Self::diagnosis_sequence) (FHIR `_diagnosisSequence`).
    #[serde(rename = "_diagnosisSequence")]
    pub diagnosis_sequence_ext: Option<Vec<Option<types::Element>>>,
    /// Applicable procedures
    pub procedure_sequence: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`procedure_sequence`](Self::procedure_sequence) (FHIR `_procedureSequence`).
    #[serde(rename = "_procedureSequence")]
    pub procedure_sequence_ext: Option<Vec<Option<types::Element>>>,
    /// Applicable exception and supporting information
    pub information_sequence: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`information_sequence`](Self::information_sequence) (FHIR `_informationSequence`).
    #[serde(rename = "_informationSequence")]
    pub information_sequence_ext: Option<Vec<Option<types::Element>>>,
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
    /// The `ExplanationOfBenefit.item.serviced[x]` choice element (0..1); see [`ExplanationOfBenefitItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ExplanationOfBenefitItemServiced>,
    /// The `ExplanationOfBenefit.item.location[x]` choice element (0..1); see [`ExplanationOfBenefitItemLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitItemLocation>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    pub note_number_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Unique device identifier
    pub udi: Option<Vec<types::Reference>>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    pub note_number_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Unique device identifier
    pub udi: Option<Vec<types::Reference>>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    pub note_number_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`item_sequence`](Self::item_sequence) (FHIR `_itemSequence`).
    #[serde(rename = "_itemSequence")]
    pub item_sequence_ext: Option<Vec<Option<types::Element>>>,
    /// Detail sequence number
    pub detail_sequence: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`detail_sequence`](Self::detail_sequence) (FHIR `_detailSequence`).
    #[serde(rename = "_detailSequence")]
    pub detail_sequence_ext: Option<Vec<Option<types::Element>>>,
    /// Subdetail sequence number
    pub sub_detail_sequence: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`sub_detail_sequence`](Self::sub_detail_sequence) (FHIR `_subDetailSequence`).
    #[serde(rename = "_subDetailSequence")]
    pub sub_detail_sequence_ext: Option<Vec<Option<types::Element>>>,
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
    /// The `ExplanationOfBenefit.addItem.serviced[x]` choice element (0..1); see [`ExplanationOfBenefitAddItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ExplanationOfBenefitAddItemServiced>,
    /// The `ExplanationOfBenefit.addItem.location[x]` choice element (0..1); see [`ExplanationOfBenefitAddItemLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitAddItemLocation>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Anatomical location
    pub body_site: Option<Vec<ExplanationOfBenefitAddItemBodySite>>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    pub note_number_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    pub note_number_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Applicable note numbers
    pub note_number: Option<Vec<types::PositiveInt>>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    pub note_number_ext: Option<Vec<Option<types::Element>>>,
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`).
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,
    /// Note purpose
    pub r#type: Option<types::CodeableConcept>,
    /// Note explanatory text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`excluded`](Self::excluded) (FHIR `_excluded`).
    #[serde(rename = "_excluded")]
    pub excluded_ext: Option<types::Element>,
    /// Short name for the benefit
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
    /// Description of the benefit or services covered
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
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
    /// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialAllowed`].
    #[serde(flatten)]
    pub allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,
    /// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialUsed`].
    #[serde(flatten)]
    pub used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
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
/// The `ExplanationOfBenefit.accident.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAccidentLocation {
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.addItem.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAddItemLocation {
    /// `locationCodeableConcept` variant.
    #[fhir("locationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.addItem.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAddItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    /// `allowedUnsignedInt` variant.
    #[fhir("allowedUnsignedInt")]
    UnsignedInt(crate::r5::choice::Primitive<types::UnsignedInt>),
    /// `allowedString` variant.
    #[fhir("allowedString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `allowedMoney` variant.
    #[fhir("allowedMoney")]
    Money(Box<types::Money>),
}

/// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    /// `usedUnsignedInt` variant.
    #[fhir("usedUnsignedInt")]
    UnsignedInt(crate::r5::choice::Primitive<types::UnsignedInt>),
    /// `usedMoney` variant.
    #[fhir("usedMoney")]
    Money(Box<types::Money>),
}

/// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.event.when[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.item.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitItemLocation {
    /// `locationCodeableConcept` variant.
    #[fhir("locationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.item.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.procedure.procedure[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitProcedureProcedure {
    /// `procedureCodeableConcept` variant.
    #[fhir("procedureCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `procedureReference` variant.
    #[fhir("procedureReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.supportingInfo.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitSupportingInfoTiming {
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.supportingInfo.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitSupportingInfoValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueIdentifier` variant.
    #[fhir("valueIdentifier")]
    Identifier(Box<types::Identifier>),
}
