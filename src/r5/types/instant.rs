//! Instant
//!
//! URL: http://hl7.org/fhir/StructureDefinition/instant
//!
//! Version: 5.0.0
//!
//! instant Type: An instant in time - known at least to the second
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Instant(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Instant::default(), Instant(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Instant("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Instant = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
