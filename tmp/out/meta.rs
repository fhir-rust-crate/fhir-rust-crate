//! Meta
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Meta
        //!
        //! Version: 5.0.0
        //!
        //! Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.
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
        pub struct Meta {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Version specific identifier
    pub version_id: Option<types::Id>, // id [0..1]

    /// When the resource version last changed
    pub last_updated: Option<types::Instant>, // instant [0..1]

    /// Identifies where the resource comes from
    pub source: Option<types::Uri>, // uri [0..1]

    /// Profiles this resource claims to conform to
    pub profile: Option<Vec<types::Canonical>>, // canonical [0..*]

    /// Security Labels applied to this resource
    pub security: Option<Vec<types::Coding>>, // Coding [0..*]

    /// Tags applied to this resource
    pub tag: Option<Vec<types::Coding>>, // Coding [0..*]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Meta;

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
        