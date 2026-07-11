//! DiagnosticReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
//!
//! Version: 5.0.0
//!
//! DiagnosticReport Resource: The findings and interpretation of diagnostic tests performed on patients, groups, products, substances, devices, locations, and/or specimens derived from these.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// DiagnosticReport
///
/// The findings and interpretation of diagnostic tests performed on patients,
/// groups of patients, products, substances, devices, and locations, and/or
/// specimens derived from these. The report includes clinical context such as
/// requesting provider information, and some mix of atomic results, images,
/// textual and coded interpretations, and formatted representations. It is
/// used to convey lab, imaging, pathology, and other diagnostic outcomes.
///
/// A DiagnosticReport is typically generated once testing on a request is
/// complete, and it acts as the aggregation point that ties together the
/// individual `Observation` results, any supporting images or documents, and
/// a narrative or coded conclusion into a single, clinically reviewable
/// report. Reports move through a lifecycle expressed by the `status` field
/// (for example `registered`, `preliminary`, `final`, and `amended`), so
/// consumers can track whether a report is still in progress or has been
/// finalized. Reports are commonly produced by laboratory, imaging,
/// pathology, cardiology, and other diagnostic services, and they are
/// referenced by ordering workflows and clinical summaries.
///
/// See also: [`Patient`](crate::r5::resources::patient::Patient) or `Group`
/// as the typical `subject` of a report, `Observation` for individual
/// result entries referenced via `result`, `ServiceRequest` for the order
/// referenced via `based_on`, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) for the coded
/// `code`, `category`, and `conclusion_code` values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::diagnostic_report::DiagnosticReport;
///
/// let value = DiagnosticReport::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DiagnosticReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
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

    /// Business identifier for report
    pub identifier: Option<Vec<types::Identifier>>,

    /// What was requested
    pub based_on: Option<Vec<types::Reference>>,

    /// registered | partial | preliminary | modified | final | amended | corrected | appended | cancelled | entered-in-error | unknown; tracks the report's lifecycle status
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Service category, such as the performing department (e.g. laboratory, radiology, cardiology)
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Name/Code for this diagnostic report, identifying the type of report or panel being reported
    pub code: types::CodeableConcept,

    /// The subject of the report - usually, but not always, the patient; may also reference a Group, Device, Location, or other subject
    pub subject: Option<types::Reference>,

    /// Health care event when test ordered
    pub encounter: Option<types::Reference>,

    /// Clinically relevant time/time-period for report (dateTime)
    pub effective_date_time: Option<types::DateTime>,

    /// Clinically relevant time/time-period for report (Period)
    pub effective_period: Option<types::Period>,

    /// DateTime this version was made
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Responsible Diagnostic Service
    pub performer: Option<Vec<types::Reference>>,

    /// Primary result interpreter
    pub results_interpreter: Option<Vec<types::Reference>>,

    /// Specimens this report is based on
    pub specimen: Option<Vec<types::Reference>>,

    /// Observations that make up the individual results contributing to this report
    pub result: Option<Vec<types::Reference>>,

    /// Comments about the diagnostic report
    pub note: Option<Vec<types::Annotation>>,

    /// Reference to full details of an analysis associated with the diagnostic report
    pub study: Option<Vec<types::Reference>>,

    /// Additional information supporting the diagnostic report
    pub supporting_info: Option<Vec<DiagnosticReportSupportingInfo>>,

    /// Key images or data associated with this report
    pub media: Option<Vec<DiagnosticReportMedia>>,

    /// Reference to a Composition resource for the DiagnosticReport structure
    pub composition: Option<types::Reference>,

    /// Clinical conclusion (interpretation) of test results, the narrative summary a clinician relies on for decision making
    pub conclusion: Option<types::Markdown>,
    /// Primitive extension sibling for [`conclusion`](Self::conclusion) (FHIR `_conclusion`).
    #[serde(rename = "_conclusion")]
    pub conclusion_ext: Option<types::Element>,

    /// Codes for the clinical conclusion of test results
    pub conclusion_code: Option<Vec<types::CodeableConcept>>,

    /// Entire report as issued
    pub presented_form: Option<Vec<types::Attachment>>,
}

/// DiagnosticReportSupportingInfo
///
/// Additional information supporting the diagnostic report, referencing other
/// resources that provide context for the results.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReportSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Supporting information role code
    pub r#type: types::CodeableConcept,

    /// Supporting information reference
    pub reference: types::Reference,
}

/// DiagnosticReportMedia
///
/// Key images or data associated with this report, such as images of a
/// microslide or a formatted representation of the source data.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReportMedia {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Comment about the image or data (e.g. explanation)
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Reference to the image or data source
    pub link: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DiagnosticReport;

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
