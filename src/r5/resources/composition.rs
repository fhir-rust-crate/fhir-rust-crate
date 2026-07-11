//! Composition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Composition
//!
//! Version: 5.0.0
//!
//! Composition Resource: A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A set of healthcare-related information that is assembled together into a
/// single logical package.
///
/// A Composition establishes a single coherent statement of meaning, defines
/// its own context, and carries clinical attestation regarding who is making
/// the statement. It provides the structure and narrative content necessary for
/// a clinical document, organized into sections. In FHIR R5 a Composition alone
/// does not constitute a document; it is typically the first entry in a Bundle
/// of type "document" that packages the referenced resources together.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::composition::Composition;
///
/// let value = Composition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Composition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Composition {
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

    /// Canonical identifier for this Composition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Version-independent identifier for the Composition
    pub identifier: Option<Vec<types::Identifier>>,

    /// An explicitly assigned identifer of a variation of the content in the Composition
    pub version: Option<types::String>,

    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    pub status: types::Code,

    /// Kind of composition (LOINC if possible)
    pub r#type: types::CodeableConcept,

    /// Categorization of Composition
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Who and/or what the composition is about
    pub subject: Option<Vec<types::Reference>>,

    /// Context of the Composition
    pub encounter: Option<types::Reference>,

    /// Composition editing time
    pub date: types::DateTime,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Who and/or what authored the composition
    pub author: Vec<types::Reference>,

    /// Name for this Composition (computer friendly)
    pub name: Option<types::String>,

    /// Human Readable name/title
    pub title: types::String,

    /// For any additional notes
    pub note: Option<Vec<types::Annotation>>,

    /// Attests to accuracy of composition
    pub attester: Option<Vec<CompositionAttester>>,

    /// Organization which maintains the composition
    pub custodian: Option<types::Reference>,

    /// Relationships to other compositions/documents
    pub relates_to: Option<Vec<types::RelatedArtifact>>,

    /// The clinical service(s) being documented
    pub event: Option<Vec<CompositionEvent>>,

    /// Composition is broken into sections
    pub section: Option<Vec<CompositionSection>>,
}

/// Attests to accuracy of composition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompositionAttester {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// personal | professional | legal | official
    pub mode: types::CodeableConcept,

    /// When the composition was attested
    pub time: Option<types::DateTime>,

    /// Who attested the composition
    pub party: Option<types::Reference>,
}

/// The clinical service(s) being documented.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompositionEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The period covered by the documentation
    pub period: Option<types::Period>,

    /// The event(s) being documented, as code(s), reference(s), or both
    pub detail: Option<Vec<types::CodeableReference>>,
}

/// Composition is broken into sections.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompositionSection {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for section (e.g. for ToC)
    pub title: Option<types::String>,

    /// Classification of section (recommended)
    pub code: Option<types::CodeableConcept>,

    /// Who and/or what authored the section
    pub author: Option<Vec<types::Reference>>,

    /// Who/what the section is about, when it is not about the subject of composition
    pub focus: Option<types::Reference>,

    /// Text summary of the section, for human interpretation
    pub text: Option<types::Narrative>,

    /// Order of section entries
    pub ordered_by: Option<types::CodeableConcept>,

    /// A reference to data that supports this section
    pub entry: Option<Vec<types::Reference>>,

    /// Why the section is empty
    pub empty_reason: Option<types::CodeableConcept>,

    /// Nested Section
    pub section: Option<Vec<CompositionSection>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Composition;

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
