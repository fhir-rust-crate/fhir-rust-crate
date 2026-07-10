//! OrganizationAffiliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OrganizationAffiliation
//!
//! Version: 5.0.0
//!
//! OrganizationAffiliation Resource: Defines an affiliation/assotiation/relationship between 2 distinct organizations, that is not a part-of relationship/sub-division relationship.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// OrganizationAffiliation describes a relationship between two distinct
/// organizations that is not a part-of/sub-division relationship. It captures
/// the role that a participating organization plays with respect to a primary
/// organization, such as providing services, being a member of a network, or
/// covering particular locations, specialties, and healthcare services.
///
/// It is commonly used in provider directories to model the associations
/// between organizations, the networks they participate in, and the contact
/// and endpoint details relevant to those affiliations.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::organization_affiliation::OrganizationAffiliation;
///
/// let value = OrganizationAffiliation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: OrganizationAffiliation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAffiliation {
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

    /// Business identifiers that are specific to this role
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this organization affiliation record is in active use
    pub active: Option<types::Boolean>,

    /// The period during which the participatingOrganization is affiliated with the primary organization
    pub period: Option<types::Period>,

    /// Organization where the role is available
    pub organization: Option<types::Reference>,

    /// Organization that provides/performs the role (e.g. providing services or is a member of)
    pub participating_organization: Option<types::Reference>,

    /// The network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)
    pub network: Option<Vec<types::Reference>>,

    /// Definition of the role the participatingOrganization plays
    pub code: Option<Vec<types::CodeableConcept>>,

    /// Specific specialty of the participatingOrganization in the context of the role
    pub specialty: Option<Vec<types::CodeableConcept>>,

    /// The location(s) at which the role occurs
    pub location: Option<Vec<types::Reference>>,

    /// Healthcare services provided through the role
    pub healthcare_service: Option<Vec<types::Reference>>,

    /// Official contact details at the participatingOrganization relevant to this Affiliation
    pub contact: Option<Vec<types::ExtendedContactDetail>>,

    /// Technical endpoints providing access to services operated for this role
    pub endpoint: Option<Vec<types::Reference>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = OrganizationAffiliation;

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
