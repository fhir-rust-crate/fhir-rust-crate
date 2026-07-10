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
use fhir_derive::Validate;

/// ImagingSelection is a selection of DICOM SOP instances and/or frames within a
/// single Study and Series.
///
/// This might include additional specifics such as an image region, an Observation
/// UID or a Segmentation Number, allowing linkage to an Observation Resource or
/// transferring this information along with the ImagingStudy Resource. It supports
/// referencing precise portions of imaging data for clinical and research workflows.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::imaging_selection::ImagingSelection;
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

    /// available | entered-in-error | unknown
    pub status: types::Code,

    /// Subject of the selected instances
    pub subject: Option<types::Reference>,

    /// Date / Time when this imaging selection was created
    pub issued: Option<types::Instant>,

    /// Selector of the instances (human or machine)
    pub performer: Option<Vec<ImagingSelectionPerformer>>,

    /// Associated request
    pub based_on: Option<Vec<types::Reference>>,

    /// Classifies the imaging selection
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Imaging Selection purpose text or code
    pub code: types::CodeableConcept,

    /// DICOM Study Instance UID
    pub study_uid: Option<types::Id>,

    /// The imaging study from which the imaging selection is derived
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
