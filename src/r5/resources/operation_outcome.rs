//! OperationOutcome
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OperationOutcome
//!
//! Version: 5.0.0
//!
//! OperationOutcome Resource: A collection of error, warning, or information messages that result from a system action.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A collection of error, warning, or information messages that result from a
/// system action. OperationOutcome is a common resource used as part of the
/// response to RESTful operations, batch/transaction results, and messaging to
/// convey the outcome of processing a request. It carries one or more `issue`
/// entries, each describing the severity, a machine-readable code, and optional
/// human-readable details, diagnostics, and locations for a single problem.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::operation_outcome::OperationOutcome;
///
/// let value = OperationOutcome::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: OperationOutcome = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperationOutcome {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content
    pub language: Option<types::Code>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A single issue associated with the action
    pub issue: Vec<OperationOutcomeIssue>,
}

/// A single issue associated with the action. Each issue describes one error,
/// warning, or informational message produced by the system, including its
/// severity, a machine-processable code, and optional details, diagnostics, and
/// the element location(s) it relates to.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperationOutcomeIssue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// fatal | error | warning | information | success
    pub severity: types::Code,

    /// Error or warning code
    pub code: types::Code,

    /// Additional details about the error
    pub details: Option<types::CodeableConcept>,

    /// Additional diagnostic information about the issue
    pub diagnostics: Option<types::String>,

    /// Deprecated: Path of element(s) related to issue
    pub location: Option<Vec<types::String>>,

    /// FHIRPath of element(s) related to issue
    pub expression: Option<Vec<types::String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = OperationOutcome;

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
