//! DeviceAssociation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceAssociation
//!
//! Version: 5.0.0
//!
//! DeviceAssociation Resource: A record of association of a device.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The DeviceAssociation resource records the association of a device with a
/// patient, group, or other subject over a period of time. It captures the
/// relationship between a physical device and the individual it is implanted in,
/// attached to, or otherwise associated with, along with the current status of
/// that association and its anatomical location. It is commonly used to track
/// implantable, wearable, and attached devices throughout their lifecycle.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_association::DeviceAssociation;
///
/// let value = DeviceAssociation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DeviceAssociation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAssociation {
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

    /// Instance identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Reference to the devices associated with the patient or group
    pub device: types::Reference,

    /// Describes the relationship between the device and subject
    pub category: Option<Vec<types::CodeableConcept>>,

    /// implanted | explanted | attached | entered-in-error | unknown
    pub status: types::CodeableConcept,

    /// The reasons given for the current association status
    pub status_reason: Option<Vec<types::CodeableConcept>>,

    /// The individual, group of individuals or device that the device is on or associated with
    pub subject: Option<types::Reference>,

    /// Current anatomical location of the device in/on subject
    pub body_structure: Option<types::Reference>,

    /// Begin and end dates and times for the device association
    pub period: Option<types::Period>,

    /// The details about the device when it is in use to describe its operation
    pub operation: Option<Vec<DeviceAssociationOperation>>,
}

/// The details about the device when it is in use to describe its operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAssociationOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Device operational condition
    pub status: types::CodeableConcept,

    /// The individual performing the action enabled by the device
    pub operator: Option<Vec<types::Reference>>,

    /// Begin and end dates and times for the device's operation
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceAssociation;

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
