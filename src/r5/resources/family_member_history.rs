//! FamilyMemberHistory
//!
//! URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
//!
//! Version: 5.0.0
//!
//! FamilyMemberHistory Resource: Significant health conditions for a person related to the patient relevant in the context of care for the patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// FamilyMemberHistory captures significant health conditions for a person
/// related to the patient that are relevant to the patient's care. It records
/// the family relationship, relevant demographic details of the family member,
/// and the conditions, procedures, and other observations pertinent to
/// assessing hereditary or familial risk. It is commonly used to build a
/// family health history for genetic or preventive care planning.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::family_member_history::FamilyMemberHistory;
///
/// let value = FamilyMemberHistory::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: FamilyMemberHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistory {
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

    /// External Id(s) for this record
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// partial | completed | entered-in-error | health-unknown
    pub status: types::Code,

    /// subject-unknown | withheld | unable-to-obtain | deferred
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// Patient history is about
    pub patient: types::Reference,

    /// When history was recorded or last updated
    pub date: Option<types::DateTime>,

    /// Who or what participated in the activities related to the family member history and how they were involved
    pub participant: Option<Vec<FamilyMemberHistoryParticipant>>,

    /// The family member described
    pub name: Option<types::String>,

    /// Relationship to the subject
    pub relationship: types::CodeableConcept,

    /// male | female | other | unknown
    pub sex: Option<types::CodeableConcept>,

    /// (approximate) date of birth
    pub born_period: Option<types::Period>,

    /// (approximate) date of birth
    pub born_date: Option<types::Date>,

    /// (approximate) date of birth
    pub born_string: Option<types::String>,

    /// (approximate) age
    pub age_age: Option<types::Age>,

    /// (approximate) age
    pub age_range: Option<types::Range>,

    /// (approximate) age
    pub age_string: Option<types::String>,

    /// Age is estimated?
    pub estimated_age: Option<types::Boolean>,

    /// Dead? How old/when?
    pub deceased_boolean: Option<types::Boolean>,

    /// Dead? How old/when?
    pub deceased_age: Option<types::Age>,

    /// Dead? How old/when?
    pub deceased_range: Option<types::Range>,

    /// Dead? How old/when?
    pub deceased_date: Option<types::Date>,

    /// Dead? How old/when?
    pub deceased_string: Option<types::String>,

    /// Why was family member history performed?
    pub reason: Option<Vec<types::CodeableReference>>,

    /// General note about related person
    pub note: Option<Vec<types::Annotation>>,

    /// Condition that the related person had
    pub condition: Option<Vec<FamilyMemberHistoryCondition>>,

    /// Procedures that the related person had
    pub procedure: Option<Vec<FamilyMemberHistoryProcedure>>,
}

/// Who or what participated in the activities related to the family member
/// history and how they were involved.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistoryParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who or what participated in the activities related to the family member history
    pub actor: types::Reference,
}

/// Condition that the related person had. This is a backbone element describing
/// the significant conditions suffered by the family member.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistoryCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Condition suffered by relation
    pub code: types::CodeableConcept,

    /// deceased | permanent disability | etc
    pub outcome: Option<types::CodeableConcept>,

    /// Whether the condition contributed to the cause of death
    pub contributed_to_death: Option<types::Boolean>,

    /// When condition first manifested
    pub onset_age: Option<types::Age>,

    /// When condition first manifested
    pub onset_range: Option<types::Range>,

    /// When condition first manifested
    pub onset_period: Option<types::Period>,

    /// When condition first manifested
    pub onset_string: Option<types::String>,

    /// Extra information about condition
    pub note: Option<Vec<types::Annotation>>,
}

/// Procedures that the related person had. This is a backbone element describing
/// the significant procedures performed on the family member.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistoryProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Procedures performed on the related person
    pub code: types::CodeableConcept,

    /// What happened following the procedure
    pub outcome: Option<types::CodeableConcept>,

    /// Whether the procedure contributed to the cause of death
    pub contributed_to_death: Option<types::Boolean>,

    /// When the procedure was performed
    pub performed_age: Option<types::Age>,

    /// When the procedure was performed
    pub performed_range: Option<types::Range>,

    /// When the procedure was performed
    pub performed_period: Option<types::Period>,

    /// When the procedure was performed
    pub performed_string: Option<types::String>,

    /// When the procedure was performed
    pub performed_date_time: Option<types::DateTime>,

    /// Extra information about the procedure
    pub note: Option<Vec<types::Annotation>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = FamilyMemberHistory;

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
