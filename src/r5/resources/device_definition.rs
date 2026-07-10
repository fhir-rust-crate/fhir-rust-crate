//! DeviceDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
//!
//! Version: 5.0.0
//!
//! DeviceDefinition Resource: This is a specialized resource that defines the characteristics and capabilities of a device.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
///
/// The DeviceDefinition resource describes the "kind" of device, in contrast to
/// the Device resource which represents a specific instance. It captures
/// manufacturer information, identifiers, classifications, versions, properties,
/// materials, packaging, and regulatory details that are common to all instances
/// of that device model. It is used to catalog and reference device models across
/// clinical, administrative, and supply workflows.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinition;
///
/// let value = DeviceDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DeviceDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition {
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

    /// Additional information to describe the device
    pub description: Option<types::Markdown>,

    /// Instance identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Unique Device Identifier (UDI) Barcode string
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifier>>,

    /// Regulatory identifier(s) associated with this device
    pub regulatory_identifier: Option<Vec<DeviceDefinitionRegulatoryIdentifier>>,

    /// The part number or catalog number of the device
    pub part_number: Option<types::String>,

    /// Name of device manufacturer
    pub manufacturer: Option<types::Reference>,

    /// The name or names of the device as given by the manufacturer
    pub device_name: Option<Vec<DeviceDefinitionDeviceName>>,

    /// The catalog or model number for the device for example as defined by the manufacturer
    pub model_number: Option<types::String>,

    /// What kind of device or device system this is
    pub classification: Option<Vec<DeviceDefinitionClassification>>,

    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    pub conforms_to: Option<Vec<DeviceDefinitionConformsTo>>,

    /// A device, part of the current one
    pub has_part: Option<Vec<DeviceDefinitionHasPart>>,

    /// Information about the packaging of the device, i.e. how the device is packaged
    pub packaging: Option<Vec<DeviceDefinitionPackaging>>,

    /// The version of the device or software
    pub version: Option<Vec<DeviceDefinitionVersion>>,

    /// Safety characteristics of the device
    pub safety: Option<Vec<types::CodeableConcept>>,

    /// Shelf Life and storage information
    pub shelf_life_storage: Option<Vec<types::ProductShelfLife>>,

    /// Language code for the human-readable text strings produced by the device (all supported)
    pub language_code: Option<Vec<types::CodeableConcept>>,

    /// Inherent, essentially fixed, characteristics of this kind of device, e.g., time properties, size, etc
    pub property: Option<Vec<DeviceDefinitionProperty>>,

    /// Organization responsible for device
    pub owner: Option<types::Reference>,

    /// Details for human/organization for support
    pub contact: Option<Vec<types::ContactPoint>>,

    /// An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device
    pub link: Option<Vec<DeviceDefinitionLink>>,

    /// Device notes and comments
    pub note: Option<Vec<types::Annotation>>,

    /// A substance used to create the material(s) of which the device is made
    pub material: Option<Vec<DeviceDefinitionMaterial>>,

    /// lot-number | manufactured-date | serial-number | expiration-date | biological-source | software-version
    pub production_identifier_in_udi: Option<Vec<types::Code>>,

    /// Information aimed at providing directions for the usage of this model of device
    pub guideline: Option<DeviceDefinitionGuideline>,

    /// Tracking of latest field safety corrective action
    pub corrective_action: Option<DeviceDefinitionCorrectiveAction>,

    /// Billing code or reference associated with the device
    pub charge_item: Option<Vec<DeviceDefinitionChargeItem>>,
}

/// Unique Device Identifier (UDI) Barcode string.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdiction provided in the DeviceDefinition.udiDeviceIdentifier
    pub device_identifier: types::String,

    /// The organization that assigns the identifier algorithm
    pub issuer: types::Uri,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: types::Uri,

    /// Indicates whether and when the device is available on the market
    pub market_distribution: Option<Vec<DeviceDefinitionUdiDeviceIdentifierMarketDistribution>>,
}

/// Indicates whether and when the device is available on the market.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionUdiDeviceIdentifierMarketDistribution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Begin and end dates for the commercial distribution of the device
    pub market_period: types::Period,

    /// National state or territory where the device is commercialized
    pub sub_jurisdiction: types::Uri,
}

/// Regulatory identifier(s) associated with this device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionRegulatoryIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// basic | master | license
    pub r#type: types::Code,

    /// The identifier itself
    pub device_identifier: types::String,

    /// The organization that issued this identifier
    pub issuer: types::Uri,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: types::Uri,
}

/// The name or names of the device as given by the manufacturer.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionDeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A name that is used to refer to the device
    pub name: types::String,

    /// registered-name | user-friendly-name | patient-reported-name
    pub r#type: types::Code,
}

/// What kind of device or device system this is.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionClassification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A classification or risk class of the device model
    pub r#type: types::CodeableConcept,

    /// Further information qualifying this classification of the device model
    pub justification: Option<Vec<types::RelatedArtifact>>,
}

/// Identifies the standards, specifications, or formal guidances for the
/// capabilities supported by the device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionConformsTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Describes the common type of the standard, specification, or formal guidance
    pub category: Option<types::CodeableConcept>,

    /// Identifies the standard, specification, or formal guidance that the device adheres to the Device Specification type
    pub specification: types::CodeableConcept,

    /// The specific form or variant of the standard, specification or formal guidance
    pub version: Option<Vec<types::String>>,

    /// Standard, regulation, certification, or guidance website, document, or other publication, or similar, supporting the conformance
    pub source: Option<Vec<types::RelatedArtifact>>,
}

/// A device, part of the current one.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionHasPart {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to the part
    pub reference: types::Reference,

    /// Number of occurrences of the part
    pub count: Option<types::Integer>,
}

/// Information about the packaging of the device, i.e. how the device is packaged.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionPackaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Business identifier of the packaged medication
    pub identifier: Option<types::Identifier>,

    /// A code that defines the specific type of packaging
    pub r#type: Option<types::CodeableConcept>,

    /// The number of items contained in the package (devices or sub-packages)
    pub count: Option<types::Integer>,

    /// An organization that distributes the packaged device
    pub distributor: Option<Vec<DeviceDefinitionPackagingDistributor>>,

    /// Unique Device Identifier (UDI) Barcode string on the packaging
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifier>>,

    /// Allows packages within packages
    pub packaging: Option<Vec<DeviceDefinitionPackaging>>,
}

/// An organization that distributes the packaged device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionPackagingDistributor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Distributor's human-readable name
    pub name: Option<types::String>,

    /// Distributor as an Organization resource
    pub organization_reference: Option<Vec<types::Reference>>,
}

/// The version of the device or software.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionVersion {
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

    /// The version text
    pub value: types::String,
}

/// Inherent, essentially fixed, characteristics of this kind of device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code that specifies the property being represented
    pub r#type: types::CodeableConcept,

    /// Value of the property (Quantity)
    pub value_quantity: Option<types::Quantity>,

    /// Value of the property (CodeableConcept)
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value of the property (string)
    pub value_string: Option<types::String>,

    /// Value of the property (boolean)
    pub value_boolean: Option<types::Boolean>,

    /// Value of the property (integer)
    pub value_integer: Option<types::Integer>,

    /// Value of the property (Range)
    pub value_range: Option<types::Range>,

    /// Value of the property (Attachment)
    pub value_attachment: Option<types::Attachment>,
}

/// An associated device, linking a previous or new device model to the focal
/// device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type indicates the relationship of the related device to the device instance
    pub relation: types::Coding,

    /// A reference to the linked device
    pub related_device: types::CodeableReference,
}

/// A substance used to create the material(s) of which the device is made.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A relevant substance that the device contains, may contain, or is made of
    pub substance: types::CodeableConcept,

    /// Indicates an alternative material of the device
    pub alternate: Option<types::Boolean>,

    /// Whether the substance is a known or suspected allergen
    pub allergenic_indicator: Option<types::Boolean>,
}

/// Information aimed at providing directions for the usage of this model of
/// device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The circumstances that form the setting for using the device
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Detailed written and visual directions for the user on how to use the device
    pub usage_instruction: Option<types::Markdown>,

    /// A source of information or reference for this guideline
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// A clinical condition for which the device was designed to be used
    pub indication: Option<Vec<types::CodeableConcept>>,

    /// A specific situation when a device should not be used because it may cause harm
    pub contraindication: Option<Vec<types::CodeableConcept>>,

    /// Specific hazard alert information that a user needs to know before using the device
    pub warning: Option<Vec<types::CodeableConcept>>,

    /// A description of the general purpose or medical use of the device or its function
    pub intended_use: Option<types::String>,
}

/// Tracking of latest field safety corrective action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionCorrectiveAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether the corrective action was a recall
    pub recall: types::Boolean,

    /// model | lot-numbers | serial-numbers
    pub scope: Option<types::Code>,

    /// Start and end dates of the corrective action
    pub period: types::Period,
}

/// Billing code or reference associated with the device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionChargeItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The code or reference for the charge item
    pub charge_item_code: types::CodeableReference,

    /// Coefficient applicable to the billing code
    pub count: types::Quantity,

    /// A specific time period in which this charge item applies
    pub effective_period: Option<types::Period>,

    /// The context to which this charge item applies
    pub use_context: Option<Vec<types::UsageContext>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceDefinition;

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
