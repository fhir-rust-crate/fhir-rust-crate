//! Identifier
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Identifier
        //!
        //! Version: 5.0.0
        //!
        //! Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
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
        pub struct Identifier {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// usual | official | temp | secondary | old (If known)
    pub r#use: Option<types::Code>, // code [0..1]

    /// Description of identifier
    pub r#type: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// The namespace for the identifier value
    pub system: Option<types::Uri>, // uri [0..1]

    /// The value that is unique
    pub value: Option<types::String>, // string [0..1]

    /// Time period when id is/was valid for use
    pub period: Option<types::Period>, // Period [0..1]

    /// Organization that issued id (may be just text)
    pub assigner: Option<types::Reference>, // Reference [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Identifier;

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
        