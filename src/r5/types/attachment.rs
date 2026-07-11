//! Attachment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Attachment
//!
//! Version: 5.0.0
//!
//! Attachment Type: For referring to data content defined in other formats.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This datatype represents binary or textual content, such as an image, PDF,
//! or document, either embedded inline as base64 data or referenced by URL.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Content in a defined format and encoding, either as inline base64 data or
/// as a reference to a URL where the content can be retrieved. It is used to
/// convey files such as images, PDFs, or other documents attached to a
/// resource, along with metadata like MIME type, size, and hash for
/// integrity checking.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::attachment::Attachment;
///
/// let value = Attachment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Attachment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// MIME type of the content, with charset etc.
    pub content_type: Option<types::Code>, // « MimeTypes! » « C »
    /// Human language of the content.
    pub language: Option<types::Code>,     // « AllLanguages! »
    /// Data inline, base64 encoded.
    pub data: Option<types::Base64Binary>, // [0..1] « C »
    /// URL where the data can be found.
    pub url: Option<types::Url>,
    /// Number of bytes of content, if inline data was not provided.
    pub size: Option<types::Integer64>,
    /// Hash of the data as SHA-1, base64 encoded.
    pub hash: Option<types::Base64Binary>,
    /// Label to display in place of the data.
    pub title: Option<types::String>,
    /// Date the attachment was first created.
    pub creation: Option<types::DateTime>,
    /// Height of the image in pixels (photo/video).
    pub height: Option<types::PositiveInt>,
    /// Width of the image in pixels (photo/video).
    pub width: Option<types::PositiveInt>,
    /// Number of frames if this is a graphic that includes multiple frames.
    pub frames: Option<types::PositiveInt>,
    /// Length in seconds, if content is a recording.
    pub duration: Option<types::Decimal>,
    /// Number of printed pages.
    pub pages: Option<types::PositiveInt>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Attachment;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            content_type: None,
            language: None,
            data: None,
            url: None,
            size: None,
            hash: None,
            title: None,
            creation: None,
            height: None,
            width: None,
            frames: None,
            duration: None,
            pages: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
