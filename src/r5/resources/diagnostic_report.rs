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
use fhir_derive::Validate;

/// DiagnosticReport
///
/// The findings and interpretation of diagnostic tests performed on patients,
/// groups of patients, products, substances, devices, and locations, and/or
/// specimens derived from these. The report includes clinical context such as
/// requesting provider information, and some mix of atomic results, images,
/// textual and coded interpretations, and formatted representations. It is
/// used to convey lab, imaging, pathology, and other diagnostic outcomes.
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

    /// Business identifier for report
    pub identifier: Option<Vec<types::Identifier>>,

    /// What was requested
    pub based_on: Option<Vec<types::Reference>>,

    /// registered | partial | preliminary | modified | final | amended | corrected | appended | cancelled | entered-in-error | unknown
    pub status: types::Code,

    /// Service category
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Name/Code for this diagnostic report
    pub code: types::CodeableConcept,

    /// The subject of the report - usually, but not always, the patient
    pub subject: Option<types::Reference>,

    /// Health care event when test ordered
    pub encounter: Option<types::Reference>,

    /// Clinically relevant time/time-period for report (dateTime)
    pub effective_date_time: Option<types::DateTime>,

    /// Clinically relevant time/time-period for report (Period)
    pub effective_period: Option<types::Period>,

    /// DateTime this version was made
    pub issued: Option<types::Instant>,

    /// Responsible Diagnostic Service
    pub performer: Option<Vec<types::Reference>>,

    /// Primary result interpreter
    pub results_interpreter: Option<Vec<types::Reference>>,

    /// Specimens this report is based on
    pub specimen: Option<Vec<types::Reference>>,

    /// Observations
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

    /// Clinical conclusion (interpretation) of test results
    pub conclusion: Option<types::Markdown>,

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
