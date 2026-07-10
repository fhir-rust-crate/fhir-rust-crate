//! BackboneElement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BackboneElement
//!
//! Version: 5.0.0
//!
//! BackboneElement Type: Base definition for all elements that are defined inside a resource - but not those in a data type.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// BackboneElement is the base definition for all elements that are defined
/// inside a resource, as opposed to those defined in a reusable data type.
/// In addition to the `id` and `extension` inherited from `Element`, it adds
/// support for `modifierExtension`: extensions that change the meaning of the
/// element in which they appear and therefore cannot be safely ignored by a
/// consuming system. It is used as the base type for the inline backbone
/// structures nested within FHIR R5 resources.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::backbone_element::BackboneElement;
///
/// let value = BackboneElement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: BackboneElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BackboneElement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = BackboneElement;

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
