//! HumanName
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/HumanName
        //!
        //! Version: 5.0.0
        //!
        //! HumanName Type: A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name parts or the need for usage information.
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
        pub struct HumanName {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// usual | official | temp | nickname | anonymous | old | maiden
    pub r#use: Option<types::Code>, // code [0..1]

    /// Text representation of the full name
    pub text: Option<types::String>, // string [0..1]

    /// Family name (often called 'Surname')
    pub family: Option<types::String>, // string [0..1]

    /// Given names (not always 'first'). Includes middle names
    pub given: Option<Vec<types::String>>, // string [0..*]

    /// Parts that come before the name
    pub prefix: Option<Vec<types::String>>, // string [0..*]

    /// Parts that come after the name
    pub suffix: Option<Vec<types::String>>, // string [0..*]

    /// Time period when name was/is in use
    pub period: Option<types::Period>, // Period [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = HumanName;

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
        