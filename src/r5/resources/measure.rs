//! Measure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Measure
//!
//! Version: 5.0.0
//!
//! Measure Resource: The Measure resource provides the definition of a quality measure.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Measure resource provides the definition of a quality measure.
///
/// A quality measure is a quantitative tool used to assess the performance of an
/// individual clinician, an organization, or a health system with respect to a
/// specified process or outcome, by measuring actions, processes, states, or
/// outcomes of clinical care. In FHIR R5 the Measure resource is a canonical,
/// publishable knowledge artifact that captures the complete, computable
/// definition of such a measure: its metadata and versioning, the clinical logic
/// libraries it relies on, its population criteria (for example initial
/// population, numerator, and denominator), its scoring and improvement notation,
/// and any stratifiers or supplemental data to report alongside the score.
///
/// Measures are typically authored and distributed by publishers or measure
/// stewards and then evaluated against patient data to produce results. The
/// clinical logic is generally expressed in a referenced Library (commonly using
/// Clinical Quality Language, CQL), and the calculated results of applying a
/// Measure to a subject or population are conveyed in a separate MeasureReport
/// resource. This separation lets the same Measure definition be shared, versioned,
/// and reused across many evaluations and reporting contexts.
///
/// # See also
///
/// The `Library` resource typically holds the measure's executable logic, and the
/// `MeasureReport` resource conveys the results of evaluating a measure. Measure
/// subjects are frequently instances of [`Patient`](crate::r5::resources::patient::Patient).
/// Many descriptive fields are typed as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::Measure;
///
/// let value = Measure::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Measure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
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

    /// Canonical identifier for this measure, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the measure
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the measure
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this measure (computer friendly)
    pub name: Option<types::String>,

    /// Name for this measure (human friendly)
    pub title: Option<types::String>,

    /// Subordinate title of the measure
    pub subtitle: Option<types::String>,

    /// Publication lifecycle state of this measure: draft, active, retired, or unknown.
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_reference: Option<types::Reference>,

    /// Population basis
    pub basis: Option<types::Code>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the measure
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for measure (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this measure is defined
    pub purpose: Option<types::Markdown>,

    /// Describes the clinical usage of the measure
    pub usage: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the measure was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the measure was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the measure is expected to be used
    pub effective_period: Option<types::Period>,

    /// The category of the measure, such as Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Canonical references to the Library resources that hold the measure's computable logic, such as CQL.
    pub library: Option<Vec<types::Canonical>>,

    /// Disclaimer for use of the measure or its referenced content
    pub disclaimer: Option<types::Markdown>,

    /// How the measure is scored: proportion, ratio, continuous-variable, or cohort.
    pub scoring: Option<types::CodeableConcept>,

    /// What units?
    pub scoring_unit: Option<types::CodeableConcept>,

    /// opportunity | all-or-nothing | linear | weighted
    pub composite_scoring: Option<types::CodeableConcept>,

    /// process | outcome | structure | patient-reported-outcome | composite
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// How risk adjustment is applied for this measure
    pub risk_adjustment: Option<types::Markdown>,

    /// How is rate aggregation performed for this measure
    pub rate_aggregation: Option<types::Markdown>,

    /// Detailed description of why the measure exists
    pub rationale: Option<types::Markdown>,

    /// Summary of clinical guidelines
    pub clinical_recommendation_statement: Option<types::Markdown>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Defined terms used in the measure documentation
    pub term: Option<Vec<MeasureTerm>>,

    /// Additional guidance for implementers (deprecated)
    pub guidance: Option<types::Markdown>,

    /// The population criteria groups that define how the measure is scored and evaluated.
    pub group: Option<Vec<MeasureGroup>>,

    /// What other data should be reported with the measure
    pub supplemental_data: Option<Vec<MeasureSupplementalData>>,
}

/// Defined terms used in the measure documentation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureTerm {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What term?
    pub code: Option<types::CodeableConcept>,

    /// Meaning of the term
    pub definition: Option<types::Markdown>,
}

/// Population criteria group.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique id for group in measure
    pub link_id: Option<types::String>,

    /// Meaning of the group
    pub code: Option<types::CodeableConcept>,

    /// Summary description
    pub description: Option<types::Markdown>,

    /// process | outcome | structure | patient-reported-outcome | composite
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_reference: Option<types::Reference>,

    /// Population basis
    pub basis: Option<types::Code>,

    /// proportion | ratio | continuous-variable | cohort
    pub scoring: Option<types::CodeableConcept>,

    /// What units?
    pub scoring_unit: Option<types::CodeableConcept>,

    /// How is rate aggregation performed for this measure
    pub rate_aggregation: Option<types::Markdown>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Logic used by the measure group
    pub library: Option<Vec<types::Canonical>>,

    /// Population criteria
    pub population: Option<Vec<MeasureGroupPopulation>>,

    /// Stratifier criteria for the measure
    pub stratifier: Option<Vec<MeasureGroupStratifier>>,
}

/// Population criteria.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique id for population in measure
    pub link_id: Option<types::String>,

    /// initial-population | numerator | numerator-exclusion | denominator | ...
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this population criteria
    pub description: Option<types::Markdown>,

    /// The criteria that defines this population
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference>,

    /// Which population
    pub input_population_id: Option<types::String>,

    /// Aggregation method for a measure score (e.g. sum, average, median, ...)
    pub aggregate_method: Option<types::CodeableConcept>,
}

/// Stratifier criteria for the measure.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupStratifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique id for stratifier in measure
    pub link_id: Option<types::String>,

    /// Meaning of the stratifier
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this stratifier
    pub description: Option<types::Markdown>,

    /// How the measure should be stratified
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference>,

    /// Stratifier criteria component for the measure
    pub component: Option<Vec<MeasureGroupStratifierComponent>>,
}

/// Stratifier criteria component for the measure.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupStratifierComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique id for stratifier component in measure
    pub link_id: Option<types::String>,

    /// Meaning of the stratifier component
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this stratifier component
    pub description: Option<types::Markdown>,

    /// Component of how the measure should be stratified
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference>,
}

/// What other data should be reported with the measure.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureSupplementalData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique id for supplementalData in measure
    pub link_id: Option<types::String>,

    /// Meaning of the supplemental data
    pub code: Option<types::CodeableConcept>,

    /// supplemental-data | risk-adjustment-factor
    pub usage: Option<Vec<types::CodeableConcept>>,

    /// The human readable description of this supplemental data
    pub description: Option<types::Markdown>,

    /// Expression describing additional data to be reported
    pub criteria: types::Expression,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Measure;

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
