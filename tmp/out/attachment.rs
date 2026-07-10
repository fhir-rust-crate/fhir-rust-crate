//! Attachment
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Attachment
        //!
        //! Version: 5.0.0
        //!
        //! Attachment Type: For referring to data content defined in other formats.
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
        pub struct Attachment {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Mime type of the content, with charset etc.
    pub content_type: Option<types::Code>, // code [0..1]

    /// Human language of the content (BCP-47)
    pub language: Option<types::Code>, // code [0..1]

    /// Data inline, base64ed
    pub data: Option<types::Base64Binary>, // base64Binary [0..1]

    /// Uri where the data can be found
    pub url: Option<types::Url>, // url [0..1]

    /// Number of bytes of content (if url provided)
    pub size: Option<types::Integer64>, // integer64 [0..1]

    /// Hash of the data (sha-1, base64ed)
    pub hash: Option<types::Base64Binary>, // base64Binary [0..1]

    /// Label to display in place of the data
    pub title: Option<types::String>, // string [0..1]

    /// Date attachment was first created
    pub creation: Option<types::DateTime>, // dateTime [0..1]

    /// Height of the image in pixels (photo/video)
    pub height: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Width of the image in pixels (photo/video)
    pub width: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Number of frames if > 1 (photo)
    pub frames: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Length in seconds (audio / video)
    pub duration: Option<types::Decimal>, // decimal [0..1]

    /// Number of printed pages
    pub pages: Option<types::PositiveInt>, // positiveInt [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Attachment;

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
        