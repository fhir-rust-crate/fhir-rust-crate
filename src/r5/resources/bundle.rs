//! Bundle
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Bundle
//!
//! Version: 5.0.0
//!
//! Bundle Resource: A container for a collection of resources.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// A container for a collection of resources.
///
/// A Bundle groups together a set of resources into a single instance for
/// transport, exchange, or storage. Bundles are used for many purposes in FHIR
/// R5, including documents, messages, transaction and batch operations and their
/// responses, search result sets, and collections of resources maintained
/// together as history. The `type` element declares which of these uses applies
/// and governs the expected content and processing of the entries.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::bundle::Bundle;
///
/// let value = Bundle::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Bundle = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,

    /// Language of the resource content
    pub language: Option<types::Code>,

    /// Persistent identifier for the bundle
    pub identifier: Option<types::Identifier>,

    /// document | message | transaction | transaction-response | batch |
    /// batch-response | history | searchset | collection |
    /// subscription-notification
    pub r#type: types::Code,

    /// When the bundle was assembled
    pub timestamp: Option<types::Instant>,

    /// If search, the total number of matches
    pub total: Option<types::UnsignedInt>,

    /// Links related to this Bundle
    pub link: Option<Vec<BundleLink>>,

    /// Entry in the bundle - will have a resource or information
    pub entry: Option<Vec<BundleEntry>>,

    /// Digital Signature
    pub signature: Option<types::Signature>,

    /// Issues with the Bundle
    pub issues: Option<::serde_json::Value>,
}

/// Links related to this Bundle.
///
/// A series of links that provide context to this bundle, such as pagination
/// links for a search result set.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BundleLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// See http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-1
    pub relation: types::Code,

    /// Reference details for the link
    pub url: types::Uri,
}

/// Entry in the bundle - will have a resource or information.
///
/// An entry in a bundle resource - will either contain a resource or information
/// about a resource (transactions and history only).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntry {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Links related to this entry
    pub link: Option<Vec<BundleLink>>,

    /// URI for resource (e.g. the absolute URL server address, URI for
    /// UUID/OID, etc.)
    pub full_url: Option<types::Uri>,

    /// A resource in the bundle
    pub resource: Option<::serde_json::Value>,

    /// Search related information
    pub search: Option<BundleEntrySearch>,

    /// Additional execution information (transaction/batch/history)
    pub request: Option<BundleEntryRequest>,

    /// Results of execution (transaction/batch/history)
    pub response: Option<BundleEntryResponse>,
}

/// Search related information.
///
/// Information about the search process that lead to the creation of this entry.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntrySearch {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// match | include - why this is in the result set
    pub mode: Option<types::Code>,

    /// Search ranking (between 0 and 1)
    pub score: Option<types::Decimal>,
}

/// Additional execution information (transaction/batch/history).
///
/// Additional information about how this entry should be processed as part of a
/// transaction or batch. For history, it shows how the entry was processed to
/// create the version contained in the entry.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntryRequest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// GET | HEAD | POST | PUT | DELETE | PATCH
    pub method: types::Code,

    /// URL for HTTP equivalent of this entry
    pub url: types::Uri,

    /// For managing cache validation
    pub if_none_match: Option<types::String>,

    /// For managing cache currency
    pub if_modified_since: Option<types::Instant>,

    /// For managing update contention
    pub if_match: Option<types::String>,

    /// For conditional creates
    pub if_none_exist: Option<types::String>,
}

/// Results of execution (transaction/batch/history).
///
/// Indicates the results of processing the corresponding 'request' entry in the
/// batch or transaction being responded to or what the results of an operation
/// where when returning history.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntryResponse {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Status response code (text optional)
    pub status: types::String,

    /// The location (if the operation returns a location)
    pub location: Option<types::Uri>,

    /// The Etag for the resource (if relevant)
    pub etag: Option<types::String>,

    /// Server's date time modified
    pub last_modified: Option<types::Instant>,

    /// OperationOutcome with hints and warnings (for batch/transaction)
    pub outcome: Option<::serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Bundle;

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
