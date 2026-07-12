//! Claim
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Claim
//!
//! Version: 5.0.0
//!
//! Claim Resource: A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement. In FHIR R5 the Claim resource requests adjudication, pre-authorization, or pre-determination of health, dental, pharmacy, vision, or professional services from an insurer. It carries the billable items, supporting information, diagnoses, procedures, and coverage details needed for processing.
///
/// A `Claim` is submitted by a provider or provider organization to a payer (insurer) to request payment,
/// pre-authorization, or pre-determination for services and products rendered, or to be rendered, to a
/// patient. The resource captures the parties involved (patient, provider, insurer, payee), the clinical
/// context (diagnoses, procedures, accident details, care team), the billable line items with their
/// pricing, and any supporting information or related claims needed by the adjudicator. The `use` element
/// distinguishes an actual claim submission from a preauthorization or predetermination request, while
/// `status` tracks the lifecycle of the claim resource itself (not the adjudication outcome, which is
/// represented separately by the `ClaimResponse` resource).
///
/// Related resources / See also: the subject of a claim is typically a [`Patient`](crate::r5::resources::patient::Patient);
/// coded values throughout the resource (such as `type`, `category`, and `product_or_service`) use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept); monetary amounts use `Money`; and the adjudication
/// result for a claim is returned in a corresponding `ClaimResponse` resource.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::claim::Claim;
///
/// let value = Claim::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Claim = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
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

    /// Business Identifier for claim
    pub identifier: Option<Vec<types::Identifier>>,

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// The status of the Claim resource itself: active | cancelled | draft | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The category or discipline of the claim, e.g. institutional, oral, pharmacy, professional, or vision
    pub r#type: types::CodeableConcept,

    /// More granular claim type
    pub sub_type: Option<types::CodeableConcept>,

    /// Indicates the intent of the request: claim | preauthorization | predetermination
    pub r#use: crate::r5::coded::Coded<crate::r5::codes::ClaimUse>,
    /// Primitive extension sibling for [`use`](Self::r#use) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// The [`Patient`](crate::r5::resources::patient::Patient) who is the recipient of the products and services
    pub patient: types::Reference,

    /// Relevant time frame for the claim
    pub billable_period: Option<types::Period>,

    /// Resource creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Author of the claim
    pub enterer: Option<types::Reference>,

    /// The insurer (payer) who is the target of the claim submission
    pub insurer: Option<types::Reference>,

    /// The practitioner or organization responsible for the claim
    pub provider: Option<types::Reference>,

    /// Desired processing urgency
    pub priority: Option<types::CodeableConcept>,

    /// For whom to reserve funds
    pub funds_reserve: Option<types::CodeableConcept>,

    /// Prior or corollary claims
    pub related: Option<Vec<ClaimRelated>>,

    /// Prescription authorizing services and products
    pub prescription: Option<types::Reference>,

    /// Original prescription if superseded by fulfiller
    pub original_prescription: Option<types::Reference>,

    /// Recipient of benefits payable
    pub payee: Option<ClaimPayee>,

    /// Treatment referral
    pub referral: Option<types::Reference>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<types::Reference>>,

    /// Servicing facility
    pub facility: Option<types::Reference>,

    /// Package billing code
    pub diagnosis_related_group: Option<types::CodeableConcept>,

    /// Event information
    pub event: Option<Vec<ClaimEvent>>,

    /// Members of the care team
    pub care_team: Option<Vec<ClaimCareTeam>>,

    /// Supporting information
    pub supporting_info: Option<Vec<ClaimSupportingInfo>>,

    /// Pertinent diagnosis information
    pub diagnosis: Option<Vec<ClaimDiagnosis>>,

    /// Clinical procedures performed
    pub procedure: Option<Vec<ClaimProcedure>>,

    /// Patient insurance information
    pub insurance: Option<Vec<ClaimInsurance>>,

    /// Details of the event
    pub accident: Option<ClaimAccident>,

    /// Paid by the patient
    pub patient_paid: Option<types::Money>,

    /// The individual line items of products or services being claimed for
    pub item: Option<Vec<ClaimItem>>,

    /// The total cost of the claim, summed across all line items
    pub total: Option<types::Money>,
}

/// Claim nested backbone element `Claim.related`. Prior or corollary claims
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRelated {
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

/// Claim nested backbone element `Claim.payee`. Recipient of benefits payable
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimPayee {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Category of recipient
    pub r#type: types::CodeableConcept,

    /// Recipient reference
    pub party: Option<types::Reference>,
}

/// Claim nested backbone element `Claim.event`. Event information
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Specific event
    pub r#type: types::CodeableConcept,

    /// The `Claim.event.when[x]` choice element (0..1); see [`ClaimEventWhen`].
    #[serde(flatten)]
    pub when: Option<ClaimEventWhen>,
}

/// Claim nested backbone element `Claim.careTeam`. Members of the care team
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimCareTeam {
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

/// Claim nested backbone element `Claim.supportingInfo`. Supporting information
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimSupportingInfo {
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

    /// The `Claim.supportingInfo.timing[x]` choice element (0..1); see [`ClaimSupportingInfoTiming`].
    #[serde(flatten)]
    pub timing: Option<ClaimSupportingInfoTiming>,

    /// The `Claim.supportingInfo.value[x]` choice element (0..1); see [`ClaimSupportingInfoValue`].
    #[serde(flatten)]
    pub value: Option<ClaimSupportingInfoValue>,

    /// Explanation for the information
    pub reason: Option<types::CodeableConcept>,
}

/// Claim nested backbone element `Claim.diagnosis`. Pertinent diagnosis information
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimDiagnosis {
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

    /// The `Claim.diagnosis.diagnosis[x]` choice element (0..1); see [`ClaimDiagnosisDiagnosis`].
    #[serde(flatten)]
    pub diagnosis: Option<ClaimDiagnosisDiagnosis>,

    /// Timing or nature of the diagnosis
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Present on admission
    pub on_admission: Option<types::CodeableConcept>,
}

/// Claim nested backbone element `Claim.procedure`. Clinical procedures performed
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimProcedure {
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

    /// The `Claim.procedure.procedure[x]` choice element (0..1); see [`ClaimProcedureProcedure`].
    #[serde(flatten)]
    pub procedure: Option<ClaimProcedureProcedure>,

    /// Unique device identifier
    pub udi: Option<Vec<types::Reference>>,
}

/// Claim nested backbone element `Claim.insurance`. Patient insurance information
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Insurance instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Coverage to be used for adjudication
    pub focal: types::Boolean,
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`).
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,

    /// Pre-assigned Claim number
    pub identifier: Option<types::Identifier>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`).
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,

    /// Prior authorization reference number
    pub pre_auth_ref: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<Vec<Option<types::Element>>>,

    /// Adjudication results
    pub claim_response: Option<types::Reference>,
}

/// Claim nested backbone element `Claim.accident`. Details of the event
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimAccident {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// When the incident occurred
    pub date: types::Date,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,

    /// The `Claim.accident.location[x]` choice element (0..1); see [`ClaimAccidentLocation`].
    #[serde(flatten)]
    pub location: Option<ClaimAccidentLocation>,
}

/// Claim nested backbone element `Claim.item`. Product or service provided
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItem {
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

    /// Number for tracking
    pub trace_number: Option<Vec<types::Identifier>>,

    /// Applicable careTeam members
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

    /// The `Claim.item.serviced[x]` choice element (0..1); see [`ClaimItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ClaimItemServiced>,

    /// The `Claim.item.location[x]` choice element (0..1); see [`ClaimItemLocation`].
    #[serde(flatten)]
    pub location: Option<ClaimItemLocation>,

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
    pub body_site: Option<Vec<ClaimItemBodySite>>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<types::Reference>>,

    /// Product or service provided
    pub detail: Option<Vec<ClaimItemDetail>>,
}

/// Claim nested backbone element `Claim.item.bodySite`. Anatomical location
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItemBodySite {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Location
    pub site: vec1::Vec1<types::CodeableReference>,

    /// Sub-location
    pub sub_site: Option<Vec<types::CodeableConcept>>,
}

/// Claim nested backbone element `Claim.item.detail`. Product or service provided
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItemDetail {
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

    /// Product or service provided
    pub sub_detail: Option<Vec<ClaimItemDetailSubDetail>>,
}

/// Claim nested backbone element `Claim.item.detail.subDetail`. Product or service provided
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItemDetailSubDetail {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Claim;

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
/// The `Claim.accident.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimAccidentLocation {
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `Claim.diagnosis.diagnosis[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `Claim.event.when[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
}

/// The `Claim.item.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimItemLocation {
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

/// The `Claim.item.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `Claim.procedure.procedure[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimProcedureProcedure {
    /// `procedureCodeableConcept` variant.
    #[fhir("procedureCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `procedureReference` variant.
    #[fhir("procedureReference")]
    Reference(Box<types::Reference>),
}

/// The `Claim.supportingInfo.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimSupportingInfoTiming {
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
}

/// The `Claim.supportingInfo.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimSupportingInfoValue {
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