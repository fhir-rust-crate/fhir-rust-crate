//! Uuid
//!
//! URL: http://hl7.org/fhir/StructureDefinition/uuid
//!
//! Version: 5.0.0
//!
//! uuid type: A UUID, represented as a URI
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Uuid(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Uuid::default(), Uuid(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Uuid("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Uuid = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
