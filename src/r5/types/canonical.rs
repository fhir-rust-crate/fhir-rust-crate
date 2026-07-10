//! Canonical
//!
//! URL: http://hl7.org/fhir/StructureDefinition/canonical
//!
//! Version: 5.0.0
//!
//! canonical type: A URI that is a reference to a canonical URL on a FHIR resource
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Canonical(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Canonical::default(), Canonical(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Canonical("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Canonical = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
