//! VisionPrescription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
//!
//! Version: 5.0.0
//!
//! VisionPrescription Resource: An authorization for the provision of glasses and/or contact lenses to a patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
///
/// A VisionPrescription records the details of an eye examination that results
/// in the authorization of corrective lenses. It captures the prescriber, the
/// patient, the date the prescription was written, and one or more lens
/// specifications describing the optical parameters for each eye. In FHIR R5 it
/// supports both spectacle and contact lens prescriptions and is commonly used
/// in claims, dispensing, and ordering workflows.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::vision_prescription::VisionPrescription;
///
/// let value = VisionPrescription::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: VisionPrescription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescription {
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

    /// Business Identifier for vision prescription
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | cancelled | draft | entered-in-error
    pub status: types::Code,

    /// Response creation date
    pub created: types::DateTime,

    /// Who prescription is for
    pub patient: types::Reference,

    /// Created during encounter / admission / stay
    pub encounter: Option<types::Reference>,

    /// When prescription was authorized
    pub date_written: types::DateTime,

    /// Who authorized the vision prescription
    pub prescriber: types::Reference,

    /// Vision lens authorization
    pub lens_specification: Vec<VisionPrescriptionLensSpecification>,
}

/// Vision lens authorization.
///
/// Each lens specification contains the optical parameters authorized for a
/// single eye, including sphere, cylinder, axis, prism, and contact lens
/// attributes.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescriptionLensSpecification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Product to be supplied
    pub product: types::CodeableConcept,

    /// right | left
    pub eye: types::Code,

    /// Power of the lens
    pub sphere: Option<types::Decimal>,

    /// Lens power for astigmatism
    pub cylinder: Option<types::Decimal>,

    /// Lens meridian which contain no power for astigmatism
    pub axis: Option<types::Integer>,

    /// Eye alignment compensation
    pub prism: Option<Vec<VisionPrescriptionLensSpecificationPrism>>,

    /// Added power for multifocal levels
    pub add: Option<types::Decimal>,

    /// Contact lens power
    pub power: Option<types::Decimal>,

    /// Contact lens back curvature
    pub back_curve: Option<types::Decimal>,

    /// Contact lens diameter
    pub diameter: Option<types::Decimal>,

    /// Lens wear duration
    pub duration: Option<types::Quantity>,

    /// Color required
    pub color: Option<types::String>,

    /// Brand required
    pub brand: Option<types::String>,

    /// Notes for coatings
    pub note: Option<Vec<types::Annotation>>,
}

/// Eye alignment compensation.
///
/// Describes the amount and base direction of prism correction applied to a
/// lens to compensate for eye alignment.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescriptionLensSpecificationPrism {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Amount of adjustment
    pub amount: types::Decimal,

    /// up | down | in | out
    pub base: types::Code,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = VisionPrescription;

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
