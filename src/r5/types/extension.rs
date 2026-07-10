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

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Optional Extension Element - found in all resources.
///
/// Every element in a FHIR resource or datatype may include one or more
/// `Extension` child elements. Extensions provide a standardized mechanism for
/// representing additional information that is not part of the base definition
/// of the element, while preserving interoperability. Each extension is
/// identified by a `url` and carries a single value drawn from the FHIR
/// `value[x]` choice, and extensions may themselves be nested to build complex
/// structures.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::extension::Extension;
///
/// let value = Extension::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Extension = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// identifies the meaning of the extension
    pub url: types::String,

    /// Value of extension (base64Binary)
    pub value_base64_binary: Option<types::Base64Binary>,

    /// Value of extension (boolean)
    pub value_boolean: Option<types::Boolean>,

    /// Value of extension (canonical)
    pub value_canonical: Option<types::Canonical>,

    /// Value of extension (code)
    pub value_code: Option<types::Code>,

    /// Value of extension (date)
    pub value_date: Option<types::Date>,

    /// Value of extension (dateTime)
    pub value_date_time: Option<types::DateTime>,

    /// Value of extension (decimal)
    pub value_decimal: Option<types::Decimal>,

    /// Value of extension (id)
    pub value_id: Option<types::Id>,

    /// Value of extension (instant)
    pub value_instant: Option<types::Instant>,

    /// Value of extension (integer)
    pub value_integer: Option<types::Integer>,

    /// Value of extension (integer64)
    pub value_integer64: Option<types::Integer64>,

    /// Value of extension (markdown)
    pub value_markdown: Option<types::Markdown>,

    /// Value of extension (oid)
    pub value_oid: Option<types::Oid>,

    /// Value of extension (positiveInt)
    pub value_positive_int: Option<types::PositiveInt>,

    /// Value of extension (string)
    pub value_string: Option<types::String>,

    /// Value of extension (time)
    pub value_time: Option<types::Time>,

    /// Value of extension (unsignedInt)
    pub value_unsigned_int: Option<types::UnsignedInt>,

    /// Value of extension (uri)
    pub value_uri: Option<types::Uri>,

    /// Value of extension (url)
    pub value_url: Option<types::Url>,

    /// Value of extension (uuid)
    pub value_uuid: Option<types::Uuid>,

    /// Value of extension (Address)
    pub value_address: Option<types::Address>,

    /// Value of extension (Age)
    pub value_age: Option<types::Age>,

    /// Value of extension (Annotation)
    pub value_annotation: Option<types::Annotation>,

    /// Value of extension (Attachment)
    pub value_attachment: Option<types::Attachment>,

    /// Value of extension (CodeableConcept)
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value of extension (CodeableReference)
    pub value_codeable_reference: Option<types::CodeableReference>,

    /// Value of extension (Coding)
    pub value_coding: Option<types::Coding>,

    /// Value of extension (ContactPoint)
    pub value_contact_point: Option<types::ContactPoint>,

    /// Value of extension (Count)
    pub value_count: Option<types::Count>,

    /// Value of extension (Distance)
    pub value_distance: Option<types::Distance>,

    /// Value of extension (Duration)
    pub value_duration: Option<types::Duration>,

    /// Value of extension (HumanName)
    pub value_human_name: Option<types::HumanName>,

    /// Value of extension (Identifier)
    pub value_identifier: Option<types::Identifier>,

    /// Value of extension (Money)
    pub value_money: Option<types::Money>,

    /// Value of extension (Period)
    pub value_period: Option<types::Period>,

    /// Value of extension (Quantity)
    pub value_quantity: Option<types::Quantity>,

    /// Value of extension (Range)
    pub value_range: Option<types::Range>,

    /// Value of extension (Ratio)
    pub value_ratio: Option<types::Ratio>,

    /// Value of extension (RatioRange)
    pub value_ratio_range: Option<types::RatioRange>,

    /// Value of extension (Reference)
    pub value_reference: Option<types::Reference>,

    /// Value of extension (SampledData)
    pub value_sampled_data: Option<types::SampledData>,

    /// Value of extension (Signature)
    pub value_signature: Option<types::Signature>,

    /// Value of extension (Timing)
    pub value_timing: Option<types::Timing>,

    /// Value of extension (ContactDetail)
    pub value_contact_detail: Option<types::ContactDetail>,

    /// Value of extension (DataRequirement)
    pub value_data_requirement: Option<types::DataRequirement>,

    /// Value of extension (Expression)
    pub value_expression: Option<types::Expression>,

    /// Value of extension (ParameterDefinition)
    pub value_parameter_definition: Option<types::ParameterDefinition>,

    /// Value of extension (RelatedArtifact)
    pub value_related_artifact: Option<types::RelatedArtifact>,

    /// Value of extension (TriggerDefinition)
    pub value_trigger_definition: Option<types::TriggerDefinition>,

    /// Value of extension (UsageContext)
    pub value_usage_context: Option<types::UsageContext>,

    /// Value of extension (Availability)
    pub value_availability: Option<types::Availability>,

    /// Value of extension (ExtendedContactDetail)
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>,

    /// Value of extension (Dosage)
    pub value_dosage: Option<types::Dosage>,

    /// Value of extension (Meta)
    pub value_meta: Option<types::Meta>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Extension;

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
