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
use fhir_derive_macros::Validate;

/// The Citation Resource enables reference to any knowledge artifact for
/// purposes of identification and attribution. It supports existing reference
/// structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable
/// resources. In FHIR R5 it captures both metadata about the citation record
/// itself and rich descriptive detail about the cited artifact being referenced.
///
/// Citation is a canonical, conformance-like resource that follows the usual
/// publication lifecycle metadata pattern shared with other knowledge artifact
/// resources: it carries a `url`, `version`, `status`, `date`, `publisher`, and
/// `use_context`, in addition to citation-specific detail. The resource is used
/// to describe bibliographic references (such as journal articles, guidelines,
/// or other source material) that support clinical decision support artifacts,
/// evidence summaries, quality measures, and other knowledge artifacts, allowing
/// implementers to trace claims and recommendations back to their supporting
/// literature. The `cited_artifact` element carries the substantive
/// bibliographic detail (titles, abstracts, publication form, and
/// contributorship), while the top-level elements describe the citation record
/// itself as a shareable, versionable FHIR artifact.
///
/// # Related resources
///
/// A `Citation` may reference other resources via [`Reference`](crate::r5::types::Reference)
/// elements, for example to link a `related_artifact` or `base_citation` back
/// to another `Citation`, an `Evidence` resource, or a supporting `Organization`.
/// Descriptive coded elements throughout this resource, such as `classification`
/// and `jurisdiction`, use [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::citation::Citation;
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
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content.
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation.
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources.
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored.
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Canonical identifier for this citation record, represented as a global URL,
    /// used to reference this citation record across systems and versions.
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier(s) for the citation record itself, distinct from any
    /// identifier of the cited artifact (such as a DOI or PMID).
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the citation record.
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `Citation.versionAlgorithm[x]` choice element (0..1); see [`CitationVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<CitationVersionAlgorithm>,

    /// Name for this citation record (computer friendly).
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this citation record (human friendly).
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication status of the citation record itself: draft | active |
    /// retired | unknown. This is a required element.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage.
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed.
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The publisher of the citation record, not the publisher of the article.
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher of the citation record.
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the citation.
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the citation record content is intended to support.
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for citation record (if applicable).
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this citation is defined.
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions for the citation record.
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s) for the citation record.
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the citation record was approved by publisher.
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the citation record was last reviewed by the publisher.
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

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

    /// The article or artifact being described, carrying the detailed
    /// bibliographic content (titles, abstracts, publication form, and
    /// contributorship) about the source referenced by this citation record.
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
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`).
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`date_accessed`](Self::date_accessed) (FHIR `_dateAccessed`).
    #[serde(rename = "_dateAccessed")]
    pub date_accessed_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`).
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Copyright notice for the abstract.
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Additional classifiers.
    pub classifier: Option<Vec<types::CodeableConcept>>,

    /// Short label.
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`).
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Brief description of the related artifact.
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Bibliographic citation for the artifact.
    pub citation: Option<types::Markdown>,
    /// Primitive extension sibling for [`citation`](Self::citation) (FHIR `_citation`).
    #[serde(rename = "_citation")]
    pub citation_ext: Option<types::Element>,

    /// What document is being referenced.
    pub document: Option<types::Attachment>,

    /// What artifact is being referenced.
    pub resource: Option<types::Canonical>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`volume`](Self::volume) (FHIR `_volume`).
    #[serde(rename = "_volume")]
    pub volume_ext: Option<types::Element>,

    /// Issue, part or supplement of journal or other collection.
    pub issue: Option<types::String>,
    /// Primitive extension sibling for [`issue`](Self::issue) (FHIR `_issue`).
    #[serde(rename = "_issue")]
    pub issue_ext: Option<types::Element>,

    /// The date the article was added to the database, or the date published.
    pub article_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`article_date`](Self::article_date) (FHIR `_articleDate`).
    #[serde(rename = "_articleDate")]
    pub article_date_ext: Option<types::Element>,

    /// Text representation of the date of publication.
    pub publication_date_text: Option<types::String>,
    /// Primitive extension sibling for [`publication_date_text`](Self::publication_date_text) (FHIR `_publicationDateText`).
    #[serde(rename = "_publicationDateText")]
    pub publication_date_text_ext: Option<types::Element>,

    /// Season in which the cited artifact was published.
    pub publication_date_season: Option<types::String>,
    /// Primitive extension sibling for [`publication_date_season`](Self::publication_date_season) (FHIR `_publicationDateSeason`).
    #[serde(rename = "_publicationDateSeason")]
    pub publication_date_season_ext: Option<types::Element>,

    /// The date the article was last revised or updated in the database.
    pub last_revision_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_revision_date`](Self::last_revision_date) (FHIR `_lastRevisionDate`).
    #[serde(rename = "_lastRevisionDate")]
    pub last_revision_date_ext: Option<types::Element>,

    /// Language(s) in which this form of the article is published.
    pub language: Option<Vec<types::CodeableConcept>>,

    /// Entry number or identifier for inclusion in a database.
    pub accession_number: Option<types::String>,
    /// Primitive extension sibling for [`accession_number`](Self::accession_number) (FHIR `_accessionNumber`).
    #[serde(rename = "_accessionNumber")]
    pub accession_number_ext: Option<types::Element>,

    /// Used for full display of pagination.
    pub page_string: Option<types::String>,
    /// Primitive extension sibling for [`page_string`](Self::page_string) (FHIR `_pageString`).
    #[serde(rename = "_pageString")]
    pub page_string_ext: Option<types::Element>,

    /// Used for isolated representation of first page.
    pub first_page: Option<types::String>,
    /// Primitive extension sibling for [`first_page`](Self::first_page) (FHIR `_firstPage`).
    #[serde(rename = "_firstPage")]
    pub first_page_ext: Option<types::Element>,

    /// Used for isolated representation of last page.
    pub last_page: Option<types::String>,
    /// Primitive extension sibling for [`last_page`](Self::last_page) (FHIR `_lastPage`).
    #[serde(rename = "_lastPage")]
    pub last_page_ext: Option<types::Element>,

    /// Number of pages or screens.
    pub page_count: Option<types::String>,
    /// Primitive extension sibling for [`page_count`](Self::page_count) (FHIR `_pageCount`).
    #[serde(rename = "_pageCount")]
    pub page_count_ext: Option<types::Element>,

    /// Copyright notice for the full article or artifact.
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Name of or resource describing the publisher.
    pub publisher: Option<types::Reference>,

    /// Geographic location of the publisher.
    pub publisher_location: Option<types::String>,
    /// Primitive extension sibling for [`publisher_location`](Self::publisher_location) (FHIR `_publisherLocation`).
    #[serde(rename = "_publisherLocation")]
    pub publisher_location_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`complete`](Self::complete) (FHIR `_complete`).
    #[serde(rename = "_complete")]
    pub complete_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`forename_initials`](Self::forename_initials) (FHIR `_forenameInitials`).
    #[serde(rename = "_forenameInitials")]
    pub forename_initials_ext: Option<types::Element>,

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
    /// Primitive extension sibling for [`corresponding_contact`](Self::corresponding_contact) (FHIR `_correspondingContact`).
    #[serde(rename = "_correspondingContact")]
    pub corresponding_contact_ext: Option<types::Element>,

    /// Ranked order of contribution.
    pub ranking_order: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`ranking_order`](Self::ranking_order) (FHIR `_rankingOrder`).
    #[serde(rename = "_rankingOrder")]
    pub ranking_order_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`).
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,
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
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
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
/// The `Citation.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CitationVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
