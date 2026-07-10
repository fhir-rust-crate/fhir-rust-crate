//! DataRequirement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DataRequirement
//!
//! Version: 5.0.0
//!
//! DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Describes a required data item for evaluation in terms of the type of data,
/// and optional code or date-based filters of the data. DataRequirement is used
/// by knowledge modules and clinical decision support artifacts to describe the
/// data that must be provided in order to evaluate the module. It can specify a
/// resource type, profiles, subject, and code- or date-based filters that narrow
/// the set of matching data.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::data_requirement::DataRequirement;
///
/// let value = DataRequirement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DataRequirement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// The type of the required data
    pub r#type: types::Code,

    /// The profile of the required data
    pub profile: Option<Vec<types::Canonical>>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_reference: Option<types::Reference>,

    /// Indicates specific structure elements that are referenced by the knowledge module
    pub must_support: Option<Vec<types::String>>,

    /// What codes are expected
    pub code_filter: Option<Vec<DataRequirementCodeFilter>>,

    /// What dates/date ranges are expected
    pub date_filter: Option<Vec<DataRequirementDateFilter>>,

    /// What values are expected
    pub value_filter: Option<Vec<DataRequirementValueFilter>>,

    /// Number of results
    pub limit: Option<types::PositiveInt>,

    /// Order of the results
    pub sort: Option<Vec<DataRequirementSort>>,
}

/// What codes are expected. Code filters specify additional constraints on the
/// data, identifying a code-valued attribute and the value set from which codes
/// in the data must be a member.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirementCodeFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// A code-valued attribute to filter on
    pub path: Option<types::String>,

    /// A coded (token) parameter to search on
    pub search_param: Option<types::String>,

    /// ValueSet for the filter
    pub value_set: Option<types::Canonical>,

    /// What code is expected
    pub code: Option<Vec<types::Coding>>,
}

/// What dates/date ranges are expected. Date filters specify additional
/// constraints on the data in terms of the applicable date range for specific
/// elements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirementDateFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// A date-valued attribute to filter on
    pub path: Option<types::String>,

    /// A date valued parameter to search on
    pub search_param: Option<types::String>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_date_time: Option<types::DateTime>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_period: Option<types::Period>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_duration: Option<types::Duration>,
}

/// What values are expected. Value filters specify additional constraints on the
/// data for elements other than code-valued or date-valued, along with a
/// comparator such as eq, gt, lt, ge, le, sa, or eb.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirementValueFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// An attribute to filter on
    pub path: Option<types::String>,

    /// A parameter to search on
    pub search_param: Option<types::String>,

    /// eq | gt | lt | ge | le | sa | eb
    pub comparator: Option<types::Code>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_date_time: Option<types::DateTime>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_period: Option<types::Period>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_duration: Option<types::Duration>,
}

/// Order of the results. Specifies the order of the results to be returned,
/// including the attribute to sort on and the direction of the sort.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirementSort {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// The name of the attribute to perform the sort
    pub path: types::String,

    /// ascending | descending
    pub direction: types::Code,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DataRequirement;

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
