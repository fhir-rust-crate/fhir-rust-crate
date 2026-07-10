//! Availability
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Availability
        //!
        //! Version: 5.0.0
        //!
        //! Availability Type: Availability data for an {item}.
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
        pub struct Availability {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Times the {item} is available
    pub available_time: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// mon | tue | wed | thu | fri | sat | sun
    pub days_of_week: Option<Vec<types::Code>>, // code [0..*]

    /// Always available? i.e. 24 hour service
    pub all_day: Option<types::Boolean>, // boolean [0..1]

    /// Opening time of day (ignored if allDay = true)
    pub available_start_time: Option<types::Time>, // time [0..1]

    /// Closing time of day (ignored if allDay = true)
    pub available_end_time: Option<types::Time>, // time [0..1]

    /// Not available during this time due to provided reason
    pub not_available_time: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Reason presented to the user explaining why time not available
    pub description: Option<types::String>, // string [0..1]

    /// Service not available during this period
    pub during: Option<types::Period>, // Period [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Availability;

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
        