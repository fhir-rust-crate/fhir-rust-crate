//! MeasureReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
//!
//! Version: 5.0.0
//!
//! MeasureReport Resource: The results of the calculation of a measure, and optionally a reference to the resources involved in that calculation.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The MeasureReport resource contains the results of the calculation of a
/// quality measure, and optionally references to the resources that were
/// involved in that calculation. In FHIR R5 it is the primary vehicle for
/// exchanging quality-measurement outcomes between systems: a reporting
/// system evaluates a `Measure` definition against a subject or population
/// over a defined reporting period, then conveys the computed scores,
/// population counts, and stratifications as a MeasureReport. Reports may be
/// produced at the level of an individual (for example, a single patient's
/// proportion score), as a subject-list enumerating the members of each
/// population, or as an aggregate summary across an entire population, and
/// may also be used purely for data exchange to submit the raw evaluated
/// data. Common uses include clinical quality measurement, value-based care
/// and pay-for-performance programs, public health surveillance, and
/// regulatory or accreditation reporting.
///
/// Related resources: a MeasureReport typically references the `Measure` it
/// was calculated from, the subject it concerns (for example a
/// [`Patient`](crate::r5::resources::patient::Patient) or a `Group`), and
/// the [`Organization`](crate::r5::resources::organization::Organization)
/// or `Practitioner` acting as reporter. Scores, improvement notation, and
/// stratifier values are expressed using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Quantity`](crate::r5::types::Quantity), and related datatypes, and the
/// reporting window is captured with a [`Period`](crate::r5::types::Period).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure_report::MeasureReport;
///
/// let value = MeasureReport::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MeasureReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReport {
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

    /// Additional identifier for the MeasureReport
    pub identifier: Option<Vec<types::Identifier>>,

    /// The processing state of the report: complete, pending, or error.
    pub status: types::Code,

    /// The kind of report: individual, subject-list, summary, or data-exchange.
    pub r#type: types::Code,

    /// incremental | snapshot
    pub data_update_type: Option<types::Code>,

    /// Canonical reference to the Measure definition that was calculated.
    pub measure: Option<types::Canonical>,

    /// The individual, group, or population that this report concerns.
    pub subject: Option<types::Reference>,

    /// When the measure was calculated
    pub date: Option<types::DateTime>,

    /// Who is reporting the data
    pub reporter: Option<types::Reference>,

    /// What vendor prepared the data
    pub reporting_vendor: Option<types::Reference>,

    /// Where the reported data is from
    pub location: Option<types::Reference>,

    /// The reporting period over which the measure was evaluated.
    pub period: types::Period,

    /// What parameters were provided to the report
    pub input_parameters: Option<types::Reference>,

    /// What scoring method (e.g. proportion, ratio, continuous-variable)
    pub scoring: Option<types::CodeableConcept>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Measure results for each group
    pub group: Option<Vec<MeasureReportGroup>>,

    /// Additional information collected for the report
    pub supplemental_data: Option<Vec<types::Reference>>,

    /// What data was used to calculate the measure score
    pub evaluated_resource: Option<Vec<types::Reference>>,
}

/// Measure results for each group.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pointer to specific group from Measure
    pub link_id: Option<types::String>,

    /// Meaning of the group
    pub code: Option<types::CodeableConcept>,

    /// What individual(s) the report is for
    pub subject: Option<types::Reference>,

    /// The populations in the group
    pub population: Option<Vec<MeasureReportGroupPopulation>>,

    /// What score this group achieved
    pub measure_score_quantity: Option<types::Quantity>,

    /// What score this group achieved
    pub measure_score_date_time: Option<types::DateTime>,

    /// What score this group achieved
    pub measure_score_codeable_concept: Option<types::CodeableConcept>,

    /// What score this group achieved
    pub measure_score_period: Option<types::Period>,

    /// What score this group achieved
    pub measure_score_range: Option<types::Range>,

    /// What score this group achieved
    pub measure_score_duration: Option<types::Duration>,

    /// Stratification results
    pub stratifier: Option<Vec<MeasureReportGroupStratifier>>,
}

/// The populations in the group.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pointer to specific population from Measure
    pub link_id: Option<types::String>,

    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    pub code: Option<types::CodeableConcept>,

    /// Size of the population
    pub count: Option<types::Integer>,

    /// For subject-list reports, the subject results in this population
    pub subject_results: Option<types::Reference>,

    /// For subject-list reports, a subject result in this population
    pub subject_report: Option<Vec<types::Reference>>,

    /// What individual(s) in the population
    pub subjects: Option<types::Reference>,
}

/// Stratification results.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pointer to specific stratifier from Measure
    pub link_id: Option<types::String>,

    /// What stratifier of the group
    pub code: Option<types::CodeableConcept>,

    /// Stratum results, one for each unique value, or set of values, in the stratifier, or stratifier components
    pub stratum: Option<Vec<MeasureReportGroupStratifierStratum>>,
}

/// Stratum results, one for each unique value, or set of values, in the
/// stratifier, or stratifier components.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifierStratum {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The stratum value, e.g. male
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// The stratum value, e.g. male
    pub value_boolean: Option<types::Boolean>,

    /// The stratum value, e.g. male
    pub value_quantity: Option<types::Quantity>,

    /// The stratum value, e.g. male
    pub value_range: Option<types::Range>,

    /// The stratum value, e.g. male
    pub value_reference: Option<types::Reference>,

    /// Stratifier component values
    pub component: Option<Vec<MeasureReportGroupStratifierStratumComponent>>,

    /// Population results in this stratum
    pub population: Option<Vec<MeasureReportGroupStratifierStratumPopulation>>,

    /// What score this stratum achieved
    pub measure_score_quantity: Option<types::Quantity>,

    /// What score this stratum achieved
    pub measure_score_date_time: Option<types::DateTime>,

    /// What score this stratum achieved
    pub measure_score_codeable_concept: Option<types::CodeableConcept>,

    /// What score this stratum achieved
    pub measure_score_period: Option<types::Period>,

    /// What score this stratum achieved
    pub measure_score_range: Option<types::Range>,

    /// What score this stratum achieved
    pub measure_score_duration: Option<types::Duration>,
}

/// Stratifier component values.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifierStratumComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pointer to specific stratifier component from Measure
    pub link_id: Option<types::String>,

    /// What stratifier component of the group
    pub code: types::CodeableConcept,

    /// The stratum component value, e.g. male
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// The stratum component value, e.g. male
    pub value_boolean: Option<types::Boolean>,

    /// The stratum component value, e.g. male
    pub value_quantity: Option<types::Quantity>,

    /// The stratum component value, e.g. male
    pub value_range: Option<types::Range>,

    /// The stratum component value, e.g. male
    pub value_reference: Option<types::Reference>,
}

/// Population results in this stratum.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifierStratumPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Pointer to specific population from Measure
    pub link_id: Option<types::String>,

    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    pub code: Option<types::CodeableConcept>,

    /// Size of the population
    pub count: Option<types::Integer>,

    /// For subject-list reports, the subject results in this population
    pub subject_results: Option<types::Reference>,

    /// For subject-list reports, a subject result in this population
    pub subject_report: Option<Vec<types::Reference>>,

    /// What individual(s) in the population
    pub subjects: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MeasureReport;

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
