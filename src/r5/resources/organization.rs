//! Organization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Organization
//!
//! Version: 5.0.0
//!
//! Organization Resource: A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A formally or informally recognized grouping of people or organizations
/// formed for the purpose of achieving some form of collective action. This
/// includes companies, institutions, corporations, departments, community
/// groups, healthcare practice groups, payers/insurers, and so on. The
/// Organization resource is commonly referenced by many other FHIR resources
/// to describe the entity responsible for or associated with an activity.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::organization::Organization;
///
/// let value = Organization::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Organization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
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

    /// Identifies this organization across multiple systems
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether the organization's record is still in active use
    pub active: Option<types::Boolean>,

    /// Kind of organization
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Name used for the organization
    pub name: Option<types::String>,

    /// A list of alternate names that the organization is known as, or was known as in the past
    pub alias: Option<Vec<types::String>>,

    /// Additional details about the Organization that could be displayed as further information to identify the Organization beyond its name
    pub description: Option<types::Markdown>,

    /// Official contact details for the Organization
    pub contact: Option<Vec<types::ExtendedContactDetail>>,

    /// The organization of which this organization forms a part
    pub part_of: Option<types::Reference>,

    /// Technical endpoints providing access to services operated for the organization
    pub endpoint: Option<Vec<types::Reference>>,

    /// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
    pub qualification: Option<Vec<OrganizationQualification>>,
}

/// Qualifications, certifications, accreditations, licenses, training, etc.
/// pertaining to the provision of care by the organization.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationQualification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// An identifier for this qualification for the organization
    pub identifier: Option<Vec<types::Identifier>>,

    /// Coded representation of the qualification
    pub code: types::CodeableConcept,

    /// Period during which the qualification is valid
    pub period: Option<types::Period>,

    /// Organization that regulates and issues the qualification
    pub issuer: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Organization;

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
