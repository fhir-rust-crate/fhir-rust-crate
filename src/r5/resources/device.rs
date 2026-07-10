//! Device
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Device
//!
//! Version: 5.0.0
//!
//! Device Resource: describes the properties, administration, and type of a physical unit.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity.
///
/// The Device resource describes the regulated, administrative, and type
/// properties of a physical unit such as a knee replacement, blood pressure
/// cuff, or MRI. These values typically do not change much within a given
/// physical unit, for example the serial number, manufacturer name, and model
/// number.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device::Device;
///
/// let value = Device::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Device = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Device {
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

    /// The name used to display by default when the device is referenced
    pub display_name: Option<types::String>,

    /// The reference to the definition for the device
    pub definition: Option<types::CodeableReference>,

    /// Unique Device Identifier (UDI) Barcode string
    pub udi_carrier: Option<Vec<DeviceUdiCarrier>>,

    /// active | inactive | entered-in-error
    pub status: Option<types::Code>,

    /// lost | damaged | destroyed | available
    pub availability_status: Option<types::CodeableConcept>,

    /// An identifier that supports traceability to the biological source event
    pub biological_source_event: Option<types::Identifier>,

    /// Name of device manufacturer
    pub manufacturer: Option<types::String>,

    /// Date when the device was made
    pub manufacture_date: Option<types::DateTime>,

    /// Date and time of expiry of this device (if applicable)
    pub expiration_date: Option<types::DateTime>,

    /// Lot number of manufacture
    pub lot_number: Option<types::String>,

    /// Serial number assigned by the manufacturer
    pub serial_number: Option<types::String>,

    /// The name or names of the device as known to the manufacturer and/or patient
    pub name: Option<Vec<DeviceName>>,

    /// The manufacturer's model number for the device
    pub model_number: Option<types::String>,

    /// The part number or catalog number of the device
    pub part_number: Option<types::String>,

    /// Indicates a high-level grouping of the device
    pub category: Option<Vec<types::CodeableConcept>>,

    /// The kind or type of device
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// The actual design of the device or software version running on the device
    pub version: Option<Vec<DeviceVersion>>,

    /// Identifies the standards, specifications, or formal guidances supported by the device
    pub conforms_to: Option<Vec<DeviceConformsTo>>,

    /// Inherent, essentially fixed, characteristics of the device
    pub property: Option<Vec<DeviceProperty>>,

    /// The designated condition for performing a task
    pub mode: Option<types::CodeableConcept>,

    /// The series of occurrences that repeats during the operation of the device
    pub cycle: Option<types::Count>,

    /// A measurement of time during the device's operation
    pub duration: Option<types::Duration>,

    /// Organization responsible for device
    pub owner: Option<types::Reference>,

    /// Details for human/organization for support
    pub contact: Option<Vec<types::ContactPoint>>,

    /// Where the device is found
    pub location: Option<types::Reference>,

    /// Network address to contact device
    pub url: Option<types::Uri>,

    /// Technical endpoints providing access to electronic services provided by the device
    pub endpoint: Option<Vec<types::Reference>>,

    /// Linked device acting as a communication/data collector, translator or controller
    pub gateway: Option<Vec<types::CodeableReference>>,

    /// Device notes and comments
    pub note: Option<Vec<types::Annotation>>,

    /// Safety Characteristics of Device
    pub safety: Option<Vec<types::CodeableConcept>>,

    /// The higher level or encompassing device that this device is a logical part of
    pub parent: Option<types::Reference>,
}

/// Unique Device Identifier (UDI) Barcode string.
///
/// Carries the unique device identifier information as encoded on a barcode
/// or RFID tag, including the mandatory device identifier and issuing
/// organization.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUdiCarrier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Mandatory fixed portion of UDI
    pub device_identifier: types::String,

    /// UDI Issuing Organization
    pub issuer: types::Uri,

    /// Regional UDI authority
    pub jurisdiction: Option<types::Uri>,

    /// UDI Machine Readable Barcode String
    pub carrier_aidc: Option<types::Base64Binary>,

    /// UDI Human Readable Barcode String
    pub carrier_hrf: Option<types::String>,

    /// barcode | rfid | manual | card | self-reported | electronic-transmission | unknown
    pub entry_type: Option<types::Code>,
}

/// The name or names of the device as known to the manufacturer and/or patient.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The term that names the device
    pub value: types::String,

    /// registered-name | user-friendly-name | patient-reported-name
    pub r#type: types::Code,

    /// The preferred device name
    pub display: Option<types::Boolean>,
}

/// The actual design of the device or software version running on the device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of the device version, e.g. manufacturer, approved, internal
    pub r#type: Option<types::CodeableConcept>,

    /// The hardware or software module of the device to which the version applies
    pub component: Option<types::Identifier>,

    /// The date the version was installed on the device
    pub install_date: Option<types::DateTime>,

    /// The version text
    pub value: types::String,
}

/// Identifies the standards, specifications, or formal guidances for the
/// capabilities supported by the device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConformsTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Describes the common type of the standard, specification, or formal guidance
    pub category: Option<types::CodeableConcept>,

    /// Identifies the standard, specification, or formal guidance that the device adheres to
    pub specification: types::CodeableConcept,

    /// Specific form or variant of the standard
    pub version: Option<types::String>,
}

/// Inherent, essentially fixed, characteristics of the device, e.g. time
/// properties, size, material.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that specifies the property being represented
    pub r#type: types::CodeableConcept,

    /// Value of the property
    pub value_quantity: Option<types::Quantity>,

    /// Value of the property
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value of the property
    pub value_string: Option<types::String>,

    /// Value of the property
    pub value_boolean: Option<types::Boolean>,

    /// Value of the property
    pub value_integer: Option<types::Integer>,

    /// Value of the property
    pub value_range: Option<types::Range>,

    /// Value of the property
    pub value_attachment: Option<types::Attachment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Device;

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
