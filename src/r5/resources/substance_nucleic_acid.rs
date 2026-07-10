//! SubstanceNucleicAcid
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceNucleicAcid
//!
//! Version: 5.0.0
//!
//! SubstanceNucleicAcid Resource: Nucleic acids are defined by three distinct elements: the base, sugar and linkage.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// SubstanceNucleicAcid describes the chemical structure of a nucleic acid
/// substance. Nucleic acids are defined by three distinct elements: the base,
/// sugar and linkage, and individual substance/moiety IDs are created for each
/// of these elements. The nucleotide sequence is always entered in the 5'-3'
/// direction. This resource is primarily used in medicinal product regulatory
/// submissions to characterize substances such as oligonucleotides.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::substance_nucleic_acid::SubstanceNucleicAcid;
///
/// let value = SubstanceNucleicAcid::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SubstanceNucleicAcid = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcid {
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

    /// The type of the sequence shall be specified based on a controlled vocabulary
    pub sequence_type: Option<types::CodeableConcept>,

    /// The number of linear sequences of nucleotides linked through phosphodiester bonds
    pub number_of_subunits: Option<types::Integer>,

    /// The area of hybridisation shall be described if applicable for double stranded RNA or DNA
    pub area_of_hybridisation: Option<types::String>,

    /// (TBC)
    pub oligo_nucleotide_type: Option<types::CodeableConcept>,

    /// Subunits are listed in order of decreasing length
    pub subunit: Option<Vec<SubstanceNucleicAcidSubunit>>,
}

/// Subunits are listed in order of decreasing length; sequences of the same
/// length will be ordered by molecular weight; subunits that have identical
/// sequences will be repeated multiple times.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Index of linear sequences of nucleic acids in order of decreasing length
    pub subunit: Option<types::Integer>,

    /// Actual nucleotide sequence notation from 5' to 3' end using standard single letter codes
    pub sequence: Option<types::String>,

    /// The length of the sequence shall be captured
    pub length: Option<types::Integer>,

    /// (TBC)
    pub sequence_attachment: Option<types::Attachment>,

    /// The nucleotide present at the 5' terminal shall be specified based on a controlled vocabulary
    pub five_prime: Option<types::CodeableConcept>,

    /// The nucleotide present at the 3' terminal shall be specified based on a controlled vocabulary
    pub three_prime: Option<types::CodeableConcept>,

    /// The linkages between sugar residues will also be captured
    pub linkage: Option<Vec<SubstanceNucleicAcidSubunitLinkage>>,

    /// 5.3.6.8.1 Sugar ID (Mandatory)
    pub sugar: Option<Vec<SubstanceNucleicAcidSubunitSugar>>,
}

/// The linkages between sugar residues will also be captured.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunitLinkage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The entity that links the sugar residues together
    pub connectivity: Option<types::String>,

    /// Each linkage will be registered as a fragment and have an ID
    pub identifier: Option<types::Identifier>,

    /// Each linkage will be registered as a fragment and have at least one name
    pub name: Option<types::String>,

    /// Residues shall be captured as described in 5.3.6.8.3
    pub residue_site: Option<types::String>,
}

/// 5.3.6.8.1 Sugar ID (Mandatory).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunitSugar {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The Substance ID of the sugar or sugar-like component that make up the nucleotide
    pub identifier: Option<types::Identifier>,

    /// The name of the sugar or sugar-like component that make up the nucleotide
    pub name: Option<types::String>,

    /// The residues that contain a given sugar will be captured
    pub residue_site: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceNucleicAcid;

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
