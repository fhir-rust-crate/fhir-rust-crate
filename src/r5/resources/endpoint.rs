//! Endpoint
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Endpoint
//!
//! Version: 5.0.0
//!
//! Endpoint Resource: The technical details of an endpoint that can be used for electronic services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The technical details of an endpoint that can be used for electronic
/// services, such as for web services providing XDS.b, a REST endpoint for
/// another FHIR server, or a s/Mime email address. This may include any
/// security context information. In FHIR R5 an Endpoint describes how to
/// connect to a system, the protocols it uses, the payloads it accepts, and the
/// organization that manages it.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::endpoint::Endpoint;
///
/// let value = Endpoint::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Endpoint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
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

    /// Identifies this endpoint across multiple systems
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | suspended | error | off | entered-in-error | test
    pub status: types::Code,

    /// Protocol/Profile/Standard to be used with this endpoint connection
    pub connection_type: Vec<types::CodeableConcept>,

    /// A name that this endpoint can be identified by
    pub name: Option<types::String>,

    /// Additional details about the endpoint that could be displayed as further
    /// information to identify the description beyond its name
    pub description: Option<types::String>,

    /// The type of environment(s) exposed at this endpoint
    pub environment_type: Option<Vec<types::CodeableConcept>>,

    /// Organization that manages this endpoint (might not be the organization
    /// that exposes the endpoint)
    pub managing_organization: Option<types::Reference>,

    /// Contact details for source (e.g. troubleshooting)
    pub contact: Option<Vec<types::ContactPoint>>,

    /// Interval the endpoint is expected to be operational
    pub period: Option<types::Period>,

    /// Set of payloads that are provided by this endpoint
    pub payload: Option<Vec<EndpointPayload>>,

    /// The technical base address for connecting to this endpoint
    pub address: types::Url,

    /// Usage depends on the channel type
    pub header: Option<Vec<types::String>>,
}

/// Set of payloads that are provided by this endpoint. Each payload describes
/// the type of content and the MIME types that may be sent to or received from
/// the endpoint.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPayload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of content that may be used at this endpoint (e.g. XDS
    /// Discharge summaries)
    pub r#type: Option<Vec<types::CodeableConcept>>,

    /// Mimetype to send. If not specified, the content could be anything
    /// (including no payload, if the connectionType defined this)
    pub mime_type: Option<Vec<types::Code>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Endpoint;

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
