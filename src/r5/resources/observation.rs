//! Observation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Observation
//!
//! Version: 5.0.0
//!
//! Observation Resource: Measurements and simple assertions made about a patient, device or other subject.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Measurements and simple assertions made about a patient, device or other
/// subject.
///
/// Observation is one of the most widely used resources in FHIR R5 and serves
/// as the general-purpose container for recording facts about a subject at a
/// point in time. It supports clinical use cases such as vital signs, laboratory
/// results, imaging findings, device measurements, social history, and clinical
/// assessments, as well as administrative and survey data. Each Observation
/// carries a code identifying what was measured or asserted, an effective time,
/// a status in the observation lifecycle, and a value expressed through one of
/// the polymorphic value[x] elements (for example a Quantity, CodeableConcept,
/// string, or Ratio) or, when no value is available, a data-absent reason.
///
/// Most observations are simple name/value pair assertions with some metadata,
/// but observations can also group other observations together logically via
/// members, or be composed of several components that share the same metadata,
/// such as the systolic and diastolic readings of a blood pressure. Reference
/// ranges, interpretations, body sites, and methods provide additional context
/// for interpreting results. Observations are commonly derived from or based on
/// orders and definitions, and they support diagnosis, monitoring of progress,
/// establishing baselines and patterns, and capturing demographic
/// characteristics.
///
/// # See also
///
/// The observation typically references a subject such as a
/// [`Patient`](crate::r5::resources::patient::Patient), an
/// [`Encounter`](crate::r5::resources::encounter::Encounter) during which it was
/// made, and optionally a [`Device`](crate::r5::resources::device::Device) or
/// [`Specimen`](crate::r5::resources::specimen::Specimen). Coded elements use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), measured values use
/// [`Quantity`](crate::r5::types::Quantity), and links to other resources use
/// [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation::Observation;
///
/// let value = Observation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Observation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
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

    /// Business Identifier for observation
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR ObservationDefinition
    pub instantiates_canonical: Option<types::Canonical>,

    /// Instantiates FHIR ObservationDefinition
    pub instantiates_reference: Option<types::Reference>,

    /// Fulfills plan, proposal or order
    pub based_on: Option<Vec<types::Reference>>,

    /// Triggering observation(s)
    pub triggered_by: Option<Vec<ObservationTriggeredBy>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// Lifecycle status of the observation, such as registered, preliminary, final, or amended; required.
    pub status: types::Code,

    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of  type of observation
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Coded concept identifying what was observed or measured, such as a LOINC code; required.
    pub code: types::CodeableConcept,

    /// Reference to who or what the observation is about, most often a patient but possibly a group, device, or location.
    pub subject: Option<types::Reference>,

    /// What the observation is about, when it is not about the subject of record
    pub focus: Option<Vec<types::Reference>>,

    /// Healthcare event during which this observation is made
    pub encounter: Option<types::Reference>,

    /// Clinically relevant time/time-period for observation
    pub effective_date_time: Option<types::DateTime>,

    /// Clinically relevant time/time-period for observation
    pub effective_period: Option<types::Period>,

    /// Clinically relevant time/time-period for observation
    pub effective_timing: Option<types::Timing>,

    /// Clinically relevant time/time-period for observation
    pub effective_instant: Option<types::Instant>,

    /// Date/Time this version was made available
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Who is responsible for the observation
    pub performer: Option<Vec<types::Reference>>,

    /// Actual result expressed as a measured quantity with a unit, the most common value form for numeric results.
    pub value_quantity: Option<types::Quantity>,

    /// Actual result expressed as a coded concept, used when the answer is a term from a value set rather than a number.
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Actual result
    pub value_string: Option<types::String>,

    /// Actual result
    pub value_boolean: Option<types::Boolean>,

    /// Actual result
    pub value_integer: Option<types::Integer>,

    /// Actual result
    pub value_range: Option<types::Range>,

    /// Actual result
    pub value_ratio: Option<types::Ratio>,

    /// Actual result
    pub value_sampled_data: Option<types::SampledData>,

    /// Actual result
    pub value_time: Option<types::Time>,

    /// Actual result
    pub value_date_time: Option<types::DateTime>,

    /// Actual result
    pub value_period: Option<types::Period>,

    /// Actual result
    pub value_attachment: Option<types::Attachment>,

    /// Actual result
    pub value_reference: Option<types::Reference>,

    /// Why the result is missing
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// High, low, normal, etc
    pub interpretation: Option<Vec<types::CodeableConcept>>,

    /// Comments about the observation
    pub note: Option<Vec<types::Annotation>>,

    /// Observed body part
    pub body_site: Option<types::CodeableConcept>,

    /// Observed body structure
    pub body_structure: Option<types::Reference>,

    /// How it was done
    pub method: Option<types::CodeableConcept>,

    /// Specimen used for this observation
    pub specimen: Option<types::Reference>,

    /// A reference to the device that generates the measurements or the device settings for the device
    pub device: Option<types::Reference>,

    /// Provides guide for interpretation
    pub reference_range: Option<Vec<ObservationReferenceRange>>,

    /// Related resource that belongs to the Observation group
    pub has_member: Option<Vec<types::Reference>>,

    /// Related resource from which the observation is made
    pub derived_from: Option<Vec<types::Reference>>,

    /// Component results
    pub component: Option<Vec<ObservationComponent>>,
}

/// Triggering observation(s).
///
/// Identifies one or more observations that triggered the performance of this
/// observation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationTriggeredBy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Triggering observation
    pub observation: types::Reference,

    /// reflex | repeat | re-run
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Reason that the observation was triggered
    pub reason: Option<types::String>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`).
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,
}

/// Provides guide for interpretation.
///
/// Guidance on how to interpret the value by comparison to a normal or
/// recommended range. Multiple reference ranges are interpreted as an "OR". In
/// other words, to represent two distinct target populations, two
/// referenceRange elements would be used.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationReferenceRange {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Low Range, if relevant
    pub low: Option<types::Quantity>,

    /// High Range, if relevant
    pub high: Option<types::Quantity>,

    /// Normal value, if relevant
    pub normal_value: Option<types::CodeableConcept>,

    /// Reference range qualifier
    pub r#type: Option<types::CodeableConcept>,

    /// Reference range population
    pub applies_to: Option<Vec<types::CodeableConcept>>,

    /// Applicable age range, if relevant
    pub age: Option<types::Range>,

    /// Text based reference range in an observation
    pub text: Option<types::Markdown>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// Component results.
///
/// Some observations have multiple component observations. These component
/// observations are expressed as separate code value pairs that share the same
/// attributes. Examples include systolic and diastolic component observations
/// for blood pressure measurement and multiple component observations for
/// genetics observations.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of component observation (code / type)
    pub code: types::CodeableConcept,

    /// Actual component result
    pub value_quantity: Option<types::Quantity>,

    /// Actual component result
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Actual component result
    pub value_string: Option<types::String>,

    /// Actual component result
    pub value_boolean: Option<types::Boolean>,

    /// Actual component result
    pub value_integer: Option<types::Integer>,

    /// Actual component result
    pub value_range: Option<types::Range>,

    /// Actual component result
    pub value_ratio: Option<types::Ratio>,

    /// Actual component result
    pub value_sampled_data: Option<types::SampledData>,

    /// Actual component result
    pub value_time: Option<types::Time>,

    /// Actual component result
    pub value_date_time: Option<types::DateTime>,

    /// Actual component result
    pub value_period: Option<types::Period>,

    /// Actual component result
    pub value_attachment: Option<types::Attachment>,

    /// Actual component result
    pub value_reference: Option<types::Reference>,

    /// Why the component result is missing
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// High, low, normal, etc
    pub interpretation: Option<Vec<types::CodeableConcept>>,

    /// Provides guide for interpretation of component result
    pub reference_range: Option<Vec<ObservationReferenceRange>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Observation;

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
