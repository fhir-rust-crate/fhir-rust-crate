//! ImagingSelection
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingSelection
//!
//! Version: 5.0.0
//!
//! ImagingSelection Resource: A selection of DICOM SOP instances and/or frames within a single Study and Series.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ImagingSelection is a selection of DICOM SOP instances and/or frames within a
/// single Study and Series.
///
/// In FHIR R5 this resource captures a curated, addressable subset of medical imaging
/// data by referencing DICOM Study, Series, and SOP Instance UIDs, and optionally narrowing
/// the selection further to individual frames or to specific 2D image regions or 3D regions
/// within a frame of reference. It is used to record clinically or scientifically significant
/// findings, key images, annotations, or measurement targets that a clinician, radiologist,
/// or automated process wishes to highlight, share, or act upon, rather than the entire study.
///
/// A selection may additionally include specifics such as an image region, an Observation
/// UID, or a Segmentation Number, allowing linkage to an Observation Resource or transferring
/// this information along with the ImagingStudy Resource. Typical uses include tumor boards,
/// teaching files, quantitative imaging biomarkers, structured reporting, and clinical
/// decision support, where precise portions of imaging data must be referenced unambiguously
/// across systems while pointing back to the source study and a retrieval endpoint.
///
/// # Related resources
///
/// The imaging data is generally derived from an [`ImagingStudy`](crate::r5::resources::imaging_study::ImagingStudy),
/// the subject is commonly a [`Patient`](crate::r5::resources::patient::Patient), and a
/// selection is frequently referenced by an [`Observation`](crate::r5::resources::observation::Observation).
/// Coded elements use [`CodeableConcept`](crate::r5::types::CodeableConcept) and
/// [`Coding`](crate::r5::types::Coding), while linkages to other resources use
/// [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_selection::ImagingSelection;
///
/// let value = ImagingSelection::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingSelection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelection {
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

    /// Business Identifier for Imaging Selection
    pub identifier: Option<Vec<types::Identifier>>,

    /// Status of the imaging selection: available, entered-in-error, or unknown.
    pub status: types::Code,

    /// Subject of the selected instances, typically the Patient whose images are referenced.
    pub subject: Option<types::Reference>,

    /// Date / Time when this imaging selection was created
    pub issued: Option<types::Instant>,

    /// Selector of the instances (human or machine)
    pub performer: Option<Vec<ImagingSelectionPerformer>>,

    /// Associated request
    pub based_on: Option<Vec<types::Reference>>,

    /// Classifies the imaging selection
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Reason for or purpose of this imaging selection, expressed as text or a coded concept.
    pub code: types::CodeableConcept,

    /// DICOM Study Instance UID identifying the study that contains the selected instances.
    pub study_uid: Option<types::Id>,

    /// The imaging study, commonly an ImagingStudy, from which this selection is derived.
    pub derived_from: Option<Vec<types::Reference>>,

    /// The network service providing retrieval for the images referenced in the imaging selection
    pub endpoint: Option<Vec<types::Reference>>,

    /// DICOM Series Instance UID
    pub series_uid: Option<types::Id>,

    /// DICOM Series Number
    pub series_number: Option<types::UnsignedInt>,

    /// The Frame of Reference UID for the selected images
    pub frame_of_reference_uid: Option<types::Id>,

    /// Body part examined
    pub body_site: Option<types::CodeableReference>,

    /// Related resource that is the focus for the imaging selection
    pub focus: Option<Vec<types::Reference>>,

    /// The selected instances
    pub instance: Option<Vec<ImagingSelectionInstance>>,
}

/// Selector of the instances (human or machine).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of performer
    pub function: Option<types::CodeableConcept>,

    /// Author (human or machine)
    pub actor: Option<types::Reference>,
}

/// The selected instances.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// DICOM SOP Instance UID
    pub uid: types::Id,

    /// DICOM Instance Number
    pub number: Option<types::UnsignedInt>,

    /// DICOM SOP Class UID
    pub sop_class: Option<types::Coding>,

    /// The selected subset of the SOP Instance
    pub subset: Option<Vec<types::String>>,

    /// A specific 2D region in a DICOM image / frame
    pub image_region2_d: Option<Vec<ImagingSelectionInstanceImageRegion2D>>,

    /// A specific 3D region in a DICOM frame of reference
    pub image_region3_d: Option<Vec<ImagingSelectionInstanceImageRegion3D>>,
}

/// A specific 2D region in a DICOM image / frame.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstanceImageRegion2D {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// point | polyline | interpolated | circle | ellipse
    pub region_type: types::Code,

    /// Specifies the coordinates that define the image region
    pub coordinate: Vec<types::Decimal>,
}

/// A specific 3D region in a DICOM frame of reference.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstanceImageRegion3D {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// point | multipoint | polyline | polygon | ellipse | ellipsoid
    pub region_type: types::Code,

    /// Specifies the coordinates that define the image region
    pub coordinate: Vec<types::Decimal>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImagingSelection;

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
