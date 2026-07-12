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
use fhir_derive_macros::{Builder, Validate};

/// A formally or informally recognized grouping of people or organizations
/// formed for the purpose of achieving some form of collective action. This
/// includes companies, institutions, corporations, departments, community
/// groups, healthcare practice groups, payers and insurers, government agencies,
/// and so on.
///
/// In FHIR R5 the Organization resource serves an administrative role: it
/// represents the legal or operational entity that provides, pays for, regulates,
/// or is otherwise accountable for healthcare activities, rather than describing
/// a physical place or a single person. Organizations frequently form
/// hierarchies, where a department or division records its parent entity via the
/// `part_of` field, allowing a whole institution to be modeled as a tree of
/// related organizations. Because so many workflows need to name a responsible
/// entity, the Organization resource is one of the most widely referenced
/// resources in the specification.
///
/// # Related resources
///
/// An organization is commonly referenced as the managing or responsible entity
/// by resources such as [`Patient`](crate::r5::resources::patient::Patient) and
/// [`Practitioner`](crate::r5::resources::practitioner::Practitioner). The
/// distinct role that a person, organization, or device plays at a location is
/// captured by the `PractitionerRole` and `Location` resources. Coded values on
/// this resource, such as the organization `type`, use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and links to other
/// resources use [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::organization::Organization;
///
/// let value = Organization::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Organization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
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

    /// Business identifiers that identify this organization across multiple systems, such as tax, national provider, or license numbers.
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether the organization's record is still in active use; a false value marks records that should no longer be selected for new activity.
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The kind or category of organization, coded so systems can classify it, for example a healthcare provider, payer, department, or educational institution.
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// The public or human-readable name used to refer to the organization.
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// A list of alternate names that the organization is known as, or was known as in the past
    pub alias: Option<Vec<types::String>>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`).
    #[serde(rename = "_alias")]
    pub alias_ext: Option<Vec<Option<types::Element>>>,

    /// Additional details about the Organization that could be displayed as further information to identify the Organization beyond its name
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Official contact details for the Organization
    pub contact: Option<Vec<types::ExtendedContactDetail>>,

    /// A reference to the parent organization of which this organization forms a part, used to build institutional hierarchies of departments and divisions.
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
