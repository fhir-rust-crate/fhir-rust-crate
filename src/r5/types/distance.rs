//! Distance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Distance
//!
//! Version: 5.0.0
//!
//! Distance Type: A length - a value with a unit that is a physical distance.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A length - a value with a unit that is a physical distance.
///
/// Distance is a measured amount (or an amount that can potentially be
/// measured) expressed as a numerical value together with a unit of
/// measure. It builds on the FHIR Quantity structure, adding a comparator
/// and a coded unit representation so distances can be compared and
/// processed unambiguously across systems.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::distance::Distance;
///
/// let value = Distance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Distance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Distance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<types::Decimal>,

    /// < | <= | >= | > | ad - how to understand the value
    pub comparator: Option<types::Code>,

    /// Unit representation
    pub unit: Option<types::String>,

    /// System that defines coded unit form
    pub system: Option<types::Uri>,

    /// Coded form of the unit
    pub code: Option<types::Code>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Distance;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
