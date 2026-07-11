//! Subscription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Subscription
//!
//! Version: 5.0.0
//!
//! Subscription Resource: The subscription resource describes a particular client's request to be notified about a SubscriptionTopic.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Subscription resource describes a particular client's request to be
/// notified about a SubscriptionTopic. A Subscription is a persistent record
/// that identifies a topic of interest, optional filters to narrow the stream
/// of events, and a notification channel (such as a REST hook, WebSocket, email,
/// or messaging endpoint) through which the server delivers notifications when
/// matching events occur.
///
/// Administratively, a Subscription represents an agreement between a client
/// and a server: the client asks to be told when resources matching the
/// referenced topic change, and the server manages the subscription's
/// lifecycle (`requested`, `active`, `error`, `off`, or `entered-in-error`),
/// delivering periodic heartbeat and handshake notifications as needed to
/// confirm the channel is healthy. This is the R5 successor to the R4
/// subscription mechanism, decoupling the general subscription request from
/// the topic-specific trigger criteria defined by a `SubscriptionTopic`.
/// Because the events delivered may reference clinical resources such as
/// [`Patient`](crate::r5::resources::patient::Patient) records, servers
/// typically apply the same security and consent constraints to
/// notifications as they would to a direct read of those resources.
///
/// # See also
///
/// - `SubscriptionTopic` — defines the event trigger criteria a Subscription refers to via `topic`.
/// - `SubscriptionStatus` — the resource used to report handshake, heartbeat, and event notifications.
/// - [`Reference`](crate::r5::types::Reference) — used for `managing_entity`.
/// - [`Coding`](crate::r5::types::Coding) — used for `channel_type`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription::Subscription;
///
/// let value = Subscription::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Subscription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
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

    /// Additional identifiers (business identifier)
    pub identifier: Option<Vec<types::Identifier>>,

    /// Human readable name for this subscription
    pub name: Option<types::String>,

    /// Current lifecycle state of the subscription: requested | active | error | off | entered-in-error
    pub status: types::Code,

    /// Canonical URL of the `SubscriptionTopic` that defines which events this subscription reacts to
    pub topic: types::Canonical,

    /// Contact details for source (e.g. troubleshooting)
    pub contact: Option<Vec<types::ContactPoint>>,

    /// When to automatically delete the subscription
    pub end: Option<types::Instant>,

    /// Reference to the party (e.g. an Organization or Practitioner) responsible for Subscription changes
    pub managing_entity: Option<types::Reference>,

    /// Description of why this subscription was created
    pub reason: Option<types::String>,

    /// Criteria for narrowing the subscription topic stream
    pub filter_by: Option<Vec<SubscriptionFilterBy>>,

    /// Coded type of notification channel, such as a REST hook, WebSocket, email, or messaging endpoint
    pub channel_type: types::Coding,

    /// URL or address that the notification channel points to (meaning depends on `channel_type`)
    pub endpoint: Option<types::Url>,

    /// Channel type
    pub parameter: Option<Vec<SubscriptionParameter>>,

    /// Interval in seconds to send 'heartbeat' notification
    pub heartbeat_period: Option<types::UnsignedInt>,

    /// Timeout in seconds to attempt notification delivery
    pub timeout: Option<types::UnsignedInt>,

    /// MIME type to send, or omit for no payload
    pub content_type: Option<types::Code>,

    /// empty | id-only | full-resource
    pub content: Option<types::Code>,

    /// Maximum number of events that can be combined in a single notification
    pub max_count: Option<types::PositiveInt>,
}

/// Criteria for narrowing the subscription topic stream. Each filter references
/// a filter parameter defined in the referenced SubscriptionTopic and applies a
/// comparator, modifier, and value to restrict the events that trigger a
/// notification.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionFilterBy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Allowed Resource (reference to definition) for this Subscription filter
    pub resource_type: Option<types::Uri>,

    /// Filter label defined in SubscriptionTopic
    pub filter_parameter: types::String,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<types::Code>,

    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<types::Code>,

    /// Literal value or resource path
    pub value: types::String,
}

/// Channel-type-specific parameter used to configure the notification channel.
/// Each parameter is a name/value pair whose meaning is defined by the channel
/// type (for example, an HTTP header for a REST hook channel).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Name (key) of the parameter
    pub name: types::String,

    /// Value of the parameter to use or pass through
    pub value: types::String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Subscription;

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
