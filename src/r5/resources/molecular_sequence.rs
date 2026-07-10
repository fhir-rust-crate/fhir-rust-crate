//! MolecularSequence
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
//!
//! Version: 5.0.0
//!
//! MolecularSequence Resource: Representation of a molecular sequence.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Representation of a molecular sequence.
///
/// The MolecularSequence resource captures a raw or relative molecular sequence
/// (amino acid, DNA, or RNA) along with the subject, specimen, device, and
/// performer context in which it was observed. A sequence may be provided
/// literally, as an embedded/attached file, or defined relative to a known
/// starting sequence together with a set of edits. It is typically used in
/// genomics workflows to share and reason about sequence data in FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::molecular_sequence::MolecularSequence;
///
/// let value = MolecularSequence::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MolecularSequence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence {
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

    /// Unique ID for this particular sequence
    pub identifier: Option<Vec<types::Identifier>>,

    /// aa | dna | rna
    pub r#type: Option<types::Code>,

    /// Subject this sequence is associated too
    pub subject: Option<types::Reference>,

    /// What the molecular sequence is about, when it is not about the subject of record
    pub focus: Option<Vec<types::Reference>>,

    /// Specimen used for sequencing
    pub specimen: Option<types::Reference>,

    /// The method for sequencing
    pub device: Option<types::Reference>,

    /// Who should be responsible for test result
    pub performer: Option<types::Reference>,

    /// Sequence that was observed
    pub literal: Option<types::String>,

    /// Embedded file or a link (URL) which contains content to represent the sequence
    pub formatted: Option<Vec<types::Attachment>>,

    /// A sequence defined relative to another sequence
    pub relative: Option<Vec<MolecularSequenceRelative>>,
}

/// A sequence defined relative to another sequence.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelative {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Ways of identifying nucleotides or amino acids within a sequence
    pub coordinate_system: types::CodeableConcept,

    /// Indicates the order in which the sequence should be considered when putting multiple 'relative' elements together
    pub ordinal_position: Option<types::Integer>,

    /// Indicates the nucleotide range in the composed sequence when multiple 'relative' elements are used together
    pub sequence_range: Option<types::Range>,

    /// A sequence used as starting sequence
    pub starting_sequence: Option<MolecularSequenceRelativeStartingSequence>,

    /// Changes in sequence from the starting sequence
    pub edit: Option<Vec<MolecularSequenceRelativeEdit>>,
}

/// A sequence used as starting sequence.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelativeStartingSequence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The genome assembly used for starting sequence, e.g. GRCh38
    pub genome_assembly: Option<types::CodeableConcept>,

    /// Chromosome Identifier
    pub chromosome: Option<types::CodeableConcept>,

    /// The reference sequence that represents the starting sequence
    pub sequence_codeable_concept: Option<types::CodeableConcept>,

    /// The reference sequence that represents the starting sequence
    pub sequence_string: Option<types::String>,

    /// The reference sequence that represents the starting sequence
    pub sequence_reference: Option<types::Reference>,

    /// Start position of the window on the starting sequence
    pub window_start: Option<types::Integer>,

    /// End position of the window on the starting sequence
    pub window_end: Option<types::Integer>,

    /// sense | antisense
    pub orientation: Option<types::Code>,

    /// watson | crick
    pub strand: Option<types::Code>,
}

/// Changes in sequence from the starting sequence.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelativeEdit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Start position of the edit on the starting sequence
    pub start: Option<types::Integer>,

    /// End position of the edit on the starting sequence
    pub end: Option<types::Integer>,

    /// Allele that was observed
    pub replacement_sequence: Option<types::String>,

    /// Allele in the starting sequence
    pub replaced_sequence: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MolecularSequence;

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
