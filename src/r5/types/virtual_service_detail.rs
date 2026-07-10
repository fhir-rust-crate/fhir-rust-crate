//! VirtualServiceDetail
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VirtualServiceDetail
//!
//! Version: 5.0.0
//!
//! VirtualServiceDetail Type: Virtual Service Contact Details.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// The VirtualServiceDetail datatype captures the contact details required to
/// connect to a virtual service, such as a video conference or telephone
/// conference. It describes the channel type (for example, a particular
/// teleconferencing product), the address or number used to reach the service,
/// and additional operational details like the maximum number of participants
/// and any session key needed to join. It is commonly used in scheduling,
/// appointment, and encounter resources to convey how a virtual meeting can be
/// accessed.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::types::virtual_service_detail::VirtualServiceDetail;
///
/// let value = VirtualServiceDetail::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: VirtualServiceDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VirtualServiceDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Channel Type
    pub channel_type: Option<types::Coding>,

    /// Contact address/number
    pub address_url: Option<types::Url>,

    /// Contact address/number
    pub address_string: Option<types::String>,

    /// Contact address/number
    pub address_contact_point: Option<types::ContactPoint>,

    /// Contact address/number
    pub address_extended_contact_detail: Option<types::ExtendedContactDetail>,

    /// Address to see alternative connection details
    pub additional_info: Option<Vec<types::Url>>,

    /// Maximum number of participants supported by the virtual service
    pub max_participants: Option<types::PositiveInt>,

    /// Session Key required by the virtual service
    pub session_key: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = VirtualServiceDetail;

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
