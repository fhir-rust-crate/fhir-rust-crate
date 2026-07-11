//! ClinicalImpression
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
//!
//! Version: 5.0.0
//!
//! ClinicalImpression Resource: A record of a clinical assessment performed to determine what problem(s) may affect the patient before planning treatments or management strategies.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management
/// strategies that are best to manage a patient's condition. Assessments are
/// often 1:1 with a clinical consultation / encounter, but this varies greatly
/// depending on the clinical workflow. This resource is called
/// "ClinicalImpression" rather than "ClinicalAssessment" to avoid confusion.
///
/// A `ClinicalImpression` captures the narrative and structured reasoning a
/// clinician goes through while evaluating a patient: the problems or
/// conditions considered, the evidence and findings that support or rule out
/// those problems, the prognosis, and any change in the patient's condition
/// relative to a previous assessment. It is a point-in-time snapshot of
/// clinical thinking, typically produced during or shortly after an encounter,
/// and is often used to justify subsequent orders, referrals, or care plans.
///
/// # Related resources
///
/// A `ClinicalImpression` is usually linked to the [`Patient`](crate::r5::resources::patient::Patient)
/// or group being assessed via `subject`, and may reference the `Encounter`
/// during which it was formed, the performing practitioner via
/// `performer`, and a prior `ClinicalImpression` via `previous`. Findings and
/// status information are commonly expressed using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), while supporting
/// evidence may reference `Observation`, `Condition`, or other diagnostic
/// resources through `supporting_info`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::clinical_impression::ClinicalImpression;
///
/// let value = ClinicalImpression::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClinicalImpression = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpression {
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

    /// Business identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// The workflow state of this assessment: preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: types::Code,

    /// Coded or textual reason explaining why the assessment currently has this status
    pub status_reason: Option<types::CodeableConcept>,

    /// Why/how the assessment was performed
    pub description: Option<types::String>,

    /// The [`Patient`](crate::r5::resources::patient::Patient) or group whose condition is being assessed
    pub subject: types::Reference,

    /// The Encounter during which this ClinicalImpression was created
    pub encounter: Option<types::Reference>,

    /// Time of assessment
    pub effective_date_time: Option<types::DateTime>,

    /// Time of assessment
    pub effective_period: Option<types::Period>,

    /// When the assessment was documented
    pub date: Option<types::DateTime>,

    /// The clinician performing the assessment
    pub performer: Option<types::Reference>,

    /// Reference to last assessment
    pub previous: Option<types::Reference>,

    /// Relevant impressions of patient state
    pub problem: Option<Vec<types::Reference>>,

    /// Change in the status/pattern of a subject's condition since previously
    /// assessed, such as worsening, improving, or no change
    pub change_pattern: Option<types::CodeableConcept>,

    /// Clinical Protocol followed
    pub protocol: Option<Vec<types::Uri>>,

    /// Summary of the assessment
    pub summary: Option<types::String>,

    /// Possible or likely findings and diagnoses
    pub finding: Option<Vec<ClinicalImpressionFinding>>,

    /// Estimate of likely outcome
    pub prognosis_codeable_concept: Option<Vec<types::CodeableConcept>>,

    /// RiskAssessment expressing likely outcome
    pub prognosis_reference: Option<Vec<types::Reference>>,

    /// Information supporting the clinical impression
    pub supporting_info: Option<Vec<types::Reference>>,

    /// Comments made about the ClinicalImpression
    pub note: Option<Vec<types::Annotation>>,
}

/// Possible or likely findings and diagnoses.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpressionFinding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// What was found
    pub item: Option<types::CodeableReference>,

    /// Which investigations support finding
    pub basis: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ClinicalImpression;

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
