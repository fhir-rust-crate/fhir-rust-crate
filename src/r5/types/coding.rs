//! Coding
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coding
//!
//! Version: 5.0.0
//!
//! Coding Type: A reference to a code defined by a terminology system.
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
pub struct Coding {
    pub system: Option<types::Uri>,
    pub version: Option<types::String>,
    pub code: Option<types::Code>, // « C »
    pub display: Option<String>,   // « C »
    pub user_selected: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coding;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            system: None,
            version: None,
            code: None,
            display: None,
            user_selected: None,
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
