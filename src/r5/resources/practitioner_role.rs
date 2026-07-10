//! PractitionerRole
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
//!
//! Version: 5.0.0
//!
//! PractitionerRole Resource: A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A specific set of Roles/Locations/specialties/services that a practitioner
/// may perform, or has performed at an organization during a period of time.
/// This resource links a Practitioner to an Organization and describes the
/// roles, specialties, locations, healthcare services, and availability under
/// which the practitioner acts. It is commonly used in provider directories and
/// scheduling to determine who can perform what, where, and when.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::practitioner_role::PractitionerRole;
///
/// let value = PractitionerRole::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PractitionerRole = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerRole {
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

    /// Identifiers for a role/location
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this practitioner role record is in active use
    pub active: Option<types::Boolean>,

    /// The period during which the practitioner is authorized to perform in these role(s)
    pub period: Option<types::Period>,

    /// Practitioner that provides services for the organization
    pub practitioner: Option<types::Reference>,

    /// Organization where the roles are available
    pub organization: Option<types::Reference>,

    /// Roles which this practitioner may perform
    pub code: Option<Vec<types::CodeableConcept>>,

    /// Specific specialty of the practitioner
    pub specialty: Option<Vec<types::CodeableConcept>>,

    /// Location(s) where the practitioner provides care
    pub location: Option<Vec<types::Reference>>,

    /// Healthcare services provided for this role's Organization/Location(s)
    pub healthcare_service: Option<Vec<types::Reference>>,

    /// Official contact details relating to this PractitionerRole
    pub contact: Option<Vec<types::ExtendedContactDetail>>,

    /// Collection of characteristics (attributes)
    pub characteristic: Option<Vec<types::CodeableConcept>>,

    /// A language the practitioner (in this role) can use in patient communication
    pub communication: Option<Vec<types::CodeableConcept>>,

    /// Times the Practitioner is available at this location and/or healthcare service (including exceptions)
    pub availability: Option<Vec<types::Availability>>,

    /// Endpoints for interacting with the practitioner in this role
    pub endpoint: Option<Vec<types::Reference>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PractitionerRole;

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
