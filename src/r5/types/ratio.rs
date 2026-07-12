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
//!
//! A `Ratio` datatype pairs a numerator and denominator `Quantity` to express a rate or proportion, such as a lab result ratio or a dosage rate.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A relationship of two `Quantity` values, expressed as a numerator and a denominator, used to represent ratios and rates such as concentrations or infusion rates.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::ratio::Ratio;
///
/// let value = Ratio::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Ratio = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Ratio {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// The value of the numerator, i.e. the value that is on the top of the ratio expression. // « C »
    pub numerator: Option<types::Quantity>,
    /// The value of the denominator, i.e. the value that is on the bottom of the ratio expression. // Quantity(SimpleQuantity)
    pub denominator: types::Quantity,
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
            ..Default::default()
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
