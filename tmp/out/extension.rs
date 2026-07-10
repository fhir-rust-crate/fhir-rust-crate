//! Extension
        //!
        //! URL: http://hl7.org/fhir/StructureDefinition/Extension
        //!
        //! Version: 5.0.0
        //!
        //! Extension Type: Optional Extension Element - found in all resources.
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
        pub struct Extension {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// identifies the meaning of the extension
    pub url: types::String, // http://hl7.org/fhirpath/System.String [1..1]

    /// Value of extension
    pub value_base_64_binary: Option<types::Base64Binary>, // base64Binary [0..1]

    /// Value of extension
    pub value_boolean: Option<types::Boolean>, // boolean [0..1]

    /// Value of extension
    pub value_canonical: Option<types::Canonical>, // canonical [0..1]

    /// Value of extension
    pub value_code: Option<types::Code>, // code [0..1]

    /// Value of extension
    pub value_date: Option<types::Date>, // date [0..1]

    /// Value of extension
    pub value_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// Value of extension
    pub value_decimal: Option<types::Decimal>, // decimal [0..1]

    /// Value of extension
    pub value_id: Option<types::Id>, // id [0..1]

    /// Value of extension
    pub value_instant: Option<types::Instant>, // instant [0..1]

    /// Value of extension
    pub value_integer: Option<types::Integer>, // integer [0..1]

    /// Value of extension
    pub value_integer_64: Option<types::Integer64>, // integer64 [0..1]

    /// Value of extension
    pub value_markdown: Option<types::Markdown>, // markdown [0..1]

    /// Value of extension
    pub value_oid: Option<types::Oid>, // oid [0..1]

    /// Value of extension
    pub value_positive_int: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Value of extension
    pub value_string: Option<types::String>, // string [0..1]

    /// Value of extension
    pub value_time: Option<types::Time>, // time [0..1]

    /// Value of extension
    pub value_unsigned_int: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// Value of extension
    pub value_uri: Option<types::Uri>, // uri [0..1]

    /// Value of extension
    pub value_url: Option<types::Url>, // url [0..1]

    /// Value of extension
    pub value_uuid: Option<types::Uuid>, // uuid [0..1]

    /// Value of extension
    pub value_address: Option<types::Address>, // Address [0..1]

    /// Value of extension
    pub value_age: Option<types::Age>, // Age [0..1]

    /// Value of extension
    pub value_annotation: Option<types::Annotation>, // Annotation [0..1]

    /// Value of extension
    pub value_attachment: Option<types::Attachment>, // Attachment [0..1]

    /// Value of extension
    pub value_codeable_concept: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// Value of extension
    pub value_codeable_reference: Option<types::CodeableReference>, // CodeableReference [0..1]

    /// Value of extension
    pub value_coding: Option<types::Coding>, // Coding [0..1]

    /// Value of extension
    pub value_contact_point: Option<types::ContactPoint>, // ContactPoint [0..1]

    /// Value of extension
    pub value_count: Option<types::Count>, // Count [0..1]

    /// Value of extension
    pub value_distance: Option<types::Distance>, // Distance [0..1]

    /// Value of extension
    pub value_duration: Option<types::Duration>, // Duration [0..1]

    /// Value of extension
    pub value_human_name: Option<types::HumanName>, // HumanName [0..1]

    /// Value of extension
    pub value_identifier: Option<types::Identifier>, // Identifier [0..1]

    /// Value of extension
    pub value_money: Option<types::Money>, // Money [0..1]

    /// Value of extension
    pub value_period: Option<types::Period>, // Period [0..1]

    /// Value of extension
    pub value_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Value of extension
    pub value_range: Option<types::Range>, // Range [0..1]

    /// Value of extension
    pub value_ratio: Option<types::Ratio>, // Ratio [0..1]

    /// Value of extension
    pub value_ratio_range: Option<types::RatioRange>, // RatioRange [0..1]

    /// Value of extension
    pub value_reference: Option<types::Reference>, // Reference [0..1]

    /// Value of extension
    pub value_sampled_data: Option<types::SampledData>, // SampledData [0..1]

    /// Value of extension
    pub value_signature: Option<types::Signature>, // Signature [0..1]

    /// Value of extension
    pub value_timing: Option<types::Timing>, // Timing [0..1]

    /// Value of extension
    pub value_contact_detail: Option<types::ContactDetail>, // ContactDetail [0..1]

    /// Value of extension
    pub value_data_requirement: Option<types::DataRequirement>, // DataRequirement [0..1]

    /// Value of extension
    pub value_expression: Option<types::Expression>, // Expression [0..1]

    /// Value of extension
    pub value_parameter_definition: Option<types::ParameterDefinition>, // ParameterDefinition [0..1]

    /// Value of extension
    pub value_related_artifact: Option<types::RelatedArtifact>, // RelatedArtifact [0..1]

    /// Value of extension
    pub value_trigger_definition: Option<types::TriggerDefinition>, // TriggerDefinition [0..1]

    /// Value of extension
    pub value_usage_context: Option<types::UsageContext>, // UsageContext [0..1]

    /// Value of extension
    pub value_availability: Option<types::Availability>, // Availability [0..1]

    /// Value of extension
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>, // ExtendedContactDetail [0..1]

    /// Value of extension
    pub value_dosage: Option<types::Dosage>, // Dosage [0..1]

    /// Value of extension
    pub value_meta: Option<types::Meta>, // Meta [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = Extension;

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
        