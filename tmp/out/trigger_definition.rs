//! TriggerDefinition
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
        //!
        //! Version: 5.0.0
        //!
        //! TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.
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
        pub struct TriggerDefinition {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended
    pub r#type: types::Code, // code [1..1]

    /// Name or URI that identifies the event
    pub name: Option<types::String>, // string [0..1]

    /// Coded definition of the event
    pub code: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// What event
    pub subscription_topic: Option<types::Canonical>, // canonical [0..1]

    /// Timing of the event
    pub timing_timing: Option<types::Timing>, // Timing [0..1]

    /// Timing of the event
    pub timing_reference: Option<types::Reference>, // Reference [0..1]

    /// Timing of the event
    pub timing_date: Option<types::Date>, // date [0..1]

    /// Timing of the event
    pub timing_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// Triggering data of the event (multiple = 'and')
    pub data: Option<Vec<types::DataRequirement>>, // DataRequirement [0..*]

    /// Whether the event triggers (boolean expression)
    pub condition: Option<types::Expression>, // Expression [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = TriggerDefinition;

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
        