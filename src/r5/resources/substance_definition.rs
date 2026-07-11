//! SubstanceDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceDefinition
//!
//! Version: 5.0.0
//!
//! SubstanceDefinition Resource: The detailed description of a substance, typically at a level beyond what is used for prescribing.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubstanceDefinition
///
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing. It captures identifying characteristics such as
/// classification, molecular structure, names, codes, and relationships to other
/// substances. This resource is used in medicinal product and regulated substance
/// contexts to describe chemicals, polymers, nucleic acids, proteins, and
/// biological materials in detail.
///
/// `SubstanceDefinition` sits at the reference-data end of the FHIR medicinal
/// product model: rather than describing a dispensed or administered item, it
/// records the scientific and regulatory identity of a raw substance, including
/// its moieties, structure, molecular weight, source material, and synonyms or
/// translated/official names. Regulators, pharmacovigilance systems, and
/// pharmaceutical manufacturers use it to maintain authoritative substance
/// catalogues (for example, aligned with ISO 11238 identification of medicinal
/// product data) that other resources can reference by identifier or code
/// rather than duplicating substance detail inline.
///
/// See also: substances are commonly referenced from medication and ingredient
/// resources such as `Medication` and `MedicinalProductDefinition`, and they
/// make heavy use of shared data types like
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Identifier`](crate::r5::types::Identifier), and
/// [`Reference`](crate::r5::types::Reference) to link to supporting literature,
/// manufacturers, and suppliers.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinition;
///
/// let value = SubstanceDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SubstanceDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinition {
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

    /// One or more business identifiers (e.g. CAS number, UNII) by which this substance is known
    pub identifier: Option<Vec<types::Identifier>>,

    /// A business level version identifier of the substance
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Lifecycle status of this substance record within the catalogue, e.g. active, retired
    pub status: Option<types::CodeableConcept>,

    /// A categorization, high level e.g. polymer or nucleic acid, or food, chemical, biological, or lower e.g. polymer linear or branch chain, or type of impurity
    pub classification: Option<Vec<types::CodeableConcept>>,

    /// If the substance applies to human or veterinary use
    pub domain: Option<types::CodeableConcept>,

    /// The quality standard, established benchmark, to which substance complies (e.g. USP/NF, BP)
    pub grade: Option<Vec<types::CodeableConcept>>,

    /// Textual description of the substance
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Supporting literature
    pub information_source: Option<Vec<types::Reference>>,

    /// Textual comment about the substance's catalogue or registry record
    pub note: Option<Vec<types::Annotation>>,

    /// The entity that creates, makes, produces or fabricates the substance
    pub manufacturer: Option<Vec<types::Reference>>,

    /// An entity that is the source for the substance. It may be different from the manufacturer
    pub supplier: Option<Vec<types::Reference>>,

    /// Moiety, for structural modifications
    pub moiety: Option<Vec<SubstanceDefinitionMoiety>>,

    /// General specifications for this substance
    pub characterization: Option<Vec<SubstanceDefinitionCharacterization>>,

    /// General specifications for this substance
    pub property: Option<Vec<SubstanceDefinitionProperty>>,

    /// General information detailing this substance
    pub reference_information: Option<types::Reference>,

    /// The average mass of a molecule of a compound
    pub molecular_weight: Option<Vec<SubstanceDefinitionMolecularWeight>>,

    /// Structural information, such as molecular formula and depictions like SMILES or InChI
    pub structure: Option<SubstanceDefinitionStructure>,

    /// Codes associated with the substance, e.g. regulatory or terminology codes and their status
    pub code: Option<Vec<SubstanceDefinitionCode>>,

    /// Names applicable to this substance, including synonyms, translations, and official names
    pub name: Option<Vec<SubstanceDefinitionName>>,

    /// A link between this substance and another, e.g. salt-to-parent or active-moiety relationships
    pub relationship: Option<Vec<SubstanceDefinitionRelationship>>,

    /// Data items specific to nucleic acids
    pub nucleic_acid: Option<types::Reference>,

    /// Data items specific to polymers
    pub polymer: Option<types::Reference>,

    /// Data items specific to proteins
    pub protein: Option<types::Reference>,

    /// Material or taxonomic/anatomical source
    pub source_material: Option<SubstanceDefinitionSourceMaterial>,
}

/// SubstanceDefinition.moiety - Moiety, for structural modifications
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionMoiety {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Role that the moiety is playing
    pub role: Option<types::CodeableConcept>,

    /// Identifier by which this moiety substance is known
    pub identifier: Option<types::Identifier>,

    /// Textual name for this moiety substance
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Stereochemistry type
    pub stereochemistry: Option<types::CodeableConcept>,

    /// Optical activity type
    pub optical_activity: Option<types::CodeableConcept>,

    /// Molecular formula for this moiety (e.g. with the Hill system)
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`).
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// Quantitative value for this moiety
    pub amount_quantity: Option<types::Quantity>,

    /// Quantitative value for this moiety
    pub amount_string: Option<types::String>,

    /// The measurement type of the quantitative value
    pub measurement_type: Option<types::CodeableConcept>,
}

/// SubstanceDefinition.characterization - General specifications for this substance
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionCharacterization {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The method used to find the characterization e.g. HPLC
    pub technique: Option<types::CodeableConcept>,

    /// Describes the nature of the chemical entity and explains, for instance, whether this is a base or a salt form
    pub form: Option<types::CodeableConcept>,

    /// The description or justification in support of the interpretation of the data file
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The data produced by the analytical instrument or a pictorial representation of that data. Examples: a JCAMP, JDX, or ADX file, or a chromatogram or spectrum analysis
    pub file: Option<Vec<types::Attachment>>,
}

/// SubstanceDefinition.property - General specifications for this substance
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A code expressing the type of property
    pub r#type: types::CodeableConcept,

    /// A value for the property
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// A value for the property
    pub value_quantity: Option<types::Quantity>,

    /// A value for the property
    pub value_date: Option<types::Date>,

    /// A value for the property
    pub value_boolean: Option<types::Boolean>,

    /// A value for the property
    pub value_attachment: Option<types::Attachment>,
}

/// SubstanceDefinition.molecularWeight - The average mass of a molecule of a compound
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionMolecularWeight {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The method by which the weight was determined
    pub method: Option<types::CodeableConcept>,

    /// Type of molecular weight e.g. exact, average, weight average
    pub r#type: Option<types::CodeableConcept>,

    /// Used to capture quantitative values for a variety of elements
    pub amount: types::Quantity,
}

/// SubstanceDefinition.structure - Structural information
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionStructure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Stereochemistry type
    pub stereochemistry: Option<types::CodeableConcept>,

    /// Optical activity type
    pub optical_activity: Option<types::CodeableConcept>,

    /// An expression which states the number and type of atoms present in a molecule of a substance
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`).
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// Specified per moiety according to the Hill system
    pub molecular_formula_by_moiety: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula_by_moiety`](Self::molecular_formula_by_moiety) (FHIR `_molecularFormulaByMoiety`).
    #[serde(rename = "_molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety_ext: Option<types::Element>,

    /// The molecular weight or weight range
    pub molecular_weight: Option<SubstanceDefinitionMolecularWeight>,

    /// The method used to find the structure e.g. X-ray, NMR
    pub technique: Option<Vec<types::CodeableConcept>>,

    /// Source of information for the structure
    pub source_document: Option<Vec<types::Reference>>,

    /// A depiction of the structure of the substance
    pub representation: Option<Vec<SubstanceDefinitionStructureRepresentation>>,
}

/// SubstanceDefinition.structure.representation - A depiction of the structure of the substance
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionStructureRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kind of structural representation (e.g. full, partial)
    pub r#type: Option<types::CodeableConcept>,

    /// The structural representation as a text string in a standard format
    pub representation: Option<types::String>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`).
    #[serde(rename = "_representation")]
    pub representation_ext: Option<types::Element>,

    /// The format of the representation e.g. InChI, SMILES, MOLFILE (note: not the physical file format)
    pub format: Option<types::CodeableConcept>,

    /// An attachment with the structural representation e.g. a structure graphic or AnIML file
    pub document: Option<types::Reference>,
}

/// SubstanceDefinition.code - Codes associated with the substance
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionCode {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The specific code
    pub code: Option<types::CodeableConcept>,

    /// Status of the code assignment, for example 'provisional', 'approved'
    pub status: Option<types::CodeableConcept>,

    /// The date at which the code status was changed
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`).
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Any comment can be provided in this field
    pub note: Option<Vec<types::Annotation>>,

    /// Supporting literature
    pub source: Option<Vec<types::Reference>>,
}

/// SubstanceDefinition.name - Names applicable to this substance
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The actual name
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name type e.g. 'systematic',  'scientific, 'brand'
    pub r#type: Option<types::CodeableConcept>,

    /// The status of the name e.g. 'current', 'proposed'
    pub status: Option<types::CodeableConcept>,

    /// If this is the preferred name for this substance
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`).
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,

    /// Human language that the name is written in
    pub language: Option<Vec<types::CodeableConcept>>,

    /// The use context of this name e.g. as an active ingredient or as a food colour additive
    pub domain: Option<Vec<types::CodeableConcept>>,

    /// The jurisdiction where this name applies
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// A synonym of this particular name, by which the substance is also known
    pub synonym: Option<Vec<SubstanceDefinitionName>>,

    /// A translation for this name into another human language
    pub translation: Option<Vec<SubstanceDefinitionName>>,

    /// Details of the official nature of this name
    pub official: Option<Vec<SubstanceDefinitionNameOfficial>>,

    /// Supporting literature
    pub source: Option<Vec<types::Reference>>,
}

/// SubstanceDefinition.name.official - Details of the official nature of this name
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionNameOfficial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Which authority uses this official name
    pub authority: Option<types::CodeableConcept>,

    /// The status of the official name, for example 'draft', 'active'
    pub status: Option<types::CodeableConcept>,

    /// Date of official name change
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
}

/// SubstanceDefinition.relationship - A link between this substance and another
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionRelationship {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A pointer to another substance, as a resource or a representational code
    pub substance_definition_reference: Option<types::Reference>,

    /// A pointer to another substance, as a resource or a representational code
    pub substance_definition_codeable_concept: Option<types::CodeableConcept>,

    /// For example "salt to parent", "active moiety"
    pub r#type: types::CodeableConcept,

    /// For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible relationships
    pub is_defining: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_defining`](Self::is_defining) (FHIR `_isDefining`).
    #[serde(rename = "_isDefining")]
    pub is_defining_ext: Option<types::Element>,

    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other
    pub amount_quantity: Option<types::Quantity>,

    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other
    pub amount_ratio: Option<types::Ratio>,

    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other
    pub amount_string: Option<types::String>,

    /// For use when the numeric has an uncertain range
    pub ratio_high_limit_amount: Option<types::Ratio>,

    /// An operator for the amount, for example "average", "approximately", "less than"
    pub comparator: Option<types::CodeableConcept>,

    /// Supporting literature
    pub source: Option<Vec<types::Reference>>,
}

/// SubstanceDefinition.sourceMaterial - Material or taxonomic/anatomical source
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinitionSourceMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Classification of the origin of the raw material. e.g. cat hair is an Animal source type
    pub r#type: Option<types::CodeableConcept>,

    /// The genus of an organism e.g. the Latin epithet of the plant/animal scientific name
    pub genus: Option<types::CodeableConcept>,

    /// The species of an organism e.g. the Latin epithet of the species of the plant/animal
    pub species: Option<types::CodeableConcept>,

    /// An anatomical origin of the source material within an organism
    pub part: Option<types::CodeableConcept>,

    /// The country or countries where the material is harvested
    pub country_of_origin: Option<Vec<types::CodeableConcept>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceDefinition;

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
