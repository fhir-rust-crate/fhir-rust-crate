//! Contributor
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Contributor
//!
//! Version: 5.0.0
//!
//! Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A contributor to the content of a knowledge asset, including authors,
/// editors, reviewers, and endorsers.
///
/// The `Contributor` datatype captures attribution for a knowledge asset by
/// naming an individual or organization and describing the kind of contribution
/// they made (author, editor, reviewer, or endorser). It also carries contact
/// details so the contributor can be reached. It is commonly used in metadata
/// resources such as knowledge artifacts, guidelines, and measures.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::contributor::Contributor;
///
/// let value = Contributor::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Contributor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// author | editor | reviewer | endorser
    pub r#type: types::Code,

    /// Who contributed the content
    pub name: types::String,

    /// Contact details of the contributor
    pub contact: Option<Vec<types::ContactDetail>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Contributor;

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
