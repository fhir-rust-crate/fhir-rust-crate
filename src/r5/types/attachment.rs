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

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub content_type: Option<types::Code>, // « MimeTypes! » « C »
    pub language: Option<types::Code>,     // « AllLanguages! »
    pub data: Option<types::Base64Binary>, // [0..1] « C »
    pub url: Option<types::Url>,
    pub size: Option<types::Integer64>,
    pub hash: Option<types::Base64Binary>,
    pub title: Option<types::String>,
    pub creation: Option<types::DateTime>,
    pub height: Option<types::PositiveInt>,
    pub width: Option<types::PositiveInt>,
    pub frames: Option<types::PositiveInt>,
    pub duration: Option<types::Decimal>,
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
