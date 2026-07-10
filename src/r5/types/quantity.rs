//! Quantity
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Quantity
//!
//! Version: 5.0.0
//!
//! Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.
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
pub struct Quantity {
    pub value: Option<types::Decimal>,
    pub comparator: Option<types::Code>, // « QuantityComparator! »
    pub unit: Option<types::String>,
    pub system: Option<types::Uri>, // « C »
    pub code: Option<types::Code>,  // « C »
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Quantity;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            value: None,
            comparator: None,
            unit: None,
            system: None,
            code: None,
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
