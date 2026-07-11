//! NamingSystem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
//!
//! Version: 5.0.0
//!
//! NamingSystem Resource: A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, and other entities. In FHIR R5,
/// NamingSystem is a conformance resource that formally registers a "system":
/// the shared namespace value that appears as the `system` element of an
/// Identifier or as the `system` element of a Coding. It captures the metadata
/// about how a namespace is administered, who is responsible for it, and which
/// unique symbols (such as an OID, a UUID, or a URI) may be used to name that
/// same system when it is referenced during electronic exchange.
///
/// NamingSystem is typically authored and published by a steward organization so
/// that implementers can discover the canonical identifiers for code systems and
/// identifier schemes, reconcile equivalent representations of one namespace, and
/// resolve which form is preferred or authoritative. It does not carry the codes
/// or values themselves; rather, it documents and governs the namespaces from
/// which those codes and identifiers are drawn.
///
/// See also the [`Identifier`](crate::r5::types::Identifier) and
/// [`Coding`](crate::r5::types::Coding) data types, whose `system` values a
/// NamingSystem registers, and the [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// type that wraps such codings. Related conformance resources include
/// `CodeSystem`, `ValueSet`, and `TerminologyCapabilities`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::naming_system::NamingSystem;
///
/// let value = NamingSystem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: NamingSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystem {
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

    /// Canonical identifier for this naming system, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the naming system (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the naming system
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `NamingSystem.versionAlgorithm[x]` choice element (0..1); see [`NamingSystemVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<NamingSystemVersionAlgorithm>,

    /// Name for this naming system (computer friendly); used internally for cross-references
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Title for this naming system (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Publication status of this naming system definition: draft | active | retired | unknown
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Indicates the purpose of the namespace: codesystem | identifier | root
    pub kind: types::Code,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

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

    /// Who maintains system namespace?
    pub responsible: Option<types::String>,
    /// Primitive extension sibling for [`responsible`](Self::responsible) (FHIR `_responsible`).
    #[serde(rename = "_responsible")]
    pub responsible_ext: Option<types::Element>,

    /// Human-readable classification of the registered entities, e.g. driver, provider, patient, bank etc
    pub r#type: Option<types::CodeableConcept>,

    /// Natural language description of the naming system
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for naming system (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this naming system is defined
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

    /// When the NamingSystem was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the NamingSystem was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the NamingSystem is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the CodeSystem
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the NamingSystem
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the NamingSystem
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the NamingSystem
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// How/where is it used
    pub usage: Option<types::String>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`).
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

    /// The unique symbols (such as an OID, UUID, or URI) that may be used to name this system; see NamingSystemUniqueId
    pub unique_id: Vec<NamingSystemUniqueId>,
}

/// Indicates how the system may be identified when referenced in electronic
/// exchange. Each NamingSystem may have multiple unique identifiers (for example
/// an OID and a URI), each of which names the same underlying system.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystemUniqueId {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// oid | uuid | uri | iri-stem | v2csmnemonic | other
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The literal symbol that names the system, formatted according to the given type
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Is this the id that should be used for this type
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`).
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,

    /// Notes about identifier usage
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// When is identifier valid?
    pub period: Option<types::Period>,

    /// Whether the identifier is authoritative
    pub authoritative: Option<types::Boolean>,
    /// Primitive extension sibling for [`authoritative`](Self::authoritative) (FHIR `_authoritative`).
    #[serde(rename = "_authoritative")]
    pub authoritative_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = NamingSystem;

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
/// The `NamingSystem.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum NamingSystemVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
