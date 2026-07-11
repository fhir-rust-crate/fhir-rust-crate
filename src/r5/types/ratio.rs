//! Ratio
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Ratio
//!
//! Version: 5.0.0
//!
//! Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Ratio {
    pub numerator: Option<types::Quantity>, // « C »
    pub denominator: types::Quantity,       // Quantity(SimpleQuantity)
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Ratio;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            numerator: None,
            denominator: types::Quantity::default(),
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
                    "denominator": {}
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
                    "denominator": {}
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
