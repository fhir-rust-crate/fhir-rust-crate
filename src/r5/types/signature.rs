//! Signature
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Signature
//!
//! Version: 5.0.0
//!
//! Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.
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
pub struct Signature {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::Coding>, // « SignatureTypeCodes? »

    pub when: Option<types::Instant>,

    pub who: Option<types::Reference>, // « Practitioner | PractitionerRole | RelatedPerson | Patient | Device | Organization »

    pub on_behalf_of: Option<types::Reference>, // « Practitioner | PractitionerRole | RelatedPerson | Patient | Device | Organization »

    pub target_format: Option<types::Code>, // « MimeTypes! »

    pub sig_format: Option<types::Code>, // « MimeTypes! »

    pub data: Option<types::Base64Binary>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Signature;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            r#type: vec![],
            when: None,
            who: None,
            on_behalf_of: None,
            target_format: None,
            sig_format: None,
            data: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "type": []
                }
            );
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
