//! Integer64
//!
//! URL: http://hl7.org/fhir/StructureDefinition/integer64
//!
//! Version: 5.0.0
//!
//! integer64 Type: A very large whole number
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

use ::serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer64(#[serde_as(as = "DisplayFromStr")] pub i64);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Integer64::default(), Integer64(0));
    }

    #[test]
    fn test_serde() {
        let value = Integer64(9_007_199_254_740_993);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("9007199254740993"));
        let back: Integer64 = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
