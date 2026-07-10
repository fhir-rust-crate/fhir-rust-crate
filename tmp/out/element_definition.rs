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
        pub struct ElementDefinition {
            /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Path of the element in the hierarchy of elements
    pub path: types::String, // string [1..1]

    /// xmlAttr | xmlText | typeAttr | cdaText | xhtml
    pub representation: Option<Vec<types::Code>>, // code [0..*]

    /// Name for this particular element (in a set of slices)
    pub slice_name: Option<types::String>, // string [0..1]

    /// If this slice definition constrains an inherited slice definition (or not)
    pub slice_is_constraining: Option<types::Boolean>, // boolean [0..1]

    /// Name for element to display with or prompt for element
    pub label: Option<types::String>, // string [0..1]

    /// Corresponding codes in terminologies
    pub code: Option<Vec<types::Coding>>, // Coding [0..*]

    /// This element is sliced - slices follow
    pub slicing: Option<types::Element>, // Element [0..1]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Element values that are used to distinguish the slices
    pub discriminator: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// value | exists | type | profile | position
    pub r#type: types::Code, // code [1..1]

    /// Path to element value
    pub path: types::String, // string [1..1]

    /// Text description of how slicing works (or not)
    pub description: Option<types::String>, // string [0..1]

    /// If elements must be in same order as slices
    pub ordered: Option<types::Boolean>, // boolean [0..1]

    /// closed | open | openAtEnd
    pub rules: types::Code, // code [1..1]

    /// Concise definition for space-constrained presentation
    pub short: Option<types::String>, // string [0..1]

    /// Full formal definition as narrative text
    pub definition: Option<types::Markdown>, // markdown [0..1]

    /// Comments about the use of this element
    pub comment: Option<types::Markdown>, // markdown [0..1]

    /// Why this resource has been created
    pub requirements: Option<types::Markdown>, // markdown [0..1]

    /// Other names
    pub alias: Option<Vec<types::String>>, // string [0..*]

    /// Minimum Cardinality
    pub min: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// Maximum Cardinality (a number or *)
    pub max: Option<types::String>, // string [0..1]

    /// Base definition information for tools
    pub base: Option<types::Element>, // Element [0..1]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Path that identifies the base element
    pub path: types::String, // string [1..1]

    /// Min cardinality of the base element
    pub min: types::UnsignedInt, // unsignedInt [1..1]

    /// Max cardinality of the base element
    pub max: types::String, // string [1..1]

    /// Reference to definition of content for the element
    pub content_reference: Option<types::Uri>, // uri [0..1]

    /// Data type and Profile for this element
    pub r#type: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Data type or Resource (reference to definition)
    pub code: types::Uri, // uri [1..1]

    /// Profiles (StructureDefinition or IG) - one must apply
    pub profile: Option<Vec<types::Canonical>>, // canonical [0..*]

    /// Profile (StructureDefinition or IG) on the Reference/canonical target - one must apply
    pub target_profile: Option<Vec<types::Canonical>>, // canonical [0..*]

    /// contained | referenced | bundled - how aggregated
    pub aggregation: Option<Vec<types::Code>>, // code [0..*]

    /// either | independent | specific
    pub versioning: Option<types::Code>, // code [0..1]

    /// Specified value if missing from instance
    pub defaultValue_base_64_binary: Option<types::Base64Binary>, // base64Binary [0..1]

    /// Specified value if missing from instance
    pub defaultValue_boolean: Option<types::Boolean>, // boolean [0..1]

    /// Specified value if missing from instance
    pub defaultValue_canonical: Option<types::Canonical>, // canonical [0..1]

    /// Specified value if missing from instance
    pub defaultValue_code: Option<types::Code>, // code [0..1]

    /// Specified value if missing from instance
    pub defaultValue_date: Option<types::Date>, // date [0..1]

    /// Specified value if missing from instance
    pub defaultValue_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// Specified value if missing from instance
    pub defaultValue_decimal: Option<types::Decimal>, // decimal [0..1]

    /// Specified value if missing from instance
    pub defaultValue_id: Option<types::Id>, // id [0..1]

    /// Specified value if missing from instance
    pub defaultValue_instant: Option<types::Instant>, // instant [0..1]

    /// Specified value if missing from instance
    pub defaultValue_integer: Option<types::Integer>, // integer [0..1]

    /// Specified value if missing from instance
    pub defaultValue_integer_64: Option<types::Integer64>, // integer64 [0..1]

    /// Specified value if missing from instance
    pub defaultValue_markdown: Option<types::Markdown>, // markdown [0..1]

    /// Specified value if missing from instance
    pub defaultValue_oid: Option<types::Oid>, // oid [0..1]

    /// Specified value if missing from instance
    pub defaultValue_positive_int: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Specified value if missing from instance
    pub defaultValue_string: Option<types::String>, // string [0..1]

    /// Specified value if missing from instance
    pub defaultValue_time: Option<types::Time>, // time [0..1]

    /// Specified value if missing from instance
    pub defaultValue_unsigned_int: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// Specified value if missing from instance
    pub defaultValue_uri: Option<types::Uri>, // uri [0..1]

    /// Specified value if missing from instance
    pub defaultValue_url: Option<types::Url>, // url [0..1]

    /// Specified value if missing from instance
    pub defaultValue_uuid: Option<types::Uuid>, // uuid [0..1]

    /// Specified value if missing from instance
    pub defaultValue_address: Option<types::Address>, // Address [0..1]

    /// Specified value if missing from instance
    pub defaultValue_age: Option<types::Age>, // Age [0..1]

    /// Specified value if missing from instance
    pub defaultValue_annotation: Option<types::Annotation>, // Annotation [0..1]

    /// Specified value if missing from instance
    pub defaultValue_attachment: Option<types::Attachment>, // Attachment [0..1]

    /// Specified value if missing from instance
    pub defaultValue_codeable_concept: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// Specified value if missing from instance
    pub defaultValue_codeable_reference: Option<types::CodeableReference>, // CodeableReference [0..1]

    /// Specified value if missing from instance
    pub defaultValue_coding: Option<types::Coding>, // Coding [0..1]

    /// Specified value if missing from instance
    pub defaultValue_contact_point: Option<types::ContactPoint>, // ContactPoint [0..1]

    /// Specified value if missing from instance
    pub defaultValue_count: Option<types::Count>, // Count [0..1]

    /// Specified value if missing from instance
    pub defaultValue_distance: Option<types::Distance>, // Distance [0..1]

    /// Specified value if missing from instance
    pub defaultValue_duration: Option<types::Duration>, // Duration [0..1]

    /// Specified value if missing from instance
    pub defaultValue_human_name: Option<types::HumanName>, // HumanName [0..1]

    /// Specified value if missing from instance
    pub defaultValue_identifier: Option<types::Identifier>, // Identifier [0..1]

    /// Specified value if missing from instance
    pub defaultValue_money: Option<types::Money>, // Money [0..1]

    /// Specified value if missing from instance
    pub defaultValue_period: Option<types::Period>, // Period [0..1]

    /// Specified value if missing from instance
    pub defaultValue_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Specified value if missing from instance
    pub defaultValue_range: Option<types::Range>, // Range [0..1]

    /// Specified value if missing from instance
    pub defaultValue_ratio: Option<types::Ratio>, // Ratio [0..1]

    /// Specified value if missing from instance
    pub defaultValue_ratio_range: Option<types::RatioRange>, // RatioRange [0..1]

    /// Specified value if missing from instance
    pub defaultValue_reference: Option<types::Reference>, // Reference [0..1]

    /// Specified value if missing from instance
    pub defaultValue_sampled_data: Option<types::SampledData>, // SampledData [0..1]

    /// Specified value if missing from instance
    pub defaultValue_signature: Option<types::Signature>, // Signature [0..1]

    /// Specified value if missing from instance
    pub defaultValue_timing: Option<types::Timing>, // Timing [0..1]

    /// Specified value if missing from instance
    pub defaultValue_contact_detail: Option<types::ContactDetail>, // ContactDetail [0..1]

    /// Specified value if missing from instance
    pub defaultValue_data_requirement: Option<types::DataRequirement>, // DataRequirement [0..1]

    /// Specified value if missing from instance
    pub defaultValue_expression: Option<types::Expression>, // Expression [0..1]

    /// Specified value if missing from instance
    pub defaultValue_parameter_definition: Option<types::ParameterDefinition>, // ParameterDefinition [0..1]

    /// Specified value if missing from instance
    pub defaultValue_related_artifact: Option<types::RelatedArtifact>, // RelatedArtifact [0..1]

    /// Specified value if missing from instance
    pub defaultValue_trigger_definition: Option<types::TriggerDefinition>, // TriggerDefinition [0..1]

    /// Specified value if missing from instance
    pub defaultValue_usage_context: Option<types::UsageContext>, // UsageContext [0..1]

    /// Specified value if missing from instance
    pub defaultValue_availability: Option<types::Availability>, // Availability [0..1]

    /// Specified value if missing from instance
    pub defaultValue_extended_contact_detail: Option<types::ExtendedContactDetail>, // ExtendedContactDetail [0..1]

    /// Specified value if missing from instance
    pub defaultValue_dosage: Option<types::Dosage>, // Dosage [0..1]

    /// Specified value if missing from instance
    pub defaultValue_meta: Option<types::Meta>, // Meta [0..1]

    /// Implicit meaning when this element is missing
    pub meaning_when_missing: Option<types::Markdown>, // markdown [0..1]

    /// What the order of the elements means
    pub order_meaning: Option<types::String>, // string [0..1]

    /// Value must be exactly this
    pub fixed_base_64_binary: Option<types::Base64Binary>, // base64Binary [0..1]

    /// Value must be exactly this
    pub fixed_boolean: Option<types::Boolean>, // boolean [0..1]

    /// Value must be exactly this
    pub fixed_canonical: Option<types::Canonical>, // canonical [0..1]

    /// Value must be exactly this
    pub fixed_code: Option<types::Code>, // code [0..1]

    /// Value must be exactly this
    pub fixed_date: Option<types::Date>, // date [0..1]

    /// Value must be exactly this
    pub fixed_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// Value must be exactly this
    pub fixed_decimal: Option<types::Decimal>, // decimal [0..1]

    /// Value must be exactly this
    pub fixed_id: Option<types::Id>, // id [0..1]

    /// Value must be exactly this
    pub fixed_instant: Option<types::Instant>, // instant [0..1]

    /// Value must be exactly this
    pub fixed_integer: Option<types::Integer>, // integer [0..1]

    /// Value must be exactly this
    pub fixed_integer_64: Option<types::Integer64>, // integer64 [0..1]

    /// Value must be exactly this
    pub fixed_markdown: Option<types::Markdown>, // markdown [0..1]

    /// Value must be exactly this
    pub fixed_oid: Option<types::Oid>, // oid [0..1]

    /// Value must be exactly this
    pub fixed_positive_int: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Value must be exactly this
    pub fixed_string: Option<types::String>, // string [0..1]

    /// Value must be exactly this
    pub fixed_time: Option<types::Time>, // time [0..1]

    /// Value must be exactly this
    pub fixed_unsigned_int: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// Value must be exactly this
    pub fixed_uri: Option<types::Uri>, // uri [0..1]

    /// Value must be exactly this
    pub fixed_url: Option<types::Url>, // url [0..1]

    /// Value must be exactly this
    pub fixed_uuid: Option<types::Uuid>, // uuid [0..1]

    /// Value must be exactly this
    pub fixed_address: Option<types::Address>, // Address [0..1]

    /// Value must be exactly this
    pub fixed_age: Option<types::Age>, // Age [0..1]

    /// Value must be exactly this
    pub fixed_annotation: Option<types::Annotation>, // Annotation [0..1]

    /// Value must be exactly this
    pub fixed_attachment: Option<types::Attachment>, // Attachment [0..1]

    /// Value must be exactly this
    pub fixed_codeable_concept: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// Value must be exactly this
    pub fixed_codeable_reference: Option<types::CodeableReference>, // CodeableReference [0..1]

    /// Value must be exactly this
    pub fixed_coding: Option<types::Coding>, // Coding [0..1]

    /// Value must be exactly this
    pub fixed_contact_point: Option<types::ContactPoint>, // ContactPoint [0..1]

    /// Value must be exactly this
    pub fixed_count: Option<types::Count>, // Count [0..1]

    /// Value must be exactly this
    pub fixed_distance: Option<types::Distance>, // Distance [0..1]

    /// Value must be exactly this
    pub fixed_duration: Option<types::Duration>, // Duration [0..1]

    /// Value must be exactly this
    pub fixed_human_name: Option<types::HumanName>, // HumanName [0..1]

    /// Value must be exactly this
    pub fixed_identifier: Option<types::Identifier>, // Identifier [0..1]

    /// Value must be exactly this
    pub fixed_money: Option<types::Money>, // Money [0..1]

    /// Value must be exactly this
    pub fixed_period: Option<types::Period>, // Period [0..1]

    /// Value must be exactly this
    pub fixed_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Value must be exactly this
    pub fixed_range: Option<types::Range>, // Range [0..1]

    /// Value must be exactly this
    pub fixed_ratio: Option<types::Ratio>, // Ratio [0..1]

    /// Value must be exactly this
    pub fixed_ratio_range: Option<types::RatioRange>, // RatioRange [0..1]

    /// Value must be exactly this
    pub fixed_reference: Option<types::Reference>, // Reference [0..1]

    /// Value must be exactly this
    pub fixed_sampled_data: Option<types::SampledData>, // SampledData [0..1]

    /// Value must be exactly this
    pub fixed_signature: Option<types::Signature>, // Signature [0..1]

    /// Value must be exactly this
    pub fixed_timing: Option<types::Timing>, // Timing [0..1]

    /// Value must be exactly this
    pub fixed_contact_detail: Option<types::ContactDetail>, // ContactDetail [0..1]

    /// Value must be exactly this
    pub fixed_data_requirement: Option<types::DataRequirement>, // DataRequirement [0..1]

    /// Value must be exactly this
    pub fixed_expression: Option<types::Expression>, // Expression [0..1]

    /// Value must be exactly this
    pub fixed_parameter_definition: Option<types::ParameterDefinition>, // ParameterDefinition [0..1]

    /// Value must be exactly this
    pub fixed_related_artifact: Option<types::RelatedArtifact>, // RelatedArtifact [0..1]

    /// Value must be exactly this
    pub fixed_trigger_definition: Option<types::TriggerDefinition>, // TriggerDefinition [0..1]

    /// Value must be exactly this
    pub fixed_usage_context: Option<types::UsageContext>, // UsageContext [0..1]

    /// Value must be exactly this
    pub fixed_availability: Option<types::Availability>, // Availability [0..1]

    /// Value must be exactly this
    pub fixed_extended_contact_detail: Option<types::ExtendedContactDetail>, // ExtendedContactDetail [0..1]

    /// Value must be exactly this
    pub fixed_dosage: Option<types::Dosage>, // Dosage [0..1]

    /// Value must be exactly this
    pub fixed_meta: Option<types::Meta>, // Meta [0..1]

    /// Value must have at least these property values
    pub pattern_base_64_binary: Option<types::Base64Binary>, // base64Binary [0..1]

    /// Value must have at least these property values
    pub pattern_boolean: Option<types::Boolean>, // boolean [0..1]

    /// Value must have at least these property values
    pub pattern_canonical: Option<types::Canonical>, // canonical [0..1]

    /// Value must have at least these property values
    pub pattern_code: Option<types::Code>, // code [0..1]

    /// Value must have at least these property values
    pub pattern_date: Option<types::Date>, // date [0..1]

    /// Value must have at least these property values
    pub pattern_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// Value must have at least these property values
    pub pattern_decimal: Option<types::Decimal>, // decimal [0..1]

    /// Value must have at least these property values
    pub pattern_id: Option<types::Id>, // id [0..1]

    /// Value must have at least these property values
    pub pattern_instant: Option<types::Instant>, // instant [0..1]

    /// Value must have at least these property values
    pub pattern_integer: Option<types::Integer>, // integer [0..1]

    /// Value must have at least these property values
    pub pattern_integer_64: Option<types::Integer64>, // integer64 [0..1]

    /// Value must have at least these property values
    pub pattern_markdown: Option<types::Markdown>, // markdown [0..1]

    /// Value must have at least these property values
    pub pattern_oid: Option<types::Oid>, // oid [0..1]

    /// Value must have at least these property values
    pub pattern_positive_int: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Value must have at least these property values
    pub pattern_string: Option<types::String>, // string [0..1]

    /// Value must have at least these property values
    pub pattern_time: Option<types::Time>, // time [0..1]

    /// Value must have at least these property values
    pub pattern_unsigned_int: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// Value must have at least these property values
    pub pattern_uri: Option<types::Uri>, // uri [0..1]

    /// Value must have at least these property values
    pub pattern_url: Option<types::Url>, // url [0..1]

    /// Value must have at least these property values
    pub pattern_uuid: Option<types::Uuid>, // uuid [0..1]

    /// Value must have at least these property values
    pub pattern_address: Option<types::Address>, // Address [0..1]

    /// Value must have at least these property values
    pub pattern_age: Option<types::Age>, // Age [0..1]

    /// Value must have at least these property values
    pub pattern_annotation: Option<types::Annotation>, // Annotation [0..1]

    /// Value must have at least these property values
    pub pattern_attachment: Option<types::Attachment>, // Attachment [0..1]

    /// Value must have at least these property values
    pub pattern_codeable_concept: Option<types::CodeableConcept>, // CodeableConcept [0..1]

    /// Value must have at least these property values
    pub pattern_codeable_reference: Option<types::CodeableReference>, // CodeableReference [0..1]

    /// Value must have at least these property values
    pub pattern_coding: Option<types::Coding>, // Coding [0..1]

    /// Value must have at least these property values
    pub pattern_contact_point: Option<types::ContactPoint>, // ContactPoint [0..1]

    /// Value must have at least these property values
    pub pattern_count: Option<types::Count>, // Count [0..1]

    /// Value must have at least these property values
    pub pattern_distance: Option<types::Distance>, // Distance [0..1]

    /// Value must have at least these property values
    pub pattern_duration: Option<types::Duration>, // Duration [0..1]

    /// Value must have at least these property values
    pub pattern_human_name: Option<types::HumanName>, // HumanName [0..1]

    /// Value must have at least these property values
    pub pattern_identifier: Option<types::Identifier>, // Identifier [0..1]

    /// Value must have at least these property values
    pub pattern_money: Option<types::Money>, // Money [0..1]

    /// Value must have at least these property values
    pub pattern_period: Option<types::Period>, // Period [0..1]

    /// Value must have at least these property values
    pub pattern_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Value must have at least these property values
    pub pattern_range: Option<types::Range>, // Range [0..1]

    /// Value must have at least these property values
    pub pattern_ratio: Option<types::Ratio>, // Ratio [0..1]

    /// Value must have at least these property values
    pub pattern_ratio_range: Option<types::RatioRange>, // RatioRange [0..1]

    /// Value must have at least these property values
    pub pattern_reference: Option<types::Reference>, // Reference [0..1]

    /// Value must have at least these property values
    pub pattern_sampled_data: Option<types::SampledData>, // SampledData [0..1]

    /// Value must have at least these property values
    pub pattern_signature: Option<types::Signature>, // Signature [0..1]

    /// Value must have at least these property values
    pub pattern_timing: Option<types::Timing>, // Timing [0..1]

    /// Value must have at least these property values
    pub pattern_contact_detail: Option<types::ContactDetail>, // ContactDetail [0..1]

    /// Value must have at least these property values
    pub pattern_data_requirement: Option<types::DataRequirement>, // DataRequirement [0..1]

    /// Value must have at least these property values
    pub pattern_expression: Option<types::Expression>, // Expression [0..1]

    /// Value must have at least these property values
    pub pattern_parameter_definition: Option<types::ParameterDefinition>, // ParameterDefinition [0..1]

    /// Value must have at least these property values
    pub pattern_related_artifact: Option<types::RelatedArtifact>, // RelatedArtifact [0..1]

    /// Value must have at least these property values
    pub pattern_trigger_definition: Option<types::TriggerDefinition>, // TriggerDefinition [0..1]

    /// Value must have at least these property values
    pub pattern_usage_context: Option<types::UsageContext>, // UsageContext [0..1]

    /// Value must have at least these property values
    pub pattern_availability: Option<types::Availability>, // Availability [0..1]

    /// Value must have at least these property values
    pub pattern_extended_contact_detail: Option<types::ExtendedContactDetail>, // ExtendedContactDetail [0..1]

    /// Value must have at least these property values
    pub pattern_dosage: Option<types::Dosage>, // Dosage [0..1]

    /// Value must have at least these property values
    pub pattern_meta: Option<types::Meta>, // Meta [0..1]

    /// Example value (as defined for type)
    pub example: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Describes the purpose of this example
    pub label: types::String, // string [1..1]

    /// Value of Example (one of allowed types)
    pub value_base_64_binary: Option<types::Base64Binary>, // base64Binary [1..1]

    /// Value of Example (one of allowed types)
    pub value_boolean: Option<types::Boolean>, // boolean [1..1]

    /// Value of Example (one of allowed types)
    pub value_canonical: Option<types::Canonical>, // canonical [1..1]

    /// Value of Example (one of allowed types)
    pub value_code: Option<types::Code>, // code [1..1]

    /// Value of Example (one of allowed types)
    pub value_date: Option<types::Date>, // date [1..1]

    /// Value of Example (one of allowed types)
    pub value_date_time: Option<types::DateTime>, // dateTime [1..1]

    /// Value of Example (one of allowed types)
    pub value_decimal: Option<types::Decimal>, // decimal [1..1]

    /// Value of Example (one of allowed types)
    pub value_id: Option<types::Id>, // id [1..1]

    /// Value of Example (one of allowed types)
    pub value_instant: Option<types::Instant>, // instant [1..1]

    /// Value of Example (one of allowed types)
    pub value_integer: Option<types::Integer>, // integer [1..1]

    /// Value of Example (one of allowed types)
    pub value_integer_64: Option<types::Integer64>, // integer64 [1..1]

    /// Value of Example (one of allowed types)
    pub value_markdown: Option<types::Markdown>, // markdown [1..1]

    /// Value of Example (one of allowed types)
    pub value_oid: Option<types::Oid>, // oid [1..1]

    /// Value of Example (one of allowed types)
    pub value_positive_int: Option<types::PositiveInt>, // positiveInt [1..1]

    /// Value of Example (one of allowed types)
    pub value_string: Option<types::String>, // string [1..1]

    /// Value of Example (one of allowed types)
    pub value_time: Option<types::Time>, // time [1..1]

    /// Value of Example (one of allowed types)
    pub value_unsigned_int: Option<types::UnsignedInt>, // unsignedInt [1..1]

    /// Value of Example (one of allowed types)
    pub value_uri: Option<types::Uri>, // uri [1..1]

    /// Value of Example (one of allowed types)
    pub value_url: Option<types::Url>, // url [1..1]

    /// Value of Example (one of allowed types)
    pub value_uuid: Option<types::Uuid>, // uuid [1..1]

    /// Value of Example (one of allowed types)
    pub value_address: Option<types::Address>, // Address [1..1]

    /// Value of Example (one of allowed types)
    pub value_age: Option<types::Age>, // Age [1..1]

    /// Value of Example (one of allowed types)
    pub value_annotation: Option<types::Annotation>, // Annotation [1..1]

    /// Value of Example (one of allowed types)
    pub value_attachment: Option<types::Attachment>, // Attachment [1..1]

    /// Value of Example (one of allowed types)
    pub value_codeable_concept: Option<types::CodeableConcept>, // CodeableConcept [1..1]

    /// Value of Example (one of allowed types)
    pub value_codeable_reference: Option<types::CodeableReference>, // CodeableReference [1..1]

    /// Value of Example (one of allowed types)
    pub value_coding: Option<types::Coding>, // Coding [1..1]

    /// Value of Example (one of allowed types)
    pub value_contact_point: Option<types::ContactPoint>, // ContactPoint [1..1]

    /// Value of Example (one of allowed types)
    pub value_count: Option<types::Count>, // Count [1..1]

    /// Value of Example (one of allowed types)
    pub value_distance: Option<types::Distance>, // Distance [1..1]

    /// Value of Example (one of allowed types)
    pub value_duration: Option<types::Duration>, // Duration [1..1]

    /// Value of Example (one of allowed types)
    pub value_human_name: Option<types::HumanName>, // HumanName [1..1]

    /// Value of Example (one of allowed types)
    pub value_identifier: Option<types::Identifier>, // Identifier [1..1]

    /// Value of Example (one of allowed types)
    pub value_money: Option<types::Money>, // Money [1..1]

    /// Value of Example (one of allowed types)
    pub value_period: Option<types::Period>, // Period [1..1]

    /// Value of Example (one of allowed types)
    pub value_quantity: Option<types::Quantity>, // Quantity [1..1]

    /// Value of Example (one of allowed types)
    pub value_range: Option<types::Range>, // Range [1..1]

    /// Value of Example (one of allowed types)
    pub value_ratio: Option<types::Ratio>, // Ratio [1..1]

    /// Value of Example (one of allowed types)
    pub value_ratio_range: Option<types::RatioRange>, // RatioRange [1..1]

    /// Value of Example (one of allowed types)
    pub value_reference: Option<types::Reference>, // Reference [1..1]

    /// Value of Example (one of allowed types)
    pub value_sampled_data: Option<types::SampledData>, // SampledData [1..1]

    /// Value of Example (one of allowed types)
    pub value_signature: Option<types::Signature>, // Signature [1..1]

    /// Value of Example (one of allowed types)
    pub value_timing: Option<types::Timing>, // Timing [1..1]

    /// Value of Example (one of allowed types)
    pub value_contact_detail: Option<types::ContactDetail>, // ContactDetail [1..1]

    /// Value of Example (one of allowed types)
    pub value_data_requirement: Option<types::DataRequirement>, // DataRequirement [1..1]

    /// Value of Example (one of allowed types)
    pub value_expression: Option<types::Expression>, // Expression [1..1]

    /// Value of Example (one of allowed types)
    pub value_parameter_definition: Option<types::ParameterDefinition>, // ParameterDefinition [1..1]

    /// Value of Example (one of allowed types)
    pub value_related_artifact: Option<types::RelatedArtifact>, // RelatedArtifact [1..1]

    /// Value of Example (one of allowed types)
    pub value_trigger_definition: Option<types::TriggerDefinition>, // TriggerDefinition [1..1]

    /// Value of Example (one of allowed types)
    pub value_usage_context: Option<types::UsageContext>, // UsageContext [1..1]

    /// Value of Example (one of allowed types)
    pub value_availability: Option<types::Availability>, // Availability [1..1]

    /// Value of Example (one of allowed types)
    pub value_extended_contact_detail: Option<types::ExtendedContactDetail>, // ExtendedContactDetail [1..1]

    /// Value of Example (one of allowed types)
    pub value_dosage: Option<types::Dosage>, // Dosage [1..1]

    /// Value of Example (one of allowed types)
    pub value_meta: Option<types::Meta>, // Meta [1..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_date: Option<types::Date>, // date [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_instant: Option<types::Instant>, // instant [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_time: Option<types::Time>, // time [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_decimal: Option<types::Decimal>, // decimal [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_integer: Option<types::Integer>, // integer [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_integer_64: Option<types::Integer64>, // integer64 [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_positive_int: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_unsigned_int: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// Minimum Allowed Value (for some types)
    pub minValue_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_date: Option<types::Date>, // date [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_date_time: Option<types::DateTime>, // dateTime [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_instant: Option<types::Instant>, // instant [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_time: Option<types::Time>, // time [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_decimal: Option<types::Decimal>, // decimal [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_integer: Option<types::Integer>, // integer [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_integer_64: Option<types::Integer64>, // integer64 [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_positive_int: Option<types::PositiveInt>, // positiveInt [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_unsigned_int: Option<types::UnsignedInt>, // unsignedInt [0..1]

    /// Maximum Allowed Value (for some types)
    pub maxValue_quantity: Option<types::Quantity>, // Quantity [0..1]

    /// Max length for string type data
    pub max_length: Option<types::Integer>, // integer [0..1]

    /// Reference to invariant about presence
    pub condition: Option<Vec<types::Id>>, // id [0..*]

    /// Condition that must evaluate to true
    pub constraint: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Target of 'condition' reference above
    pub key: types::Id, // id [1..1]

    /// Why this constraint is necessary or appropriate
    pub requirements: Option<types::Markdown>, // markdown [0..1]

    /// error | warning
    pub severity: types::Code, // code [1..1]

    /// Suppress warning or hint in profile
    pub suppress: Option<types::Boolean>, // boolean [0..1]

    /// Human description of constraint
    pub human: types::String, // string [1..1]

    /// FHIRPath expression of constraint
    pub expression: Option<types::String>, // string [0..1]

    /// Reference to original source of constraint
    pub source: Option<types::Canonical>, // canonical [0..1]

    /// For primitives, that a value must be present - not replaced by an extension
    pub must_have_value: Option<types::Boolean>, // boolean [0..1]

    /// Extensions that are allowed to replace a primitive value
    pub value_alternatives: Option<Vec<types::Canonical>>, // canonical [0..*]

    /// If the element must be supported (discouraged - see obligations)
    pub must_support: Option<types::Boolean>, // boolean [0..1]

    /// If this modifies the meaning of other elements
    pub is_modifier: Option<types::Boolean>, // boolean [0..1]

    /// Reason that this element is marked as a modifier
    pub is_modifier_reason: Option<types::String>, // string [0..1]

    /// Include when _summary = true?
    pub is_summary: Option<types::Boolean>, // boolean [0..1]

    /// ValueSet details if this is coded
    pub binding: Option<types::Element>, // Element [0..1]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// required | extensible | preferred | example
    pub strength: types::Code, // code [1..1]

    /// Intended use of codes in the bound value set
    pub description: Option<types::Markdown>, // markdown [0..1]

    /// Source of value set
    pub value_set: Option<types::Canonical>, // canonical [0..1]

    /// Additional Bindings - more rules about the binding
    pub additional: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// maximum | minimum | required | extensible | candidate | current | preferred | ui | starter | component
    pub purpose: types::Code, // code [1..1]

    /// The value set for the additional binding
    pub value_set: types::Canonical, // canonical [1..1]

    /// Documentation of the purpose of use of the binding
    pub documentation: Option<types::Markdown>, // markdown [0..1]

    /// Concise documentation - for summary tables
    pub short_doco: Option<types::String>, // string [0..1]

    /// Qualifies the usage - jurisdiction, gender, workflow status etc.
    pub usage: Option<Vec<types::UsageContext>>, // UsageContext [0..*]

    /// Whether binding can applies to all repeats, or just one
    pub any: Option<types::Boolean>, // boolean [0..1]

    /// Map element to another set of definitions
    pub mapping: Option<Vec<types::Element>>, // Element [0..*]

    /// Unique id for inter-element referencing
    pub id: Option<types::String>, // http://hl7.org/fhirpath/System.String [0..1]

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>, // Extension [0..*]

    /// Reference to mapping declaration
    pub identity: types::Id, // id [1..1]

    /// Computable language of mapping
    pub language: Option<types::Code>, // code [0..1]

    /// Details of the mapping
    pub map: types::String, // string [1..1]

    /// Comments about the mapping or its use
    pub comment: Option<types::Markdown>, // markdown [0..1]

        }

        #[cfg(test)]
        mod tests {
            use super::*;
            type T = ElementDefinition;

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
        