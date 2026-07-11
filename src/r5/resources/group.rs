//! Group
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Group
//!
//! Version: 5.0.0
//!
//! Group Resource: Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively.
///
/// A Group resource describes a set of entities of the same kind — such as
/// people, animals, practitioners, devices, or specimens — that are grouped
/// together for a common purpose. Membership may be defined intensionally by a
/// set of characteristics (a definitional group) or extensionally by explicitly
/// listing members (an enumerated group). In FHIR R5 it is commonly used for
/// cohorts, research study populations, herds of animals, or collections of
/// devices, and is distinct from an Organization, which represents a formally
/// or legally recognized entity.
///
/// Clinically and administratively, Group is used wherever an action, order,
/// communication, or observation needs to apply to many entities at once
/// rather than to a single record — for example targeting a public health
/// intervention at a cohort of patients, enrolling a herd of animals in a
/// veterinary study, or scoping a `CarePlan` or `Communication` to a set of
/// recipients. The `type` and `membership` fields establish what kind of
/// entities the group contains and whether membership is rule-based
/// (`characteristic`) or explicitly enumerated (`member`), while `quantity`
/// and `managingEntity` support cases where the exact membership list is not
/// tracked by the system.
///
/// See also: [`Patient`](crate::r5::resources::patient::Patient) and
/// `Practitioner`, `Device`, `Specimen`, and `Organization`, which are the
/// typical kinds of entities referenced as group members, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), used to describe
/// group characteristics.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::group::Group;
///
/// let value = Group::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Group = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Group {
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

    /// Business Identifier for this Group
    pub identifier: Option<Vec<types::Identifier>>,

    /// Whether this group's record is in active use, as opposed to being retired or entered in error
    pub active: Option<types::Boolean>,

    /// The kind of entities held by this group: person | animal | practitioner | device | careteam | healthcareservice | location | organization | relatedperson | specimen
    pub r#type: types::Code,

    /// Basis for membership: definitional (rule-based, via `characteristic`) or enumerated (explicitly listed, via `member`)
    pub membership: types::Code,

    /// Kind of Group members
    pub code: Option<types::CodeableConcept>,

    /// Label for Group
    pub name: Option<types::String>,

    /// Natural language description of the group
    pub description: Option<types::Markdown>,

    /// Number of members
    pub quantity: Option<types::UnsignedInt>,

    /// Entity that is the custodian of the Group's definition
    pub managing_entity: Option<types::Reference>,

    /// Rules for including or excluding members of a definitional Group by trait
    pub characteristic: Option<Vec<GroupCharacteristic>>,

    /// The explicit list of entities that are members of an enumerated Group
    pub member: Option<Vec<GroupMember>>,
}

/// Include / Exclude group members by Trait.
///
/// Identifies traits whose presence or absence is used to describe members of a
/// definitional Group.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GroupCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Kind of characteristic
    pub code: types::CodeableConcept,

    /// Value held by characteristic
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// Value held by characteristic
    pub value_boolean: Option<types::Boolean>,

    /// Value held by characteristic
    pub value_quantity: Option<types::Quantity>,

    /// Value held by characteristic
    pub value_range: Option<types::Range>,

    /// Value held by characteristic
    pub value_reference: Option<types::Reference>,

    /// Group includes or excludes
    pub exclude: types::Boolean,

    /// Period over which characteristic is tested
    pub period: Option<types::Period>,
}

/// Who or what is in group.
///
/// Identifies the resource instances that are members of an enumerated Group.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Reference to the group member
    pub entity: types::Reference,

    /// Period member belonged to the group
    pub period: Option<types::Period>,

    /// If member is no longer in group
    pub inactive: Option<types::Boolean>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Group;

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
