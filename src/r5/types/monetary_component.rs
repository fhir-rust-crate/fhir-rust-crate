//! MonetaryComponent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MonetaryComponent
//!
//! Version: 5.0.0
//!
//! MonetaryComponent Type: Availability data for an {item}.
//!
//! Represents a single component (such as a base price, tax, surcharge, or
//! discount) that contributes to an overall monetary total.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// MonetaryComponent is a FHIR R5 complex datatype that expresses a single
/// price component contributing to an overall monetary total, such as a base
/// price, surcharge, deduction, discount, or tax. Each component carries a
/// classifying `type`, an optional coded `code` to distinguish kinds of the
/// same type, and either a proportional `factor` or an explicit `amount`.
/// It is used within pricing and charge-item structures to itemize how a
/// final cost is calculated.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::monetary_component::MonetaryComponent;
///
/// let value = MonetaryComponent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MonetaryComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MonetaryComponent {
    /// base | surcharge | deduction | discount | tax | informational
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc.
    pub code: Option<types::CodeableConcept>,

    /// Factor used for calculating this component
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Explicit value amount to be used
    pub amount: Option<types::Money>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MonetaryComponent;

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
