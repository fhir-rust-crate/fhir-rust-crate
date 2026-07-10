//! RegulatedAuthorization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization
//!
//! Version: 5.0.0
//!
//! RegulatedAuthorization Resource: Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Regulatory approval, clearance or licencing related to a regulated product,
/// treatment, facility or activity that is cited in a guidance, regulation, rule
/// or legislative act. An example is Market Authorization relating to a Medicinal
/// Product. This resource captures the authorizing body, subject, territory,
/// status and case/procedure information that governs the regulated authorization.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::regulated_authorization::RegulatedAuthorization;
///
/// let value = RegulatedAuthorization::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: RegulatedAuthorization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegulatedAuthorization {
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

    /// Business identifier for the authorization, typically assigned by the authorizing body
    pub identifier: Option<Vec<types::Identifier>>,

    /// The product type, treatment, facility or activity that is being authorized
    pub subject: Option<Vec<types::Reference>>,

    /// Overall type of this authorization, for example drug marketing approval, orphan drug designation
    pub r#type: Option<types::CodeableConcept>,

    /// General textual supporting information
    pub description: Option<types::Markdown>,

    /// The territory in which the authorization has been granted
    pub region: Option<Vec<types::CodeableConcept>>,

    /// The status that is authorised e.g. approved. Intermediate states can be tracked with cases and applications
    pub status: Option<types::CodeableConcept>,

    /// The date at which the current status was assigned
    pub status_date: Option<types::DateTime>,

    /// The time period in which the regulatory approval etc. is in effect, e.g. a Marketing Authorization includes the date of authorization and/or expiration date
    pub validity_period: Option<types::Period>,

    /// Condition for which the use of the regulated product applies
    pub indication: Option<Vec<types::CodeableReference>>,

    /// The intended use of the product, e.g. prevention, treatment
    pub intended_use: Option<types::CodeableConcept>,

    /// The legal/regulatory framework or reasons under which this authorization is granted
    pub basis: Option<Vec<types::CodeableConcept>>,

    /// The organization that has been granted this authorization, by the regulator
    pub holder: Option<types::Reference>,

    /// The regulatory authority or authorizing body granting the authorization
    pub regulator: Option<types::Reference>,

    /// Additional information or supporting documentation about the authorization
    pub attached_document: Option<Vec<types::Reference>>,

    /// The case or regulatory procedure for granting or amending a regulated authorization
    pub case: Option<RegulatedAuthorizationCase>,
}

/// The case or regulatory procedure for granting or amending a regulated
/// authorization. Note: This area is subject to ongoing review and the workgroup
/// is seeking implementer feedback on its use.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegulatedAuthorizationCase {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Identifier by which this case can be referenced
    pub identifier: Option<types::Identifier>,

    /// The defining type of case
    pub r#type: Option<types::CodeableConcept>,

    /// The status associated with the case
    pub status: Option<types::CodeableConcept>,

    /// Relevant date for this case
    pub date_period: Option<types::Period>,

    /// Relevant date for this case
    pub date_date_time: Option<types::DateTime>,

    /// Applications submitted to obtain a regulated authorization. Steps within the longer running case or procedure
    pub application: Option<Vec<RegulatedAuthorizationCase>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RegulatedAuthorization;

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
