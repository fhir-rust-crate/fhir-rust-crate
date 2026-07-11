//! AllergyIntolerance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
//!
//! Version: 5.0.0
//!
//! AllergyIntolerance Resource: Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Risk of harmful or undesirable, physiological response which is unique to an
/// individual and associated with exposure to a substance.
///
/// AllergyIntolerance records a clinical assessment of a propensity, or a
/// potential risk to an individual, of an adverse reaction upon future exposure
/// to the specified substance, or class of substance. It is used in FHIR R5 to
/// capture the allergy or intolerance itself along with any reaction events that
/// have been observed and the participants who asserted or recorded the record.
///
/// Clinically, this resource supports decision support and safety checking (for
/// example, prescribing or order-entry alerts), and it distinguishes between an
/// allergy (an immune-mediated response) and an intolerance (a non-immune
/// adverse reaction), while also allowing the underlying mechanism to be left
/// unspecified when it is not known. A single resource instance describes one
/// propensity for a subject; each observed occurrence of a reaction is recorded
/// as a repeating `reaction` component with details such as manifestation,
/// severity, and onset, and the individuals who reported or verified the
/// assessment can be captured via the `participant` component.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject for whom
///   the allergy or intolerance is recorded, referenced via the `patient` field.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for coded
///   values such as `clinical_status`, `verification_status`, `type`, and `code`.
/// - `Encounter` — optionally referenced via `encounter` to indicate the
///   clinical context in which the allergy or intolerance was asserted.
/// - `Condition` and `Observation` — related clinical resources that may also
///   reference or corroborate an allergy or intolerance assessment.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::allergy_intolerance::AllergyIntolerance;
///
/// let value = AllergyIntolerance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AllergyIntolerance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntolerance {
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

    /// External ids for this item
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | inactive | resolved - current clinical status of this allergy or intolerance
    pub clinical_status: Option<types::CodeableConcept>,

    /// unconfirmed | presumed | confirmed | refuted | entered-in-error - assertion confidence
    pub verification_status: Option<types::CodeableConcept>,

    /// allergy | intolerance - Underlying mechanism (if known)
    pub r#type: Option<types::CodeableConcept>,

    /// food | medication | environment | biologic
    pub category: Option<Vec<types::Code>>,

    /// low | high | unable-to-assess - estimated risk of harm from future exposure
    pub criticality: Option<types::Code>,

    /// Code that identifies the allergy or intolerance, e.g. a substance or product code
    pub code: Option<types::CodeableConcept>,

    /// Who the allergy or intolerance is for; typically a reference to a [`Patient`](crate::r5::resources::patient::Patient)
    pub patient: types::Reference,

    /// Encounter when the allergy or intolerance was asserted
    pub encounter: Option<types::Reference>,

    /// When allergy or intolerance was identified
    pub onset_date_time: Option<types::DateTime>,

    /// When allergy or intolerance was identified
    pub onset_age: Option<types::Age>,

    /// When allergy or intolerance was identified
    pub onset_period: Option<types::Period>,

    /// When allergy or intolerance was identified
    pub onset_range: Option<types::Range>,

    /// When allergy or intolerance was identified
    pub onset_string: Option<types::String>,

    /// Date allergy or intolerance was first recorded
    pub recorded_date: Option<types::DateTime>,

    /// Who or what participated in the activities related to the allergy or
    /// intolerance and how they were involved
    pub participant: Option<Vec<AllergyIntoleranceParticipant>>,

    /// Date(/time) of last known occurrence of a reaction
    pub last_occurrence: Option<types::DateTime>,

    /// Additional text not captured in other fields
    pub note: Option<Vec<types::Annotation>>,

    /// Adverse Reaction Events linked to exposure to substance
    pub reaction: Option<Vec<AllergyIntoleranceReaction>>,
}

/// Who or what participated in the activities related to the allergy or
/// intolerance and how they were involved.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntoleranceParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who or what participated in the activities related to the allergy or
    /// intolerance
    pub actor: types::Reference,
}

/// Adverse Reaction Events linked to exposure to substance.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntoleranceReaction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Specific substance or pharmaceutical product considered to be responsible
    /// for event
    pub substance: Option<types::CodeableConcept>,

    /// Clinical symptoms/signs associated with the Event
    pub manifestation: Vec<types::CodeableReference>,

    /// Description of the event as a whole
    pub description: Option<types::String>,

    /// Date(/time) when manifestations showed
    pub onset: Option<types::DateTime>,

    /// mild | moderate | severe (of event as a whole)
    pub severity: Option<types::Code>,

    /// How the subject was exposed to the substance
    pub exposure_route: Option<types::CodeableConcept>,

    /// Text about event not captured in other fields
    pub note: Option<Vec<types::Annotation>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AllergyIntolerance;

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
