//! Duration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Duration
//!
//! Version: 5.0.0
//!
//! Duration Type: A length of time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Duration
///
/// A length of time. Duration is a specialization of the FHIR Quantity
/// datatype constrained to represent an elapsed span of time, expressed as a
/// numeric value together with a coded unit drawn from the UCUM time-unit
/// value set (for example seconds, minutes, hours, or days). It is used
/// throughout FHIR R5 wherever a period of elapsed time is needed, such as a
/// medication administration window, an appointment length, or a study
/// interval.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::duration::Duration;
///
/// let value = Duration::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Duration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
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
    type T = Duration;

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
