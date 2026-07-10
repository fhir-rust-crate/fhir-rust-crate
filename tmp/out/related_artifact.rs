//! RelatedArtifact
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/RelatedArtifact
        //!
        //! Version: 5.0.0
        //!
        //! RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.
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
        pub struct RelatedArtifact {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comments-on | comment-in | contains | contained-in | corrects | correction-in | replaces | replaced-with | retracts | retracted-by | signs | similar-to | supports | supported-with | transforms | transformed-into | transformed-with | documents | specification-of | created-with | cite-as
    pub r#type: types::Code, // code [1..1]

    /// Additional classifiers
    pub classifier: Option<Vec<types::CodeableConcept>>, // CodeableConcept [0..*]

    /// Short label
    pub label: Option<types::String>, // string [0..1]

    /// Brief description of the related artifact
    pub display: Option<types::String>, // string [0..1]

    /// Bibliographic citation for the artifact
    pub citation: Option<types::Markdown>, // markdown [0..1]

    /// What document is being referenced
    pub document: Option<types::Attachment>, // Attachment [0..1]

    /// What artifact is being referenced
    pub resource: Option<types::Canonical>, // canonical [0..1]

    /// What artifact, if not a conformance resource
    pub resource_reference: Option<types::Reference>, // Reference [0..1]

    /// draft | active | retired | unknown
    pub publication_status: Option<types::Code>, // code [0..1]

    /// Date of publication of the artifact being referred to
    pub publication_date: Option<types::Date>, // date [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = RelatedArtifact;

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
        