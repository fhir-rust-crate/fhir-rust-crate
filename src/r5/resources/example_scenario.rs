//! ExampleScenario
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExampleScenario
//!
//! Version: 5.0.0
//!
//! ExampleScenario Resource: A walkthrough of a workflow showing the interaction between systems and the instances shared, possibly including the evolution of instances over time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ExampleScenario
///
/// A walkthrough of a workflow showing the interaction between systems and the
/// instances shared, possibly including the evolution of instances over time.
/// It is used to describe expected system behaviour by illustrating actors,
/// example instances of data, and the ordered processes and steps through which
/// those instances are exchanged. This makes it a documentation and testing
/// artifact rather than something used directly in production data exchange.
///
/// In FHIR R5, `ExampleScenario` is a canonical, conformance resource used
/// mainly by implementation guide authors, tooling vendors, and reviewers to
/// illustrate how a set of exchanges should work end to end. An actor
/// (`ExampleScenarioActor`) represents a person or system taking part; an
/// instance (`ExampleScenarioInstance`) represents a piece of example data,
/// optionally with versions showing how it changes over time
/// (`ExampleScenarioInstanceVersion`); and a process (`ExampleScenarioProcess`)
/// groups the ordered steps (`ExampleScenarioProcessStep`) that move those
/// instances between actors, including nested processes, referenced workflows,
/// simple request/response operations (`ExampleScenarioProcessStepOperation`),
/// and alternative branches (`ExampleScenarioProcessStepAlternative`). Because
/// it is a narrative and testing artifact rather than clinical data, it is
/// typically published alongside profiles and capability statements in an
/// implementation guide rather than exchanged during routine patient care.
///
/// # Related resources
///
/// The example instances referenced within a scenario commonly represent other
/// FHIR resources, such as [`Patient`](crate::r5::resources::patient::Patient),
/// and scenario elements such as jurisdiction make use of shared data types
/// like [`CodeableConcept`](crate::r5::types::CodeableConcept). See also the
/// related conformance resources `StructureDefinition` and
/// `ImplementationGuide`, which `ExampleScenario` is often published alongside.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenario;
///
/// let value = ExampleScenario::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ExampleScenario = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario {
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

    /// Canonical identifier for this example scenario, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the example scenario
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the example scenario
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// To be removed?
    pub name: Option<types::String>,

    /// Name for this example scenario (human friendly)
    pub title: Option<types::String>,

    /// The publication lifecycle status of this scenario: draft | active | retired | unknown.
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description summarizing the purpose and content of the ExampleScenario.
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for example scenario (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// The purpose of the example, e.g. to illustrate a scenario
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// The people or systems (actors) that participate in and exchange data within the scenario.
    pub actor: Option<Vec<ExampleScenarioActor>>,

    /// The example data instances, and their versions over time, that are exchanged in the scenario.
    pub instance: Option<Vec<ExampleScenarioInstance>>,

    /// The major, ordered processes composed of steps that describe how the scenario unfolds.
    pub process: Option<Vec<ExampleScenarioProcess>>,
}

/// ExampleScenarioActor
///
/// A system or person who shares or receives an instance within the scenario.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// ID or acronym of the actor
    pub key: types::String,

    /// person | system
    pub r#type: types::Code,

    /// Label for actor when rendering
    pub title: types::String,

    /// Details about actor
    pub description: Option<types::Markdown>,
}

/// ExampleScenarioInstance
///
/// A single piece of example data used within the scenario, described by its
/// structure and optionally versioned over time.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// ID or acronym of the instance
    pub key: types::String,

    /// Data structure for example
    pub structure_type: types::Coding,

    /// E.g. 4.0.1
    pub structure_version: Option<types::String>,

    /// Rules instance adheres to
    pub structure_profile_canonical: Option<types::Canonical>,

    /// Rules instance adheres to
    pub structure_profile_uri: Option<types::Uri>,

    /// Label for instance
    pub title: types::String,

    /// Human-friendly description of the instance
    pub description: Option<types::Markdown>,

    /// Example instance data
    pub content: Option<types::Reference>,

    /// Snapshot of instance that changes
    pub version: Option<Vec<ExampleScenarioInstanceVersion>>,

    /// Resources contained in the instance
    pub contained_instance: Option<Vec<ExampleScenarioInstanceContainedInstance>>,
}

/// ExampleScenarioInstanceVersion
///
/// A specific snapshot of an instance representing how it appears at a
/// particular point in the workflow.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioInstanceVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// ID or acronym of the version
    pub key: types::String,

    /// Label for instance version
    pub title: types::String,

    /// Details about version
    pub description: Option<types::Markdown>,

    /// Example instance version data
    pub content: Option<types::Reference>,
}

/// ExampleScenarioInstanceContainedInstance
///
/// A reference from one instance to another instance (and optionally a specific
/// version of it) that it contains.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioInstanceContainedInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Key of contained instance
    pub instance_reference: types::String,

    /// Key of contained instance version
    pub version_reference: Option<types::String>,
}

/// ExampleScenarioProcess
///
/// A major process within the scenario, comprising an ordered set of steps and
/// the conditions that hold before and after it runs.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcess {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for procss
    pub title: types::String,

    /// Human-friendly description of the process
    pub description: Option<types::Markdown>,

    /// Status before process starts
    pub pre_conditions: Option<types::Markdown>,

    /// Status after successful completion
    pub post_conditions: Option<types::Markdown>,

    /// Event within of the process
    pub step: Option<Vec<ExampleScenarioProcessStep>>,
}

/// ExampleScenarioProcessStep
///
/// A single event within a process, which may itself be a nested process, a
/// referenced workflow, a simple operation, or a pause, and may carry
/// alternative branches.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcessStep {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Sequential number of the step
    pub number: Option<types::String>,

    /// Step is nested process
    pub process: Option<ExampleScenarioProcess>,

    /// Step is nested workflow
    pub workflow: Option<types::Canonical>,

    /// Step is simple action
    pub operation: Option<ExampleScenarioProcessStepOperation>,

    /// Alternate non-typical step action
    pub alternative: Option<Vec<ExampleScenarioProcessStepAlternative>>,

    /// Pause in the flow?
    pub pause: Option<types::Boolean>,
}

/// ExampleScenarioProcessStepOperation
///
/// A simple action within a step describing an exchange between an initiator and
/// a receiver, including the request and response instances involved.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcessStepOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Kind of action
    pub r#type: Option<types::Coding>,

    /// Label for step
    pub title: types::String,

    /// Who starts the operation
    pub initiator: Option<types::String>,

    /// Who receives the operation
    pub receiver: Option<types::String>,

    /// Human-friendly description of the operation
    pub description: Option<types::Markdown>,

    /// Initiator stays active?
    pub initiator_active: Option<types::Boolean>,

    /// Receiver stays active?
    pub receiver_active: Option<types::Boolean>,

    /// Instance transmitted on invocation
    pub request: Option<ExampleScenarioInstanceContainedInstance>,

    /// Instance transmitted on invocation response
    pub response: Option<ExampleScenarioInstanceContainedInstance>,
}

/// ExampleScenarioProcessStepAlternative
///
/// An alternate, non-typical set of steps that may occur in place of the main
/// step under described conditions.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcessStepAlternative {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Label for alternative
    pub title: types::String,

    /// Human-readable description of option
    pub description: Option<types::Markdown>,

    /// Alternative action(s)
    pub step: Option<Vec<ExampleScenarioProcessStep>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExampleScenario;

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
