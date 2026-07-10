//! GenomicStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GenomicStudy
//!
//! Version: 5.0.0
//!
//! GenomicStudy Resource: A set of analyses performed to analyze and generate genomic data.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A set of analyses performed to analyze and generate genomic data.
///
/// A GenomicStudy resource captures a set of one or more genomic analyses
/// performed for a subject, such as a patient, and the data those analyses
/// produce. Each analysis records the methods used, the genomic changes and
/// regions studied, the specimens and devices involved, the performers, and the
/// input and output files. In FHIR R5 it provides a container that ties together
/// the workflow, provenance, and results of genomic testing.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::genomic_study::GenomicStudy;
///
/// let value = GenomicStudy::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: GenomicStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudy {
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

    /// Identifiers for this genomic study
    pub identifier: Option<Vec<types::Identifier>>,

    /// registered | available | cancelled | entered-in-error | unknown
    pub status: types::Code,

    /// The type of the study (e.g., Familial variant segregation, Functional variation detection, or Gene expression profiling)
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// The primary subject of the genomic study
    pub subject: types::Reference,

    /// The healthcare event with which this genomics study is associated
    pub encounter: Option<types::Reference>,

    /// When the genomic study was started
    pub start_date: Option<types::DateTime>,

    /// Event resources that the genomic study is based on
    pub based_on: Option<Vec<types::Reference>>,

    /// Healthcare professional who requested or referred the genomic study
    pub referrer: Option<types::Reference>,

    /// Healthcare professionals who interpreted the genomic study
    pub interpreter: Option<Vec<types::Reference>>,

    /// Why the genomic study was performed
    pub reason: Option<Vec<types::CodeableReference>>,

    /// The defined protocol that describes the study
    pub instantiates_canonical: Option<types::Canonical>,

    /// The URL pointing to an externally maintained protocol that describes the study
    pub instantiates_uri: Option<types::Uri>,

    /// Comments related to the genomic study
    pub note: Option<Vec<types::Annotation>>,

    /// Description of the genomic study
    pub description: Option<types::Markdown>,

    /// Genomic Analysis Event
    pub analysis: Option<Vec<GenomicStudyAnalysis>>,
}

/// Genomic Analysis Event
///
/// A single genomic analysis performed as part of the study, capturing the
/// methods, genomic changes, regions, specimens, devices, performers, and the
/// input and output files associated with that analysis.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifiers for the analysis event
    pub identifier: Option<Vec<types::Identifier>>,

    /// Type of the methods used in the analysis (e.g., FISH, Karyotyping, MSI)
    pub method_type: Option<Vec<types::CodeableConcept>>,

    /// Type of the genomic changes studied in the analysis (e.g., DNA, RNA, or AA change)
    pub change_type: Option<Vec<types::CodeableConcept>>,

    /// Genome build that is used in this analysis
    pub genome_build: Option<types::CodeableConcept>,

    /// The defined protocol that describes the analysis
    pub instantiates_canonical: Option<types::Canonical>,

    /// The URL pointing to an externally maintained protocol that describes the analysis
    pub instantiates_uri: Option<types::Uri>,

    /// Name of the analysis event (human friendly)
    pub title: Option<types::String>,

    /// What the genomic analysis is about, when it is not about the subject of record
    pub focus: Option<Vec<types::Reference>>,

    /// The specimen used in the analysis event
    pub specimen: Option<Vec<types::Reference>>,

    /// The date of the analysis event
    pub date: Option<types::DateTime>,

    /// Any notes capture with the analysis event
    pub note: Option<Vec<types::Annotation>>,

    /// The protocol that was performed for the analysis event
    pub protocol_performed: Option<types::Reference>,

    /// The genomic regions to be studied in the analysis (BED file)
    pub regions_studied: Option<Vec<types::Reference>>,

    /// Genomic regions actually called in the analysis event (BED file)
    pub regions_called: Option<Vec<types::Reference>>,

    /// Inputs for the analysis event
    pub input: Option<Vec<GenomicStudyAnalysisInput>>,

    /// Outputs for the analysis event
    pub output: Option<Vec<GenomicStudyAnalysisOutput>>,

    /// Performer for the analysis event
    pub performer: Option<Vec<GenomicStudyAnalysisPerformer>>,

    /// Devices used for the analysis (e.g., instruments, software), with settings and parameters
    pub device: Option<Vec<GenomicStudyAnalysisDevice>>,
}

/// Inputs for the analysis event
///
/// An input file consumed by the analysis event, along with its data type and
/// the analysis event or GenomicStudy that generated it.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// File containing input data
    pub file: Option<types::Reference>,

    /// Type of input data (e.g., BAM, CRAM, or FASTA)
    pub r#type: Option<types::CodeableConcept>,

    /// The analysis event or other GenomicStudy that generated this input file
    pub generated_by_identifier: Option<types::Identifier>,

    /// The analysis event or other GenomicStudy that generated this input file
    pub generated_by_reference: Option<types::Reference>,
}

/// Outputs for the analysis event
///
/// An output file produced by the analysis event, along with its data type.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// File containing output data
    pub file: Option<types::Reference>,

    /// Type of output data (e.g., VCF, MAF, or BAM)
    pub r#type: Option<types::CodeableConcept>,
}

/// Performer for the analysis event
///
/// A participant in performing the analysis event, described by the actor and
/// the role that actor played.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The organization, healthcare professional, or others who participated in performing this analysis
    pub actor: Option<types::Reference>,

    /// Role of the actor for this analysis
    pub role: Option<types::CodeableConcept>,
}

/// Devices used for the analysis
///
/// A device (such as an instrument or software) used for the analysis event,
/// together with the specific function it performed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenomicStudyAnalysisDevice {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Device used for the analysis
    pub device: Option<types::Reference>,

    /// Specific function for the device used for the analysis
    pub function: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = GenomicStudy;

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
