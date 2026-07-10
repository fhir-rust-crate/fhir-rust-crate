//! Signature
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Signature
        //!
        //! Version: 5.0.0
        //!
        //! Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.
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
        pub struct Signature {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Indication of the reason the entity signed the object(s)
    pub r#type: Option<Vec<types::Coding>>, // Coding [0..*]

    /// When the signature was created
    pub when: Option<types::Instant>, // instant [0..1]

    /// Who signed
    pub who: Option<types::Reference>, // Reference [0..1]

    /// The party represented
    pub on_behalf_of: Option<types::Reference>, // Reference [0..1]

    /// The technical format of the signed resources
    pub target_format: Option<types::Code>, // code [0..1]

    /// The technical format of the signature
    pub sig_format: Option<types::Code>, // code [0..1]

    /// The actual signature content (XML DigSig. JWS, picture, etc.)
    pub data: Option<types::Base64Binary>, // base64Binary [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Signature;

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
        