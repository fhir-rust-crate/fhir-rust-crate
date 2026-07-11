//! TestScript
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestScript
//!
//! Version: 5.0.0
//!
//! TestScript Resource: A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
///
/// TestScript is a conformance resource that describes an executable suite of
/// tests, along with the fixtures, variables, and required server capabilities
/// needed to run them. It organizes work into optional setup, one or more
/// tests, and an optional teardown, where each action is either an operation
/// invoked against a server or an assertion checked against a response. In FHIR
/// R5 it is typically paired with TestReport, which records the outcome of
/// executing a TestScript.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScript;
///
/// let value = TestScript::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScript = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScript {
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

    /// Canonical identifier for this test script, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Additional identifier for the test script
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the test script
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this test script (computer friendly)
    pub name: types::String,

    /// Name for this test script (human friendly)
    pub title: Option<types::String>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the test script
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for test script (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this test script is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// An abstract server representing a client or sender in a message exchange
    pub origin: Option<Vec<TestScriptOrigin>>,

    /// An abstract server representing a destination or receiver in a message exchange
    pub destination: Option<Vec<TestScriptDestination>>,

    /// Required capability that is assumed to function correctly on the FHIR server being tested
    pub metadata: Option<TestScriptMetadata>,

    /// Indication of the artifact(s) that are tested by this test case
    pub scope: Option<Vec<TestScriptScope>>,

    /// Fixture in the test script - by reference (uri)
    pub fixture: Option<Vec<TestScriptFixture>>,

    /// Reference of the validation profile
    pub profile: Option<Vec<types::Canonical>>,

    /// Placeholder for evaluated elements
    pub variable: Option<Vec<TestScriptVariable>>,

    /// A series of required setup operations before tests are executed
    pub setup: Option<TestScriptSetup>,

    /// A test in this script
    pub test: Option<Vec<TestScriptTest>>,

    /// A series of required clean up steps
    pub teardown: Option<TestScriptTeardown>,
}

/// An abstract server representing a client or sender in a message exchange.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptOrigin {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The index of the abstract origin server starting at 1
    pub index: types::Integer,

    /// FHIR-Client | FHIR-SDC-FormFiller
    pub profile: types::Coding,

    /// The url path of the origin server
    pub url: Option<types::Url>,
}

/// An abstract server representing a destination or receiver in a message exchange.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptDestination {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The index of the abstract destination server starting at 1
    pub index: types::Integer,

    /// FHIR-Server | FHIR-SDC-FormManager | FHIR-SDC-FormReceiver | FHIR-SDC-FormProcessor
    pub profile: types::Coding,

    /// The url path of the destination server
    pub url: Option<types::Url>,
}

/// Required capability that is assumed to function correctly on the FHIR server being tested.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptMetadata {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Links to the FHIR specification
    pub link: Option<Vec<TestScriptMetadataLink>>,

    /// Capabilities  that are assumed to function correctly on the FHIR server being tested
    pub capability: Vec<TestScriptMetadataCapability>,
}

/// Links to the FHIR specification that describe the capabilities being tested.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptMetadataLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// URL to the specification
    pub url: types::Uri,

    /// Short description
    pub description: Option<types::String>,
}

/// Capabilities that are assumed to function correctly on the FHIR server being tested.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptMetadataCapability {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Are the capabilities required?
    pub required: types::Boolean,

    /// Are the capabilities validated?
    pub validated: types::Boolean,

    /// The expected capabilities of the server
    pub description: Option<types::String>,

    /// Which origin server these requirements apply to
    pub origin: Option<Vec<types::Integer>>,

    /// Which server these requirements apply to
    pub destination: Option<types::Integer>,

    /// Links to the FHIR specification
    pub link: Option<Vec<types::Uri>>,

    /// Required Capability Statement
    pub capabilities: types::Canonical,
}

/// Indication of the artifact(s) that are tested by this test case.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptScope {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The specific conformance artifact being tested
    pub artifact: types::Canonical,

    /// required | optional | strict
    pub conformance: Option<types::CodeableConcept>,

    /// unit | integration | production
    pub phase: Option<types::CodeableConcept>,
}

/// Fixture in the test script - by reference (uri).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptFixture {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Whether or not to implicitly create the fixture during setup
    pub autocreate: types::Boolean,

    /// Whether or not to implicitly delete the fixture during teardown
    pub autodelete: types::Boolean,

    /// Reference of the resource
    pub resource: Option<types::Reference>,
}

/// Placeholder for evaluated elements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptVariable {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Descriptive name for this variable
    pub name: types::String,

    /// Default, hard-coded, or user-defined value for this variable
    pub default_value: Option<types::String>,

    /// Natural language description of the variable
    pub description: Option<types::String>,

    /// The FHIRPath expression against the fixture body
    pub expression: Option<types::String>,

    /// HTTP header field name for source
    pub header_field: Option<types::String>,

    /// Hint help text for default value to enter
    pub hint: Option<types::String>,

    /// XPath or JSONPath against the fixture body
    pub path: Option<types::String>,

    /// Fixture Id of source expression or headerField within this variable
    pub source_id: Option<types::Id>,
}

/// A series of required setup operations before tests are executed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A setup operation or assert to perform
    pub action: Vec<TestScriptSetupAction>,
}

/// A setup operation or assert to perform.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// The setup operation to perform.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The operation code type that will be executed
    pub r#type: Option<types::Coding>,

    /// Resource type
    pub resource: Option<types::Uri>,

    /// Tracking/logging operation label
    pub label: Option<types::String>,

    /// Tracking/reporting operation description
    pub description: Option<types::String>,

    /// Mime type to accept in the payload of the response, with charset etc
    pub accept: Option<types::Code>,

    /// Mime type of the request payload contents, with charset etc
    pub content_type: Option<types::Code>,

    /// Server responding to the request
    pub destination: Option<types::Integer>,

    /// Whether or not to send the request url in encoded format
    pub encode_request_url: types::Boolean,

    /// delete | get | options | patch | post | put | head
    pub method: Option<types::Code>,

    /// Server initiating the request
    pub origin: Option<types::Integer>,

    /// Explicitly defined path parameters
    pub params: Option<types::String>,

    /// Each operation can have one or more header elements
    pub request_header: Option<Vec<TestScriptSetupActionOperationRequestHeader>>,

    /// Fixture Id of mapped request
    pub request_id: Option<types::Id>,

    /// Fixture Id of mapped response
    pub response_id: Option<types::Id>,

    /// Fixture Id of body for PUT and POST requests
    pub source_id: Option<types::Id>,

    /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests
    pub target_id: Option<types::Id>,

    /// Request URL
    pub url: Option<types::String>,
}

/// Each operation can have one or more header elements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionOperationRequestHeader {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// HTTP header field name
    pub field: types::String,

    /// HTTP headerfield value
    pub value: types::String,
}

/// The assertion to perform.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionAssert {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Tracking/logging assertion label
    pub label: Option<types::String>,

    /// Tracking/reporting assertion description
    pub description: Option<types::String>,

    /// response | request
    pub direction: Option<types::Code>,

    /// Id of the source fixture to be evaluated
    pub compare_to_source_id: Option<types::String>,

    /// The FHIRPath expression to evaluate against the source fixture
    pub compare_to_source_expression: Option<types::String>,

    /// XPath or JSONPath expression to evaluate against the source fixture
    pub compare_to_source_path: Option<types::String>,

    /// Mime type to compare against the 'Content-Type' header
    pub content_type: Option<types::Code>,

    /// fail | pass | skip | stop
    pub default_manual_completion: Option<types::Code>,

    /// The FHIRPath expression to be evaluated
    pub expression: Option<types::String>,

    /// HTTP header field name
    pub header_field: Option<types::String>,

    /// Fixture Id of minimum content resource
    pub minimum_id: Option<types::String>,

    /// Perform validation on navigation links?
    pub navigation_links: Option<types::Boolean>,

    /// equals | notEquals | in | notIn | greaterThan | lessThan | empty | notEmpty | contains | notContains | eval | manualEval
    pub operator: Option<types::Code>,

    /// XPath or JSONPath expression
    pub path: Option<types::String>,

    /// delete | get | options | patch | post | put | head
    pub request_method: Option<types::Code>,

    /// Request URL comparison value
    pub request_url: Option<types::String>,

    /// Resource type
    pub resource: Option<types::Uri>,

    /// HTTP response status code family
    pub response: Option<types::Code>,

    /// HTTP response code to test
    pub response_code: Option<types::String>,

    /// Fixture Id of source expression or headerField
    pub source_id: Option<types::Id>,

    /// If this assert fails, will the current test execution stop?
    pub stop_test_on_fail: types::Boolean,

    /// Profile Id of validation profile reference
    pub validate_profile_id: Option<types::Id>,

    /// The value to compare to
    pub value: Option<types::String>,

    /// Will this assert produce a warning only on error?
    pub warning_only: types::Boolean,

    /// Links or references to the testing requirements
    pub requirement: Option<Vec<TestScriptSetupActionAssertRequirement>>,
}

/// Links or references to the testing requirements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetupActionAssertRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Link or reference to the testing requirement
    pub link_uri: Option<types::Uri>,

    /// Link or reference to the testing requirement
    pub link_canonical: Option<types::Canonical>,
}

/// A test in this script.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Tracking/logging name of this test
    pub name: Option<types::String>,

    /// Tracking/reporting short description of the test
    pub description: Option<types::String>,

    /// A test operation or assert to perform
    pub action: Vec<TestScriptTestAction>,
}

/// A test operation or assert to perform.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTestAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The setup assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// A series of required clean up steps.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTeardown {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// One or more teardown operations to perform
    pub action: Vec<TestScriptTeardownAction>,
}

/// One or more teardown operations to perform.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptTeardownAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The teardown operation to perform
    pub operation: TestScriptSetupActionOperation,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TestScript;

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
