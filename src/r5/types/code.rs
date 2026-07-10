//! Code
//!
//! URL: http://hl7.org/fhir/StructureDefinition/code
//!
//! Version: 5.0.0
//!
//! code type: A string which has at least one character and no leading or trailing whitespace and where there is no whitespace other than single spaces in the contents
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Code(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Code::default(), Code(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Code("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Code = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
