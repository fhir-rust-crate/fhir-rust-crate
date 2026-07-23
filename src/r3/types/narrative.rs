//! Narrative
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Narrative
//!
//! Version: 
//!
//! A human-readable formatted text, including images
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Narrative Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::narrative::Narrative;
///
/// let value = Narrative::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Narrative = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Narrative {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// generated | extensions | additional | empty
    pub status: crate::coded::Coded<crate::r3::codes::NarrativeStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Limited xhtml content
    pub div: types::Xhtml,
    /// Primitive extension sibling for [`div`](Self::div) (FHIR `_div`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_div")]
    pub div_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Narrative;

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
