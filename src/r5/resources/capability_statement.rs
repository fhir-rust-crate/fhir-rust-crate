//! CapabilityStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
//!
//! Version: 5.0.0
//!
//! CapabilityStatement Resource: A set of capabilities (behaviors) of a FHIR Server or Client for a particular version of FHIR.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server or Client for a particular version of FHIR that may be used as a
/// statement of actual server functionality or a statement of required or
/// desired server implementation. It provides for a degree of automatic
/// negotiation of features and interoperability between FHIR systems.
///
/// Servers typically publish a `CapabilityStatement` at the `/metadata` endpoint so
/// that clients can discover which resource types, interactions, search parameters,
/// and operations are supported before attempting to exchange data. Implementation
/// guides also use `CapabilityStatement` to express conformance requirements that
/// implementations must satisfy, distinguishing between the `kind` values of
/// `instance` (an actual running system), `capability` (a reusable base
/// definition), and `requirements` (an abstract set of expectations). Because a
/// `CapabilityStatement` is itself a canonical resource, it carries the usual
/// metadata fields (`url`, `version`, `status`, `date`, `publisher`) shared with
/// other conformance resources such as `StructureDefinition` and `OperationDefinition`.
///
/// # Related resources
///
/// A `CapabilityStatement` describes a server or client's support for other
/// resource types, such as [`Patient`](crate::r5::resources::patient::Patient),
/// and it references coded terminology using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) and `Coding` for
/// values like security services and messaging protocols. See also
/// `StructureDefinition`, `OperationDefinition`, and `SearchParameter`, which
/// a `CapabilityStatement` may point to via canonical URLs.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatement;
///
/// let value = CapabilityStatement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CapabilityStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Canonical identifier for this capability statement, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the CapabilityStatement (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the capability statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `CapabilityStatement.versionAlgorithm[x]` choice element (0..1); see [`CapabilityStatementVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<CapabilityStatementVersionAlgorithm>,

    /// Name for this capability statement (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this capability statement (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication lifecycle status of this capability statement, one of draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the capability statement
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for capability statement (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this capability statement is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Whether this statement describes an actual running instance, a reusable base capability, or a set of requirements, one of instance | capability | requirements
    pub kind: crate::r5::coded::Coded<crate::r5::codes::CapabilityStatementKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Canonical URL of another capability statement this implements
    pub instantiates: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`).
    #[serde(rename = "_instantiates")]
    pub instantiates_ext: Option<Vec<Option<types::Element>>>,

    /// Canonical URL of another capability statement this adds to
    pub imports: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`imports`](Self::imports) (FHIR `_imports`).
    #[serde(rename = "_imports")]
    pub imports_ext: Option<Vec<Option<types::Element>>>,

    /// Software that is covered by this capability statement
    pub software: Option<CapabilityStatementSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<CapabilityStatementImplementation>,

    /// The FHIR specification version that this capability statement describes support for
    pub fhir_version: types::Code,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`).
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// formats supported (xml | json | ttl | mime type)
    pub format: Vec<types::Code>,
    /// Primitive extension sibling for [`format`](Self::format) (FHIR `_format`).
    #[serde(rename = "_format")]
    pub format_ext: Option<Vec<Option<types::Element>>>,

    /// Patch formats supported
    pub patch_format: Option<Vec<types::Code>>,
    /// Primitive extension sibling for [`patch_format`](Self::patch_format) (FHIR `_patchFormat`).
    #[serde(rename = "_patchFormat")]
    pub patch_format_ext: Option<Vec<Option<types::Element>>>,

    /// Languages supported
    pub accept_language: Option<Vec<types::Code>>,
    /// Primitive extension sibling for [`accept_language`](Self::accept_language) (FHIR `_acceptLanguage`).
    #[serde(rename = "_acceptLanguage")]
    pub accept_language_ext: Option<Vec<Option<types::Element>>>,

    /// Implementation guides supported
    pub implementation_guide: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`implementation_guide`](Self::implementation_guide) (FHIR `_implementationGuide`).
    #[serde(rename = "_implementationGuide")]
    pub implementation_guide_ext: Option<Vec<Option<types::Element>>>,

    /// One or more RESTful endpoint descriptions, each covering the resources, interactions, and search parameters supported
    pub rest: Option<Vec<CapabilityStatementRest>>,

    /// Descriptions of messaging-based interfaces this system supports, if any
    pub messaging: Option<Vec<CapabilityStatementMessaging>>,

    /// Document definition
    pub document: Option<Vec<CapabilityStatementDocument>>,
}

/// Software that is covered by this capability statement.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementSoftware {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A name the software is known by
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Version covered by this statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Date this version was released
    pub release_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`release_date`](Self::release_date) (FHIR `_releaseDate`).
    #[serde(rename = "_releaseDate")]
    pub release_date_ext: Option<types::Element>,
}

/// If this describes a specific instance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementImplementation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Describes this specific instance
    pub description: types::Markdown,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Base URL for the installation
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Organization that manages the data
    pub custodian: Option<types::Reference>,
}

/// If the endpoint is a RESTful one.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// client | server
    pub mode: crate::r5::coded::Coded<crate::r5::codes::RestfulCapabilityMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// General description of implementation
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Information about security of implementation
    pub security: Option<CapabilityStatementRestSecurity>,

    /// Resource served on the REST interface
    pub resource: Option<Vec<CapabilityStatementRestResource>>,

    /// What operations are supported?
    pub interaction: Option<Vec<CapabilityStatementRestInteraction>>,

    /// Search parameters for searching all resources
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>>,

    /// Definition of a system level operation
    pub operation: Option<Vec<CapabilityStatementRestResourceOperation>>,

    /// Compartments served/used by system
    pub compartment: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`compartment`](Self::compartment) (FHIR `_compartment`).
    #[serde(rename = "_compartment")]
    pub compartment_ext: Option<Vec<Option<types::Element>>>,
}

/// Information about security of implementation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestSecurity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Adds CORS Headers (http://enable-cors.org/)
    pub cors: Option<types::Boolean>,
    /// Primitive extension sibling for [`cors`](Self::cors) (FHIR `_cors`).
    #[serde(rename = "_cors")]
    pub cors_ext: Option<types::Element>,

    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    pub service: Option<Vec<types::CodeableConcept>>,

    /// General description of how security works
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Resource served on the REST interface.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A resource type that is supported
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// System-wide profile
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,

    /// Use-case specific profiles
    pub supported_profile: Option<Vec<types::Canonical>>,
    /// Primitive extension sibling for [`supported_profile`](Self::supported_profile) (FHIR `_supportedProfile`).
    #[serde(rename = "_supportedProfile")]
    pub supported_profile_ext: Option<Vec<Option<types::Element>>>,

    /// Additional information about the use of the resource type
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// What operations are supported?
    pub interaction: Option<Vec<CapabilityStatementRestResourceInteraction>>,

    /// no-version | versioned | versioned-update
    pub versioning: Option<crate::r5::coded::Coded<crate::r5::codes::VersioningPolicy>>,
    /// Primitive extension sibling for [`versioning`](Self::versioning) (FHIR `_versioning`).
    #[serde(rename = "_versioning")]
    pub versioning_ext: Option<types::Element>,

    /// Whether vRead can return past versions
    pub read_history: Option<types::Boolean>,
    /// Primitive extension sibling for [`read_history`](Self::read_history) (FHIR `_readHistory`).
    #[serde(rename = "_readHistory")]
    pub read_history_ext: Option<types::Element>,

    /// If update can commit to a new identity
    pub update_create: Option<types::Boolean>,
    /// Primitive extension sibling for [`update_create`](Self::update_create) (FHIR `_updateCreate`).
    #[serde(rename = "_updateCreate")]
    pub update_create_ext: Option<types::Element>,

    /// If allows/uses conditional create
    pub conditional_create: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_create`](Self::conditional_create) (FHIR `_conditionalCreate`).
    #[serde(rename = "_conditionalCreate")]
    pub conditional_create_ext: Option<types::Element>,

    /// not-supported | modified-since | not-match | full-support
    pub conditional_read: Option<crate::r5::coded::Coded<crate::r5::codes::ConditionalReadStatus>>,
    /// Primitive extension sibling for [`conditional_read`](Self::conditional_read) (FHIR `_conditionalRead`).
    #[serde(rename = "_conditionalRead")]
    pub conditional_read_ext: Option<types::Element>,

    /// If allows/uses conditional update
    pub conditional_update: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_update`](Self::conditional_update) (FHIR `_conditionalUpdate`).
    #[serde(rename = "_conditionalUpdate")]
    pub conditional_update_ext: Option<types::Element>,

    /// If allows/uses conditional patch
    pub conditional_patch: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_patch`](Self::conditional_patch) (FHIR `_conditionalPatch`).
    #[serde(rename = "_conditionalPatch")]
    pub conditional_patch_ext: Option<types::Element>,

    /// not-supported | single | multiple - how conditional delete is supported
    pub conditional_delete: Option<crate::r5::coded::Coded<crate::r5::codes::ConditionalDeleteStatus>>,
    /// Primitive extension sibling for [`conditional_delete`](Self::conditional_delete) (FHIR `_conditionalDelete`).
    #[serde(rename = "_conditionalDelete")]
    pub conditional_delete_ext: Option<types::Element>,

    /// literal | logical | resolves | enforced | local
    pub reference_policy: Option<Vec<crate::r5::coded::Coded<crate::r5::codes::ReferenceHandlingPolicy>>>,
    /// Primitive extension sibling for [`reference_policy`](Self::reference_policy) (FHIR `_referencePolicy`).
    #[serde(rename = "_referencePolicy")]
    pub reference_policy_ext: Option<Vec<Option<types::Element>>>,

    /// _include values supported by the server
    pub search_include: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`search_include`](Self::search_include) (FHIR `_searchInclude`).
    #[serde(rename = "_searchInclude")]
    pub search_include_ext: Option<Vec<Option<types::Element>>>,

    /// _revinclude values supported by the server
    pub search_rev_include: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`search_rev_include`](Self::search_rev_include) (FHIR `_searchRevInclude`).
    #[serde(rename = "_searchRevInclude")]
    pub search_rev_include_ext: Option<Vec<Option<types::Element>>>,

    /// Search parameters supported by implementation
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>>,

    /// Definition of a resource operation
    pub operation: Option<Vec<CapabilityStatementRestResourceOperation>>,
}

/// What operations are supported on a resource type?
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// read | vread | update | patch | delete | history-instance | history-type | create | search-type
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Anything special about operation behavior
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Search parameters supported by implementation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceSearchParam {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name for parameter in search url
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Source of definition for parameter
    pub definition: Option<types::Canonical>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::SearchParamType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Server-specific usage
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Definition of a resource operation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name by which the operation/query is invoked
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The defined operation/query
    pub definition: types::Canonical,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Specific details about operation behavior
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// What operations are supported at the system (all-resources) level?
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// transaction | batch | search-system | history-system
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Anything special about operation behavior
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// If messaging is supported.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Where messages should be sent
    pub endpoint: Option<Vec<CapabilityStatementMessagingEndpoint>>,

    /// Reliable Message Cache Length (min)
    pub reliable_cache: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`reliable_cache`](Self::reliable_cache) (FHIR `_reliableCache`).
    #[serde(rename = "_reliableCache")]
    pub reliable_cache_ext: Option<types::Element>,

    /// Messaging interface behavior details
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Messages supported by this system
    pub supported_message: Option<Vec<CapabilityStatementMessagingSupportedMessage>>,
}

/// Where messages should be sent.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingEndpoint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// http | ftp | mllp +
    pub protocol: types::Coding,

    /// Network address or identifier of the end-point
    pub address: types::Url,
    /// Primitive extension sibling for [`address`](Self::address) (FHIR `_address`).
    #[serde(rename = "_address")]
    pub address_ext: Option<types::Element>,
}

/// Messages supported by this system.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingSupportedMessage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// sender | receiver
    pub mode: crate::r5::coded::Coded<crate::r5::codes::EventCapabilityMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Message supported by this system
    pub definition: types::Canonical,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,
}

/// Document definition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementDocument {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// producer | consumer
    pub mode: crate::r5::coded::Coded<crate::r5::codes::DocumentMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Description of document support
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Constraint on the resources used in the document
    pub profile: types::Canonical,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CapabilityStatement;

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
/// The `CapabilityStatement.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CapabilityStatementVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
