//! MedicationKnowledge
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
//!
//! Version: 5.0.0
//!
//! MedicationKnowledge Resource: Information about a medication that is used to support knowledge.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Information about a medication that is used to support knowledge.
///
/// MedicationKnowledge is a definitional resource that captures broad,
/// reference-style information about a medication independent of any single
/// patient or administration event. Rather than recording that a particular
/// product was prescribed, dispensed, or administered, it describes what is
/// generally known about a medication: its names and codes, product type and
/// classification, ingredients and dose form, cost and packaging, storage and
/// regulatory constraints, monitoring programs, associated monographs, and
/// indication-specific dosing guidelines. In FHIR R5 it is typically populated
/// by drug knowledge vendors, formularies, and pharmacy systems, then consumed
/// to power decision support, formulary lookups, drug reference databases,
/// pricing, and clinician-facing monographs.
///
/// It is distinct from the Medication resource, which represents a specific
/// product used in a clinical workflow; MedicationKnowledge instead provides
/// the surrounding definitional and reference knowledge about such products.
///
/// # Related resources
///
/// See also the `Medication` resource for a specific product instance, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) and
/// [`Reference`](crate::r5::types::Reference), which are used throughout this
/// resource to carry coded values and links to related resources.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_knowledge::MedicationKnowledge;
///
/// let value = MedicationKnowledge::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationKnowledge = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledge {
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

    /// Business identifier(s) for this medication knowledge record, distinct from any identifiers on a specific Medication product
    pub identifier: Option<Vec<types::Identifier>>,

    /// Code that identifies this medication, typically drawn from a drug terminology such as RxNorm or SNOMED CT
    pub code: Option<types::CodeableConcept>,

    /// Lifecycle status of this knowledge record: active, entered-in-error, or inactive
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::MedicationknowledgeStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Creator or owner of the knowledge or information about the medication
    pub author: Option<types::Reference>,

    /// Codes that identify the different jurisdictions for which the information of this resource was created
    pub intended_jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// A name associated with the medication being described, such as a brand, generic, or synonym name
    pub name: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<Vec<Option<types::Element>>>,

    /// Associated or related medication information
    pub related_medication_knowledge: Option<Vec<MedicationKnowledgeRelatedMedicationKnowledge>>,

    /// The set of medication resources that are associated with this medication
    pub associated_medication: Option<Vec<types::Reference>>,

    /// Category of the medication or product
    pub product_type: Option<Vec<types::CodeableConcept>>,

    /// Associated documentation about the medication
    pub monograph: Option<Vec<MedicationKnowledgeMonograph>>,

    /// The instructions for preparing the medication
    pub preparation_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`preparation_instruction`](Self::preparation_instruction) (FHIR `_preparationInstruction`).
    #[serde(rename = "_preparationInstruction")]
    pub preparation_instruction_ext: Option<types::Element>,

    /// The pricing of the medication
    pub cost: Option<Vec<MedicationKnowledgeCost>>,

    /// Program under which a medication is reviewed
    pub monitoring_program: Option<Vec<MedicationKnowledgeMonitoringProgram>>,

    /// Guidelines or protocols for administration of the medication for an indication
    pub indication_guideline: Option<Vec<MedicationKnowledgeIndicationGuideline>>,

    /// Categorization of the medication within a formulary or classification system
    pub medicine_classification: Option<Vec<MedicationKnowledgeMedicineClassification>>,

    /// Details about packaged medications
    pub packaging: Option<Vec<MedicationKnowledgePackaging>>,

    /// Potential clinical issue with or between medication(s), such as a known interaction, contraindication, or warning
    pub clinical_use_issue: Option<Vec<types::Reference>>,

    /// How the medication should be stored
    pub storage_guideline: Option<Vec<MedicationKnowledgeStorageGuideline>>,

    /// Regulatory information about a medication
    pub regulatory: Option<Vec<MedicationKnowledgeRegulatory>>,

    /// Minimal definitional information such as dose form, route, and ingredients that characterizes the medication
    pub definitional: Option<MedicationKnowledgeDefinitional>,
}

/// Associated or related medication information.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Category of medicationKnowledge
    pub r#type: types::CodeableConcept,

    /// Associated documentation about the associated medication knowledge
    pub reference: Vec<types::Reference>,
}

/// Associated documentation about the medication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeMonograph {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The category of medication document
    pub r#type: Option<types::CodeableConcept>,

    /// Associated documentation about the medication
    pub source: Option<types::Reference>,
}

/// The pricing of the medication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The date range for which the cost is effective
    pub effective_date: Option<Vec<types::Period>>,

    /// The category of the cost information
    pub r#type: types::CodeableConcept,

    /// The source or owner for the price information
    pub source: Option<types::String>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`).
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// The `MedicationKnowledge.cost.cost[x]` choice element (0..1); see [`MedicationKnowledgeCostCost`].
    #[serde(flatten)]
    pub cost: Option<MedicationKnowledgeCostCost>,
}

/// Program under which a medication is reviewed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeMonitoringProgram {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of program under which the medication is monitored
    pub r#type: Option<types::CodeableConcept>,

    /// Name of the reviewing program
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
}

/// Guidelines or protocols for administration of the medication for an indication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Indication for use that applies to the specific administration guideline
    pub indication: Option<Vec<types::CodeableReference>>,

    /// Guidelines for dosage of the medication
    pub dosing_guideline: Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuideline>>,
}

/// Guidelines for dosage of the medication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Intention of the treatment
    pub treatment_intent: Option<types::CodeableConcept>,

    /// Dosage for the medication for the specific guidelines
    pub dosage: Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage>>,

    /// Type of treatment the guideline applies to
    pub administration_treatment: Option<types::CodeableConcept>,

    /// Characteristics of the patient that are relevant to the administration guidelines
    pub patient_characteristic:
        Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic>>,
}

/// Dosage for the medication for the specific guidelines.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Category of dosage for a medication
    pub r#type: types::CodeableConcept,

    /// Dosage for the medication for the specific guidelines
    pub dosage: Vec<types::Dosage>,
}

/// Characteristics of the patient that are relevant to the administration guidelines.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Categorization of specific characteristic that is relevant to the administration guideline
    pub r#type: types::CodeableConcept,

    /// The `MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.value[x]` choice element (0..1); see [`MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue>,
}

/// Categorization of the medication within a formulary or classification system.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeMedicineClassification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of category for the medication (for example, therapeutic classification, therapeutic sub-classification)
    pub r#type: types::CodeableConcept,

    /// The `MedicationKnowledge.medicineClassification.source[x]` choice element (0..1); see [`MedicationKnowledgeMedicineClassificationSource`].
    #[serde(flatten)]
    pub source: Option<MedicationKnowledgeMedicineClassificationSource>,

    /// Specific category assigned to the medication
    pub classification: Option<Vec<types::CodeableConcept>>,
}

/// Details about packaged medications.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgePackaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Cost of the packaged medication
    pub cost: Option<Vec<MedicationKnowledgeCost>>,

    /// The packaged medication that is being priced
    pub packaged_product: Option<types::Reference>,
}

/// How the medication should be stored.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeStorageGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to additional information
    pub reference: Option<types::Uri>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`).
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Additional storage notes
    pub note: Option<Vec<types::Annotation>>,

    /// Duration remains stable
    pub stability_duration: Option<types::Duration>,

    /// Setting or value of environment for adequate storage
    pub environmental_setting: Option<Vec<MedicationKnowledgeStorageGuidelineEnvironmentalSetting>>,
}

/// Setting or value of environment for adequate storage.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeStorageGuidelineEnvironmentalSetting {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Categorization of the setting
    pub r#type: types::CodeableConcept,

    /// The `MedicationKnowledge.storageGuideline.environmentalSetting.value[x]` choice element (0..1); see [`MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue`].
    #[serde(flatten)]
    pub value: Option<MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue>,
}

/// Regulatory information about a medication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRegulatory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Specifies the authority of the regulation
    pub regulatory_authority: types::Reference,

    /// Specifies if changes are allowed when dispensing a medication from a regulatory perspective
    pub substitution: Option<Vec<MedicationKnowledgeRegulatorySubstitution>>,

    /// Specifies the schedule of a medication in jurisdiction
    pub schedule: Option<Vec<types::CodeableConcept>>,

    /// The maximum number of units of the medication that can be dispensed in a period
    pub max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
}

/// Specifies if changes are allowed when dispensing a medication from a regulatory perspective.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRegulatorySubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Specifies the type of substitution allowed
    pub r#type: types::CodeableConcept,

    /// Specifies if regulation allows for changes in the medication when dispensing
    pub allowed: types::Boolean,
    /// Primitive extension sibling for [`allowed`](Self::allowed) (FHIR `_allowed`).
    #[serde(rename = "_allowed")]
    pub allowed_ext: Option<types::Element>,
}

/// The maximum number of units of the medication that can be dispensed in a period.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The maximum number of units of the medication that can be dispensed
    pub quantity: types::Quantity,

    /// The period that applies to the maximum number of units
    pub period: Option<types::Duration>,
}

/// Minimal definition information about the medication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeDefinitional {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Definitional resources that provide more information about this medication
    pub definition: Option<Vec<types::Reference>>,

    /// powder | tablets | capsule +
    pub dose_form: Option<types::CodeableConcept>,

    /// The intended or approved route of administration
    pub intended_route: Option<Vec<types::CodeableConcept>>,

    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationKnowledgeDefinitionalIngredient>>,

    /// Specifies descriptive properties of the medicine
    pub drug_characteristic: Option<Vec<MedicationKnowledgeDefinitionalDrugCharacteristic>>,
}

/// Active or inactive ingredient.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeDefinitionalIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Substances contained in the medication
    pub item: types::CodeableReference,

    /// A code that defines the type of ingredient, active, base, etc
    pub r#type: Option<types::CodeableConcept>,

    /// The `MedicationKnowledge.definitional.ingredient.strength[x]` choice element (0..1); see [`MedicationKnowledgeDefinitionalIngredientStrength`].
    #[serde(flatten)]
    pub strength: Option<MedicationKnowledgeDefinitionalIngredientStrength>,
}

/// Specifies descriptive properties of the medicine.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledgeDefinitionalDrugCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code specifying the type of characteristic of medication
    pub r#type: Option<types::CodeableConcept>,

    /// The `MedicationKnowledge.definitional.drugCharacteristic.value[x]` choice element (0..1); see [`MedicationKnowledgeDefinitionalDrugCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<MedicationKnowledgeDefinitionalDrugCharacteristicValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationKnowledge;

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
/// The `MedicationKnowledge.cost.cost[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeCostCost {
    /// `costMoney` variant.
    #[fhir("costMoney")]
    Money(Box<types::Money>),
    /// `costCodeableConcept` variant.
    #[fhir("costCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `MedicationKnowledge.definitional.drugCharacteristic.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeDefinitionalDrugCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r5::choice::Primitive<types::Base64Binary>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}

/// The `MedicationKnowledge.definitional.ingredient.strength[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeDefinitionalIngredientStrength {
    /// `strengthRatio` variant.
    #[fhir("strengthRatio")]
    Ratio(Box<types::Ratio>),
    /// `strengthCodeableConcept` variant.
    #[fhir("strengthCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `strengthQuantity` variant.
    #[fhir("strengthQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
}

/// The `MedicationKnowledge.medicineClassification.source[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeMedicineClassificationSource {
    /// `sourceString` variant.
    #[fhir("sourceString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `sourceUri` variant.
    #[fhir("sourceUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
}

/// The `MedicationKnowledge.storageGuideline.environmentalSetting.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}
