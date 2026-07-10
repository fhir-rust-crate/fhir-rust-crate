//! Base64Binary
//!
//! URL: http://hl7.org/fhir/StructureDefinition/base64Binary
//!
//! Version: 5.0.0
//!
//! base64Binary Type: A stream of bytes
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Base64Binary(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Base64Binary::default(), Base64Binary(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Base64Binary("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Base64Binary = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
