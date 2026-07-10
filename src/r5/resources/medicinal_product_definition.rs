//! MedicinalProductDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductDefinition
//!
//! Version: 5.0.0
//!
//! MedicinalProductDefinition Resource: Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// MedicinalProductDefinition
///
/// A detailed definition of a medicinal product, typically for uses other than
/// direct patient care such as regulatory submissions, drug catalogs, prescribing
/// support, and adverse event management. It captures the identity, naming,
/// classification, ingredients, and administrative details of a product as a
/// whole, complementing more granular resources like Ingredient and
/// PackagedProductDefinition.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::medicinal_product_definition::MedicinalProductDefinition;
///
/// let value = MedicinalProductDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicinalProductDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinition {
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

    /// Business identifier for this product. Could be an MPID
    pub identifier: Option<Vec<types::Identifier>>,

    /// Regulatory type, e.g. Investigational or Authorized
    pub r#type: Option<types::CodeableConcept>,

    /// If this medicine applies to human or veterinary uses
    pub domain: Option<types::CodeableConcept>,

    /// A business identifier relating to a specific version of the product
    pub version: Option<types::String>,

    /// The status within the lifecycle of this product record
    pub status: Option<types::CodeableConcept>,

    /// The date at which the given status became applicable
    pub status_date: Option<types::DateTime>,

    /// General description of this product
    pub description: Option<types::Markdown>,

    /// The dose form for a single part product, or combined form of a multiple part product
    pub combined_pharmaceutical_dose_form: Option<types::CodeableConcept>,

    /// The path by which the product is taken into or makes contact with the body
    pub route: Option<Vec<types::CodeableConcept>>,

    /// Description of indication(s) for this product, used when structured indications are not required
    pub indication: Option<types::Markdown>,

    /// The legal status of supply of the medicinal product as classified by the regulator
    pub legal_status_of_supply: Option<types::CodeableConcept>,

    /// Whether the Medicinal Product is subject to additional monitoring for regulatory reasons
    pub additional_monitoring_indicator: Option<types::CodeableConcept>,

    /// Whether the Medicinal Product is subject to special measures for regulatory reasons
    pub special_measures: Option<Vec<types::CodeableConcept>>,

    /// If authorised for use in children
    pub pediatric_use_indicator: Option<types::CodeableConcept>,

    /// Allows the product to be classified by various systems
    pub classification: Option<Vec<types::CodeableConcept>>,

    /// Marketing status of the medicinal product, in contrast to marketing authorization
    pub marketing_status: Option<Vec<types::MarketingStatus>>,

    /// Package type for the product
    pub packaged_medicinal_product: Option<Vec<types::CodeableConcept>>,

    /// Types of medicinal manufactured items and/or devices that this product consists of, such as tablets, capsule, or syringes
    pub comprised_of: Option<Vec<types::Reference>>,

    /// The ingredients of this medicinal product - when not detailed in other resources
    pub ingredient: Option<Vec<types::CodeableConcept>>,

    /// Any component of the drug product which is not the chemical entity defined as the drug substance, or an excipient in the drug product
    pub impurity: Option<Vec<types::CodeableReference>>,

    /// Additional documentation about the medicinal product
    pub attached_document: Option<Vec<types::Reference>>,

    /// A master file for the medicinal product (e.g. Pharmacovigilance System Master File)
    pub master_file: Option<Vec<types::Reference>>,

    /// A product specific contact, person (in a role), or an organization
    pub contact: Option<Vec<MedicinalProductDefinitionContact>>,

    /// Clinical trials or studies that this product is involved in
    pub clinical_trial: Option<Vec<types::Reference>>,

    /// A code that this product is known by, within some formal terminology
    pub code: Option<Vec<types::Coding>>,

    /// The product's name, including full name and possibly coded parts
    pub name: Vec<MedicinalProductDefinitionName>,

    /// Reference to another product, e.g. for linking authorised to investigational product
    pub cross_reference: Option<Vec<MedicinalProductDefinitionCrossReference>>,

    /// A manufacturing or administrative process for the medicinal product
    pub operation: Option<Vec<MedicinalProductDefinitionOperation>>,

    /// Key product features such as "sugar free", "modified release"
    pub characteristic: Option<Vec<MedicinalProductDefinitionCharacteristic>>,
}

/// MedicinalProductDefinition.contact
///
/// A product specific contact, person (in a role), or an organization.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionContact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Allows the contact to be classified, for example QPPV, Pharmacovigilance Enquiry Information
    pub r#type: Option<types::CodeableConcept>,

    /// A product specific contact, person (in a role), or an organization
    pub contact: types::Reference,
}

/// MedicinalProductDefinition.name
///
/// The product's name, including full name and possibly coded parts.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The full product name
    pub product_name: types::String,

    /// Type of product name, such as rINN, BAN, Proprietary, Non-Proprietary
    pub r#type: Option<types::CodeableConcept>,

    /// Coding words or phrases of the name
    pub part: Option<Vec<MedicinalProductDefinitionNamePart>>,

    /// Country and jurisdiction where the name applies
    pub usage: Option<Vec<MedicinalProductDefinitionNameUsage>>,
}

/// MedicinalProductDefinition.name.part
///
/// Coding words or phrases of the name.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionNamePart {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A fragment of a product name
    pub part: types::String,

    /// Identifying type for this part of the name (e.g. strength part)
    pub r#type: types::CodeableConcept,
}

/// MedicinalProductDefinition.name.usage
///
/// Country and jurisdiction where the name applies.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionNameUsage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Country code for where this name applies
    pub country: types::CodeableConcept,

    /// Jurisdiction code for where this name applies
    pub jurisdiction: Option<types::CodeableConcept>,

    /// Language code for this name
    pub language: types::CodeableConcept,
}

/// MedicinalProductDefinition.crossReference
///
/// Reference to another product, e.g. for linking authorised to investigational product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionCrossReference {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to another product, e.g. for linking authorised to investigational product
    pub product: types::CodeableReference,

    /// The type of relationship, for instance branded to generic or virtual to actual product
    pub r#type: Option<types::CodeableConcept>,
}

/// MedicinalProductDefinition.operation
///
/// A manufacturing or administrative process for the medicinal product.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of manufacturing operation e.g. manufacturing itself, re-packaging
    pub r#type: Option<types::CodeableReference>,

    /// Date range of applicability
    pub effective_date: Option<types::Period>,

    /// The organization responsible for the particular process, e.g. the manufacturer or importer
    pub organization: Option<Vec<types::Reference>>,

    /// Specifies whether this process is considered proprietary or confidential
    pub confidentiality_indicator: Option<types::CodeableConcept>,
}

/// MedicinalProductDefinition.characteristic
///
/// Key product features such as "sugar free", "modified release".
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A code expressing the type of characteristic
    pub r#type: types::CodeableConcept,

    /// A value for the characteristic
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// A value for the characteristic
    pub value_markdown: Option<types::Markdown>,

    /// A value for the characteristic
    pub value_quantity: Option<types::Quantity>,

    /// A value for the characteristic
    pub value_integer: Option<types::Integer>,

    /// A value for the characteristic
    pub value_date: Option<types::Date>,

    /// A value for the characteristic
    pub value_boolean: Option<types::Boolean>,

    /// A value for the characteristic
    pub value_attachment: Option<types::Attachment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicinalProductDefinition;

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
