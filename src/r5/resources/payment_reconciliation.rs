//! PaymentReconciliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
//!
//! Version: 5.0.0
//!
//! PaymentReconciliation Resource: This resource provides the details including amount of a payment and allocates the payment items being paid.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// This resource provides the details including amount of a payment and
/// allocates the payment items being paid. A PaymentReconciliation is issued by
/// a payer (such as an insurer) to convey the outcome of processing one or more
/// payment requests, describing the total payment made and how that payment is
/// distributed across the individual claims, invoices, or other payables. It
/// supports the electronic remittance advice workflow in the FHIR financial
/// module.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::payment_reconciliation::PaymentReconciliation;
///
/// let value = PaymentReconciliation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PaymentReconciliation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliation {
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

    /// Business Identifier for a payment reconciliation
    pub identifier: Option<Vec<types::Identifier>>,

    /// Category of payment
    pub r#type: types::CodeableConcept,

    /// active | cancelled | draft | entered-in-error
    pub status: types::Code,

    /// Workflow originating payment
    pub kind: Option<types::CodeableConcept>,

    /// Period covered
    pub period: Option<types::Period>,

    /// Creation date
    pub created: types::DateTime,

    /// Who entered the payment
    pub enterer: Option<types::Reference>,

    /// Nature of the source
    pub issuer_type: Option<types::CodeableConcept>,

    /// Party generating payment
    pub payment_issuer: Option<types::Reference>,

    /// Reference to requesting resource
    pub request: Option<types::Reference>,

    /// Responsible practitioner
    pub requestor: Option<types::Reference>,

    /// queued | complete | error | partial
    pub outcome: Option<types::Code>,

    /// Disposition message
    pub disposition: Option<types::String>,

    /// When payment issued
    pub date: types::Date,

    /// Where payment collected
    pub location: Option<types::Reference>,

    /// Payment instrument
    pub method: Option<types::CodeableConcept>,

    /// Type of card
    pub card_brand: Option<types::String>,

    /// Digits for verification
    pub account_number: Option<types::String>,

    /// Expiration year-month
    pub expiration_date: Option<types::Date>,

    /// Processor name
    pub processor: Option<types::String>,

    /// Check number or payment reference
    pub reference_number: Option<types::String>,

    /// Authorization number
    pub authorization: Option<types::String>,

    /// Amount offered by the issuer
    pub tendered_amount: Option<types::Money>,

    /// Amount returned by the receiver
    pub returned_amount: Option<types::Money>,

    /// Total amount of Payment
    pub amount: types::Money,

    /// Business identifier for the payment
    pub payment_identifier: Option<types::Identifier>,

    /// Settlement particulars
    pub allocation: Option<Vec<PaymentReconciliationAllocation>>,

    /// Printed form identifier
    pub form_code: Option<types::CodeableConcept>,

    /// Note concerning processing
    pub process_note: Option<Vec<PaymentReconciliationProcessNote>>,
}

/// Settlement particulars: distribution of the total payment across the
/// individual payables being reconciled.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliationAllocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Business identifier of the payment detail
    pub identifier: Option<types::Identifier>,

    /// Business identifier of the prior payment detail
    pub predecessor: Option<types::Identifier>,

    /// Subject of the payment
    pub target: Option<types::Reference>,

    /// Sub-element of the subject
    pub target_item_string: Option<types::String>,

    /// Sub-element of the subject
    pub target_item_identifier: Option<types::Identifier>,

    /// Sub-element of the subject
    pub target_item_positive_int: Option<types::PositiveInt>,

    /// Applied-to encounter
    pub encounter: Option<types::Reference>,

    /// Applied-to account
    pub account: Option<types::Reference>,

    /// Category of payment
    pub r#type: Option<types::CodeableConcept>,

    /// Submitter of the request
    pub submitter: Option<types::Reference>,

    /// Response committing to a payment
    pub response: Option<types::Reference>,

    /// Date of commitment to pay
    pub date: Option<types::Date>,

    /// Contact for the response
    pub responsible: Option<types::Reference>,

    /// Recipient of the payment
    pub payee: Option<types::Reference>,

    /// Amount allocated to this payable
    pub amount: Option<types::Money>,
}

/// Note concerning processing: suggested notes to convey to or explain the
/// processing outcome to the requestor.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliationProcessNote {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// display | print | printoper
    pub r#type: Option<types::Code>,

    /// Note explanatory text
    pub text: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PaymentReconciliation;

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
