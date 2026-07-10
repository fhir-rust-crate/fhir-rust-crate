//! Binary
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Binary
//!
//! Version: 5.0.0
//!
//! Binary Resource: A resource that represents the data of a single raw artifact as digital content accessible in its native format.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A resource that represents the data of a single raw artifact as digital
/// content accessible in its native format.
///
/// A Binary resource can contain any content, whether text, image, pdf, zip
/// archive, etc. Unlike most FHIR resources it is not modeled as structured
/// data but instead carries an opaque payload identified by a MIME content
/// type. In FHIR R5 it is typically used to serve document attachments,
/// images, or other blobs that other resources reference.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::binary::Binary;
///
/// let value = Binary::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Binary = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Binary {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content
    pub language: Option<types::Code>,

    /// MimeType of the binary content
    pub content_type: types::Code,

    /// Identifies another resource to use as proxy when enforcing access control
    pub security_context: Option<types::Reference>,

    /// The actual content
    pub data: Option<types::Base64Binary>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Binary;

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
