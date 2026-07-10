//! Procedure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Procedure
//!
//! Version: 5.0.0
//!
//! Procedure Resource: An action that is or was performed on or for a patient, practitioner, device, organization, or location.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// An action that is or was performed on or for a patient, practitioner, device,
/// organization, or location. For example, this can be a physical intervention on
/// a patient like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy. It can also be a quality or safety inspection for
/// a location, organization, or device, or an accreditation procedure on a
/// practitioner.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::procedure::Procedure;
///
/// let value = Procedure::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Procedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Procedure {
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

    /// External Identifiers for this procedure
    pub identifier: Option<Vec<types::Identifier>>,

    /// Instantiates FHIR protocol or definition
    pub instantiates_canonical: Option<Vec<types::Canonical>>,

    /// Instantiates external protocol or definition
    pub instantiates_uri: Option<Vec<types::Uri>>,

    /// A request for this procedure
    pub based_on: Option<Vec<types::Reference>>,

    /// Part of referenced event
    pub part_of: Option<Vec<types::Reference>>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: types::Code,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Classification of the procedure
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Identification of the procedure
    pub code: Option<types::CodeableConcept>,

    /// Individual or entity the procedure was performed on
    pub subject: types::Reference,

    /// Who is the target of the procedure when it is not the subject of record only
    pub focus: Option<types::Reference>,

    /// The Encounter during which this Procedure was created
    pub encounter: Option<types::Reference>,

    /// When the procedure occurred or is occurring
    pub occurrence_date_time: Option<types::DateTime>,

    /// When the procedure occurred or is occurring
    pub occurrence_period: Option<types::Period>,

    /// When the procedure occurred or is occurring
    pub occurrence_string: Option<types::String>,

    /// When the procedure occurred or is occurring
    pub occurrence_age: Option<types::Age>,

    /// When the procedure occurred or is occurring
    pub occurrence_range: Option<types::Range>,

    /// When the procedure occurred or is occurring
    pub occurrence_timing: Option<types::Timing>,

    /// When the procedure was first captured in the subject's record
    pub recorded: Option<types::DateTime>,

    /// Who recorded the procedure
    pub recorder: Option<types::Reference>,

    /// Reported rather than primary record
    pub reported_boolean: Option<types::Boolean>,

    /// Reported rather than primary record
    pub reported_reference: Option<types::Reference>,

    /// Who performed the procedure and what they did
    pub performer: Option<Vec<ProcedurePerformer>>,

    /// Where the procedure happened
    pub location: Option<types::Reference>,

    /// The justification that the procedure was performed
    pub reason: Option<Vec<types::CodeableReference>>,

    /// Target body sites
    pub body_site: Option<Vec<types::CodeableConcept>>,

    /// The result of procedure
    pub outcome: Option<types::CodeableConcept>,

    /// Any report resulting from the procedure
    pub report: Option<Vec<types::Reference>>,

    /// Complication following the procedure
    pub complication: Option<Vec<types::CodeableReference>>,

    /// Instructions for follow up
    pub follow_up: Option<Vec<types::CodeableConcept>>,

    /// Additional information about the procedure
    pub note: Option<Vec<types::Annotation>>,

    /// Manipulated, implanted, or removed device
    pub focal_device: Option<Vec<ProcedureFocalDevice>>,

    /// Items used during procedure
    pub used: Option<Vec<types::CodeableReference>>,

    /// Extra information relevant to the procedure
    pub supporting_info: Option<Vec<types::Reference>>,
}

/// Who performed the procedure and what they did.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProcedurePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who performed the procedure
    pub actor: types::Reference,

    /// Organization the device or practitioner was acting for
    pub on_behalf_of: Option<types::Reference>,

    /// When the performer performed the procedure
    pub period: Option<types::Period>,
}

/// Manipulated, implanted, or removed device.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProcedureFocalDevice {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Kind of change to device
    pub action: Option<types::CodeableConcept>,

    /// Device that was changed
    pub manipulated: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Procedure;

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
