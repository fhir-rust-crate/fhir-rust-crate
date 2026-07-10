//! Parameters
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Parameters
//!
//! Version: 5.0.0
//!
//! Parameters Resource: This resource is used to pass information into and back from an operation.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// This resource is used to pass information into and back from an operation
/// (whether invoked directly from REST or within a messaging environment). It is
/// not persisted or allowed to be referenced by other resources except as
/// described in the definition of the Parameters resource. Each parameter carries
/// a name plus either a data-type value, a whole resource, or nested part
/// parameters.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::parameters::Parameters;
///
/// let value = Parameters::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Parameters = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content
    pub language: Option<types::Code>,

    /// Operation Parameter
    pub parameter: Option<Vec<ParametersParameter>>,
}

/// A parameter passed to or received from the operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ParametersParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name from the definition
    pub name: types::String,

    /// If parameter is a data type
    pub value_base64_binary: Option<types::Base64Binary>,

    /// If parameter is a data type
    pub value_boolean: Option<types::Boolean>,

    /// If parameter is a data type
    pub value_canonical: Option<types::Canonical>,

    /// If parameter is a data type
    pub value_code: Option<types::Code>,

    /// If parameter is a data type
    pub value_date: Option<types::Date>,

    /// If parameter is a data type
    pub value_date_time: Option<types::DateTime>,

    /// If parameter is a data type
    pub value_decimal: Option<types::Decimal>,

    /// If parameter is a data type
    pub value_id: Option<types::Id>,

    /// If parameter is a data type
    pub value_instant: Option<types::Instant>,

    /// If parameter is a data type
    pub value_integer: Option<types::Integer>,

    /// If parameter is a data type
    pub value_integer64: Option<types::Integer64>,

    /// If parameter is a data type
    pub value_markdown: Option<types::Markdown>,

    /// If parameter is a data type
    pub value_oid: Option<types::Oid>,

    /// If parameter is a data type
    pub value_positive_int: Option<types::PositiveInt>,

    /// If parameter is a data type
    pub value_string: Option<types::String>,

    /// If parameter is a data type
    pub value_time: Option<types::Time>,

    /// If parameter is a data type
    pub value_unsigned_int: Option<types::UnsignedInt>,

    /// If parameter is a data type
    pub value_uri: Option<types::Uri>,

    /// If parameter is a data type
    pub value_url: Option<types::Url>,

    /// If parameter is a data type
    pub value_uuid: Option<types::Uuid>,

    /// If parameter is a data type
    pub value_address: Option<types::Address>,

    /// If parameter is a data type
    pub value_age: Option<types::Age>,

    /// If parameter is a data type
    pub value_annotation: Option<types::Annotation>,

    /// If parameter is a data type
    pub value_attachment: Option<types::Attachment>,

    /// If parameter is a data type
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// If parameter is a data type
    pub value_codeable_reference: Option<types::CodeableReference>,

    /// If parameter is a data type
    pub value_coding: Option<types::Coding>,

    /// If parameter is a data type
    pub value_contact_point: Option<types::ContactPoint>,

    /// If parameter is a data type
    pub value_count: Option<types::Count>,

    /// If parameter is a data type
    pub value_distance: Option<types::Distance>,

    /// If parameter is a data type
    pub value_duration: Option<types::Duration>,

    /// If parameter is a data type
    pub value_human_name: Option<types::HumanName>,

    /// If parameter is a data type
    pub value_identifier: Option<types::Identifier>,

    /// If parameter is a data type
    pub value_money: Option<types::Money>,

    /// If parameter is a data type
    pub value_period: Option<types::Period>,

    /// If parameter is a data type
    pub value_quantity: Option<types::Quantity>,

    /// If parameter is a data type
    pub value_range: Option<types::Range>,

    /// If parameter is a data type
    pub value_ratio: Option<types::Ratio>,

    /// If parameter is a data type
    pub value_ratio_range: Option<types::RatioRange>,

    /// If parameter is a data type
    pub value_reference: Option<types::Reference>,

    /// If parameter is a data type
    pub value_sampled_data: Option<types::SampledData>,

    /// If parameter is a data type
    pub value_signature: Option<types::Signature>,

    /// If parameter is a data type
    pub value_timing: Option<types::Timing>,

    /// If parameter is a data type
    pub value_contact_detail: Option<types::ContactDetail>,

    /// If parameter is a data type
    pub value_data_requirement: Option<types::DataRequirement>,

    /// If parameter is a data type
    pub value_expression: Option<types::Expression>,

    /// If parameter is a data type
    pub value_parameter_definition: Option<types::ParameterDefinition>,

    /// If parameter is a data type
    pub value_related_artifact: Option<types::RelatedArtifact>,

    /// If parameter is a data type
    pub value_trigger_definition: Option<types::TriggerDefinition>,

    /// If parameter is a data type
    pub value_usage_context: Option<types::UsageContext>,

    /// If parameter is a data type
    pub value_availability: Option<types::Availability>,

    /// If parameter is a data type
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>,

    /// If parameter is a data type
    pub value_dosage: Option<types::Dosage>,

    /// If parameter is a data type
    pub value_meta: Option<types::Meta>,

    /// If parameter is a whole resource
    pub resource: Option<::serde_json::Value>,

    /// Named part of a multi-part parameter
    pub part: Option<Vec<ParametersParameter>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Parameters;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
