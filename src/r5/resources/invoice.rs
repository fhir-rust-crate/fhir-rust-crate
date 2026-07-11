//! Invoice
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Invoice
//!
//! Version: 5.0.0
//!
//! Invoice Resource: Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
///
/// The Invoice resource represents a formal statement of financial charges
/// issued to a subject or organization. It aggregates individual ChargeItem
/// entries drawn from an [`Account`](crate::r5::resources::account::Account)
/// into discrete line items, each carrying its own price components, and rolls
/// them up into calculated net and gross totals. Invoices are produced by an
/// issuing organization once the services or goods recorded against an account
/// are ready to be billed, and they carry the status of the billing process
/// (draft, issued, balanced, cancelled, or entered-in-error) as it progresses.
///
/// Within the FHIR R5 financial and billing workflows, an Invoice sits
/// downstream of clinical and administrative activity: charges captured as
/// they occur are collected onto an account, and the Invoice then presents
/// those charges in a structured, human- and machine-readable form suitable
/// for submission, payment, and reconciliation. Payment terms, participants
/// involved in its creation, and free-text notes may accompany the charges.
///
/// See also: [`Account`](crate::r5::resources::account::Account) for the
/// running balance being invoiced, [`Reference`](crate::r5::types::Reference)
/// for links to the subject, recipient, and issuer, and
/// [`Money`](crate::r5::types::Money) and
/// [`MonetaryComponent`](crate::r5::types::MonetaryComponent) for how amounts
/// are expressed. The recipient and issuer are commonly a `Patient`,
/// `RelatedPerson`, or `Organization`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::invoice::Invoice;
///
/// let value = Invoice::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Invoice = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
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

    /// Business Identifier for item
    pub identifier: Option<Vec<types::Identifier>>,

    /// Current state of the invoice in the billing process: draft, issued, balanced, cancelled, or entered-in-error.
    pub status: types::Code,

    /// Reason for cancellation of this Invoice
    pub cancelled_reason: Option<types::String>,

    /// Type of Invoice
    pub r#type: Option<types::CodeableConcept>,

    /// Recipient(s) of goods and services
    pub subject: Option<types::Reference>,

    /// Recipient of this invoice
    pub recipient: Option<types::Reference>,

    /// DEPRICATED
    pub date: Option<types::DateTime>,

    /// When posted
    pub creation: Option<types::DateTime>,

    /// Billing date or period
    pub period_date: Option<types::Date>,

    /// Billing date or period
    pub period_period: Option<types::Period>,

    /// Participant in creation of this Invoice
    pub participant: Option<Vec<InvoiceParticipant>>,

    /// Issuing Organization of Invoice
    pub issuer: Option<types::Reference>,

    /// Reference to the Account whose collected charges are being balanced by this invoice.
    pub account: Option<types::Reference>,

    /// Individual charge lines that make up this invoice, each with its own price components.
    pub line_item: Option<Vec<InvoiceLineItem>>,

    /// Components of Invoice total
    pub total_price_component: Option<Vec<types::MonetaryComponent>>,

    /// Net total of this invoice, excluding taxes and surcharges captured as components.
    pub total_net: Option<types::Money>,

    /// Gross total of this invoice, including all price components such as taxes.
    pub total_gross: Option<types::Money>,

    /// Payment details
    pub payment_terms: Option<types::Markdown>,

    /// Comments made about the invoice
    pub note: Option<Vec<types::Annotation>>,
}

/// Participant in creation of this Invoice.
///
/// Indicates who or what was involved in the creation of the Invoice and in
/// what role.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Type of involvement in creation of this Invoice
    pub role: Option<types::CodeableConcept>,

    /// Individual who was involved
    pub actor: types::Reference,
}

/// Line items of this Invoice.
///
/// Each line item captures a specific charge, referencing a ChargeItem or an
/// inline billing code, together with its price components.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLineItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Sequence number of line item
    pub sequence: Option<types::PositiveInt>,

    /// Service data or period
    pub serviced_date: Option<types::Date>,

    /// Service data or period
    pub serviced_period: Option<types::Period>,

    /// Reference to ChargeItem containing details of this line item or an inline billing code
    pub charge_item_reference: Option<types::Reference>,

    /// Reference to ChargeItem containing details of this line item or an inline billing code
    pub charge_item_codeable_concept: Option<types::CodeableConcept>,

    /// Components of total line item price
    pub price_component: Option<Vec<types::MonetaryComponent>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Invoice;

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
