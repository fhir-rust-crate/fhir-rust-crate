//! PaymentNotice
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentNotice
//!
//! Version: 5.0.0
//!
//! PaymentNotice Resource: This resource provides the status of the payment for goods and services rendered, and the request and response resource references.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// This resource provides the status of the payment for goods and services
/// rendered.
///
/// PaymentNotice is a financial resource used to indicate to a payer or other
/// interested party that a payment has been, or is expected to be, made. It
/// references the originating request and response resources and reports the
/// monetary amount, the party being paid, and the party being notified. It is
/// commonly exchanged between providers, payers, and clearinghouses to convey
/// the payment status for a claim or other financial transaction.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::payment_notice::PaymentNotice;
///
/// let value = PaymentNotice::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PaymentNotice = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentNotice {
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

    /// Business Identifier for the payment notice
    pub identifier: Option<Vec<types::Identifier>>,

    /// active | cancelled | draft | entered-in-error
    pub status: types::Code,

    /// Request reference
    pub request: Option<types::Reference>,

    /// Response reference
    pub response: Option<types::Reference>,

    /// Creation date
    pub created: types::DateTime,

    /// Responsible practitioner
    pub reporter: Option<types::Reference>,

    /// Payment reference
    pub payment: Option<types::Reference>,

    /// Payment or clearing date
    pub payment_date: Option<types::Date>,

    /// Party being paid
    pub payee: Option<types::Reference>,

    /// Party being notified
    pub recipient: types::Reference,

    /// Monetary amount of the payment
    pub amount: types::Money,

    /// Issued or cleared Status of the payment
    pub payment_status: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PaymentNotice;

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
