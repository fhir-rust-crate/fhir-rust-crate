//! VerificationResult
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VerificationResult
//!
//! Version: 5.0.0
//!
//! VerificationResult Resource: Describes validation requirements, source(s), status and dates for one or more elements.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// VerificationResult
///
/// Describes validation requirements, source(s), status and dates for one or
/// more elements. It records how a piece of information (the target) was
/// validated, who validated it, against which primary sources, and when
/// revalidation is due. This resource is commonly used in provider directory
/// and credentialing workflows to track the trustworthiness of data.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::verification_result::VerificationResult;
///
/// let value = VerificationResult::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: VerificationResult = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResult {
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

    /// A resource that was validated
    pub target: Option<Vec<types::Reference>>,

    /// The fhirpath location(s) within the resource that was validated
    pub target_location: Option<Vec<types::String>>,

    /// none | initial | periodic
    pub need: Option<types::CodeableConcept>,

    /// attested | validated | in-process | req-revalid | val-fail | reval-fail | entered-in-error
    pub status: types::Code,

    /// When the validation status was updated
    pub status_date: Option<types::DateTime>,

    /// nothing | primary | multiple
    pub validation_type: Option<types::CodeableConcept>,

    /// The primary process by which the target is validated (edit check; value set; primary source; multiple sources; standalone; in context)
    pub validation_process: Option<Vec<types::CodeableConcept>>,

    /// Frequency of revalidation
    pub frequency: Option<types::Timing>,

    /// The date/time validation was last completed (including failed validations)
    pub last_performed: Option<types::DateTime>,

    /// The date when target is next validated, if appropriate
    pub next_scheduled: Option<types::Date>,

    /// fatal | warn | rec-only | none
    pub failure_action: Option<types::CodeableConcept>,

    /// Information about the primary source(s) involved in validation
    pub primary_source: Option<Vec<VerificationResultPrimarySource>>,

    /// Information about the entity attesting to information
    pub attestation: Option<VerificationResultAttestation>,

    /// Information about the entity validating information
    pub validator: Option<Vec<VerificationResultValidator>>,
}

/// Information about the primary source(s) involved in validation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResultPrimarySource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to the primary source
    pub who: Option<types::Reference>,

    /// Type of primary source (License Board; Primary Education; Continuing Education; Postal Service; Relationship owner; Registration Authority; legal source; issuing source; authoritative source)
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Method for exchanging information with the primary source
    pub communication_method: Option<Vec<types::CodeableConcept>>,

    /// successful | failed | unknown
    pub validation_status: Option<types::CodeableConcept>,

    /// When the target was validated against the primary source
    pub validation_date: Option<types::DateTime>,

    /// yes | no | undetermined
    pub can_push_updates: Option<types::CodeableConcept>,

    /// specific | any | source
    pub push_type_available: Option<Vec<types::CodeableConcept>>,
}

/// Information about the entity attesting to information.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResultAttestation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The individual or organization attesting to information
    pub who: Option<types::Reference>,

    /// When the who is asserting on behalf of another (organization or individual)
    pub on_behalf_of: Option<types::Reference>,

    /// The method by which attested information was submitted/retrieved
    pub communication_method: Option<types::CodeableConcept>,

    /// The date the information was attested to
    pub date: Option<types::Date>,

    /// A digital identity certificate associated with the attestation source
    pub source_identity_certificate: Option<types::String>,

    /// A digital identity certificate associated with the proxy entity submitting attested information on behalf of the attestation source
    pub proxy_identity_certificate: Option<types::String>,

    /// Proxy signature (digital or image)
    pub proxy_signature: Option<types::Signature>,

    /// Attester signature (digital or image)
    pub source_signature: Option<types::Signature>,
}

/// Information about the entity validating information.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResultValidator {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to the organization validating information
    pub organization: types::Reference,

    /// A digital identity certificate associated with the validator
    pub identity_certificate: Option<types::String>,

    /// Validator signature (digital or image)
    pub attestation_signature: Option<types::Signature>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = VerificationResult;

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
