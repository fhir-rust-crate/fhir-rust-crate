//! SubstanceProtein
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceProtein
//!
//! Version: 5.0.0
//!
//! SubstanceProtein Resource: A single unit of a linear amino acid sequence, or a combination of covalently linked or defined stoichiometric subunits.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubstanceProtein
///
/// A SubstanceProtein is defined as a single unit of a linear amino acid
/// sequence, or a combination of subunits that are either covalently linked or
/// have a defined invariant stoichiometric relationship. This includes all
/// synthetic, recombinant and purified SubstanceProteins of defined sequence,
/// whether the use is therapeutic or prophylactic. It is used to describe
/// albumins, coagulation factors, cytokines, growth factors, and similar
/// biological substances.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_protein::SubstanceProtein;
///
/// let value = SubstanceProtein::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SubstanceProtein = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceProtein {
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

    /// The SubstanceProtein descriptive elements will only be used when a complete or partial amino acid sequence is available or derivable from a nucleic acid sequence
    pub sequence_type: Option<types::CodeableConcept>,

    /// Number of linear sequences of amino acids linked through peptide bonds
    pub number_of_subunits: Option<types::Integer>,

    /// The disulphide bond between two cysteine residues shall be described
    pub disulfide_linkage: Option<Vec<types::String>>,

    /// This subclause refers to the description of each subunit constituting the SubstanceProtein
    pub subunit: Option<Vec<SubstanceProteinSubunit>>,
}

/// SubstanceProteinSubunit
///
/// This subclause refers to the description of each subunit constituting the
/// SubstanceProtein. A subunit is a linear sequence of amino acids linked
/// through peptide bonds.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceProteinSubunit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Index of primary sequences of amino acids linked through peptide bonds in order of decreasing length
    pub subunit: Option<types::Integer>,

    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes
    pub sequence: Option<types::String>,

    /// Length of linear sequences of amino acids contained in the subunit
    pub length: Option<types::Integer>,

    /// The sequence information shall be provided as an attachment
    pub sequence_attachment: Option<types::Attachment>,

    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    pub n_terminal_modification_id: Option<types::Identifier>,

    /// The name of the fragment modified at the N-terminal of the SubstanceProtein shall be specified
    pub n_terminal_modification: Option<types::String>,

    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    pub c_terminal_modification_id: Option<types::Identifier>,

    /// The modification at the C-terminal shall be specified
    pub c_terminal_modification: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceProtein;

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
