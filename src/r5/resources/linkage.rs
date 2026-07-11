//! Linkage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Linkage
//!
//! Version: 5.0.0
//!
//! Linkage Resource: Identifies two or more records (resource instances) that refer to the same real-world "occurrence".
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Identifies two or more records (resource instances) that refer to the same
/// real-world "occurrence". A Linkage asserts that a set of resources — for
/// example multiple [`Patient`](crate::r5::resources::patient::Patient) records —
/// all describe the same underlying entity, distinguishing the authoritative
/// "source" record from "alternate" or "historical" ones. Rather than physically
/// merging the linked records, a Linkage keeps each resource intact and records
/// the relationship between them, so that consumers can resolve which instance is
/// current and which are superseded or duplicated. In FHIR R5 it is typically used
/// during record matching, deduplication, patient identity reconciliation, and
/// master data management, where enterprises must relate records drawn from
/// multiple systems while preserving their provenance and history.
///
/// See also: each linked resource is referenced through a
/// [`Reference`](crate::r5::types::Reference), the assertion may be attributed to
/// an author such as a `Practitioner` or `Organization`, and the classification of
/// each entry is carried as a [`Code`](crate::r5::types::Code). For merging patient
/// identities specifically, compare with the `Patient.link` mechanism.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::linkage::Linkage;
///
/// let value = Linkage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Linkage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Linkage {
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

    /// Whether this linkage assertion is currently active; an inactive assertion is retained for history but no longer treated as valid.
    pub active: Option<types::Boolean>,

    /// Reference to the party, such as a Practitioner or Organization, responsible for asserting and maintaining these linkages.
    pub author: Option<types::Reference>,

    /// The set of records being linked together; each entry names a resource and its role within the collection.
    pub item: Vec<LinkageItem>,
}

/// Identifies which record considered as the reference to the same real-world
/// occurrence as well as how the items should be evaluated within the collection
/// of linked items.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LinkageItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Role of this record within the linkage, coded as source, alternate, or historical.
    pub r#type: types::Code,

    /// Reference to the specific resource instance being included in this linkage.
    pub resource: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Linkage;

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
