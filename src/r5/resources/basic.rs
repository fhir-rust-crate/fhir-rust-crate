//! Basic
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Basic
//!
//! Version: 5.0.0
//!
//! Basic Resource: Basic is used for handling concepts not yet defined in FHIR, narrative-only resources that don't map to an existing resource, and custom resources not appropriate for inclusion in the FHIR specification.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Basic is used for handling concepts not yet defined in FHIR, narrative-only
/// resources that don't map to an existing resource, and custom resources not
/// appropriate for inclusion in the FHIR specification. It provides a minimal
/// framework — primarily a `code` describing the kind of thing represented,
/// plus optional subject, author, and creation date — so that implementers can
/// exchange information that FHIR does not otherwise model. The bulk of the
/// meaning is typically conveyed through extensions and the narrative text.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::basic::Basic;
///
/// let value = Basic::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Basic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Basic {
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

    /// Business identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Kind of Resource
    pub code: types::CodeableConcept,

    /// Identifies the focus of this resource
    pub subject: Option<types::Reference>,

    /// When created
    pub created: Option<types::DateTime>,

    /// Who created
    pub author: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Basic;

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
