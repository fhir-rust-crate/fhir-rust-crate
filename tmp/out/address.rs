//! Address
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Address
        //!
        //! Version: 5.0.0
        //!
        //! Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.
The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.
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
        pub struct Address {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// home | work | temp | old | billing - purpose of this address
    pub r#use: Option<types::Code>, // code [0..1]

    /// postal | physical | both
    pub r#type: Option<types::Code>, // code [0..1]

    /// Text representation of the address
    pub text: Option<types::String>, // string [0..1]

    /// Street name, number, direction & P.O. Box etc.
    pub line: Option<Vec<types::String>>, // string [0..*]

    /// Name of city, town etc.
    pub city: Option<types::String>, // string [0..1]

    /// District name (aka county)
    pub district: Option<types::String>, // string [0..1]

    /// Sub-unit of country (abbreviations ok)
    pub state: Option<types::String>, // string [0..1]

    /// Postal code for area
    pub postal_code: Option<types::String>, // string [0..1]

    /// Country (e.g. may be ISO 3166 2 or 3 letter code)
    pub country: Option<types::String>, // string [0..1]

    /// Time period when address was/is in use
    pub period: Option<types::Period>, // Period [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Address;

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
        