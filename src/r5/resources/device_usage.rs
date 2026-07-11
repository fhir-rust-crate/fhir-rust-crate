//! DeviceUsage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceUsage
//!
//! Version: 5.0.0
//!
//! DeviceUsage Resource: A record of a device being used by a patient where the record is the result of a report from the patient or a clinician.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a device being used by a patient where the record is the result
/// of a report from the patient or a clinician.
///
/// DeviceUsage records that a patient is or was using a particular device. In
/// FHIR R5 it captures who is using the device, which device is involved, the
/// timing and context of the usage, and any adherence, status, or reason
/// information supplied by the patient or a clinician. It is commonly used to
/// track patient-reported or clinician-reported device use over time.
///
/// Unlike [`DeviceRequest`](crate::r5::resources::device_request::DeviceRequest),
/// which represents an order or proposal for device use, DeviceUsage is a
/// statement of fact describing actual (or historical) use as reported by the
/// patient, a caregiver, or a clinician; it is not itself an authorization. It
/// is administratively useful for medication-reconciliation-style workflows for
/// durable medical equipment and other devices, for tracking compliance with a
/// prescribed device regimen, and for capturing why a patient stopped, changed,
/// or continued using a device.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject who is using the device.
/// - `Device` — the physical or virtual device being used, referenced via the `device` field.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for coded values such as `usage_status` and `usage_reason`.
/// - `Encounter` — the encounter or episode of care providing context, referenced via `context`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_usage::DeviceUsage;
///
/// let value = DeviceUsage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DeviceUsage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUsage {
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

    /// External identifier for this record
    pub identifier: Option<Vec<types::Identifier>>,

    /// Fulfills plan, proposal or order
    pub based_on: Option<Vec<types::Reference>>,

    /// The current state of this device usage record: active | completed | not-done | entered-in-error +
    pub status: types::Code,

    /// The category of the statement - classifying how the statement is made
    pub category: Option<Vec<types::CodeableConcept>>,

    /// A reference to the [`Patient`](crate::r5::resources::patient::Patient) (or group) reported to be using the device
    pub patient: types::Reference,

    /// Supporting information
    pub derived_from: Option<Vec<types::Reference>>,

    /// The encounter or episode of care that establishes the context for this device use statement
    pub context: Option<types::Reference>,

    /// How often the device was used
    pub timing_timing: Option<types::Timing>,

    /// How often the device was used
    pub timing_period: Option<types::Period>,

    /// How often the device was used
    pub timing_date_time: Option<types::DateTime>,

    /// When the statement was made (and recorded)
    pub date_asserted: Option<types::DateTime>,

    /// The status of the device usage, for example always, sometimes, never. This is not the same as the status of the statement
    pub usage_status: Option<types::CodeableConcept>,

    /// The reason for asserting the usage status - for example forgot, lost, stolen, broken
    pub usage_reason: Option<Vec<types::CodeableConcept>>,

    /// How device is being used
    pub adherence: Option<DeviceUsageAdherence>,

    /// Who made the statement
    pub information_source: Option<types::Reference>,

    /// The device that was used, given either as a coded value or as a reference to a `Device` resource
    pub device: types::CodeableReference,

    /// Why device was used
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Target body site
    pub body_site: Option<types::CodeableReference>,

    /// Addition details (comments, instructions)
    pub note: Option<Vec<types::Annotation>>,
}

/// How device is being used.
///
/// Backbone element describing the patient's adherence to the device usage,
/// including a code such as always, never, or sometimes, and the reasons that
/// explain that adherence status.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUsageAdherence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// always | never | sometimes
    pub code: types::CodeableConcept,

    /// lost | stolen | prescribed | broken | burned | forgot
    pub reason: Vec<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceUsage;

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
