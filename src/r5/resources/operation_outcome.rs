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
use fhir_derive_macros::Validate;

/// A collection of error, warning, or information messages that result from a
/// system action. OperationOutcome is a common infrastructure resource used
/// across FHIR R5 to communicate the detailed outcome of processing a request:
/// it is returned by RESTful interactions when an operation fails or produces
/// warnings, embedded in the entries of a batch or transaction `Bundle` to report
/// per-entry results, returned by the `$validate` and other operations, and used
/// in FHIR messaging to signal processing status. Rather than conveying business
/// data, it exists purely to describe what happened, so that clients can present
/// meaningful diagnostics to users or drive automated error handling.
///
/// The resource carries one or more `issue` entries, each describing the
/// severity, a machine-readable code, and optional human-readable details,
/// diagnostics, and element locations for a single problem. An OperationOutcome
/// with no issues of severity `error` or `fatal` generally indicates success,
/// possibly accompanied by `warning` or `information` issues.
///
/// # Related resources
///
/// The `issue` details are expressed using [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// and this resource is frequently returned within the entries of a `Bundle`
/// during batch, transaction, and search interactions.
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
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// One or more issues describing each error, warning, or informational message from the action; at least one is expected
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

    /// Severity of this issue, one of fatal, error, warning, information, or success, indicating how it affects the overall action
    pub severity: types::Code,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`).
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// Machine-processable code identifying the type of error or warning, drawn from the FHIR IssueType value set
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Additional details about the error, as a CodeableConcept that may carry a more specific error code and text
    pub details: Option<types::CodeableConcept>,

    /// Additional diagnostic information about the issue
    pub diagnostics: Option<types::String>,
    /// Primitive extension sibling for [`diagnostics`](Self::diagnostics) (FHIR `_diagnostics`).
    #[serde(rename = "_diagnostics")]
    pub diagnostics_ext: Option<types::Element>,

    /// Deprecated: Path of element(s) related to issue
    pub location: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`location`](Self::location) (FHIR `_location`).
    #[serde(rename = "_location")]
    pub location_ext: Option<Vec<Option<types::Element>>>,

    /// FHIRPath of element(s) related to issue
    pub expression: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`).
    #[serde(rename = "_expression")]
    pub expression_ext: Option<Vec<Option<types::Element>>>,
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
