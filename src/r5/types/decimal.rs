//! Decimal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/decimal
//!
//! Version: 5.0.0
//!
//! decimal Type: A rational number with implicit precision
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Decimal(pub ::serde_json::Number);

impl Default for Decimal {
    fn default() -> Self {
        Decimal(::serde_json::Number::from(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Decimal::default(), Decimal(::serde_json::Number::from(0)));
    }

    #[test]
    fn test_serde() {
        let value = Decimal(::serde_json::Number::from_f64(3.5).unwrap());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(3.5));
        let back: Decimal = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
