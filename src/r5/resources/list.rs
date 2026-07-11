//! List
//!
//! URL: http://hl7.org/fhir/StructureDefinition/List
//!
//! Version: 5.0.0
//!
//! List Resource: A List is a curated collection of resources, for things such as problem lists, allergy lists, facility list, organization list, etc.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A List is a curated, human- or system-maintained collection of resources.
///
/// The List resource groups references to other resources into a single named
/// collection, such as a problem list, allergy list, medication list, a set of
/// results, or a working set of items assembled for a particular purpose. Unlike
/// a Bundle, which is a transient container used to move a set of resources
/// together, a List is a persistent, first-class resource that records the
/// intent and provenance behind the grouping: how and why the collection was
/// assembled, including its status (current, retired, or entered-in-error), its
/// mode (a working list, a point-in-time snapshot, or a change set), the ordering
/// applied to entries, the source that authored it, and the date it was prepared.
/// Each entry can carry workflow flags, an indication that an item has been
/// deleted, and the date the item was added, which lets the List act as a
/// managed, curated view whose membership evolves over time. In FHIR R5 it is
/// used both for clinically curated lists and for administrative or operational
/// collections, and an empty list can explain its emptiness through a reason code.
///
/// # See also
///
/// The list `subject` and `source` fields, and each entry's `item`, are typically
/// references to resources such as [`Patient`](crate::r5::resources::patient::Patient),
/// [`Condition`](crate::r5::resources::condition::Condition), or an
/// [`Encounter`](crate::r5::resources::encounter::Encounter). Coded fields such as
/// `code`, `ordered_by`, and `empty_reason` use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and references use
/// [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::list::List;
///
/// let value = List::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: List = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct List {
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

    /// Business identifier
    pub identifier: Option<Vec<types::Identifier>>,

    /// Lifecycle status of the list: current, retired, or entered-in-error.
    pub status: types::Code,

    /// How the list was assembled: a working list, a point-in-time snapshot, or a change set.
    pub mode: types::Code,

    /// Descriptive name for the list
    pub title: Option<types::String>,

    /// What the purpose of this list is
    pub code: Option<types::CodeableConcept>,

    /// If all resources have the same subject(s)
    pub subject: Option<Vec<types::Reference>>,

    /// Context in which list created
    pub encounter: Option<types::Reference>,

    /// When the list was prepared
    pub date: Option<types::DateTime>,

    /// Who and/or what defined the list contents (aka Author)
    pub source: Option<types::Reference>,

    /// What order the list has
    pub ordered_by: Option<types::CodeableConcept>,

    /// Comments about the list
    pub note: Option<Vec<types::Annotation>>,

    /// Entries in the list, each referencing an item and its optional flags, deletion state, and date.
    pub entry: Option<Vec<ListEntry>>,

    /// Why list is empty
    pub empty_reason: Option<types::CodeableConcept>,
}

/// Entries in the list.
///
/// Each entry references an actual item in the list and may carry status or
/// workflow flags, an indication that the item has been deleted, and the date
/// the item was added.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ListEntry {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Status/Workflow information about this item
    pub flag: Option<types::CodeableConcept>,

    /// If this item is actually marked as deleted
    pub deleted: Option<types::Boolean>,

    /// When item added to list
    pub date: Option<types::DateTime>,

    /// Actual entry
    pub item: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = List;

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
