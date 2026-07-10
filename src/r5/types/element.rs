//! Element
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Element
//!
//! Version: 5.0.0
//!
//! Element Type: Base definition for all elements in a resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The base definition for all elements contained in a resource. Every FHIR
/// element carries an optional `id` for inter-element referencing and an
/// optional set of `extension`s that add data not present in the base
/// definition.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::element::Element;
///
/// let element = Element::default();
/// let json = ::serde_json::to_value(&element).unwrap();
/// assert_eq!(json, ::serde_json::json!({}));
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    pub extension: Option<Vec<types::Extension>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Element;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            id: None,
            extension: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
