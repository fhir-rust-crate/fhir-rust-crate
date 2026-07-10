//! Markdown
//!
//! URL: http://hl7.org/fhir/StructureDefinition/markdown
//!
//! Version: 5.0.0
//!
//! markdown type: A string that may contain Github Flavored Markdown syntax for optional processing by a mark down presentation engine
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Markdown(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Markdown::default(), Markdown(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Markdown("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Markdown = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
