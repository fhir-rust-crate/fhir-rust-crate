//! ActivityDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
//!
//! Version: 5.0.0
//!
//! ActivityDefinition Resource: This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance
/// context. An ActivityDefinition describes the shape of an activity that can
/// be instantiated (for example, to create an order or care plan activity), and
/// is commonly referenced from PlanDefinition or care planning workflows in
/// FHIR R5. It carries clinical detail such as timing, dosage, participants, and
/// dynamic values used to customize the instantiated activity.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::activity_definition::ActivityDefinition;
///
/// let value = ActivityDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ActivityDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDefinition {
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

    /// Canonical identifier for this activity definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the activity definition
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the activity definition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this activity definition (computer friendly)
    pub name: Option<types::String>,

    /// Name for this activity definition (human friendly)
    pub title: Option<types::String>,

    /// Subordinate title of the activity definition
    pub subtitle: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Type of individual the activity definition is intended for
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// Type of individual the activity definition is intended for
    pub subject_reference: Option<types::Reference>,

    /// Type of individual the activity definition is intended for
    pub subject_canonical: Option<types::Canonical>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the activity definition
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for activity definition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this activity definition is defined
    pub purpose: Option<types::Markdown>,

    /// Describes the clinical usage of the activity definition
    pub usage: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When the activity definition was approved by publisher
    pub approval_date: Option<types::Date>,

    /// When the activity definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// When the activity definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<types::CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<types::ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<types::ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<types::ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<types::ContactDetail>>,

    /// Additional documentation, citations, etc
    pub related_artifact: Option<Vec<types::RelatedArtifact>>,

    /// Logic used by the activity definition
    pub library: Option<Vec<types::Canonical>>,

    /// Kind of resource
    pub kind: Option<types::Code>,

    /// What profile the resource needs to conform to
    pub profile: Option<types::Canonical>,

    /// Detail type of activity
    pub code: Option<types::CodeableConcept>,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: Option<types::Code>,

    /// routine | urgent | asap | stat
    pub priority: Option<types::Code>,

    /// True if the activity should not be performed
    pub do_not_perform: Option<types::Boolean>,

    /// When activity is to occur
    pub timing_timing: Option<types::Timing>,

    /// When activity is to occur
    pub timing_age: Option<types::Age>,

    /// When activity is to occur
    pub timing_range: Option<types::Range>,

    /// When activity is to occur
    pub timing_duration: Option<types::Duration>,

    /// Preconditions for service
    pub as_needed_boolean: Option<types::Boolean>,

    /// Preconditions for service
    pub as_needed_codeable_concept: Option<types::CodeableConcept>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

    /// Who should participate in the action
    pub participant: Option<Vec<ActivityDefinitionParticipant>>,

    /// What's administered/supplied
    pub product_reference: Option<types::Reference>,

    /// What's administered/supplied
    pub product_codeable_concept: Option<types::CodeableConcept>,

    /// How much is administered/consumed/supplied
    pub quantity: Option<types::Quantity>,

    /// Detailed dosage instructions
    pub dosage: Option<Vec<types::Dosage>>,

    /// What part of body to perform on
    pub body_site: Option<Vec<types::CodeableConcept>>,

    /// What specimens are required to perform this action
    pub specimen_requirement: Option<Vec<types::Canonical>>,

    /// What observations are required to perform this action
    pub observation_requirement: Option<Vec<types::Canonical>>,

    /// What observations must be produced by this action
    pub observation_result_requirement: Option<Vec<types::Canonical>>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,

    /// Dynamic aspects of the definition
    pub dynamic_value: Option<Vec<ActivityDefinitionDynamicValue>>,
}

/// Who should participate in the action being defined.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDefinitionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<types::Code>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent, etc
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,
}

/// Dynamic aspects of the definition that are used to customize the activity
/// when it is instantiated.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDefinitionDynamicValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The path to the element to be set dynamically
    pub path: types::String,

    /// An expression that provides the dynamic value for the customization
    pub expression: types::Expression,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ActivityDefinition;

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
