//! ResearchStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
//!
//! Version: 5.0.0
//!
//! ResearchStudy Resource: A scientific study of nature that sometimes includes processes involved in health and disease.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A scientific study of nature that sometimes includes processes involved in
/// health and disease.
///
/// For example, clinical trials are research studies that involve people. These
/// studies may be related to new ways to screen, prevent, diagnose, and treat
/// disease. They may also study certain outcomes and certain groups of people by
/// looking at data collected in the past or future. In FHIR R5 a ResearchStudy
/// captures the design, sponsors, objectives, outcome measures, comparison
/// groups, recruitment, and status of such an investigation.
///
/// A `ResearchStudy` acts as the administrative and organizational record for a
/// study protocol: it tracks the study's lifecycle status, phase, and type of
/// investigation; who sponsors and conducts it; where and when it takes place;
/// what conditions or focuses it addresses; how participants are recruited and
/// grouped for comparison; and what objectives and outcome measures define
/// success. It is commonly referenced by other resources that represent
/// individual participation or data collected under the study, such as
/// `ResearchSubject`, and it may reference [`Patient`](crate::r5::resources::patient::Patient)
/// or `Group` resources for enrolled or observed populations, as well as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values for
/// classifying status, phase, condition, and role information throughout.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudy;
///
/// let value = ResearchStudy::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ResearchStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudy {
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

    /// Canonical identifier for this study resource
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business Identifier for study
    pub identifier: Option<Vec<types::Identifier>>,

    /// The business version for the study record
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this study (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human readable name of the study, typically shown in listings and reports
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Additional names for the study
    pub label: Option<Vec<ResearchStudyLabel>>,

    /// Steps followed in executing study
    pub protocol: Option<Vec<types::Reference>>,

    /// Part of larger study
    pub part_of: Option<Vec<types::Reference>>,

    /// References, URLs, and attachments
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Date the resource last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The current lifecycle status of the study record itself: draft | active | retired | unknown
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// treatment | prevention | diagnostic | supportive-care | screening | health-services-research | basic-science | device-feasibility
    pub primary_purpose_type: Option<types::CodeableConcept>,

    /// n-a | early-phase-1 | phase-1 | phase-1-phase-2 | phase-2 | phase-2-phase-3 | phase-3 | phase-4
    pub phase: Option<types::CodeableConcept>,

    /// Classifications of the study design characteristics
    pub study_design: Option<Vec<types::CodeableConcept>>,

    /// Drugs, devices, etc. under study
    pub focus: Option<Vec<types::CodeableReference>>,

    /// Condition, disease, or health concern being studied
    pub condition: Option<Vec<types::CodeableConcept>>,

    /// Used to search for the study
    pub keyword: Option<Vec<types::CodeableConcept>>,

    /// Geographic area for the study
    pub region: Option<Vec<types::CodeableConcept>>,

    /// Brief text explaining the study
    pub description_summary: Option<types::Markdown>,
    /// Primitive extension sibling for [`description_summary`](Self::description_summary) (FHIR `_descriptionSummary`).
    #[serde(rename = "_descriptionSummary")]
    pub description_summary_ext: Option<types::Element>,

    /// Detailed narrative of the study
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// When the study began and ended
    pub period: Option<types::Period>,

    /// Facility where study activities are conducted
    pub site: Option<Vec<types::Reference>>,

    /// Comments made about the study
    pub note: Option<Vec<types::Annotation>>,

    /// Classification for the study
    pub classifier: Option<Vec<types::CodeableConcept>>,

    /// Sponsors, collaborators, and other parties
    pub associated_party: Option<Vec<ResearchStudyAssociatedParty>>,

    /// Status of study with time for that status
    pub progress_status: Option<Vec<ResearchStudyProgressStatus>>,

    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    pub why_stopped: Option<types::CodeableConcept>,

    /// Target or actual group of participants enrolled in study
    pub recruitment: Option<ResearchStudyRecruitment>,

    /// Defined path through the study for a subject
    pub comparison_group: Option<Vec<ResearchStudyComparisonGroup>>,

    /// A goal for the study
    pub objective: Option<Vec<ResearchStudyObjective>>,

    /// A variable measured during the study
    pub outcome_measure: Option<Vec<ResearchStudyOutcomeMeasure>>,

    /// Link to results generated during the study
    pub result: Option<Vec<types::Reference>>,
}

/// Additional names for the study.
///
/// Provides alternative labels for the study such as official titles, acronyms,
/// scientific titles, or plain-language names, each classified by a type.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyLabel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// primary | official | scientific | plain-language | subtitle | short-title | acronym | earlier-title | language | auto-translated | human-use | machine-use | duplicate-uid
    pub r#type: Option<types::CodeableConcept>,

    /// The name
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Sponsors, collaborators, and other parties.
///
/// Identifies organizations and individuals associated with the study along with
/// their role, the period during which they held that role, and any classifiers
/// describing the nature of the party.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyAssociatedParty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name of associated party
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// sponsor | lead-sponsor | sponsor-investigator | primary-investigator | collaborator | funding-source | general-contact | recruitment-contact | sub-investigator | study-director | study-chair
    pub role: types::CodeableConcept,

    /// When active in the role
    pub period: Option<Vec<types::Period>>,

    /// nih | fda | government | nonprofit | academic | industry
    pub classifier: Option<Vec<types::CodeableConcept>>,

    /// Individual or organization associated with study (use practitionerRole to specify their organisation)
    pub party: Option<types::Reference>,
}

/// Status of study with time for that status.
///
/// Records a labeled state of the study (for example recruitment status), whether
/// it is actual or anticipated, and the date range over which it applied.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyProgressStatus {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for status or state (e.g. recruitment status)
    pub state: types::CodeableConcept,

    /// Actual if true else anticipated
    pub actual: Option<types::Boolean>,
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`).
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// Date range
    pub period: Option<types::Period>,
}

/// Target or actual group of participants enrolled in study.
///
/// Describes the intended and enrolled participant counts along with references
/// to the eligibility criteria and the actual group of enrolled participants.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyRecruitment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Estimated total number of participants to be enrolled
    pub target_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`target_number`](Self::target_number) (FHIR `_targetNumber`).
    #[serde(rename = "_targetNumber")]
    pub target_number_ext: Option<types::Element>,

    /// Actual total number of participants enrolled in study
    pub actual_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`actual_number`](Self::actual_number) (FHIR `_actualNumber`).
    #[serde(rename = "_actualNumber")]
    pub actual_number_ext: Option<types::Element>,

    /// Inclusion and exclusion criteria
    pub eligibility: Option<types::Reference>,

    /// Group of participants who were enrolled in study
    pub actual_group: Option<types::Reference>,
}

/// Defined path through the study for a subject.
///
/// Describes a comparison group or study arm, including its name, type, intended
/// exposures or interventions, and the group of participants observed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyComparisonGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Allows the comparisonGroup for the study and the comparisonGroup for the subject to be linked easily
    pub link_id: Option<types::Id>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Label for study comparisonGroup
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Categorization of study comparisonGroup
    pub r#type: Option<types::CodeableConcept>,

    /// Short explanation of study path
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Interventions or exposures in this comparisonGroup or cohort
    pub intended_exposure: Option<Vec<types::Reference>>,

    /// Group of participants who were enrolled in study comparisonGroup
    pub observed_group: Option<types::Reference>,
}

/// A goal for the study.
///
/// States an objective of the study, categorized as primary, secondary, or
/// exploratory, with a descriptive narrative.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyObjective {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for the objective
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// primary | secondary | exploratory
    pub r#type: Option<types::CodeableConcept>,

    /// Description of the objective
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// A variable measured during the study.
///
/// Defines an outcome measure of the study, including its name, type, narrative
/// description, and an optional reference to a structured outcome definition.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyOutcomeMeasure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for the outcome
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// primary | secondary | exploratory
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Description of the outcome
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Structured outcome definition
    pub reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ResearchStudy;

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
