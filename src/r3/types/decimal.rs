//! decimal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/decimal
//!
//! Version: 
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for decimal Type: A rational number with implicit
/// precision
///
/// # Examples
///
/// ```
/// use fhir::r3::types::decimal::Decimal;
///
/// let value = Decimal::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Decimal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Decimal(
    /// The numeric value, preserved exactly as written so that significant
    /// trailing zeros survive a round trip.
    pub ::serde_json::Number,
);

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
        let value = Decimal(::serde_json::Number::from(42));
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(42));
        let back: Decimal = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
