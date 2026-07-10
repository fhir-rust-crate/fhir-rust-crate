//! Timing
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Timing
        //!
        //! Version: 5.0.0
        //!
        //! Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.
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
        pub struct Timing {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// When the event occurs
    pub event: Option<Vec<types::DateTime>>, // dateTime [0..*]

    /// When the event is to occur
    pub repeat: Option<types::Element>, // Element [0..1]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Length/Range of lengths, or (Start and/or end) limits
    pub bounds_duration: Option<types::Duration>, // Duration [0..1]

    /// Length/Range of lengths, or (Start and/or end) limits
    pub bounds_range: Option<types::Range>, // Range [0..1]

    /// Length/Range of lengths, or (Start and/or end) limits
    pub bounds_period: Option<types::Period>, // Period [0..1]

    /// Number of times to repeat
    pub count: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Maximum number of times to repeat
    pub count_max: Option<types::PositiveInt>, // positiveInt [0..1]

    /// How long when it happens
    pub duration: Option<types::Decimal>, // decimal [0..1]

    /// How long when it happens (Max)
    pub duration_max: Option<types::Decimal>, // decimal [0..1]

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    pub duration_unit: Option<types::Code>, // code [0..1]

    /// Indicates the number of repetitions that should occur within a period. I.e. Event occurs frequency times per period
    pub frequency: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Event occurs up to frequencyMax times per period
    pub frequency_max: Option<types::PositiveInt>, // positiveInt [0..1]

    /// The duration to which the frequency applies. I.e. Event occurs frequency times per period
    pub period: Option<types::Decimal>, // decimal [0..1]

    /// Upper limit of period (3-4 hours)
    pub period_max: Option<types::Decimal>, // decimal [0..1]

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    pub period_unit: Option<types::Code>, // code [0..1]

    /// mon | tue | wed | thu | fri | sat | sun
    pub day_of_week: Option<Vec<types::Code>>, // code [0..*]

    /// Time of day for action
    pub time_of_day: Option<Vec<types::Time>>, // time [0..*]

    /// Code for time period of occurrence
    pub when: Option<Vec<types::Code>>, // code [0..*]

    /// Minutes from event (before or after)
    pub offset: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// C | BID | TID | QID | AM | PM | QD | QOD | +
    pub code: Option<types::CodeableConcept>, // CodeableConcept [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Timing;

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
        