//! Dosage
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Dosage
        //!
        //! Version: 5.0.0
        //!
        //! Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.
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
        pub struct Dosage {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// The order of the dosage instructions
    pub sequence: Option<types::Integer>, // integer [0..1]

    /// Free text dosage instructions e.g. SIG
    pub text: Option<types::String>, // string [0..1]

    /// Supplemental instruction or warnings to the patient - e.g. "with meals", "may cause drowsiness"
    pub additional_instruction: Option<Vec<types::CodeableConcept>>, // CodeableConcept [0..*]

    /// Patient or consumer oriented instructions
    pub patient_instruction: Option<types::String>, // string [0..1]

    /// When medication should be administered
    pub timing: Option<types::Timing>, // Timing [0..1]

    /// Take "as needed"
    pub as_needed: Option<types::Boolean>, // boolean [0..1]

    /// Take "as needed" (for x)
    pub as_needed_for: Option<Vec<types::CodeableConcept>>, // CodeableConcept [0..*]

    /// Body site to administer to
    pub site: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// How drug should enter body
    pub route: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// Technique for administering medication
    pub method: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// Amount of medication administered, to be administered or typical amount to be administered
    pub dose_and_rate: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// The kind of dose or rate specified
    pub r#type: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// Amount of medication per dose
    pub dose_range: Option<types::Range>, // Range [0..1]

    /// Amount of medication per dose
    pub dose_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Amount of medication per unit of time
    pub rate_ratio: Option<types::Ratio>, // Ratio [0..1]

    /// Amount of medication per unit of time
    pub rate_range: Option<types::Range>, // Range [0..1]

    /// Amount of medication per unit of time
    pub rate_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Upper limit on medication per unit of time
    pub max_dose_per_period: Option<Vec<types::Ratio>>, // Ratio [0..*]

    /// Upper limit on medication per administration
    pub max_dose_per_administration: Option<types::Quantity>, // Quantity [0..1]

    /// Upper limit on medication per lifetime of the patient
    pub max_dose_per_lifetime: Option<types::Quantity>, // Quantity [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Dosage;

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
        