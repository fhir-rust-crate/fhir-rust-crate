//! Citation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Citation
//!
//! Version: 5.0.0
//!
//! Citation Resource: Enables reference to any knowledge artifact for purposes of identification and attribution.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The Citation Resource enables reference to any knowledge artifact for
/// purposes of identification and attribution. It supports existing reference
/// structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable
/// resources. In FHIR R5 it captures both metadata about the citation record
/// itself and rich descriptive detail about the cited artifact being referenced.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::citation::Citation;
///
/// let value = Citation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Citation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    /// Logical id of this artifact.
    pub id: Option<types::String>,

    /// Metadata about the resource.
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created.
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content.
    pub language: Option<types::Code>,

    /// Text summary of the resource, for human interpretation.
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources.
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Canonical identifier for this citation record, represented as a global URL.
    pub url: Option<types::Uri>,

    /// Identifier for the citation record itself.
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the citation record.
    pub version: Option<types::String>,

    /// How to compare versions (string form).
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions (Coding form).
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this citation record (computer friendly).
    pub name: Option<types::String>,

    /// Name for this citation record (human friendly).
    pub title: Option<types::String>,

    /// draft | active | retired | unknown.
    pub status: types::Code,

    /// For testing purposes, not real usage.
    pub experimental: Option<types::Boolean>,

    /// Date last changed.
    pub date: Option<types::DateTime>,

    /// The publisher of the citation record, not the publisher of the article.
    pub publisher: Option<types::String>,

    /// Contact details for the publisher of the citation record.
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the citation.
    pub description: Option<types::Markdown>,

    /// The context that the citation record content is intended to support.
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for citation record (if applicable).
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this citation is defined.
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions for the citation record.
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s) for the citation record.
    pub copyright_label: Option<types::String>,

    /// When the citation record was approved by publisher.
    pub approval_date: Option<types::Date>,

    /// When the citation record was last reviewed by the publisher.
    pub last_review_date: Option<types::Date>,

    /// When the citation record is expected to be used.
    pub effective_period: Option<types::Period>,

    /// Who authored the citation record.
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the citation record.
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the citation record.
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the citation record.
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// A human-readable display of key concepts to represent the citation.
    pub summary: Option<Vec<CitationSummary>>,

    /// The assignment to an organizing scheme.
    pub classification: Option<Vec<CitationClassification>>,

    /// Used for general notes and annotations not coded elsewhere.
    pub note: Option<Vec<types::Annotation>>,

    /// The status of the citation record.
    pub current_state: Option<Vec<types::CodeableConcept>>,

    /// An effective date or period for a status of the citation record.
    pub status_date: Option<Vec<CitationStatusDate>>,

    /// Artifact related to the citation record.
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// The article or artifact being described.
    pub cited_artifact: Option<CitationCitedArtifact>,
}

/// A human-readable display of key concepts to represent the citation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationSummary {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Format for display of the citation summary.
    pub style: Option<types::CodeableConcept>,

    /// The human-readable display of the citation summary.
    pub text: types::Markdown,
}

/// The assignment to an organizing scheme.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationClassification {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kind of classifier (e.g. publication type, keyword).
    pub r#type: Option<types::CodeableConcept>,

    /// The specific classification value.
    pub classifier: Option<Vec<types::CodeableConcept>>,
}

/// An effective date or period for a status of the citation record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationStatusDate {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Classification of the status.
    pub activity: types::CodeableConcept,

    /// Either occurred or expected.
    pub actual: Option<types::Boolean>,

    /// When the status started and/or ended.
    pub period: types::Period,
}

/// The article or artifact being described.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifact {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Unique identifier. May include DOI, PMID, PMCID, etc.
    pub identifier: Option<Vec<types::Identifier>>,

    /// Identifier not unique to the cited artifact.
    pub related_identifier: Option<Vec<types::Identifier>>,

    /// When the cited artifact was accessed.
    pub date_accessed: Option<types::DateTime>,

    /// The defined version of the cited artifact.
    pub version: Option<CitationCitedArtifactVersion>,

    /// The status of the cited artifact.
    pub current_state: Option<Vec<types::CodeableConcept>>,

    /// An effective date or period for a status of the cited artifact.
    pub status_date: Option<Vec<CitationCitedArtifactStatusDate>>,

    /// The title details of the article or artifact.
    pub title: Option<Vec<CitationCitedArtifactTitle>>,

    /// Summary of the article or artifact.
    pub r#abstract: Option<Vec<CitationCitedArtifactAbstract>>,

    /// The component of the article or artifact.
    pub part: Option<CitationCitedArtifactPart>,

    /// The artifact related to the cited artifact.
    pub relates_to: Option<Vec<CitationCitedArtifactRelatesTo>>,

    /// Alternative forms of the article that are published.
    pub publication_form: Option<Vec<CitationCitedArtifactPublicationForm>>,

    /// Used for any URL for the article or artifact cited.
    pub web_location: Option<Vec<CitationCitedArtifactWebLocation>>,

    /// The assignment to an organizing scheme.
    pub classification: Option<Vec<CitationCitedArtifactClassification>>,

    /// Attribution of authors and other contributors.
    pub contributorship: Option<CitationCitedArtifactContributorship>,

    /// Any additional information or content for the article or artifact.
    pub note: Option<Vec<types::Annotation>>,
}

/// The defined version of the cited artifact.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactVersion {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The version number or other version identifier.
    pub value: types::String,

    /// Citation for the main version of the cited artifact.
    pub base_citation: Option<types::Reference>,
}

/// An effective date or period for a status of the cited artifact.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactStatusDate {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Classification of the status.
    pub activity: types::CodeableConcept,

    /// Either occurred or expected.
    pub actual: Option<types::Boolean>,

    /// When the status started and/or ended.
    pub period: types::Period,
}

/// The title details of the article or artifact.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactTitle {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kind of title.
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Used to express the specific language.
    pub language: Option<types::CodeableConcept>,

    /// The title of the article or artifact.
    pub text: types::Markdown,
}

/// Summary of the article or artifact.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactAbstract {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kind of abstract.
    pub r#type: Option<types::CodeableConcept>,

    /// Used to express the specific language.
    pub language: Option<types::CodeableConcept>,

    /// Abstract content.
    pub text: types::Markdown,

    /// Copyright notice for the abstract.
    pub copyright: Option<types::Markdown>,
}

/// The component of the article or artifact.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPart {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kind of component.
    pub r#type: Option<types::CodeableConcept>,

    /// The specification of the component.
    pub value: Option<types::String>,

    /// The citation for the full article or artifact.
    pub base_citation: Option<types::Reference>,
}

/// The artifact related to the cited artifact.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactRelatesTo {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// documentation | justification | citation | predecessor | successor | etc.
    pub r#type: types::Code,

    /// Additional classifiers.
    pub classifier: Option<Vec<types::CodeableConcept>>,

    /// Short label.
    pub label: Option<types::String>,

    /// Brief description of the related artifact.
    pub display: Option<types::String>,

    /// Bibliographic citation for the artifact.
    pub citation: Option<types::Markdown>,

    /// What document is being referenced.
    pub document: Option<types::Attachment>,

    /// What artifact is being referenced.
    pub resource: Option<types::Canonical>,

    /// What artifact, if not a conformance resource.
    pub resource_reference: Option<types::Reference>,
}

/// Alternative forms of the article that are published.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPublicationForm {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The collection the cited article or artifact is published in.
    pub published_in: Option<CitationCitedArtifactPublicationFormPublishedIn>,

    /// Internet or Print.
    pub cited_medium: Option<types::CodeableConcept>,

    /// Volume number of journal or other collection.
    pub volume: Option<types::String>,

    /// Issue, part or supplement of journal or other collection.
    pub issue: Option<types::String>,

    /// The date the article was added to the database, or the date published.
    pub article_date: Option<types::DateTime>,

    /// Text representation of the date of publication.
    pub publication_date_text: Option<types::String>,

    /// Season in which the cited artifact was published.
    pub publication_date_season: Option<types::String>,

    /// The date the article was last revised or updated in the database.
    pub last_revision_date: Option<types::DateTime>,

    /// Language(s) in which this form of the article is published.
    pub language: Option<Vec<types::CodeableConcept>>,

    /// Entry number or identifier for inclusion in a database.
    pub accession_number: Option<types::String>,

    /// Used for full display of pagination.
    pub page_string: Option<types::String>,

    /// Used for isolated representation of first page.
    pub first_page: Option<types::String>,

    /// Used for isolated representation of last page.
    pub last_page: Option<types::String>,

    /// Number of pages or screens.
    pub page_count: Option<types::String>,

    /// Copyright notice for the full article or artifact.
    pub copyright: Option<types::Markdown>,
}

/// The collection the cited article or artifact is published in.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPublicationFormPublishedIn {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Kind of container (e.g. Periodical, database, or book).
    pub r#type: Option<types::CodeableConcept>,

    /// Journal or book identifiers (ISSN, ISO Abbreviation, NLMuniqueID, etc.).
    pub identifier: Option<Vec<types::Identifier>>,

    /// Name of the database or title of the book or journal.
    pub title: Option<types::String>,

    /// Name of or resource describing the publisher.
    pub publisher: Option<types::Reference>,

    /// Geographic location of the publisher.
    pub publisher_location: Option<types::String>,
}

/// Used for any URL for the article or artifact cited.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactWebLocation {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code the reason for different URLs, e.g. abstract and full-text.
    pub classifier: Option<Vec<types::CodeableConcept>>,

    /// The specific URL.
    pub url: Option<types::Uri>,
}

/// The assignment to an organizing scheme for the cited artifact.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactClassification {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kind of classifier (e.g. publication type, keyword).
    pub r#type: Option<types::CodeableConcept>,

    /// The specific classification value.
    pub classifier: Option<Vec<types::CodeableConcept>>,

    /// Complex or externally created classification.
    pub artifact_assessment: Option<Vec<types::Reference>>,
}

/// Attribution of authors and other contributors.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorship {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Indicates if the list includes all authors and/or contributors.
    pub complete: Option<types::Boolean>,

    /// An individual entity named as a contributor.
    pub entry: Option<Vec<CitationCitedArtifactContributorshipEntry>>,

    /// Used to record a display of the author/contributor list.
    pub summary: Option<Vec<CitationCitedArtifactContributorshipSummary>>,
}

/// An individual entity named as a contributor.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorshipEntry {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The identity of the individual contributor.
    pub contributor: types::Reference,

    /// For citation styles that use initials.
    pub forename_initials: Option<types::String>,

    /// Organizational affiliation.
    pub affiliation: Option<Vec<types::Reference>>,

    /// The specific contribution.
    pub contribution_type: Option<Vec<types::CodeableConcept>>,

    /// The role of the contributor (e.g. author, editor, reviewer, funder).
    pub role: Option<types::CodeableConcept>,

    /// Contributions with accounting for time or number.
    pub contribution_instance: Option<Vec<CitationCitedArtifactContributorshipEntryContributionInstance>>,

    /// Whether the contributor is the corresponding contributor for the role.
    pub corresponding_contact: Option<types::Boolean>,

    /// Ranked order of contribution.
    pub ranking_order: Option<types::PositiveInt>,
}

/// Contributions with accounting for time or number.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorshipEntryContributionInstance {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The specific contribution.
    pub r#type: types::CodeableConcept,

    /// The time that the contribution was made.
    pub time: Option<types::DateTime>,
}

/// Used to record a display of the author/contributor list.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorshipSummary {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Such as author list, contributorship statement, funding statement, etc.
    pub r#type: Option<types::CodeableConcept>,

    /// The format for the display string.
    pub style: Option<types::CodeableConcept>,

    /// Used to code the producer or rule for creating the display string.
    pub source: Option<types::CodeableConcept>,

    /// The display string for the author list or contributor list.
    pub value: types::Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Citation;

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
