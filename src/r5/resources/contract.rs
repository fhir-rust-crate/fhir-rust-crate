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
use fhir_derive::Validate;

/// Contract Resource.
///
/// Legally enforceable, formally recorded unilateral or bilateral directive
/// i.e., a policy or agreement. In FHIR R5 the Contract resource captures the
/// terms of a legal agreement between parties, including the parties involved,
/// the assets and actions covered, security labels, and human-friendly and
/// machine-computable representations of the agreement.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::contract::Contract;
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

    /// Contract number
    pub identifier: Option<Vec<types::Identifier>>,

    /// Basal definition
    pub url: Option<types::Uri>,

    /// Business edition
    pub version: Option<types::String>,

    /// amended | appended | cancelled | disputed | entered-in-error | executable +
    pub status: Option<types::Code>,

    /// Negotiation status
    pub legal_state: Option<types::CodeableConcept>,

    /// Source Contract Definition
    pub instantiates_canonical: Option<types::Reference>,

    /// External Contract Definition
    pub instantiates_uri: Option<types::Uri>,

    /// Content derived from the basal information
    pub content_derivative: Option<types::CodeableConcept>,

    /// When this Contract was issued
    pub issued: Option<types::DateTime>,

    /// Effective time
    pub applies: Option<types::Period>,

    /// Contract cessation cause
    pub expiration_type: Option<types::CodeableConcept>,

    /// Contract Target Entity
    pub subject: Option<Vec<types::Reference>>,

    /// Authority under which this Contract has standing
    pub authority: Option<Vec<types::Reference>>,

    /// A sphere of control governed by an authoritative jurisdiction, organization, or person
    pub domain: Option<Vec<types::Reference>>,

    /// Specific Location
    pub site: Option<Vec<types::Reference>>,

    /// Computer friendly designation
    pub name: Option<types::String>,

    /// Human Friendly name
    pub title: Option<types::String>,

    /// Subordinate Friendly name
    pub subtitle: Option<types::String>,

    /// Acronym or short name
    pub alias: Option<Vec<types::String>>,

    /// Source of Contract
    pub author: Option<types::Reference>,

    /// Range of Legal Concerns
    pub scope: Option<types::CodeableConcept>,

    /// Focus of contract interest
    pub topic_codeable_concept: Option<types::CodeableConcept>,

    /// Focus of contract interest
    pub topic_reference: Option<types::Reference>,

    /// Legal instrument category
    pub r#type: Option<types::CodeableConcept>,

    /// Subtype within the context of type
    pub sub_type: Option<Vec<types::CodeableConcept>>,

    /// Contract precursor content
    pub content_definition: Option<ContractContentDefinition>,

    /// Contract Term List
    pub term: Option<Vec<ContractTerm>>,

    /// Extra Information
    pub supporting_info: Option<Vec<types::Reference>>,

    /// Key event in Contract History
    pub relevant_history: Option<Vec<types::Reference>>,

    /// Contract Signatory
    pub signer: Option<Vec<ContractSigner>>,

    /// Contract Friendly Language
    pub friendly: Option<Vec<ContractFriendly>>,

    /// Contract Legal Language
    pub legal: Option<Vec<ContractLegal>>,

    /// Computable Contract Language
    pub rule: Option<Vec<ContractRule>>,

    /// Binding Contract
    pub legally_binding_attachment: Option<types::Attachment>,

    /// Binding Contract
    pub legally_binding_reference: Option<types::Reference>,
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

    /// amended | appended | cancelled | disputed | entered-in-error | executable +
    pub publication_status: types::Code,

    /// Publication Ownership
    pub copyright: Option<types::Markdown>,
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

    /// Contract Term Effective Time
    pub applies: Option<types::Period>,

    /// Term Concern
    pub topic_codeable_concept: Option<types::CodeableConcept>,

    /// Term Concern
    pub topic_reference: Option<types::Reference>,

    /// Contract Term Type or Form
    pub r#type: Option<types::CodeableConcept>,

    /// Contract Term Type specific classification
    pub sub_type: Option<types::CodeableConcept>,

    /// Term Statement
    pub text: Option<types::String>,

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

    /// Pointer to text
    pub link_id: Option<Vec<types::String>>,

    /// Offer restriction numbers
    pub security_label_number: Option<Vec<types::UnsignedInt>>,
}

/// Offer Recipient.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermOfferParty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Referenced entity
    pub reference: Vec<types::Reference>,

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

    /// The actual answer response
    pub value_boolean: Option<types::Boolean>,

    /// The actual answer response
    pub value_decimal: Option<types::Decimal>,

    /// The actual answer response
    pub value_integer: Option<types::Integer>,

    /// The actual answer response
    pub value_date: Option<types::Date>,

    /// The actual answer response
    pub value_date_time: Option<types::DateTime>,

    /// The actual answer response
    pub value_time: Option<types::Time>,

    /// The actual answer response
    pub value_string: Option<types::String>,

    /// The actual answer response
    pub value_uri: Option<types::Uri>,

    /// The actual answer response
    pub value_attachment: Option<types::Attachment>,

    /// The actual answer response
    pub value_coding: Option<types::Coding>,

    /// The actual answer response
    pub value_quantity: Option<types::Quantity>,

    /// The actual answer response
    pub value_reference: Option<types::Reference>,
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

    /// Asset availability types
    pub period_type: Option<Vec<types::CodeableConcept>>,

    /// Time period of the asset
    pub period: Option<Vec<types::Period>>,

    /// Time period
    pub use_period: Option<Vec<types::Period>>,

    /// Asset clause or question text
    pub text: Option<types::String>,

    /// Pointer to asset text
    pub link_id: Option<Vec<types::String>>,

    /// Response to assets
    pub answer: Option<Vec<ContractTermOfferAnswer>>,

    /// Asset restriction numbers
    pub security_label_number: Option<Vec<types::UnsignedInt>>,

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

    /// Contract Valued Item Type
    pub entity_codeable_concept: Option<types::CodeableConcept>,

    /// Contract Valued Item Type
    pub entity_reference: Option<types::Reference>,

    /// Contract Valued Item Number
    pub identifier: Option<types::Identifier>,

    /// Contract Valued Item Effective Tiem
    pub effective_time: Option<types::DateTime>,

    /// Count of Contract Valued Items
    pub quantity: Option<types::Quantity>,

    /// Contract Valued Item fee, charge, or cost
    pub unit_price: Option<types::Money>,

    /// Contract Valued Item Price Scaling Factor
    pub factor: Option<types::Decimal>,

    /// Contract Valued Item Difficulty Scaling Factor
    pub points: Option<types::Decimal>,

    /// Total Contract Valued Item Value
    pub net: Option<types::Money>,

    /// Terms of valuation
    pub payment: Option<types::String>,

    /// When payment is due
    pub payment_date: Option<types::DateTime>,

    /// Who will make payment
    pub responsible: Option<types::Reference>,

    /// Who will receive payment
    pub recipient: Option<types::Reference>,

    /// Pointer to specific item
    pub link_id: Option<Vec<types::String>>,

    /// Security Labels that define affected terms
    pub security_label_number: Option<Vec<types::UnsignedInt>>,
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

    /// Type or form of the action
    pub r#type: types::CodeableConcept,

    /// Entity of the action
    pub subject: Option<Vec<ContractTermActionSubject>>,

    /// Purpose for the Contract Term Action
    pub intent: types::CodeableConcept,

    /// Pointer to specific item
    pub link_id: Option<Vec<types::String>>,

    /// State of the action
    pub status: types::CodeableConcept,

    /// Episode associated with action
    pub context: Option<types::Reference>,

    /// Pointer to specific item
    pub context_link_id: Option<Vec<types::String>>,

    /// When action happens
    pub occurrence_date_time: Option<types::DateTime>,

    /// When action happens
    pub occurrence_period: Option<types::Period>,

    /// When action happens
    pub occurrence_timing: Option<types::Timing>,

    /// Who asked for action
    pub requester: Option<Vec<types::Reference>>,

    /// Pointer to specific item
    pub requester_link_id: Option<Vec<types::String>>,

    /// Kind of service performer
    pub performer_type: Option<Vec<types::CodeableConcept>>,

    /// Competency of the performer
    pub performer_role: Option<types::CodeableConcept>,

    /// Actor that wil execute (or not) the action
    pub performer: Option<types::Reference>,

    /// Pointer to specific item
    pub performer_link_id: Option<Vec<types::String>>,

    /// Why is action (not) needed?
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Pointer to specific item
    pub reason_link_id: Option<Vec<types::String>>,

    /// Comments about the action
    pub note: Option<Vec<types::Annotation>>,

    /// Action restriction numbers
    pub security_label_number: Option<Vec<types::UnsignedInt>>,
}

/// Entity of the action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermActionSubject {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Entity of the action
    pub reference: Vec<types::Reference>,

    /// Role type of the agent
    pub role: Option<types::CodeableConcept>,
}

/// Contract Signatory.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
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
    pub signature: Vec<types::Signature>,
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

    /// Easily comprehended representation of this Contract
    pub content_attachment: Option<types::Attachment>,

    /// Easily comprehended representation of this Contract
    pub content_reference: Option<types::Reference>,
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

    /// Contract Legal Text
    pub content_attachment: Option<types::Attachment>,

    /// Contract Legal Text
    pub content_reference: Option<types::Reference>,
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

    /// Computable Contract Rules
    pub content_attachment: Option<types::Attachment>,

    /// Computable Contract Rules
    pub content_reference: Option<types::Reference>,
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
