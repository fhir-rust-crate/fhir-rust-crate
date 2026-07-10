//! ParameterDefinition
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/ParameterDefinition
        //!
        //! Version: 5.0.0
        //!
        //! ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.
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
        pub struct ParameterDefinition {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Name used to access the parameter value
    pub name: Option<types::Code>, // code [0..1]

    /// in | out
    pub r#use: types::Code, // code [1..1]

    /// Minimum cardinality
    pub min: Option<types::Integer>, // integer [0..1]

    /// Maximum cardinality (a number of *)
    pub max: Option<types::String>, // string [0..1]

    /// A brief description of the parameter
    pub documentation: Option<types::String>, // string [0..1]

    /// What type of value
    pub r#type: types::Code, // code [1..1]

    /// What profile the value is expected to be
    pub profile: Option<types::Canonical>, // canonical [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = ParameterDefinition;

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
        