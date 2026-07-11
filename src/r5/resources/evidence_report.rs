//! EvidenceReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EvidenceReport
//!
//! Version: 5.0.0
//!
//! EvidenceReport Resource: The EvidenceReport Resource is a specialized container for a collection of resources and codeable concepts, adapted to support compositions of Evidence, EvidenceVariable, and Citation resources and related concepts.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// EvidenceReport
///
/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts. It functions as
/// a structured document with sections, allowing the assembly of research findings
/// and related artifacts into a reviewable, shareable report. Typical uses include
/// systematic reviews, evidence syntheses, and structured summaries of clinical
/// research.
///
/// Like the general-purpose `Composition` resource, an EvidenceReport is organized
/// as a hierarchy of sections (see [`EvidenceReportSection`]) that may carry
/// narrative text, coded entries, quantities, or references to other resources.
/// Its `subject` element characterizes the population, exposure, or other focus
/// that the report addresses, and its `relatesTo` element expresses relationships
/// (such as replaces, amends, or appends) to other reports or compositions,
/// supporting versioning and provenance of published evidence syntheses.
///
/// # Related resources
///
/// - [`Citation`](crate::r5::resources::citation::Citation) — bibliographic
///   citation for the artifacts an EvidenceReport summarizes or references.
/// - [`Evidence`](crate::r5::resources::evidence::Evidence) — a single
///   evidence concept that may be included or discussed within a report.
/// - [`EvidenceVariable`](crate::r5::resources::evidence_variable::EvidenceVariable) —
///   population, exposure, or outcome definitions referenced by the report's
///   subject characteristics.
/// - [`Composition`](crate::r5::resources::composition::Composition) — the
///   general-purpose document resource that EvidenceReport specializes.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_report::EvidenceReport;
///
/// let value = EvidenceReport::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EvidenceReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReport {
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

    /// Canonical identifier for this EvidenceReport, represented as a globally unique URI
    /// that allows the report to be referenced from other resources.
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// The publication lifecycle status of this report: draft | active | retired | unknown.
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Unique identifier for the evidence report
    pub identifier: Option<Vec<types::Identifier>>,

    /// Identifiers for articles that may relate to more than one evidence report
    pub related_identifier: Option<Vec<types::Identifier>>,

    /// Citation for this report
    pub cite_as_reference: Option<types::Reference>,

    /// Citation for this report
    pub cite_as_markdown: Option<types::Markdown>,

    /// Kind of report
    pub r#type: Option<types::CodeableConcept>,

    /// Used for footnotes and annotations
    pub note: Option<Vec<types::Annotation>>,

    /// Link, description or reference to artifact associated with the report
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Focus of the report, describing the population, exposure, or other
    /// characteristics the evidence addresses.
    pub subject: EvidenceReportSubject,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Who authored the content
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Relationships to other compositions/documents, such as prior versions
    /// this report replaces or amends.
    pub relates_to: Option<Vec<EvidenceReportRelatesTo>>,

    /// The hierarchical sections that make up the body of the report.
    pub section: Option<Vec<EvidenceReportSection>>,
}

/// EvidenceReportSubject
///
/// Specifies the subject or focus of the report, describing the population,
/// exposures, or other characteristics that the evidence report addresses.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSubject {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Characteristic
    pub characteristic: Option<Vec<EvidenceReportSubjectCharacteristic>>,

    /// Footnotes and/or explanatory notes
    pub note: Option<Vec<types::Annotation>>,
}

/// EvidenceReportSubjectCharacteristic
///
/// Describes a single characteristic that defines the subject of the report, such
/// as a coded value with an optional exclusion flag and timeframe.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSubjectCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Characteristic code
    pub code: types::CodeableConcept,

    /// Characteristic value
    pub value_reference: Option<types::Reference>,

    /// Characteristic value
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Characteristic value
    pub value_boolean: Option<types::Boolean>,

    /// Characteristic value
    pub value_quantity: Option<types::Quantity>,

    /// Characteristic value
    pub value_range: Option<types::Range>,

    /// Is used to express not the characteristic
    pub exclude: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`).
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// Timeframe for the characteristic
    pub period: Option<types::Period>,
}

/// EvidenceReportRelatesTo
///
/// Expresses a relationship between this evidence report and another composition
/// or document, such as a replacement, amendment, or transformation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// replaces | amends | appends | transforms | replacedWith | amendedWith | appendedWith | transformedWith
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Target of the relationship
    pub target: EvidenceReportRelatesToTarget,
}

/// EvidenceReportRelatesToTarget
///
/// Identifies the target of a relationship expressed by the report, referenced by
/// URL, identifier, display text, or a resource reference.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportRelatesToTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Target of the relationship URL
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Target of the relationship Identifier
    pub identifier: Option<types::Identifier>,

    /// Target of the relationship Display
    pub display: Option<types::Markdown>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Target of the relationship Resource reference
    pub resource: Option<types::Reference>,
}

/// EvidenceReportSection
///
/// Represents a section of the composition, which may contain narrative text,
/// entries referencing other resources, and nested subsections, enabling a
/// hierarchical document structure.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSection {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for section (e.g. for ToC)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Classification of section (recommended)
    pub focus: Option<types::CodeableConcept>,

    /// Classification of section by Resource
    pub focus_reference: Option<types::Reference>,

    /// Who and/or what authored the section
    pub author: Option<Vec<types::Reference>>,

    /// Text summary of the section, for human interpretation
    pub text: Option<types::Narrative>,

    /// working | snapshot | changes
    pub mode: Option<types::Code>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Order of section entries
    pub ordered_by: Option<types::CodeableConcept>,

    /// Extensible classifiers as content
    pub entry_classifier: Option<Vec<types::CodeableConcept>>,

    /// Reference to resources as content
    pub entry_reference: Option<Vec<types::Reference>>,

    /// Quantity as content
    pub entry_quantity: Option<Vec<types::Quantity>>,

    /// Why the section is empty
    pub empty_reason: Option<types::CodeableConcept>,

    /// Nested Section
    pub section: Option<Vec<EvidenceReportSection>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EvidenceReport;

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
