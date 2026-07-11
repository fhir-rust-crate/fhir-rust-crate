//! Expression
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Expression
//!
//! Version: 5.0.0
//!
//! Expression Type: A expression that is evaluated in a specified context and returns a value.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Expression is a FHIR R5 complex datatype that captures an expression which is
/// evaluated in a specified context and returns a value. The context of use must
/// specify the environment in which the expression is evaluated and how its result
/// is used. Expressions are commonly written in languages such as FHIRPath, CQL, or
/// as FHIR query strings, and may be provided inline or referenced by URI.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::expression::Expression;
///
/// let value = Expression::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Expression = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Natural language description of the condition
    pub description: Option<types::String>,

    /// Short name assigned to expression for reuse
    pub name: Option<types::Code>,

    /// text/cql | text/fhirpath | application/x-fhir-query | etc.
    pub language: Option<types::Code>,

    /// Expression in specified language
    pub expression: Option<types::String>,

    /// Where the expression is found
    pub reference: Option<types::Uri>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Expression;

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
