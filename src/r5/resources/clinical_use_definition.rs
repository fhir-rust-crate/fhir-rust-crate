//! ClinicalUseDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClinicalUseDefinition
//!
//! Version: 5.0.0
//!
//! ClinicalUseDefinition Resource: A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ClinicalUseDefinition describes a single clinical issue relating to a
/// medicinal product, medication, device or procedure. The issue may be an
/// indication, a contraindication, an interaction, an undesirable effect, or a
/// warning. In FHIR R5 it is used chiefly within regulated medicine and product
/// information to capture structured, machine-readable clinical usage details.
///
/// Each instance records exactly one such issue via the `type` element, with
/// the corresponding detail carried in one of the `contraindication`,
/// `indication`, `interaction`, `undesirable_effect`, or `warning`
/// backbone elements. This granular, one-issue-per-resource model lets
/// publishers of drug and device knowledge bases (such as national
/// formularies or product labeling authorities) describe, cross-reference,
/// and version individual clinical facts independently, rather than bundling
/// them into a single monolithic document. Consuming systems, including
/// clinical decision support and e-prescribing tools, can then query for
/// specific issues relevant to a given subject or population.
///
/// Related resources: the `subject` element typically references a
/// `MedicinalProductDefinition`, `Medication`, `ActivityDefinition`,
/// `PlanDefinition`, `Device`, `DeviceDefinition`, or `Substance`, while
/// `population` typically references a `Group` describing the affected
/// patient population. See also [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// and [`CodeableReference`](crate::r5::types::CodeableReference), which are
/// used extensively throughout the nested backbone elements to convey coded
/// or free-text clinical detail.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::clinical_use_definition::ClinicalUseDefinition;
///
/// let value = ClinicalUseDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClinicalUseDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinition {
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

    /// Business identifier for this issue
    pub identifier: Option<Vec<types::Identifier>>,

    /// The kind of issue this instance describes: indication | contraindication | interaction | undesirable-effect | warning; this determines which of the backbone elements below carries the detail
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ClinicalUseDefinitionType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// A categorisation of the issue, primarily for dividing warnings into subject heading areas
    pub category: Option<Vec<types::CodeableConcept>>,

    /// The medication, product, substance, device, procedure etc. for which this is an indication; typically a reference to a product or activity definition resource
    pub subject: Option<Vec<types::Reference>>,

    /// Whether this is a current issue or one that has been retired, withdrawn, or superseded etc
    pub status: Option<types::CodeableConcept>,

    /// Specifics for when this is a contraindication
    pub contraindication: Option<ClinicalUseDefinitionContraindication>,

    /// Specifics for when this is an indication
    pub indication: Option<ClinicalUseDefinitionIndication>,

    /// Specifics for when this is an interaction
    pub interaction: Option<ClinicalUseDefinitionInteraction>,

    /// The population group to which this applies
    pub population: Option<Vec<types::Reference>>,

    /// Logic used by the clinical use definition
    pub library: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`).
    #[serde(rename = "_library")]
    pub library_ext: Option<Vec<Option<types::Element>>>,

    /// A possible negative outcome from the use of this treatment
    pub undesirable_effect: Option<ClinicalUseDefinitionUndesirableEffect>,

    /// Critical environmental, health or physical risks or hazards
    pub warning: Option<ClinicalUseDefinitionWarning>,
}

/// Specifics for when this is a contraindication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionContraindication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The situation that is being documented as contraindicating against this item
    pub disease_symptom_procedure: Option<types::CodeableReference>,

    /// The status of the disease or symptom for the contraindication
    pub disease_status: Option<types::CodeableReference>,

    /// A comorbidity (concurrent condition) or coinfection
    pub comorbidity: Option<Vec<types::CodeableReference>>,

    /// The indication which this is a contraidication for
    pub indication: Option<Vec<types::Reference>>,

    /// An expression that returns true or false, indicating whether the indication is applicable or not
    pub applicability: Option<types::Expression>,

    /// Information about use of the product in relation to other therapies described as part of the contraindication
    pub other_therapy: Option<Vec<ClinicalUseDefinitionContraindicationOtherTherapy>>,
}

/// Information about use of the product in relation to other therapies described
/// as part of the contraindication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionContraindicationOtherTherapy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of relationship between the product indication/contraindication and another therapy
    pub relationship_type: types::CodeableConcept,

    /// Reference to a specific medication, substance etc. as part of an indication or contraindication
    pub treatment: types::CodeableReference,
}

/// Specifics for when this is an indication.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionIndication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The situation that is being documented as an indicaton for this item
    pub disease_symptom_procedure: Option<types::CodeableReference>,

    /// The status of the disease or symptom for the indication
    pub disease_status: Option<types::CodeableReference>,

    /// A comorbidity or coinfection as part of the indication
    pub comorbidity: Option<Vec<types::CodeableReference>>,

    /// The intended effect, aim or strategy to be achieved
    pub intended_effect: Option<types::CodeableReference>,

    /// The `ClinicalUseDefinition.indication.duration[x]` choice element (0..1); see [`ClinicalUseDefinitionIndicationDuration`].
    #[serde(flatten)]
    pub duration: Option<ClinicalUseDefinitionIndicationDuration>,

    /// An unwanted side effect or negative outcome of the subject of this resource when being used for this indication
    pub undesirable_effect: Option<Vec<types::Reference>>,

    /// An expression that returns true or false, indicating whether the indication is applicable or not
    pub applicability: Option<types::Expression>,

    /// The use of the medicinal product in relation to other therapies described as part of the indication
    pub other_therapy: Option<Vec<ClinicalUseDefinitionContraindicationOtherTherapy>>,
}

/// Specifics for when this is an interaction.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The specific medication, product, food etc. or laboratory test that interacts
    pub interactant: Option<Vec<ClinicalUseDefinitionInteractionInteractant>>,

    /// The type of the interaction e.g. drug-drug interaction, drug-lab test interaction
    pub r#type: Option<types::CodeableConcept>,

    /// The effect of the interaction, for example "reduced gastric absorption of primary medication"
    pub effect: Option<types::CodeableReference>,

    /// The incidence of the interaction, e.g. theoretical, observed
    pub incidence: Option<types::CodeableConcept>,

    /// Actions for managing the interaction
    pub management: Option<Vec<types::CodeableConcept>>,
}

/// The specific medication, product, food etc. or laboratory test that
/// interacts.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionInteractionInteractant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The `ClinicalUseDefinition.interaction.interactant.item[x]` choice element (0..1); see [`ClinicalUseDefinitionInteractionInteractantItem`].
    #[serde(flatten)]
    pub item: Option<ClinicalUseDefinitionInteractionInteractantItem>,
}

/// A possible negative outcome from the use of this treatment.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionUndesirableEffect {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The situation in which the undesirable effect may manifest
    pub symptom_condition_effect: Option<types::CodeableReference>,

    /// High level classification of the effect
    pub classification: Option<types::CodeableConcept>,

    /// How often the effect is seen
    pub frequency_of_occurrence: Option<types::CodeableConcept>,
}

/// Critical environmental, health or physical risks or hazards. For example
/// 'Do not operate heavy machinery', 'May cause drowsiness'.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionWarning {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A textual definition of this warning, with formatting
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// A coded or unformatted textual definition of this warning
    pub code: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ClinicalUseDefinition;

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
/// The `ClinicalUseDefinition.indication.duration[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalUseDefinitionIndicationDuration {
    /// `durationRange` variant.
    #[fhir("durationRange")]
    Range(Box<types::Range>),
    /// `durationString` variant.
    #[fhir("durationString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `ClinicalUseDefinition.interaction.interactant.item[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalUseDefinitionInteractionInteractantItem {
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}
