//! ProductShelfLife
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ProductShelfLife
//!
//! Version: 5.0.0
//!
//! ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The shelf-life and storage information for a medicinal product item or
/// container can be described using this class.
///
/// `ProductShelfLife` is a complex datatype used within medicinal product
/// definitions to capture how long an item remains usable and under what
/// conditions it must be stored. It records the type of shelf life being
/// described, the duration of that period, and any special storage precautions,
/// each expressed using appropriate controlled vocabularies.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::product_shelf_life::ProductShelfLife;
///
/// let value = ProductShelfLife::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ProductShelfLife = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductShelfLife {
    /// This describes the shelf life, taking into account various scenarios such
    /// as shelf life of the packaged Medicinal Product itself, shelf life after
    /// transformation where necessary and shelf life after the first opening of
    /// a bottle, etc. The shelf life type shall be specified using an
    /// appropriate controlled vocabulary The controlled term and the controlled
    /// term identifier shall be specified.
    pub r#type: Option<types::CodeableConcept>,

    /// The shelf life time period can be specified using a numerical value for
    /// the period of time and its unit of time measurement The unit of
    /// measurement shall be specified in accordance with ISO 11240 and the
    /// resulting terminology The symbol and the symbol identifier shall be used.
    /// This is the `Duration` variant of the `period[x]` choice element.
    pub period_duration: Option<types::Duration>,

    /// The shelf life time period can be specified using a numerical value for
    /// the period of time and its unit of time measurement. This is the
    /// `string` variant of the `period[x]` choice element.
    pub period_string: Option<types::String>,

    /// Special precautions for storage, if any, can be specified using an
    /// appropriate controlled vocabulary The controlled term and the controlled
    /// term identifier shall be specified.
    pub special_precautions_for_storage: Option<Vec<types::CodeableConcept>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ProductShelfLife;

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
