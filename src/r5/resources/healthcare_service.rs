//! HealthcareService
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
//!
//! Version: 5.0.0
//!
//! HealthcareService Resource: The details of a healthcare service available at a location or in a catalog.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The details of a healthcare service available at a location or in a catalog.
///
/// In the case where there is a hierarchy of services (for example, Lab ->
/// Pathology -> Wound Cultures), this can be represented using a set of linked
/// HealthcareServices. It describes the type of service, where it is offered,
/// who provides it, and the conditions and availability under which the service
/// can be accessed.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::healthcare_service::HealthcareService;
///
/// let value = HealthcareService::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: HealthcareService = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareService {
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

    /// External identifiers for this item
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this HealthcareService record is in active use
    pub active: Option<types::Boolean>,

    /// Organization that provides this service
    pub provided_by: Option<types::Reference>,

    /// The service within which this service is offered
    pub offered_in: Option<Vec<types::Reference>>,

    /// Broad category of service being performed or delivered
    pub category: Option<Vec<types::CodeableConcept>>,

    /// Type of service that may be delivered or performed
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Specialties handled by the HealthcareService
    pub specialty: Option<Vec<types::CodeableConcept>>,

    /// Location(s) where service may be provided
    pub location: Option<Vec<types::Reference>>,

    /// Description of service as presented to a consumer while searching
    pub name: Option<types::String>,

    /// Additional description and/or any specific issues not covered elsewhere
    pub comment: Option<types::Markdown>,

    /// Extra details about the service that can't be placed in the other fields
    pub extra_details: Option<types::Markdown>,

    /// Facilitates quick identification of the service
    pub photo: Option<types::Attachment>,

    /// Official contact details for the HealthcareService
    pub contact: Option<Vec<types::ExtendedContactDetail>>,

    /// Location(s) service is intended for/available to
    pub coverage_area: Option<Vec<types::Reference>>,

    /// Conditions under which service is available/offered
    pub service_provision_code: Option<Vec<types::CodeableConcept>>,

    /// Specific eligibility requirements required to use the service
    pub eligibility: Option<Vec<HealthcareServiceEligibility>>,

    /// Programs that this service is applicable to
    pub program: Option<Vec<types::CodeableConcept>>,

    /// Collection of characteristics (attributes)
    pub characteristic: Option<Vec<types::CodeableConcept>>,

    /// The language that this service is offered in
    pub communication: Option<Vec<types::CodeableConcept>>,

    /// Ways that the service accepts referrals
    pub referral_method: Option<Vec<types::CodeableConcept>>,

    /// If an appointment is required for access to this service
    pub appointment_required: Option<types::Boolean>,

    /// Times the healthcare service is available (including exceptions)
    pub availability: Option<Vec<types::Availability>>,

    /// Technical endpoints providing access to electronic services operated for the healthcare service
    pub endpoint: Option<Vec<types::Reference>>,
}

/// Specific eligibility requirements required to use the service.
///
/// Does this service have specific eligibility requirements that need to be met
/// in order to use the service?
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareServiceEligibility {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Coded value for the eligibility
    pub code: Option<types::CodeableConcept>,

    /// Describes the eligibility conditions for the service
    pub comment: Option<types::Markdown>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = HealthcareService;

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
