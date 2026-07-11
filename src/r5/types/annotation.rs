//! Annotation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Annotation
//!
//! Version: 5.0.0
//!
//! Annotation Type: A  text note which also  contains information about who made the statement and when.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A text note that also records information about who made the statement
/// and when it was made. Annotations are used throughout FHIR resources to
/// capture free-text comments, clinical notes, or remarks alongside optional
/// attribution to an author and a timestamp. The author may be identified
/// either by a reference to a resource or by a plain text name.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::annotation::Annotation;
///
/// let value = Annotation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Annotation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// The individual responsible for the annotation, as a reference to a
    /// resource. This is the `Reference` variant of the `author[x]` choice
    /// element, serialized as `authorReference`. Cardinality 0..1.
    ///
    /// « Reference( Practitioner | PractitionerRole | Patient | RelatedPerson | Organization ) »
    ///
    pub author_reference: Option<types::Reference>,

    /// The individual responsible for the annotation, given as free text
    /// rather than a reference. This is the `string` variant of the
    /// `author[x]` choice element, serialized as `authorString`. Cardinality
    /// 0..1.
    ///
    /// See [`Self::author_reference`] for the reference variant.
    ///
    pub author_string: Option<types::String>,

    /// The date and time this annotation was made. Cardinality 0..1, data
    /// type `dateTime`.
    ///
    pub time: Option<types::DateTime>,

    /// The text content of the annotation, which may include markdown
    /// formatting. This is the required content of the note itself.
    ///
    pub text: types::Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Annotation;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            author_reference: None,
            author_string: None,
            time: None,
            text: types::Markdown::default(),
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;

        #[test]
        fn test_serde_json_round_trip() {
            let value = T::default();
            let json = ::serde_json::to_value(&value).expect("to_value");
            let back: T = ::serde_json::from_value(json).expect("from_value");
            assert_eq!(value, back);
        }
    }
}
