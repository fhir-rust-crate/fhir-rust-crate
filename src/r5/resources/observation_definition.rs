//! ObservationDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
//!
//! Version: 5.0.0
//!
//! ObservationDefinition Resource: Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A set of definitional characteristics for a kind of observation or
/// measurement produced or consumed by an orderable health care service. An
/// ObservationDefinition describes the permitted data types, units, reference
/// ranges, qualified values, and component structure that conforming
/// observations are expected to follow. It is used to formally specify what a
/// laboratory or diagnostic service can produce or requires, supporting both
/// order catalogs and result validation in FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::observation_definition::ObservationDefinition;
///
/// let value = ObservationDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ObservationDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinition {
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

    /// Logical canonical URL to reference this ObservationDefinition (globally unique)
    pub url: Option<types::Uri>,

    /// Business identifier of the ObservationDefinition
    pub identifier: Option<types::Identifier>,

    /// Business version of the ObservationDefinition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this ObservationDefinition (computer friendly)
    pub name: Option<types::String>,

    /// Name for this ObservationDefinition (human friendly)
    pub title: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// The name of the individual or organization that published the ObservationDefinition
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the ObservationDefinition
    pub description: Option<types::Markdown>,

    /// Content intends to support these contexts
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for this ObservationDefinition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this ObservationDefinition is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When ObservationDefinition was approved by publisher
    pub approval_date: Option<types::Date>,

    /// Date on which the asset content was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// The effective date range for the ObservationDefinition
    pub effective_period: Option<types::Period>,

    /// Based on FHIR definition of another observation
    pub derived_from_canonical: Option<Vec<types::Canonical>>,

    /// Based on external definition
    pub derived_from_uri: Option<Vec<types::Uri>>,

    /// Type of subject for the defined observation
    pub subject: Option<Vec<types::CodeableConcept>>,

    /// Desired kind of performer for such kind of observation
    pub performer_type: Option<types::CodeableConcept>,

    /// General type of observation
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Type of observation
    pub code: types::CodeableConcept,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    pub permitted_data_type: Option<Vec<types::Code>>,

    /// Multiple results allowed for conforming observations
    pub multiple_results_allowed: Option<types::Boolean>,

    /// Body part to be observed
    pub body_site: Option<types::CodeableConcept>,

    /// Method used to produce the observation
    pub method: Option<types::CodeableConcept>,

    /// Kind of specimen used by this type of observation
    pub specimen: Option<Vec<types::Reference>>,

    /// Measurement device or model of device
    pub device: Option<Vec<types::Reference>>,

    /// The preferred name to be used when reporting the observation results
    pub preferred_report_name: Option<types::String>,

    /// Unit for quantitative results
    pub permitted_unit: Option<Vec<types::Coding>>,

    /// Set of qualified values for observation results
    pub qualified_value: Option<Vec<ObservationDefinitionQualifiedValue>>,

    /// Definitions of related resources belonging to this kind of observation group
    pub has_member: Option<Vec<types::Reference>>,

    /// Component results
    pub component: Option<Vec<ObservationDefinitionComponent>>,
}

/// Set of qualified values for observation results.
///
/// A qualified value bundles together a context (population, gender, age,
/// gestational age, condition) with the reference, critical, or absolute
/// ranges and the value sets of valid, normal, abnormal, and critical coded
/// values that apply to conforming observations under that context.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionQualifiedValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Context qualifier for the set of qualified values
    pub context: Option<types::CodeableConcept>,

    /// Targetted population for the set of qualified values
    pub applies_to: Option<Vec<types::CodeableConcept>>,

    /// male | female | other | unknown
    pub gender: Option<types::Code>,

    /// Applicable age range for the set of qualified values
    pub age: Option<types::Range>,

    /// Applicable gestational age range for the set of qualified values
    pub gestational_age: Option<types::Range>,

    /// Condition associated with the set of qualified values
    pub condition: Option<types::String>,

    /// reference | critical | absolute
    pub range_category: Option<types::Code>,

    /// The range for continuous or ordinal observations
    pub range: Option<types::Range>,

    /// Value set of valid coded values as part of this set of qualified values
    pub valid_coded_value_set: Option<types::Canonical>,

    /// Value set of normal coded values as part of this set of qualified values
    pub normal_coded_value_set: Option<types::Canonical>,

    /// Value set of abnormal coded values as part of this set of qualified values
    pub abnormal_coded_value_set: Option<types::Canonical>,

    /// Value set of critical coded values as part of this set of qualified values
    pub critical_coded_value_set: Option<types::Canonical>,
}

/// Component results.
///
/// Describes a component observation that is part of a composite observation
/// definition, specifying its code, permitted data types and units, and the
/// set of qualified values that apply to the component's results.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of observation
    pub code: types::CodeableConcept,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    pub permitted_data_type: Option<Vec<types::Code>>,

    /// Unit for quantitative results
    pub permitted_unit: Option<Vec<types::Coding>>,

    /// Set of qualified values for observation results
    pub qualified_value: Option<Vec<ObservationDefinitionQualifiedValue>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ObservationDefinition;

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
