//! UsageContext
//!
//! URL: http://hl7.org/fhir/StructureDefinition/UsageContext
//!
//! Version: 5.0.0
//!
//! UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// UsageContext specifies clinical, business, or other metadata that can be
/// used to retrieve, index, and/or categorize an artifact. The metadata may
/// describe the applicable population (e.g., age category, DRG) or the specific
/// context of care (e.g., venue, care setting, provider of care). Each
/// UsageContext pairs a `code` identifying the type of context with a value
/// that defines the context itself.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::usage_context::UsageContext;
///
/// let value = UsageContext::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: UsageContext = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UsageContext {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Type of context being specified
    pub code: types::Coding,

    /// Value that defines the context
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value that defines the context
    pub value_quantity: Option<types::Quantity>,

    /// Value that defines the context
    pub value_range: Option<types::Range>,

    /// Value that defines the context
    pub value_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = UsageContext;

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
