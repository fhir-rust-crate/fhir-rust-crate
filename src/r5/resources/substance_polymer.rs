//! SubstancePolymer
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
//!
//! Version: 5.0.0
//!
//! SubstancePolymer Resource: Properties of a substance specific to it being a polymer.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubstancePolymer
///
/// Properties of a substance specific to it being a polymer. This resource
/// captures the structural characteristics of polymeric substances, including
/// the overall polymer class and geometry, the monomer sets and starting
/// materials used in synthesis, and the structural repeat units together with
/// their degree of polymerisation and graphical representations. It is used
/// in pharmaceutical, regulatory, and manufacturing contexts to precisely
/// characterize polymeric excipients and active substances (for example,
/// biologics, coatings, and drug-delivery polymers) so that their chemical
/// identity and structure can be compared, verified, and referenced across
/// systems. It is typically referenced alongside a SubstanceDefinition to
/// describe medicinal or chemical substances that are polymers, and rarely
/// stands alone in clinical workflows.
///
/// Related resources: `SubstanceDefinition` provides the general substance
/// identity that this resource extends with polymer-specific detail, while
/// classifications, connectivity, and unit types throughout this resource
/// are expressed using [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// and structural files are attached using
/// [`Attachment`](crate::r5::types::Attachment).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_polymer::SubstancePolymer;
///
/// let value = SubstancePolymer::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SubstancePolymer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymer {
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

    /// A business identifier for this polymer, but typically this is handled by a SubstanceDefinition identifier
    pub identifier: Option<types::Identifier>,

    /// Overall type of the polymer, e.g. random, block, or graft copolymer
    pub class: Option<types::CodeableConcept>,

    /// Polymer geometry, e.g. linear, branched, cross-linked, network or dendritic
    pub geometry: Option<types::CodeableConcept>,

    /// Descrtibes the copolymer sequence type (polymer connectivity)
    pub copolymer_connectivity: Option<Vec<types::CodeableConcept>>,

    /// Todo - this is intended to connect to a repeating full modification structure, also used by Protein and Nucleic Acid . String is just a placeholder
    pub modification: Option<types::String>,
    /// Primitive extension sibling for [`modification`](Self::modification) (FHIR `_modification`).
    #[serde(rename = "_modification")]
    pub modification_ext: Option<types::Element>,

    /// The monomer sets and their starting materials used to synthesize the polymer
    pub monomer_set: Option<Vec<SubstancePolymerMonomerSet>>,

    /// Specifies and quantifies the structural repeat units and their configuration
    pub repeat: Option<Vec<SubstancePolymerRepeat>>,
}

/// SubstancePolymerMonomerSet
///
/// Todo.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerMonomerSet {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Captures the type of ratio to the entire polymer, e.g. Monomer/Polymer ratio, SRU/Polymer Ratio
    pub ratio_type: Option<types::CodeableConcept>,

    /// The starting materials - monomer(s) used in the synthesis of the polymer
    pub starting_material: Option<Vec<SubstancePolymerMonomerSetStartingMaterial>>,
}

/// SubstancePolymerMonomerSetStartingMaterial
///
/// The starting materials - monomer(s) used in the synthesis of the polymer.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of substance for this starting material
    pub code: Option<types::CodeableConcept>,

    /// Substance high level category, e.g. chemical substance
    pub category: Option<types::CodeableConcept>,

    /// Used to specify whether the attribute described is a defining element for the unique identification of the polymer
    pub is_defining: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_defining`](Self::is_defining) (FHIR `_isDefining`).
    #[serde(rename = "_isDefining")]
    pub is_defining_ext: Option<types::Element>,

    /// A percentage
    pub amount: Option<types::Quantity>,
}

/// SubstancePolymerRepeat
///
/// Specifies and quantifies the repeated units and their configuration.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeat {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A representation of an (average) molecular formula from a polymer
    pub average_molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`average_molecular_formula`](Self::average_molecular_formula) (FHIR `_averageMolecularFormula`).
    #[serde(rename = "_averageMolecularFormula")]
    pub average_molecular_formula_ext: Option<types::Element>,

    /// How the quantitative amount of Structural Repeat Units is captured (e.g. Exact, Numeric, Average)
    pub repeat_unit_amount_type: Option<types::CodeableConcept>,

    /// An SRU - Structural Repeat Unit
    pub repeat_unit: Option<Vec<SubstancePolymerRepeatRepeatUnit>>,
}

/// SubstancePolymerRepeatRepeatUnit
///
/// An SRU - Structural Repeat Unit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeatRepeatUnit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Structural repeat units are essential elements for defining polymers
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`).
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,

    /// The orientation of the polymerisation, e.g. head-tail, head-head, random
    pub orientation: Option<types::CodeableConcept>,

    /// Number of repeats of this unit
    pub amount: Option<types::Integer>,
    /// Primitive extension sibling for [`amount`](Self::amount) (FHIR `_amount`).
    #[serde(rename = "_amount")]
    pub amount_ext: Option<types::Element>,

    /// Applies to homopolymer and block co-polymers where the degree of polymerisation within a block can be described
    pub degree_of_polymerisation: Option<Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>>,

    /// A graphical structure for this SRU
    pub structural_representation: Option<Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>>,
}

/// SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation
///
/// Applies to homopolymer and block co-polymers where the degree of
/// polymerisation within a block can be described.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of the degree of polymerisation shall be described, e.g. SRU/Polymer Ratio
    pub r#type: Option<types::CodeableConcept>,

    /// An average amount of polymerisation
    pub average: Option<types::Integer>,
    /// Primitive extension sibling for [`average`](Self::average) (FHIR `_average`).
    #[serde(rename = "_average")]
    pub average_ext: Option<types::Element>,

    /// A low expected limit of the amount
    pub low: Option<types::Integer>,
    /// Primitive extension sibling for [`low`](Self::low) (FHIR `_low`).
    #[serde(rename = "_low")]
    pub low_ext: Option<types::Element>,

    /// A high expected limit of the amount
    pub high: Option<types::Integer>,
    /// Primitive extension sibling for [`high`](Self::high) (FHIR `_high`).
    #[serde(rename = "_high")]
    pub high_ext: Option<types::Element>,
}

/// SubstancePolymerRepeatRepeatUnitStructuralRepresentation
///
/// A graphical structure for this SRU.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of structure (e.g. Full, Partial, Representative)
    pub r#type: Option<types::CodeableConcept>,

    /// The structural representation as text string in a standard format e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF
    pub representation: Option<types::String>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`).
    #[serde(rename = "_representation")]
    pub representation_ext: Option<types::Element>,

    /// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF
    pub format: Option<types::CodeableConcept>,

    /// An attached file with the structural representation
    pub attachment: Option<types::Attachment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstancePolymer;

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
