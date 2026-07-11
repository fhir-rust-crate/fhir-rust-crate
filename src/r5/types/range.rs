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
//!
//! A Range represents a bounded interval of quantities, expressed as a low and high limit, and is used wherever a value is best described as falling somewhere between two measured or observed quantities.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A set of ordered quantities defined by a low and high limit, used to express a value that
/// falls somewhere within a bounded interval, such as a normal reference range for an
/// observation or a dosage range. Both the low and high limits are simple quantities that
/// share the same unit of measure, and either limit may be omitted when the range is
/// open-ended.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::range::Range;
///
/// let value = Range::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Range = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    /// The low limit of the range; omitted if the range has no lower bound. // « C »
    pub low: types::Quantity,
    /// The high limit of the range; omitted if the range has no upper bound. // « C »
    pub high: types::Quantity,
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
