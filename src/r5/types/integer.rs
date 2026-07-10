//! Integer
//!
//! URL: http://hl7.org/fhir/StructureDefinition/integer
//!
//! Version: 5.0.0
//!
//! integer Type: A whole number
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer(pub i32);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Integer::default(), Integer(0));
    }

    #[test]
    fn test_serde() {
        let value = Integer(42);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(42));
        let back: Integer = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
