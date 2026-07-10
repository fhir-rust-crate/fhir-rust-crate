//! DeviceMetric
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
//!
//! Version: 5.0.0
//!
//! DeviceMetric Resource: Describes a measurement, calculation or setting capability of a device.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Describes a measurement, calculation or setting capability of a device.
///
/// The DeviceMetric resource describes a single measurement, calculation, or
/// setting capability of a medical device, such as a heart rate reading or a
/// PEEP setting on a ventilator. It is derived from the ISO/IEEE 11073-10201
/// Domain Information Model standard but is more widely applicable. Each metric
/// links to its parent Device and can describe its unit of measure, operational
/// status, category, measurement frequency, and any calibrations performed.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_metric::DeviceMetric;
///
/// let value = DeviceMetric::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DeviceMetric = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetric {
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

    /// Identity of metric, for example Heart Rate or PEEP Setting
    pub r#type: types::CodeableConcept,

    /// Unit of Measure for the Metric
    pub unit: Option<types::CodeableConcept>,

    /// Describes the link to the Device
    pub device: types::Reference,

    /// on | off | standby | entered-in-error
    pub operational_status: Option<types::Code>,

    /// Color name (from CSS4) or #RRGGBB code
    pub color: Option<types::Code>,

    /// measurement | setting | calculation | unspecified
    pub category: types::Code,

    /// Indicates how often the metric is taken or recorded
    pub measurement_frequency: Option<types::Quantity>,

    /// Describes the calibrations that have been performed or that are required to be performed
    pub calibration: Option<Vec<DeviceMetricCalibration>>,
}

/// Describes the calibrations that have been performed or that are required to be performed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetricCalibration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// unspecified | offset | gain | two-point
    pub r#type: Option<types::Code>,

    /// not-calibrated | calibration-required | calibrated | unspecified
    pub state: Option<types::Code>,

    /// Describes the time last calibration has been performed
    pub time: Option<types::Instant>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceMetric;

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
