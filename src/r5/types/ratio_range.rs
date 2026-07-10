//! RatioRange
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RatioRange
//!
//! Version: 5.0.0
//!
//! RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.
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
pub struct RatioRange {
    pub low_numerator: Option<types::Quantity>, // Quantity(SimpleQuantity) [0..1] « C »
    pub high_numerator: Option<types::Quantity>, // Quantity(SimpleQuantity) [0..1] « C »
    pub denominator: Option<types::Quantity>,   // Quantity(SimpleQuantity) [0..1] « C »
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RatioRange;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            low_numerator: None,
            high_numerator: None,
            denominator: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
