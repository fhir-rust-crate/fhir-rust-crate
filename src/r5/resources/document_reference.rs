//! DocumentReference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
//!
//! Version: 5.0.0
//!
//! DocumentReference Resource: A reference to a document of any kind for any purpose.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A reference to a document of any kind for any purpose.
///
/// While the term "document" implies a narrow focus, for this resource a
/// "document" encompasses any serialized object with a mime-type. It includes
/// formal patient-centric documents (CDA), clinical notes, scanned paper, and
/// non-patient specific documents like policy text, as well as photos, videos,
/// or audio recordings acquired or used in healthcare. In FHIR R5 it provides
/// metadata about the document plus one or more content attachments describing
/// where and how to access the actual bytes.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::document_reference::DocumentReference;
///
/// let value = DocumentReference::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DocumentReference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReference {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content
    pub language: Option<types::Code>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Business identifiers for the document
    pub identifier: Option<Vec<types::Identifier>>,

    /// An explicitly assigned identifer of a variation of the content in the DocumentReference
    pub version: Option<types::String>,

    /// Procedure that caused this media to be created
    pub based_on: Option<Vec<types::Reference>>,

    /// current | superseded | entered-in-error
    pub status: types::Code,

    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    pub doc_status: Option<types::Code>,

    /// Imaging modality used
    pub modality: Option<Vec<types::CodeableConcept>>,

    /// Kind of document (LOINC if possible)
    pub r#type: Option<types::CodeableConcept>,

    /// Categorization of document
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Who/what is the subject of the document
    pub subject: Option<types::Reference>,

    /// Context of the document content
    pub context: Option<Vec<types::Reference>>,

    /// Main clinical acts documented
    pub event: Option<Vec<types::CodeableReference>>,

    /// Body part included
    pub body_site: Option<Vec<types::CodeableReference>>,

    /// Kind of facility where patient was seen
    pub facility_type: Option<types::CodeableConcept>,

    /// Additional details about where the content was created (e.g. clinical specialty)
    pub practice_setting: Option<types::CodeableConcept>,

    /// Time of service that is being documented
    pub period: Option<types::Period>,

    /// When this document reference was created
    pub date: Option<types::Instant>,

    /// Who and/or what authored the document
    pub author: Option<Vec<types::Reference>>,

    /// Attests to accuracy of the document
    pub attester: Option<Vec<DocumentReferenceAttester>>,

    /// Organization which maintains the document
    pub custodian: Option<types::Reference>,

    /// Relationships to other documents
    pub relates_to: Option<Vec<DocumentReferenceRelatesTo>>,

    /// Human-readable description
    pub description: Option<types::Markdown>,

    /// Document security-tags
    pub security_label: Option<Vec<types::CodeableConcept>>,

    /// Document referenced
    pub content: Vec<DocumentReferenceContent>,
}

/// Attests to accuracy of the document.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceAttester {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// personal | professional | legal | official
    pub mode: types::CodeableConcept,

    /// When the document was attested
    pub time: Option<types::DateTime>,

    /// Who attested the document
    pub party: Option<types::Reference>,
}

/// Relationships to other documents.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The relationship type with another document
    pub code: types::CodeableConcept,

    /// Target of the relationship
    pub target: types::Reference,
}

/// Document referenced.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceContent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Where to access the document
    pub attachment: types::Attachment,

    /// Content profile rules for the document
    pub profile: Option<Vec<DocumentReferenceContentProfile>>,
}

/// Content profile rules for the document.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceContentProfile {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Code|uri|canonical (as Coding)
    pub value_coding: Option<types::Coding>,

    /// Code|uri|canonical (as uri)
    pub value_uri: Option<types::Uri>,

    /// Code|uri|canonical (as canonical)
    pub value_canonical: Option<types::Canonical>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DocumentReference;

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
