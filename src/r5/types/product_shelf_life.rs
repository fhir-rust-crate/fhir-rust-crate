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
use fhir_derive_macros::Validate;

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
/// use fhir::r5::types::product_shelf_life::ProductShelfLife;
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
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,
    /// The type of shelf life being described, e.g. in-use or transformation
    /// shelf life, from an appropriate controlled vocabulary.
    pub r#type: Option<types::CodeableConcept>,

    /// The shelf life period expressed as a duration.
    /// This is the `Duration` variant of the `period[x]` choice element.
    pub period_duration: Option<types::Duration>,

    /// The shelf life period expressed as free text.
    /// This is the `string` variant of the `period[x]` choice element.
    pub period_string: Option<types::String>,

    /// Special precautions for storage of the item, if any.
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
