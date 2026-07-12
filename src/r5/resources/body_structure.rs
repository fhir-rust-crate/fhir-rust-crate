//! BodyStructure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BodyStructure
//!
//! Version: 5.0.0
//!
//! BodyStructure Resource: Record details about an anatomical structure.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// BodyStructure
///
/// Record details about an anatomical structure. This resource may be used when
/// a coded concept does not provide the necessary detail needed for the use
/// case. It describes an anatomical location on or in a patient, optionally
/// including laterality, landmark orientation, qualifiers, and attached images.
///
/// Clinically, `BodyStructure` is commonly referenced by other resources that
/// need to describe precisely where a procedure, observation, specimen
/// collection, or device is located, such as a specific lesion, tumor, or
/// surgical site that cannot be adequately captured by a single coded body
/// site value alone. Rather than repeating a detailed anatomical description
/// on every referencing resource, a `BodyStructure` instance can be created
/// once and then referenced wherever that specific structure needs to be
/// identified, for example from a `Procedure`, `Observation`, `Specimen`, or
/// `ImagingStudy`.
///
/// Related resources / see also: the subject of a `BodyStructure` is a
/// [`Patient`](crate::r5::resources::patient::Patient); the `morphology` and
/// `structure` elements use [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// values (often drawn from SNOMED CT body site value sets); `image` elements
/// use [`Attachment`](crate::r5::types::Attachment); and `spatial_reference`
/// elements use [`Reference`](crate::r5::types::Reference), typically pointing
/// to an `ImagingSelection` or other resource that provides a spatial anchor.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::body_structure::BodyStructure;
///
/// let value = BodyStructure::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: BodyStructure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructure {
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

    /// Bodystructure identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this record is in active use; defaults to true if absent
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Kind of structure, e.g. tumor, lesion, or excised tissue sample type
    pub morphology: Option<types::CodeableConcept>,

    /// Included anatomic location(s) that together define the body structure
    pub included_structure: vec1::Vec1<BodyStructureIncludedStructure>,

    /// Excluded anatomic locations(s)
    pub excluded_structure: Option<Vec<BodyStructureIncludedStructure>>,

    /// Text description of this structure, for additional human-readable detail
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Attached images illustrating or documenting the structure
    pub image: Option<Vec<types::Attachment>>,

    /// The patient this body structure belongs to
    pub patient: types::Reference,
}

/// BodyStructureIncludedStructure
///
/// Included anatomic location(s) that describe the included structure, its
/// laterality, qualifiers, spatial references, and landmark orientation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructureIncludedStructure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that represents the included structure
    pub structure: types::CodeableConcept,

    /// Code that represents the included structure laterality
    pub laterality: Option<types::CodeableConcept>,

    /// Landmark relative location
    pub body_landmark_orientation: Option<Vec<BodyStructureIncludedStructureBodyLandmarkOrientation>>,

    /// Cartesian reference for structure
    pub spatial_reference: Option<Vec<types::Reference>>,

    /// Code that represents the included structure qualifier
    pub qualifier: Option<Vec<types::CodeableConcept>>,
}

/// BodyStructureIncludedStructureBodyLandmarkOrientation
///
/// Landmark relative location describing the body landmark, clockface
/// orientation, distance from a landmark, and surface orientation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Body landmark description
    pub landmark_description: Option<Vec<types::CodeableConcept>>,

    /// Clockface orientation
    pub clock_face_position: Option<Vec<types::CodeableConcept>>,

    /// Landmark relative location
    pub distance_from_landmark: Option<Vec<BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark>>,

    /// Relative landmark surface orientation
    pub surface_orientation: Option<Vec<types::CodeableConcept>>,
}

/// BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark
///
/// Landmark relative location describing the measurement device and the
/// measured distance from a body landmark.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Measurement device
    pub device: Option<Vec<types::CodeableReference>>,

    /// Measured distance from body landmark
    pub value: Option<Vec<types::Quantity>>,
}
