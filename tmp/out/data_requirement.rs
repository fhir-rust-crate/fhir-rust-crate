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

        /// Use the FHIR R5 datatypes referenced by this struct's fields.
        use crate::r5::types;

        /// Use serde to serialize Rust into JSON and deserialize JSON to Rust.
        use ::serde::{Deserialize, Serialize};

        /// Skip serializing each attributes that is an option and set to none.
        #[serde_with::skip_serializing_none]
        /// Derive all our typical things for programming, serde, comparing, etc.
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
        /// Rename all the snake case Rust attributes into camel case JSON keys.
        #[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
        pub struct DataRequirement {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// The type of the required data
    pub r#type: types::Code, // code [1..1]

    /// The profile of the required data
    pub profile: Option<Vec<types::Canonical>>, // canonical [0..*]

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_codeable_concept: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject_reference: Option<types::Reference>, // Reference [0..1]

    /// Indicates specific structure elements that are referenced by the knowledge module
    pub must_support: Option<Vec<types::String>>, // string [0..*]

    /// What codes are expected
    pub code_filter: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// A code-valued attribute to filter on
    pub path: Option<types::String>, // string [0..1]

    /// A coded (token) parameter to search on
    pub search_param: Option<types::String>, // string [0..1]

    /// ValueSet for the filter
    pub value_set: Option<types::Canonical>, // canonical [0..1]

    /// What code is expected
    pub code: Option<Vec<types::Coding>>, // Coding [0..*]

    /// What dates/date ranges are expected
    pub date_filter: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// A date-valued attribute to filter on
    pub path: Option<types::String>, // string [0..1]

    /// A date valued parameter to search on
    pub search_param: Option<types::String>, // string [0..1]

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_period: Option<types::Period>, // Period [0..1]

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_duration: Option<types::Duration>, // Duration [0..1]

    /// What values are expected
    pub value_filter: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// An attribute to filter on
    pub path: Option<types::String>, // string [0..1]

    /// A parameter to search on
    pub search_param: Option<types::String>, // string [0..1]

    /// eq | gt | lt | ge | le | sa | eb
    pub comparator: Option<types::Code>, // code [0..1]

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_period: Option<types::Period>, // Period [0..1]

    /// The value of the filter, as a Period, DateTime, or Duration value
    pub value_duration: Option<types::Duration>, // Duration [0..1]

    /// Number of results
    pub limit: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Order of the results
    pub sort: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// The name of the attribute to perform the sort
    pub path: types::String, // string [1..1]

    /// ascending | descending
    pub direction: types::Code, // code [1..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = DataRequirement;

            #[test]
            fn test_default() {
                let actual = T::default();
                let expect = T {};
                assert_eq!(actual, expect);
            }

            mod serde_json {
                use super::*;
                use ::serde_json::json;

                #[test]
                fn test_serde_json_from_value() {
                    let json = json!({});
                    let actual: T = ::serde_json::from_value(json).expect("from_value");
                    let expect: T = T::default();
                    assert_eq!(actual, expect);
                }

                #[test]
                fn test_serde_json_to_value() {
                    let actual: ::serde_json::Value =
                        ::serde_json::to_value(T::default()).expect("to_value");
                    let expect: ::serde_json::Value = json!({});
                    assert_eq!(actual, expect);
                }
            }
        }
        