//! Age
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Age
//!
//! Version: 5.0.0
//!
//! Age Type: A duration of time during which an organism (or a process) has existed.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Age is a specialization of the FHIR Quantity datatype used to express a
/// duration of time during which an organism (or a process) has existed. It
/// carries a numerical value together with a coded unit of measure, and
/// optionally a comparator to indicate how the value should be understood.
///
/// In FHIR R5, an Age typically uses UCUM units of time (such as years, months,
/// or days) and constrains the underlying Quantity so that the value is not
/// negative. It is commonly used in clinical contexts such as age at onset,
/// age at diagnosis, or gestational age.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::age::Age;
///
/// let value = Age::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Age = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Age {
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
    type T = Age;

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
