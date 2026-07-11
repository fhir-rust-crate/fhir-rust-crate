//! DetectedIssue
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
//!
//! Version: 5.0.0
//!
//! DetectedIssue Resource: Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, gaps in care, etc.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient.
///
/// A DetectedIssue records a problem discovered during clinical decision
/// support or review, such as a drug-drug interaction, duplicate therapy,
/// ineffective treatment frequency, or a procedure-condition conflict. It
/// captures the severity, the resources implicated in the issue, supporting
/// evidence, and any mitigation steps taken to address it. In FHIR R5 it is
/// commonly produced by decision-support systems and reviewed by clinicians.
/// A single detected issue may reference one or more implicated resources
/// (such as a `MedicationRequest` or `Procedure`), and downstream workflows
/// can track whether the issue is still open or has been mitigated by
/// recording one or more `DetectedIssueMitigation` entries.
///
/// # Related resources
///
/// - The `subject` of the detected issue is typically a
///   [`Patient`](crate::r5::resources::patient::Patient).
/// - `category`, `code`, and `severity` use
///   [`CodeableConcept`](crate::r5::types::CodeableConcept) to classify the
///   nature and seriousness of the issue.
/// - The `implicated` field references the clinical resources (such as
///   `MedicationRequest`, `Procedure`, or `Condition`) that are involved in
///   the issue.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::detected_issue::DetectedIssue;
///
/// let value = DetectedIssue::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DetectedIssue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssue {
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

    /// Unique id for the detected issue
    pub identifier: Option<Vec<types::Identifier>>,

    /// The workflow status of this issue: preliminary | final | entered-in-error | mitigated
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Broad category of the detected issue, e.g. drug-drug interaction, duplicate therapy, etc
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Specific type of detected issue, e.g. drug-drug, duplicate therapy, etc
    pub code: Option<types::CodeableConcept>,

    /// Indicates the potential clinical seriousness of the issue: high | moderate | low
    pub severity: Option<types::Code>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`).
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// The patient, or other subject, for whom the issue was detected
    pub subject: Option<types::Reference>,

    /// Encounter detected issue is part of
    pub encounter: Option<types::Reference>,

    /// When identified
    pub identified_date_time: Option<types::DateTime>,

    /// When identified
    pub identified_period: Option<types::Period>,

    /// The provider or device that identified the issue
    pub author: Option<types::Reference>,

    /// The clinical resource(s), such as a medication order or procedure, that are implicated in the issue
    pub implicated: Option<Vec<types::Reference>>,

    /// Supporting evidence
    pub evidence: Option<Vec<DetectedIssueEvidence>>,

    /// Description and context
    pub detail: Option<types::Markdown>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`).
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,

    /// Authority for issue
    pub reference: Option<types::Uri>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`).
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Step taken to address
    pub mitigation: Option<Vec<DetectedIssueMitigation>>,
}

/// Supporting evidence.
///
/// Supporting evidence or manifestations that provide the basis for identifying
/// the detected issue.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssueEvidence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Manifestation
    pub code: Option<Vec<types::CodeableConcept>>,

    /// Supporting information
    pub detail: Option<Vec<types::Reference>>,
}

/// Step taken to address.
///
/// Indicates an action that has been taken or is committed to reduce or
/// eliminate the likelihood or severity of the identified issue.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssueMitigation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What mitigation?
    pub action: types::CodeableConcept,

    /// Date committed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is committing?
    pub author: Option<types::Reference>,

    /// Additional notes about the mitigation
    pub note: Option<Vec<types::Annotation>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DetectedIssue;

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
