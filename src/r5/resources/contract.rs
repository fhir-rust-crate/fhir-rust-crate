//! Contract
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Contract
//!
//! Version: 5.0.0
//!
//! Contract Resource: Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Contract Resource.
///
/// Legally enforceable, formally recorded unilateral or bilateral directive
/// i.e., a policy or agreement. In FHIR R5 the Contract resource captures the
/// terms of a legal agreement between parties, including the parties involved,
/// the assets and actions covered, security labels, and human-friendly and
/// machine-computable representations of the agreement.
///
/// A Contract is used to record the full lifecycle of an agreement: its
/// authoring, the offer and acceptance between parties, the assets or
/// services in scope, the actions each party is obligated (or forbidden) to
/// perform, and the signatures that bind it. Typical uses include consent
/// directives, data sharing and privacy agreements, insurance policies,
/// service level agreements, and other legal or administrative contracts
/// that link real-world parties to computable and human-readable terms.
///
/// # Related resources
///
/// The parties, subjects, and supporting evidence referenced by a Contract
/// are typically other resources such as
/// [`Patient`](crate::r5::resources::patient::Patient),
/// `Organization`, `Practitioner`, or `RelatedPerson`, connected via
/// [`Reference`](crate::r5::types::Reference) elements. Terms and offers are
/// classified using [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// and signatures use [`Signature`](crate::r5::types::Signature).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::contract::Contract;
///
/// let value = Contract::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Contract = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
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

    /// Contract number
    pub identifier: Option<Vec<types::Identifier>>,

    /// Basal definition
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business edition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Current lifecycle status of the contract, e.g. amended | appended | cancelled | disputed | entered-in-error | executable +
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::ContractStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Negotiation status
    pub legal_state: Option<types::CodeableConcept>,

    /// Source Contract Definition
    pub instantiates_canonical: Option<types::Reference>,

    /// External Contract Definition
    pub instantiates_uri: Option<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<types::Element>,

    /// Content derived from the basal information
    pub content_derivative: Option<types::CodeableConcept>,

    /// When this Contract was issued
    pub issued: Option<types::DateTime>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Effective time
    pub applies: Option<types::Period>,

    /// Contract cessation cause
    pub expiration_type: Option<types::CodeableConcept>,

    /// The entity or entities the contract governs, such as a [`Patient`](crate::r5::resources::patient::Patient) or other party
    pub subject: Option<Vec<types::Reference>>,

    /// Authority under which this Contract has standing
    pub authority: Option<Vec<types::Reference>>,

    /// A sphere of control governed by an authoritative jurisdiction, organization, or person
    pub domain: Option<Vec<types::Reference>>,

    /// Specific Location
    pub site: Option<Vec<types::Reference>>,

    /// Computer friendly designation
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human Friendly name
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate Friendly name
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`).
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// Acronym or short name
    pub alias: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`).
    #[serde(rename = "_alias")]
    pub alias_ext: Option<Vec<Option<types::Element>>>,

    /// Source of Contract
    pub author: Option<types::Reference>,

    /// Range of Legal Concerns
    pub scope: Option<types::CodeableConcept>,

    /// The `Contract.topic[x]` choice element (0..1); see [`ContractTopic`].
    #[serde(flatten)]
    pub topic: Option<ContractTopic>,

    /// Legal instrument category
    pub r#type: Option<types::CodeableConcept>,

    /// Subtype within the context of type
    pub sub_type: Option<Vec<types::CodeableConcept>>,

    /// Contract precursor content
    pub content_definition: Option<ContractContentDefinition>,

    /// The ordered list of individual terms that make up the substantive content of the contract
    pub term: Option<Vec<ContractTerm>>,

    /// Extra Information
    pub supporting_info: Option<Vec<types::Reference>>,

    /// Key event in Contract History
    pub relevant_history: Option<Vec<types::Reference>>,

    /// The parties who have signed the contract, along with their role and signature
    pub signer: Option<Vec<ContractSigner>>,

    /// Contract Friendly Language
    pub friendly: Option<Vec<ContractFriendly>>,

    /// Contract Legal Language
    pub legal: Option<Vec<ContractLegal>>,

    /// Computable Contract Language
    pub rule: Option<Vec<ContractRule>>,

    /// The `Contract.legallyBinding[x]` choice element (0..1); see [`ContractLegallyBinding`].
    #[serde(flatten)]
    pub legally_binding: Option<ContractLegallyBinding>,
}

/// Contract precursor content.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractContentDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Content structure and use
    pub r#type: types::CodeableConcept,

    /// Detailed Content Type Definition
    pub sub_type: Option<types::CodeableConcept>,

    /// Publisher Entity
    pub publisher: Option<types::Reference>,

    /// When published
    pub publication_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`publication_date`](Self::publication_date) (FHIR `_publicationDate`).
    #[serde(rename = "_publicationDate")]
    pub publication_date_ext: Option<types::Element>,

    /// amended | appended | cancelled | disputed | entered-in-error | executable +
    pub publication_status: crate::r5::coded::Coded<crate::r5::codes::ContractPublicationstatus>,
    /// Primitive extension sibling for [`publication_status`](Self::publication_status) (FHIR `_publicationStatus`).
    #[serde(rename = "_publicationStatus")]
    pub publication_status_ext: Option<types::Element>,

    /// Publication Ownership
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,
}

/// Contract Term List.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerm {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Contract Term Number
    pub identifier: Option<types::Identifier>,

    /// Contract Term Issue Date Time
    pub issued: Option<types::DateTime>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Contract Term Effective Time
    pub applies: Option<types::Period>,

    /// The `Contract.term.topic[x]` choice element (0..1); see [`ContractTermTopic`].
    #[serde(flatten)]
    pub topic: Option<ContractTermTopic>,

    /// Contract Term Type or Form
    pub r#type: Option<types::CodeableConcept>,

    /// Contract Term Type specific classification
    pub sub_type: Option<types::CodeableConcept>,

    /// Term Statement
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Protection for the Term
    pub security_label: Option<Vec<ContractTermSecurityLabel>>,

    /// Context of the Contract term
    pub offer: ContractTermOffer,

    /// Contract Term Asset List
    pub asset: Option<Vec<ContractTermAsset>>,

    /// Entity being ascribed responsibility
    pub action: Option<Vec<ContractTermAction>>,

    /// Nested Contract Term Group
    pub group: Option<Vec<ContractTerm>>,
}

/// Protection for the Term.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermSecurityLabel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Link to Security Labels
    pub number: Option<Vec<types::UnsignedInt>>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`).
    #[serde(rename = "_number")]
    pub number_ext: Option<Vec<Option<types::Element>>>,

    /// Confidentiality Protection
    pub classification: types::Coding,

    /// Applicable Policy
    pub category: Option<Vec<types::Coding>>,

    /// Handling Instructions
    pub control: Option<Vec<types::Coding>>,
}

/// Context of the Contract term.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermOffer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Offer business ID
    pub identifier: Option<Vec<types::Identifier>>,

    /// Offer Recipient
    pub party: Option<Vec<ContractTermOfferParty>>,

    /// Negotiable offer asset
    pub topic: Option<types::Reference>,

    /// Contract Offer Type or Form
    pub r#type: Option<types::CodeableConcept>,

    /// Accepting party choice
    pub decision: Option<types::CodeableConcept>,

    /// How decision is conveyed
    pub decision_mode: Option<Vec<types::CodeableConcept>>,

    /// Response to offer text
    pub answer: Option<Vec<ContractTermOfferAnswer>>,

    /// Human readable offer text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Pointer to text
    pub link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<Vec<Option<types::Element>>>,

    /// Offer restriction numbers
    pub security_label_number: Option<Vec<types::UnsignedInt>>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`).
    #[serde(rename = "_securityLabelNumber")]
    pub security_label_number_ext: Option<Vec<Option<types::Element>>>,
}

/// Offer Recipient.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermOfferParty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Referenced entity
    pub reference: vec1::Vec1<types::Reference>,

    /// Participant engagement type
    pub role: types::CodeableConcept,
}

/// Response to offer text.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermOfferAnswer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The `Contract.term.offer.answer.value[x]` choice element (0..1); see [`ContractTermOfferAnswerValue`].
    #[serde(flatten)]
    pub value: Option<ContractTermOfferAnswerValue>,
}

/// Contract Term Asset List.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAsset {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Range of asset
    pub scope: Option<types::CodeableConcept>,

    /// Asset category
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Associated entities
    pub type_reference: Option<Vec<types::Reference>>,

    /// Asset sub-category
    pub subtype: Option<Vec<types::CodeableConcept>>,

    /// Kinship of the asset
    pub relationship: Option<types::Coding>,

    /// Circumstance of the asset
    pub context: Option<Vec<ContractTermAssetContext>>,

    /// Quality desctiption of asset
    pub condition: Option<types::String>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`).
    #[serde(rename = "_condition")]
    pub condition_ext: Option<types::Element>,

    /// Asset availability types
    pub period_type: Option<Vec<types::CodeableConcept>>,

    /// Time period of the asset
    pub period: Option<Vec<types::Period>>,

    /// Time period
    pub use_period: Option<Vec<types::Period>>,

    /// Asset clause or question text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Pointer to asset text
    pub link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<Vec<Option<types::Element>>>,

    /// Response to assets
    pub answer: Option<Vec<ContractTermOfferAnswer>>,

    /// Asset restriction numbers
    pub security_label_number: Option<Vec<types::UnsignedInt>>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`).
    #[serde(rename = "_securityLabelNumber")]
    pub security_label_number_ext: Option<Vec<Option<types::Element>>>,

    /// Contract Valued Item List
    pub valued_item: Option<Vec<ContractTermAssetValuedItem>>,
}

/// Circumstance of the asset.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAssetContext {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Creator,custodian or owner
    pub reference: Option<types::Reference>,

    /// Codeable asset context
    pub code: Option<Vec<types::CodeableConcept>>,

    /// Context description
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// Contract Valued Item List.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAssetValuedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The `Contract.term.asset.valuedItem.entity[x]` choice element (0..1); see [`ContractTermAssetValuedItemEntity`].
    #[serde(flatten)]
    pub entity: Option<ContractTermAssetValuedItemEntity>,

    /// Contract Valued Item Number
    pub identifier: Option<types::Identifier>,

    /// Contract Valued Item Effective Tiem
    pub effective_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`effective_time`](Self::effective_time) (FHIR `_effectiveTime`).
    #[serde(rename = "_effectiveTime")]
    pub effective_time_ext: Option<types::Element>,

    /// Count of Contract Valued Items
    pub quantity: Option<types::Quantity>,

    /// Contract Valued Item fee, charge, or cost
    pub unit_price: Option<types::Money>,

    /// Contract Valued Item Price Scaling Factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Contract Valued Item Difficulty Scaling Factor
    pub points: Option<types::Decimal>,
    /// Primitive extension sibling for [`points`](Self::points) (FHIR `_points`).
    #[serde(rename = "_points")]
    pub points_ext: Option<types::Element>,

    /// Total Contract Valued Item Value
    pub net: Option<types::Money>,

    /// Terms of valuation
    pub payment: Option<types::String>,
    /// Primitive extension sibling for [`payment`](Self::payment) (FHIR `_payment`).
    #[serde(rename = "_payment")]
    pub payment_ext: Option<types::Element>,

    /// When payment is due
    pub payment_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`payment_date`](Self::payment_date) (FHIR `_paymentDate`).
    #[serde(rename = "_paymentDate")]
    pub payment_date_ext: Option<types::Element>,

    /// Who will make payment
    pub responsible: Option<types::Reference>,

    /// Who will receive payment
    pub recipient: Option<types::Reference>,

    /// Pointer to specific item
    pub link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<Vec<Option<types::Element>>>,

    /// Security Labels that define affected terms
    pub security_label_number: Option<Vec<types::UnsignedInt>>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`).
    #[serde(rename = "_securityLabelNumber")]
    pub security_label_number_ext: Option<Vec<Option<types::Element>>>,
}

/// Entity being ascribed responsibility.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// True if the term prohibits the action
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`).
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// Type or form of the action
    pub r#type: types::CodeableConcept,

    /// Entity of the action
    pub subject: Option<Vec<ContractTermActionSubject>>,

    /// Purpose for the Contract Term Action
    pub intent: types::CodeableConcept,

    /// Pointer to specific item
    pub link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<Vec<Option<types::Element>>>,

    /// State of the action
    pub status: types::CodeableConcept,

    /// Episode associated with action
    pub context: Option<types::Reference>,

    /// Pointer to specific item
    pub context_link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`context_link_id`](Self::context_link_id) (FHIR `_contextLinkId`).
    #[serde(rename = "_contextLinkId")]
    pub context_link_id_ext: Option<Vec<Option<types::Element>>>,

    /// The `Contract.term.action.occurrence[x]` choice element (0..1); see [`ContractTermActionOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ContractTermActionOccurrence>,

    /// Who asked for action
    pub requester: Option<Vec<types::Reference>>,

    /// Pointer to specific item
    pub requester_link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`requester_link_id`](Self::requester_link_id) (FHIR `_requesterLinkId`).
    #[serde(rename = "_requesterLinkId")]
    pub requester_link_id_ext: Option<Vec<Option<types::Element>>>,

    /// Kind of service performer
    pub performer_type: Option<Vec<types::CodeableConcept>>,

    /// Competency of the performer
    pub performer_role: Option<types::CodeableConcept>,

    /// Actor that wil execute (or not) the action
    pub performer: Option<types::Reference>,

    /// Pointer to specific item
    pub performer_link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`performer_link_id`](Self::performer_link_id) (FHIR `_performerLinkId`).
    #[serde(rename = "_performerLinkId")]
    pub performer_link_id_ext: Option<Vec<Option<types::Element>>>,

    /// Why is action (not) needed?
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Pointer to specific item
    pub reason_link_id: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`reason_link_id`](Self::reason_link_id) (FHIR `_reasonLinkId`).
    #[serde(rename = "_reasonLinkId")]
    pub reason_link_id_ext: Option<Vec<Option<types::Element>>>,

    /// Comments about the action
    pub note: Option<Vec<types::Annotation>>,

    /// Action restriction numbers
    pub security_label_number: Option<Vec<types::UnsignedInt>>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`).
    #[serde(rename = "_securityLabelNumber")]
    pub security_label_number_ext: Option<Vec<Option<types::Element>>>,
}

/// Entity of the action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermActionSubject {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Entity of the action
    pub reference: vec1::Vec1<types::Reference>,

    /// Role type of the agent
    pub role: Option<types::CodeableConcept>,
}

/// Contract Signatory.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractSigner {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Contract Signatory Role
    pub r#type: types::Coding,

    /// Contract Signatory Party
    pub party: types::Reference,

    /// Contract Documentation Signature
    pub signature: vec1::Vec1<types::Signature>,
}

/// Contract Friendly Language.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractFriendly {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The `Contract.friendly.content[x]` choice element (0..1); see [`ContractFriendlyContent`].
    #[serde(flatten)]
    pub content: Option<ContractFriendlyContent>,
}

/// Contract Legal Language.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractLegal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The `Contract.legal.content[x]` choice element (0..1); see [`ContractLegalContent`].
    #[serde(flatten)]
    pub content: Option<ContractLegalContent>,
}

/// Computable Contract Language.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractRule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The `Contract.rule.content[x]` choice element (0..1); see [`ContractRuleContent`].
    #[serde(flatten)]
    pub content: Option<ContractRuleContent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Contract;

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
/// The `Contract.friendly.content[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractFriendlyContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.legal.content[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractLegalContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.legallyBinding[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractLegallyBinding {
    /// `legallyBindingAttachment` variant.
    #[fhir("legallyBindingAttachment")]
    Attachment(Box<types::Attachment>),
    /// `legallyBindingReference` variant.
    #[fhir("legallyBindingReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.rule.content[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractRuleContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.term.action.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermActionOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `Contract.term.asset.valuedItem.entity[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermAssetValuedItemEntity {
    /// `entityCodeableConcept` variant.
    #[fhir("entityCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `entityReference` variant.
    #[fhir("entityReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.term.offer.answer.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermOfferAnswerValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.term.topic[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermTopic {
    /// `topicCodeableConcept` variant.
    #[fhir("topicCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `topicReference` variant.
    #[fhir("topicReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.topic[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ContractTopic {
    /// `topicCodeableConcept` variant.
    #[fhir("topicCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `topicReference` variant.
    #[fhir("topicReference")]
    Reference(Box<types::Reference>),
}