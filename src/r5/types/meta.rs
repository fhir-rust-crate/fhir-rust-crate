//! Meta
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Meta
//!
//! Version: 5.0.0
//!
//! Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The metadata about a resource. This is content in the resource that is
/// maintained by the infrastructure. Changes to the content might not always be
/// associated with version changes to the resource. In FHIR R5, `Meta` carries
/// system-managed information such as the version identifier, the last-updated
/// timestamp, the source of the resource, conformance profiles, and security and
/// tag labels.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::meta::Meta;
///
/// let value = Meta::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Meta = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Version specific identifier
    pub version_id: Option<types::Id>,

    /// When the resource version last changed
    pub last_updated: Option<types::Instant>,

    /// Identifies where the resource comes from
    pub source: Option<types::Uri>,

    /// Profiles this resource claims to conform to
    pub profile: Option<Vec<types::Canonical>>,

    /// Security Labels applied to this resource
    pub security: Option<Vec<types::Coding>>,

    /// Tags applied to this resource
    pub tag: Option<Vec<types::Coding>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Meta;

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
