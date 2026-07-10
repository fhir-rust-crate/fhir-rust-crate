//! ImagingStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
//!
//! Version: 5.0.0
//!
//! ImagingStudy Resource: Representation of the content produced in a DICOM imaging study.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Representation of the content produced in a DICOM imaging study.
///
/// An ImagingStudy comprises a set of series, each of which includes a set of
/// Service-Object Pair Instances (SOP Instances - images or other data) acquired
/// or produced in a common context. A series is of only one modality (e.g.
/// X-ray, CT, MR, ultrasound), but a study may have multiple series of different
/// modalities. In FHIR R5 the resource is used to catalog imaging content and to
/// provide access endpoints for retrieval from PACS and other DICOM systems.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_study::ImagingStudy;
///
/// let value = ImagingStudy::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudy {
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

    /// Identifiers for the whole study
    pub identifier: Option<Vec<types::Identifier>>,

    /// registered | available | cancelled | entered-in-error | unknown
    pub status: types::Code,

    /// All of the distinct values for series' modalities
    pub modality: Option<Vec<types::CodeableConcept>>,

    /// Who or what is the subject of the study
    pub subject: types::Reference,

    /// Encounter with which this imaging study is associated
    pub encounter: Option<types::Reference>,

    /// When the study was started
    pub started: Option<types::DateTime>,

    /// Request fulfilled
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// Referring physician
    pub referrer: Option<types::Reference>,

    /// Study access endpoint
    pub endpoint: Option<Vec<types::Reference>>,

    /// Number of Study Related Series
    pub number_of_series: Option<types::UnsignedInt>,

    /// Number of Study Related Instances
    pub number_of_instances: Option<types::UnsignedInt>,

    /// The performed procedure or code
    pub procedure: Option<Vec<types::CodeableReference>>,

    /// Where ImagingStudy occurred
    pub location: Option<types::Reference>,

    /// Why the study was requested / performed
    pub reason: Option<Vec<types::CodeableReference>>,

    /// User-defined comments
    pub note: Option<Vec<types::Annotation>>,

    /// Institution-generated description
    pub description: Option<types::String>,

    /// Each study has one or more series of instances
    pub series: Option<Vec<ImagingStudySeries>>,
}

/// Each study has one or more series of instances.
///
/// A series is of only one modality (e.g. X-ray, CT, MR, ultrasound) and groups
/// together the SOP instances acquired or produced in a common context.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudySeries {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// DICOM Series Instance UID for the series
    pub uid: types::Id,

    /// Numeric identifier of this series
    pub number: Option<types::UnsignedInt>,

    /// The modality used for this series
    pub modality: types::CodeableConcept,

    /// A short human readable summary of the series
    pub description: Option<types::String>,

    /// Number of Series Related Instances
    pub number_of_instances: Option<types::UnsignedInt>,

    /// Series access endpoint
    pub endpoint: Option<Vec<types::Reference>>,

    /// Body part examined
    pub body_site: Option<types::CodeableReference>,

    /// Body part laterality
    pub laterality: Option<types::CodeableConcept>,

    /// Specimen imaged
    pub specimen: Option<Vec<types::Reference>>,

    /// When the series started
    pub started: Option<types::DateTime>,

    /// Who performed the series
    pub performer: Option<Vec<ImagingStudySeriesPerformer>>,

    /// A single SOP instance from the series
    pub instance: Option<Vec<ImagingStudySeriesInstance>>,
}

/// Who performed the series.
///
/// Indicates who or what performed the series and, optionally, how they were
/// involved via a function code.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudySeriesPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who performed the series
    pub actor: types::Reference,
}

/// A single SOP instance from the series.
///
/// Describes an individual Service-Object Pair (SOP) Instance such as an image
/// or other DICOM data object contained within the series.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudySeriesInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// DICOM SOP Instance UID
    pub uid: types::Id,

    /// DICOM class type
    pub sop_class: types::Coding,

    /// The number of this instance in the series
    pub number: Option<types::UnsignedInt>,

    /// Description of instance
    pub title: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImagingStudy;

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
