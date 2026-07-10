//! SampledData
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/SampledData
        //!
        //! Version: 5.0.0
        //!
        //! SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
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
        pub struct SampledData {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Zero value and units
    pub origin: types::Quantity, // Quantity [1..1]

    /// Number of intervalUnits between samples
    pub interval: Option<types::Decimal>, // decimal [0..1]

    /// The measurement unit of the interval between samples
    pub interval_unit: types::Code, // code [1..1]

    /// Multiply data by this before adding to origin
    pub factor: Option<types::Decimal>, // decimal [0..1]

    /// Lower limit of detection
    pub lower_limit: Option<types::Decimal>, // decimal [0..1]

    /// Upper limit of detection
    pub upper_limit: Option<types::Decimal>, // decimal [0..1]

    /// Number of sample points at each time point
    pub dimensions: types::PositiveInt, // positiveInt [1..1]

    /// Defines the codes used in the data
    pub code_map: Option<types::Canonical>, // canonical [0..1]

    /// Offsets, typically in time, at which data values were taken
    pub offsets: Option<types::String>, // string [0..1]

    /// Decimal values with spaces, or "E" | "U" | "L", or another code
    pub data: Option<types::String>, // string [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = SampledData;

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
        