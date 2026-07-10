//! ElementDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ElementDefinition
//!
//! Version: 5.0.0
//!
//! ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// ElementDefinition is one of the most complex FHIR datatypes. It captures the
/// constraints, metadata, bindings, and mappings that apply to a single element
/// within a resource, profile, data type, or extension definition. Each
/// StructureDefinition contains many ElementDefinition entries that together
/// describe the shape and rules of the resource. It supports slicing, cardinality,
/// terminology bindings, invariants, examples, and default/fixed/pattern values.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::element_definition::ElementDefinition;
///
/// let value = ElementDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ElementDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Path of the element in the hierarchy of elements
    pub path: types::String,

    /// xmlAttr | xmlText | typeAttr | cdaText | xhtml
    pub representation: Option<Vec<types::Code>>,

    /// Name for this particular element (in a set of slices)
    pub slice_name: Option<types::String>,

    /// If this slice definition constrains an inherited slice definition (or not)
    pub slice_is_constraining: Option<types::Boolean>,

    /// Name for element to display with or prompt for element
    pub label: Option<types::String>,

    /// Corresponding codes in terminologies
    pub code: Option<Vec<types::Coding>>,

    /// This element is sliced - slices follow
    pub slicing: Option<ElementDefinitionSlicing>,

    /// Concise definition for space-constrained presentation
    pub short: Option<types::String>,

    /// Full formal definition as narrative text
    pub definition: Option<types::Markdown>,

    /// Comments about the use of this element
    pub comment: Option<types::Markdown>,

    /// Why this resource has been created
    pub requirements: Option<types::Markdown>,

    /// Other names
    pub alias: Option<Vec<types::String>>,

    /// Minimum Cardinality
    pub min: Option<types::UnsignedInt>,

    /// Maximum Cardinality (a number or *)
    pub max: Option<types::String>,

    /// Base definition information for tools
    pub base: Option<ElementDefinitionBase>,

    /// Reference to definition of content for the element
    pub content_reference: Option<types::Uri>,

    /// Data type and Profile for this element
    pub r#type: Option<Vec<ElementDefinitionType>>,

    /// Specified value if missing from instance
    pub default_value_base64_binary: Option<types::Base64Binary>,
    /// Specified value if missing from instance
    pub default_value_boolean: Option<types::Boolean>,
    /// Specified value if missing from instance
    pub default_value_canonical: Option<types::Canonical>,
    /// Specified value if missing from instance
    pub default_value_code: Option<types::Code>,
    /// Specified value if missing from instance
    pub default_value_date: Option<types::Date>,
    /// Specified value if missing from instance
    pub default_value_date_time: Option<types::DateTime>,
    /// Specified value if missing from instance
    pub default_value_decimal: Option<types::Decimal>,
    /// Specified value if missing from instance
    pub default_value_id: Option<types::Id>,
    /// Specified value if missing from instance
    pub default_value_instant: Option<types::Instant>,
    /// Specified value if missing from instance
    pub default_value_integer: Option<types::Integer>,
    /// Specified value if missing from instance
    pub default_value_integer64: Option<types::Integer64>,
    /// Specified value if missing from instance
    pub default_value_markdown: Option<types::Markdown>,
    /// Specified value if missing from instance
    pub default_value_oid: Option<types::Oid>,
    /// Specified value if missing from instance
    pub default_value_positive_int: Option<types::PositiveInt>,
    /// Specified value if missing from instance
    pub default_value_string: Option<types::String>,
    /// Specified value if missing from instance
    pub default_value_time: Option<types::Time>,
    /// Specified value if missing from instance
    pub default_value_unsigned_int: Option<types::UnsignedInt>,
    /// Specified value if missing from instance
    pub default_value_uri: Option<types::Uri>,
    /// Specified value if missing from instance
    pub default_value_url: Option<types::Url>,
    /// Specified value if missing from instance
    pub default_value_uuid: Option<types::Uuid>,
    /// Specified value if missing from instance
    pub default_value_address: Option<types::Address>,
    /// Specified value if missing from instance
    pub default_value_age: Option<types::Age>,
    /// Specified value if missing from instance
    pub default_value_annotation: Option<types::Annotation>,
    /// Specified value if missing from instance
    pub default_value_attachment: Option<types::Attachment>,
    /// Specified value if missing from instance
    pub default_value_codeable_concept: Option<types::CodeableConcept>,
    /// Specified value if missing from instance
    pub default_value_codeable_reference: Option<types::CodeableReference>,
    /// Specified value if missing from instance
    pub default_value_coding: Option<types::Coding>,
    /// Specified value if missing from instance
    pub default_value_contact_point: Option<types::ContactPoint>,
    /// Specified value if missing from instance
    pub default_value_count: Option<types::Count>,
    /// Specified value if missing from instance
    pub default_value_distance: Option<types::Distance>,
    /// Specified value if missing from instance
    pub default_value_duration: Option<types::Duration>,
    /// Specified value if missing from instance
    pub default_value_human_name: Option<types::HumanName>,
    /// Specified value if missing from instance
    pub default_value_identifier: Option<types::Identifier>,
    /// Specified value if missing from instance
    pub default_value_money: Option<types::Money>,
    /// Specified value if missing from instance
    pub default_value_period: Option<types::Period>,
    /// Specified value if missing from instance
    pub default_value_quantity: Option<types::Quantity>,
    /// Specified value if missing from instance
    pub default_value_range: Option<types::Range>,
    /// Specified value if missing from instance
    pub default_value_ratio: Option<types::Ratio>,
    /// Specified value if missing from instance
    pub default_value_ratio_range: Option<types::RatioRange>,
    /// Specified value if missing from instance
    pub default_value_reference: Option<types::Reference>,
    /// Specified value if missing from instance
    pub default_value_sampled_data: Option<types::SampledData>,
    /// Specified value if missing from instance
    pub default_value_signature: Option<types::Signature>,
    /// Specified value if missing from instance
    pub default_value_timing: Option<types::Timing>,
    /// Specified value if missing from instance
    pub default_value_contact_detail: Option<types::ContactDetail>,
    /// Specified value if missing from instance
    pub default_value_data_requirement: Option<types::DataRequirement>,
    /// Specified value if missing from instance
    pub default_value_expression: Option<types::Expression>,
    /// Specified value if missing from instance
    pub default_value_parameter_definition: Option<types::ParameterDefinition>,
    /// Specified value if missing from instance
    pub default_value_related_artifact: Option<types::RelatedArtifact>,
    /// Specified value if missing from instance
    pub default_value_trigger_definition: Option<types::TriggerDefinition>,
    /// Specified value if missing from instance
    pub default_value_usage_context: Option<types::UsageContext>,
    /// Specified value if missing from instance
    pub default_value_availability: Option<types::Availability>,
    /// Specified value if missing from instance
    pub default_value_extended_contact_detail: Option<types::ExtendedContactDetail>,
    /// Specified value if missing from instance
    pub default_value_dosage: Option<types::Dosage>,
    /// Specified value if missing from instance
    pub default_value_meta: Option<types::Meta>,

    /// Implicit meaning when this element is missing
    pub meaning_when_missing: Option<types::Markdown>,

    /// What the order of the elements means
    pub order_meaning: Option<types::String>,

    /// Value must be exactly this
    pub fixed_base64_binary: Option<types::Base64Binary>,
    /// Value must be exactly this
    pub fixed_boolean: Option<types::Boolean>,
    /// Value must be exactly this
    pub fixed_canonical: Option<types::Canonical>,
    /// Value must be exactly this
    pub fixed_code: Option<types::Code>,
    /// Value must be exactly this
    pub fixed_date: Option<types::Date>,
    /// Value must be exactly this
    pub fixed_date_time: Option<types::DateTime>,
    /// Value must be exactly this
    pub fixed_decimal: Option<types::Decimal>,
    /// Value must be exactly this
    pub fixed_id: Option<types::Id>,
    /// Value must be exactly this
    pub fixed_instant: Option<types::Instant>,
    /// Value must be exactly this
    pub fixed_integer: Option<types::Integer>,
    /// Value must be exactly this
    pub fixed_integer64: Option<types::Integer64>,
    /// Value must be exactly this
    pub fixed_markdown: Option<types::Markdown>,
    /// Value must be exactly this
    pub fixed_oid: Option<types::Oid>,
    /// Value must be exactly this
    pub fixed_positive_int: Option<types::PositiveInt>,
    /// Value must be exactly this
    pub fixed_string: Option<types::String>,
    /// Value must be exactly this
    pub fixed_time: Option<types::Time>,
    /// Value must be exactly this
    pub fixed_unsigned_int: Option<types::UnsignedInt>,
    /// Value must be exactly this
    pub fixed_uri: Option<types::Uri>,
    /// Value must be exactly this
    pub fixed_url: Option<types::Url>,
    /// Value must be exactly this
    pub fixed_uuid: Option<types::Uuid>,
    /// Value must be exactly this
    pub fixed_address: Option<types::Address>,
    /// Value must be exactly this
    pub fixed_age: Option<types::Age>,
    /// Value must be exactly this
    pub fixed_annotation: Option<types::Annotation>,
    /// Value must be exactly this
    pub fixed_attachment: Option<types::Attachment>,
    /// Value must be exactly this
    pub fixed_codeable_concept: Option<types::CodeableConcept>,
    /// Value must be exactly this
    pub fixed_codeable_reference: Option<types::CodeableReference>,
    /// Value must be exactly this
    pub fixed_coding: Option<types::Coding>,
    /// Value must be exactly this
    pub fixed_contact_point: Option<types::ContactPoint>,
    /// Value must be exactly this
    pub fixed_count: Option<types::Count>,
    /// Value must be exactly this
    pub fixed_distance: Option<types::Distance>,
    /// Value must be exactly this
    pub fixed_duration: Option<types::Duration>,
    /// Value must be exactly this
    pub fixed_human_name: Option<types::HumanName>,
    /// Value must be exactly this
    pub fixed_identifier: Option<types::Identifier>,
    /// Value must be exactly this
    pub fixed_money: Option<types::Money>,
    /// Value must be exactly this
    pub fixed_period: Option<types::Period>,
    /// Value must be exactly this
    pub fixed_quantity: Option<types::Quantity>,
    /// Value must be exactly this
    pub fixed_range: Option<types::Range>,
    /// Value must be exactly this
    pub fixed_ratio: Option<types::Ratio>,
    /// Value must be exactly this
    pub fixed_ratio_range: Option<types::RatioRange>,
    /// Value must be exactly this
    pub fixed_reference: Option<types::Reference>,
    /// Value must be exactly this
    pub fixed_sampled_data: Option<types::SampledData>,
    /// Value must be exactly this
    pub fixed_signature: Option<types::Signature>,
    /// Value must be exactly this
    pub fixed_timing: Option<types::Timing>,
    /// Value must be exactly this
    pub fixed_contact_detail: Option<types::ContactDetail>,
    /// Value must be exactly this
    pub fixed_data_requirement: Option<types::DataRequirement>,
    /// Value must be exactly this
    pub fixed_expression: Option<types::Expression>,
    /// Value must be exactly this
    pub fixed_parameter_definition: Option<types::ParameterDefinition>,
    /// Value must be exactly this
    pub fixed_related_artifact: Option<types::RelatedArtifact>,
    /// Value must be exactly this
    pub fixed_trigger_definition: Option<types::TriggerDefinition>,
    /// Value must be exactly this
    pub fixed_usage_context: Option<types::UsageContext>,
    /// Value must be exactly this
    pub fixed_availability: Option<types::Availability>,
    /// Value must be exactly this
    pub fixed_extended_contact_detail: Option<types::ExtendedContactDetail>,
    /// Value must be exactly this
    pub fixed_dosage: Option<types::Dosage>,
    /// Value must be exactly this
    pub fixed_meta: Option<types::Meta>,

    /// Value must have at least these property values
    pub pattern_base64_binary: Option<types::Base64Binary>,
    /// Value must have at least these property values
    pub pattern_boolean: Option<types::Boolean>,
    /// Value must have at least these property values
    pub pattern_canonical: Option<types::Canonical>,
    /// Value must have at least these property values
    pub pattern_code: Option<types::Code>,
    /// Value must have at least these property values
    pub pattern_date: Option<types::Date>,
    /// Value must have at least these property values
    pub pattern_date_time: Option<types::DateTime>,
    /// Value must have at least these property values
    pub pattern_decimal: Option<types::Decimal>,
    /// Value must have at least these property values
    pub pattern_id: Option<types::Id>,
    /// Value must have at least these property values
    pub pattern_instant: Option<types::Instant>,
    /// Value must have at least these property values
    pub pattern_integer: Option<types::Integer>,
    /// Value must have at least these property values
    pub pattern_integer64: Option<types::Integer64>,
    /// Value must have at least these property values
    pub pattern_markdown: Option<types::Markdown>,
    /// Value must have at least these property values
    pub pattern_oid: Option<types::Oid>,
    /// Value must have at least these property values
    pub pattern_positive_int: Option<types::PositiveInt>,
    /// Value must have at least these property values
    pub pattern_string: Option<types::String>,
    /// Value must have at least these property values
    pub pattern_time: Option<types::Time>,
    /// Value must have at least these property values
    pub pattern_unsigned_int: Option<types::UnsignedInt>,
    /// Value must have at least these property values
    pub pattern_uri: Option<types::Uri>,
    /// Value must have at least these property values
    pub pattern_url: Option<types::Url>,
    /// Value must have at least these property values
    pub pattern_uuid: Option<types::Uuid>,
    /// Value must have at least these property values
    pub pattern_address: Option<types::Address>,
    /// Value must have at least these property values
    pub pattern_age: Option<types::Age>,
    /// Value must have at least these property values
    pub pattern_annotation: Option<types::Annotation>,
    /// Value must have at least these property values
    pub pattern_attachment: Option<types::Attachment>,
    /// Value must have at least these property values
    pub pattern_codeable_concept: Option<types::CodeableConcept>,
    /// Value must have at least these property values
    pub pattern_codeable_reference: Option<types::CodeableReference>,
    /// Value must have at least these property values
    pub pattern_coding: Option<types::Coding>,
    /// Value must have at least these property values
    pub pattern_contact_point: Option<types::ContactPoint>,
    /// Value must have at least these property values
    pub pattern_count: Option<types::Count>,
    /// Value must have at least these property values
    pub pattern_distance: Option<types::Distance>,
    /// Value must have at least these property values
    pub pattern_duration: Option<types::Duration>,
    /// Value must have at least these property values
    pub pattern_human_name: Option<types::HumanName>,
    /// Value must have at least these property values
    pub pattern_identifier: Option<types::Identifier>,
    /// Value must have at least these property values
    pub pattern_money: Option<types::Money>,
    /// Value must have at least these property values
    pub pattern_period: Option<types::Period>,
    /// Value must have at least these property values
    pub pattern_quantity: Option<types::Quantity>,
    /// Value must have at least these property values
    pub pattern_range: Option<types::Range>,
    /// Value must have at least these property values
    pub pattern_ratio: Option<types::Ratio>,
    /// Value must have at least these property values
    pub pattern_ratio_range: Option<types::RatioRange>,
    /// Value must have at least these property values
    pub pattern_reference: Option<types::Reference>,
    /// Value must have at least these property values
    pub pattern_sampled_data: Option<types::SampledData>,
    /// Value must have at least these property values
    pub pattern_signature: Option<types::Signature>,
    /// Value must have at least these property values
    pub pattern_timing: Option<types::Timing>,
    /// Value must have at least these property values
    pub pattern_contact_detail: Option<types::ContactDetail>,
    /// Value must have at least these property values
    pub pattern_data_requirement: Option<types::DataRequirement>,
    /// Value must have at least these property values
    pub pattern_expression: Option<types::Expression>,
    /// Value must have at least these property values
    pub pattern_parameter_definition: Option<types::ParameterDefinition>,
    /// Value must have at least these property values
    pub pattern_related_artifact: Option<types::RelatedArtifact>,
    /// Value must have at least these property values
    pub pattern_trigger_definition: Option<types::TriggerDefinition>,
    /// Value must have at least these property values
    pub pattern_usage_context: Option<types::UsageContext>,
    /// Value must have at least these property values
    pub pattern_availability: Option<types::Availability>,
    /// Value must have at least these property values
    pub pattern_extended_contact_detail: Option<types::ExtendedContactDetail>,
    /// Value must have at least these property values
    pub pattern_dosage: Option<types::Dosage>,
    /// Value must have at least these property values
    pub pattern_meta: Option<types::Meta>,

    /// Example value (as defined for type)
    pub example: Option<Vec<ElementDefinitionExample>>,

    /// Minimum Allowed Value (for some types)
    pub min_value_date: Option<types::Date>,
    /// Minimum Allowed Value (for some types)
    pub min_value_date_time: Option<types::DateTime>,
    /// Minimum Allowed Value (for some types)
    pub min_value_instant: Option<types::Instant>,
    /// Minimum Allowed Value (for some types)
    pub min_value_time: Option<types::Time>,
    /// Minimum Allowed Value (for some types)
    pub min_value_decimal: Option<types::Decimal>,
    /// Minimum Allowed Value (for some types)
    pub min_value_integer: Option<types::Integer>,
    /// Minimum Allowed Value (for some types)
    pub min_value_integer64: Option<types::Integer64>,
    /// Minimum Allowed Value (for some types)
    pub min_value_positive_int: Option<types::PositiveInt>,
    /// Minimum Allowed Value (for some types)
    pub min_value_unsigned_int: Option<types::UnsignedInt>,
    /// Minimum Allowed Value (for some types)
    pub min_value_quantity: Option<types::Quantity>,

    /// Maximum Allowed Value (for some types)
    pub max_value_date: Option<types::Date>,
    /// Maximum Allowed Value (for some types)
    pub max_value_date_time: Option<types::DateTime>,
    /// Maximum Allowed Value (for some types)
    pub max_value_instant: Option<types::Instant>,
    /// Maximum Allowed Value (for some types)
    pub max_value_time: Option<types::Time>,
    /// Maximum Allowed Value (for some types)
    pub max_value_decimal: Option<types::Decimal>,
    /// Maximum Allowed Value (for some types)
    pub max_value_integer: Option<types::Integer>,
    /// Maximum Allowed Value (for some types)
    pub max_value_integer64: Option<types::Integer64>,
    /// Maximum Allowed Value (for some types)
    pub max_value_positive_int: Option<types::PositiveInt>,
    /// Maximum Allowed Value (for some types)
    pub max_value_unsigned_int: Option<types::UnsignedInt>,
    /// Maximum Allowed Value (for some types)
    pub max_value_quantity: Option<types::Quantity>,

    /// Max length for string type data
    pub max_length: Option<types::Integer>,

    /// Reference to invariant about presence
    pub condition: Option<Vec<types::Id>>,

    /// Condition that must evaluate to true
    pub constraint: Option<Vec<ElementDefinitionConstraint>>,

    /// For primitives, that a value must be present - not replaced by an extension
    pub must_have_value: Option<types::Boolean>,

    /// Extensions that are allowed to replace a primitive value
    pub value_alternatives: Option<Vec<types::Canonical>>,

    /// If the element must be supported (discouraged - see obligations)
    pub must_support: Option<types::Boolean>,

    /// If this modifies the meaning of other elements
    pub is_modifier: Option<types::Boolean>,

    /// Reason that this element is marked as a modifier
    pub is_modifier_reason: Option<types::String>,

    /// Include when _summary = true?
    pub is_summary: Option<types::Boolean>,

    /// ValueSet details if this is coded
    pub binding: Option<ElementDefinitionBinding>,

    /// Map element to another set of definitions
    pub mapping: Option<Vec<ElementDefinitionMapping>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionSlicing {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Element values that are used to distinguish the slices
    pub discriminator: Option<Vec<ElementDefinitionSlicingDiscriminator>>,

    /// Text description of how slicing works (or not)
    pub description: Option<types::String>,

    /// If elements must be in same order as slices
    pub ordered: Option<types::Boolean>,

    /// closed | open | openAtEnd
    pub rules: types::Code,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionSlicingDiscriminator {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// value | exists | type | profile | position
    pub r#type: types::Code,

    /// Path to element value
    pub path: types::String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionBase {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Path that identifies the base element
    pub path: types::String,

    /// Min cardinality of the base element
    pub min: types::UnsignedInt,

    /// Max cardinality of the base element
    pub max: types::String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionType {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Data type or Resource (reference to definition)
    pub code: types::Uri,

    /// Profiles (StructureDefinition or IG) - one must apply
    pub profile: Option<Vec<types::Canonical>>,

    /// Profile on the Reference/canonical target - one must apply
    pub target_profile: Option<Vec<types::Canonical>>,

    /// contained | referenced | bundled - how aggregated
    pub aggregation: Option<Vec<types::Code>>,

    /// either | independent | specific
    pub versioning: Option<types::Code>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionExample {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Describes the purpose of this example
    pub label: types::String,

    /// Value of Example (one of allowed types)
    pub value_base64_binary: Option<types::Base64Binary>,
    /// Value of Example (one of allowed types)
    pub value_boolean: Option<types::Boolean>,
    /// Value of Example (one of allowed types)
    pub value_canonical: Option<types::Canonical>,
    /// Value of Example (one of allowed types)
    pub value_code: Option<types::Code>,
    /// Value of Example (one of allowed types)
    pub value_date: Option<types::Date>,
    /// Value of Example (one of allowed types)
    pub value_date_time: Option<types::DateTime>,
    /// Value of Example (one of allowed types)
    pub value_decimal: Option<types::Decimal>,
    /// Value of Example (one of allowed types)
    pub value_id: Option<types::Id>,
    /// Value of Example (one of allowed types)
    pub value_instant: Option<types::Instant>,
    /// Value of Example (one of allowed types)
    pub value_integer: Option<types::Integer>,
    /// Value of Example (one of allowed types)
    pub value_integer64: Option<types::Integer64>,
    /// Value of Example (one of allowed types)
    pub value_markdown: Option<types::Markdown>,
    /// Value of Example (one of allowed types)
    pub value_oid: Option<types::Oid>,
    /// Value of Example (one of allowed types)
    pub value_positive_int: Option<types::PositiveInt>,
    /// Value of Example (one of allowed types)
    pub value_string: Option<types::String>,
    /// Value of Example (one of allowed types)
    pub value_time: Option<types::Time>,
    /// Value of Example (one of allowed types)
    pub value_unsigned_int: Option<types::UnsignedInt>,
    /// Value of Example (one of allowed types)
    pub value_uri: Option<types::Uri>,
    /// Value of Example (one of allowed types)
    pub value_url: Option<types::Url>,
    /// Value of Example (one of allowed types)
    pub value_uuid: Option<types::Uuid>,
    /// Value of Example (one of allowed types)
    pub value_address: Option<types::Address>,
    /// Value of Example (one of allowed types)
    pub value_age: Option<types::Age>,
    /// Value of Example (one of allowed types)
    pub value_annotation: Option<types::Annotation>,
    /// Value of Example (one of allowed types)
    pub value_attachment: Option<types::Attachment>,
    /// Value of Example (one of allowed types)
    pub value_codeable_concept: Option<types::CodeableConcept>,
    /// Value of Example (one of allowed types)
    pub value_codeable_reference: Option<types::CodeableReference>,
    /// Value of Example (one of allowed types)
    pub value_coding: Option<types::Coding>,
    /// Value of Example (one of allowed types)
    pub value_contact_point: Option<types::ContactPoint>,
    /// Value of Example (one of allowed types)
    pub value_count: Option<types::Count>,
    /// Value of Example (one of allowed types)
    pub value_distance: Option<types::Distance>,
    /// Value of Example (one of allowed types)
    pub value_duration: Option<types::Duration>,
    /// Value of Example (one of allowed types)
    pub value_human_name: Option<types::HumanName>,
    /// Value of Example (one of allowed types)
    pub value_identifier: Option<types::Identifier>,
    /// Value of Example (one of allowed types)
    pub value_money: Option<types::Money>,
    /// Value of Example (one of allowed types)
    pub value_period: Option<types::Period>,
    /// Value of Example (one of allowed types)
    pub value_quantity: Option<types::Quantity>,
    /// Value of Example (one of allowed types)
    pub value_range: Option<types::Range>,
    /// Value of Example (one of allowed types)
    pub value_ratio: Option<types::Ratio>,
    /// Value of Example (one of allowed types)
    pub value_ratio_range: Option<types::RatioRange>,
    /// Value of Example (one of allowed types)
    pub value_reference: Option<types::Reference>,
    /// Value of Example (one of allowed types)
    pub value_sampled_data: Option<types::SampledData>,
    /// Value of Example (one of allowed types)
    pub value_signature: Option<types::Signature>,
    /// Value of Example (one of allowed types)
    pub value_timing: Option<types::Timing>,
    /// Value of Example (one of allowed types)
    pub value_contact_detail: Option<types::ContactDetail>,
    /// Value of Example (one of allowed types)
    pub value_data_requirement: Option<types::DataRequirement>,
    /// Value of Example (one of allowed types)
    pub value_expression: Option<types::Expression>,
    /// Value of Example (one of allowed types)
    pub value_parameter_definition: Option<types::ParameterDefinition>,
    /// Value of Example (one of allowed types)
    pub value_related_artifact: Option<types::RelatedArtifact>,
    /// Value of Example (one of allowed types)
    pub value_trigger_definition: Option<types::TriggerDefinition>,
    /// Value of Example (one of allowed types)
    pub value_usage_context: Option<types::UsageContext>,
    /// Value of Example (one of allowed types)
    pub value_availability: Option<types::Availability>,
    /// Value of Example (one of allowed types)
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>,
    /// Value of Example (one of allowed types)
    pub value_dosage: Option<types::Dosage>,
    /// Value of Example (one of allowed types)
    pub value_meta: Option<types::Meta>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionConstraint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Target of 'condition' reference above
    pub key: types::Id,

    /// Why this constraint is necessary or appropriate
    pub requirements: Option<types::Markdown>,

    /// error | warning
    pub severity: types::Code,

    /// Suppress warning or hint in profile
    pub suppress: Option<types::Boolean>,

    /// Human description of constraint
    pub human: types::String,

    /// FHIRPath expression of constraint
    pub expression: Option<types::String>,

    /// Reference to original source of constraint
    pub source: Option<types::Canonical>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionBinding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// required | extensible | preferred | example
    pub strength: types::Code,

    /// Intended use of codes in the bound value set
    pub description: Option<types::Markdown>,

    /// Source of value set
    pub value_set: Option<types::Canonical>,

    /// Additional Bindings - more rules about the binding
    pub additional: Option<Vec<ElementDefinitionBindingAdditional>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionBindingAdditional {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// maximum | minimum | required | extensible | candidate | current | preferred | ui | starter | component
    pub purpose: types::Code,

    /// The value set for the additional binding
    pub value_set: types::Canonical,

    /// Documentation of the purpose of use of the binding
    pub documentation: Option<types::Markdown>,

    /// Concise documentation - for summary tables
    pub short_doco: Option<types::String>,

    /// Qualifies the usage - jurisdiction, gender, workflow status etc.
    pub usage: Option<Vec<types::UsageContext>>,

    /// Whether binding can applies to all repeats, or just one
    pub any: Option<types::Boolean>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinitionMapping {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Reference to mapping declaration
    pub identity: types::Id,

    /// Computable language of mapping
    pub language: Option<types::Code>,

    /// Details of the mapping
    pub map: types::String,

    /// Comments about the mapping or its use
    pub comment: Option<types::Markdown>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ElementDefinition;

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
