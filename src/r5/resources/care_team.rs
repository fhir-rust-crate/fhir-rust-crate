//! CareTeam
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CareTeam
//!
//! Version: 5.0.0
//!
//! CareTeam Resource: The Care Team includes all the people and organizations who plan to participate in the coordination and delivery of care.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The Care Team includes all the people and organizations who plan to
/// participate in the coordination and delivery of care.
///
/// A CareTeam resource identifies the group of practitioners, patients,
/// caregivers, and organizations that work together to coordinate and deliver
/// care for a particular subject, condition, or episode. Each participant may
/// have a specified role, coverage period, and the organization on whose behalf
/// they act. In FHIR R5 it is frequently referenced by CarePlan and other
/// workflow resources to describe who is responsible for a patient's care.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::care_team::CareTeam;
///
/// let value = CareTeam::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CareTeam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CareTeam {
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

    /// External Ids for this team
    pub identifier: Option<Vec<types::Identifier>>,

    /// proposed | active | suspended | inactive | entered-in-error
    pub status: Option<types::Code>,

    /// Type of team
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Name of the team, such as crisis assessment team
    pub name: Option<types::String>,

    /// Who care team is for
    pub subject: Option<types::Reference>,

    /// Time period team covers
    pub period: Option<types::Period>,

    /// Members of the team
    pub participant: Option<Vec<CareTeamParticipant>>,

    /// Why the care team exists
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Organization responsible for the care team
    pub managing_organization: Option<Vec<types::Reference>>,

    /// A contact detail for the care team (that applies to all members)
    pub telecom: Option<Vec<types::ContactPoint>>,

    /// Comments made about the CareTeam
    pub note: Option<Vec<types::Annotation>>,
}

/// Members of the team.
///
/// Identifies all people and organizations who are expected to be involved in
/// the care team, along with their role, the coverage period during which they
/// are generally available, and the organization on whose behalf they act.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CareTeamParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of involvement
    pub role: Option<types::CodeableConcept>,

    /// Who is involved
    pub member: Option<types::Reference>,

    /// Organization of the practitioner
    pub on_behalf_of: Option<types::Reference>,

    /// When the member is generally available within this care team
    pub coverage_period: Option<types::Period>,

    /// When the member is generally available within this care team
    pub coverage_timing: Option<types::Timing>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CareTeam;

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
