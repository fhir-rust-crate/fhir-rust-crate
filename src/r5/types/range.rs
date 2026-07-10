//! Range
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Range
//!
//! Version: 5.0.0
//!
//! Range Type: A set of ordered Quantities defined by a low and high limit.
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
pub struct Range {
    pub low: types::Quantity,  // Quantity(SimpleQuantity) [0..1] « C »
    pub high: types::Quantity, // Quantity(SimpleQuantity) [0..1] « C »
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Range;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            low: types::Quantity::default(),
            high: types::Quantity::default(),
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "low": {},
                    "high": {}
                }
            );
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!(
                {
                    "low": {},
                    "high": {}
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
