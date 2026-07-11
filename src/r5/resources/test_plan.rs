//! TestPlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestPlan
//!
//! Version: 5.0.0
//!
//! TestPlan Resource: A plan for executing testing on an artifact or specifications
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// TestPlan
///
/// A plan for executing testing on an artifact or specifications. It describes
/// the intended scope, the required preconditions, the test cases with their
/// data and assertions, and the exit criteria for considering the plan
/// successfully executed. TestPlan is typically used to document and coordinate
/// conformance, acceptance, unit, or performance testing of FHIR resources and
/// other specifications. It captures the overall test strategy for an artifact
/// such as an implementation guide, capability statement, or profile, including
/// the tools required, the dependencies that must be satisfied before testing
/// begins, and the individual test cases (each with its own scope, test data,
/// test run instructions, and assertions) that together demonstrate conformance.
/// A TestPlan is often referenced by, or paired with, one or more TestScript
/// resources that provide the executable, machine-readable steps for a given
/// test case.
///
/// See also: [`CodeableConcept`](crate::r5::types::CodeableConcept) for
/// categorizing the plan, and [`Reference`](crate::r5::types::Reference) for
/// linking to the scoped artifacts, predecessor test plans, or `TestScript`
/// resources under test.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlan;
///
/// let value = TestPlan::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestPlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlan {
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

    /// Canonical identifier for this test plan, represented as a URI (globally unique)
    pub url: Option<types::Uri>,

    /// Business identifier identifier for the test plan
    pub identifier: Option<Vec<types::Identifier>>,

    /// Business version of the test plan
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this test plan (computer friendly)
    pub name: Option<types::String>,

    /// Name for this test plan (human friendly)
    pub title: Option<types::String>,

    /// The publication status of this test plan: draft | active | retired | unknown
    pub status: types::Code,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,

    /// Date last changed
    pub date: Option<types::DateTime>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the test plan
    pub description: Option<types::Markdown>,

    /// The context that the content is intended to support
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction where the test plan applies (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this test plan is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// The category of the Test Plan, e.g. acceptance, unit, or performance testing
    pub category: Option<Vec<types::CodeableConcept>>,

    /// The artifact(s) under test, referenced via a conformance resource, narrative criteria, or an external reference
    pub scope: Option<Vec<types::Reference>>,

    /// A description of test tools to be used in the test plan - narrative for now
    pub test_tools: Option<types::Markdown>,

    /// The required criteria to execute the test plan - e.g. preconditions, previous tests
    pub dependency: Option<Vec<TestPlanDependency>>,

    /// The threshold or criteria for the test plan to be considered successfully executed - narrative
    pub exit_criteria: Option<types::Markdown>,

    /// The individual test cases, each with its own scope, dependencies, test run, data, and assertions, that constitute this plan
    pub test_case: Option<Vec<TestPlanTestCase>>,
}

/// TestPlan.dependency
///
/// The required criteria to execute the test plan - e.g. preconditions,
/// previous tests.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanDependency {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Description of the dependency criterium
    pub description: Option<types::Markdown>,

    /// Link to predecessor test plans
    pub predecessor: Option<types::Reference>,
}

/// TestPlan.testCase
///
/// The test cases that constitute this plan.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCase {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Sequence of test case in the test plan
    pub sequence: Option<types::Integer>,

    /// The scope or artifact covered by the case
    pub scope: Option<Vec<types::Reference>>,

    /// Required criteria to execute the test case
    pub dependency: Option<Vec<TestPlanTestCaseDependency>>,

    /// The actual test to be executed
    pub test_run: Option<Vec<TestPlanTestCaseTestRun>>,

    /// The test data used in the test case
    pub test_data: Option<Vec<TestPlanTestCaseTestData>>,

    /// Test assertions or expectations
    pub assertion: Option<Vec<TestPlanTestCaseAssertion>>,
}

/// TestPlan.testCase.dependency
///
/// Required criteria to execute the test case.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseDependency {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Description of the criteria
    pub description: Option<types::Markdown>,

    /// Link to predecessor test plans
    pub predecessor: Option<types::Reference>,
}

/// TestPlan.testCase.testRun
///
/// The actual test to be executed.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseTestRun {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The narrative description of the tests
    pub narrative: Option<types::Markdown>,

    /// The test cases in a structured language e.g. gherkin, Postman, or FHIR TestScript
    pub script: Option<TestPlanTestCaseTestRunScript>,
}

/// TestPlan.testCase.testRun.script
///
/// The test cases in a structured language e.g. gherkin, Postman, or FHIR
/// TestScript.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseTestRunScript {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The language for the test cases e.g. 'gherkin', 'testscript'
    pub language: Option<types::CodeableConcept>,

    /// The actual content of the cases - references to TestScripts or externally defined content
    pub source_string: Option<types::String>,

    /// The actual content of the cases - references to TestScripts or externally defined content
    pub source_reference: Option<types::Reference>,
}

/// TestPlan.testCase.testData
///
/// The test data used in the test case.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseTestData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of test data description, e.g. 'synthea'
    pub r#type: types::Coding,

    /// The actual test resources when they exist
    pub content: Option<types::Reference>,

    /// Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc
    pub source_string: Option<types::String>,

    /// Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc
    pub source_reference: Option<types::Reference>,
}

/// TestPlan.testCase.assertion
///
/// Test assertions or expectations.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseAssertion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Assertion type - for example 'informative' or 'required'
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// The focus or object of the assertion
    pub object: Option<Vec<types::CodeableReference>>,

    /// The actual result assertion
    pub result: Option<Vec<types::CodeableReference>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TestPlan;

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
