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
use fhir_derive_macros::Validate;

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
/// use fhir::r5::types::data_requirement::DataRequirement;
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
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The profile of the required data
    pub profile: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<Vec<Option<types::Element>>>,

    /// The `DataRequirement.subject[x]` choice element (0..1); see [`DataRequirementSubject`].
    #[serde(flatten)]
    pub subject: Option<DataRequirementSubject>,

    /// Indicates specific structure elements that are referenced by the knowledge module
    pub must_support: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`must_support`](Self::must_support) (FHIR `_mustSupport`).
    #[serde(rename = "_mustSupport")]
    pub must_support_ext: Option<Vec<Option<types::Element>>>,

    /// What codes are expected
    pub code_filter: Option<Vec<DataRequirementCodeFilter>>,

    /// What dates/date ranges are expected
    pub date_filter: Option<Vec<DataRequirementDateFilter>>,

    /// What values are expected
    pub value_filter: Option<Vec<DataRequirementValueFilter>>,

    /// Number of results
    pub limit: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`limit`](Self::limit) (FHIR `_limit`).
    #[serde(rename = "_limit")]
    pub limit_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// A coded (token) parameter to search on
    pub search_param: Option<types::String>,
    /// Primitive extension sibling for [`search_param`](Self::search_param) (FHIR `_searchParam`).
    #[serde(rename = "_searchParam")]
    pub search_param_ext: Option<types::Element>,

    /// ValueSet for the filter
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// A date valued parameter to search on
    pub search_param: Option<types::String>,
    /// Primitive extension sibling for [`search_param`](Self::search_param) (FHIR `_searchParam`).
    #[serde(rename = "_searchParam")]
    pub search_param_ext: Option<types::Element>,

    /// The `DataRequirement.dateFilter.value[x]` choice element (0..1); see [`DataRequirementDateFilterValue`].
    #[serde(flatten)]
    pub value: Option<DataRequirementDateFilterValue>,
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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// A parameter to search on
    pub search_param: Option<types::String>,
    /// Primitive extension sibling for [`search_param`](Self::search_param) (FHIR `_searchParam`).
    #[serde(rename = "_searchParam")]
    pub search_param_ext: Option<types::Element>,

    /// eq | gt | lt | ge | le | sa | eb
    pub comparator: Option<types::Code>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`).
    #[serde(rename = "_comparator")]
    pub comparator_ext: Option<types::Element>,

    /// The `DataRequirement.valueFilter.value[x]` choice element (0..1); see [`DataRequirementValueFilterValue`].
    #[serde(flatten)]
    pub value: Option<DataRequirementValueFilterValue>,
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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// ascending | descending
    pub direction: types::Code,
    /// Primitive extension sibling for [`direction`](Self::direction) (FHIR `_direction`).
    #[serde(rename = "_direction")]
    pub direction_ext: Option<types::Element>,
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
/// The `DataRequirement.dateFilter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementDateFilterValue {
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueDuration` variant.
    #[fhir("valueDuration")]
    Duration(Box<types::Duration>),
}

/// The `DataRequirement.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `DataRequirement.valueFilter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementValueFilterValue {
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueDuration` variant.
    #[fhir("valueDuration")]
    Duration(Box<types::Duration>),
}
