//! SpecimenDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
//!
//! Version: 5.0.0
//!
//! SpecimenDefinition Resource: A kind of specimen with associated set of requirements.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A kind of specimen with associated set of requirements.
///
/// The SpecimenDefinition resource describes the characteristics and
/// requirements of a kind of specimen, including how it is collected,
/// the containers and additives it is placed in, and how it must be
/// handled prior to testing. It supports laboratory catalogs and order
/// entry by defining reusable specimen requirements.
///
/// SpecimenDefinition is a canonical, definitional resource: instances are
/// typically authored and maintained by a laboratory or diagnostic service
/// as part of its test catalog, and are referenced by orders and by the
/// resulting `Specimen` instances collected for a given
/// [`Patient`](crate::r5::resources::patient::Patient) or other subject.
/// A single SpecimenDefinition may describe several
/// acceptable specimen/container combinations (via `type_tested`), each
/// with its own preference, handling, and rejection criteria, allowing an
/// ordering system to present the range of valid collection options for a
/// given kind of test. It is closely related to `ServiceRequest` (which
/// orders a test that requires a specimen of a defined kind) and to
/// `ObservationDefinition` (which defines the expected observation produced
/// once the specimen is tested).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::specimen_definition::SpecimenDefinition;
///
/// let value = SpecimenDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SpecimenDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinition {
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

    /// Logical canonical URL to reference this SpecimenDefinition (globally unique)
    pub url: Option<types::Uri>,

    /// Business identifier used by catalogs and order systems to identify this kind of specimen
    pub identifier: Option<types::Identifier>,

    /// Business version of the SpecimenDefinition
    pub version: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_string: Option<types::String>,

    /// How to compare versions
    pub version_algorithm_coding: Option<types::Coding>,

    /// Name for this {{title}} (computer friendly)
    pub name: Option<types::String>,

    /// Name for this SpecimenDefinition (Human friendly)
    pub title: Option<types::String>,

    /// Based on FHIR definition of another SpecimenDefinition
    pub derived_from_canonical: Option<Vec<types::Canonical>>,

    /// Based on external definition
    pub derived_from_uri: Option<Vec<types::Uri>>,

    /// Publication status of this definition: draft | active | retired | unknown
    pub status: types::Code,

    /// If this SpecimenDefinition is not for real usage
    pub experimental: Option<types::Boolean>,

    /// Type of subject for specimen collection
    pub subject_codeable_concept: Option<types::CodeableConcept>,

    /// Type of subject for specimen collection
    pub subject_reference: Option<types::Reference>,

    /// Date status first applied
    pub date: Option<types::DateTime>,

    /// The name of the individual or organization that published the SpecimenDefinition
    pub publisher: Option<types::String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<types::ContactDetail>>,

    /// Natural language description of the SpecimenDefinition
    pub description: Option<types::Markdown>,

    /// Content intends to support these contexts
    pub use_context: Option<Vec<types::UsageContext>>,

    /// Intended jurisdiction for this SpecimenDefinition (if applicable)
    pub jurisdiction: Option<Vec<types::CodeableConcept>>,

    /// Why this SpecimenDefinition is defined
    pub purpose: Option<types::Markdown>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,

    /// When SpecimenDefinition was approved by publisher
    pub approval_date: Option<types::Date>,

    /// The date on which the asset content was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,

    /// The effective date range for the SpecimenDefinition
    pub effective_period: Option<types::Period>,

    /// Kind of material to collect, coded via a [`CodeableConcept`](crate::r5::types::CodeableConcept) such as blood or urine
    pub type_collected: Option<types::CodeableConcept>,

    /// Patient preparation for collection
    pub patient_preparation: Option<Vec<types::CodeableConcept>>,

    /// Time aspect for collection
    pub time_aspect: Option<types::String>,

    /// Specimen collection procedure
    pub collection: Option<Vec<types::CodeableConcept>>,

    /// One or more acceptable specimen/container combinations for testing by the lab, each with its own preference and handling
    pub type_tested: Option<Vec<SpecimenDefinitionTypeTested>>,
}

/// Specimen in container intended for testing by lab.
///
/// Describes a kind of specimen conditioned for testing, expected from
/// the collected specimen, including the container and additives, the
/// handling requirements, and the acceptable rejection criteria.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTested {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Primary or secondary specimen
    pub is_derived: Option<types::Boolean>,

    /// Type of intended specimen
    pub r#type: Option<types::CodeableConcept>,

    /// preferred | alternate
    pub preference: types::Code,

    /// The specimen's container
    pub container: Option<SpecimenDefinitionTypeTestedContainer>,

    /// Requirements for specimen delivery and special handling
    pub requirement: Option<types::Markdown>,

    /// The usual time for retaining this kind of specimen
    pub retention_time: Option<types::Duration>,

    /// Specimen for single use only
    pub single_use: Option<types::Boolean>,

    /// Criterion specified for specimen rejection
    pub rejection_criterion: Option<Vec<types::CodeableConcept>>,

    /// Specimen handling before testing
    pub handling: Option<Vec<SpecimenDefinitionTypeTestedHandling>>,

    /// Where the specimen will be tested
    pub testing_destination: Option<Vec<types::CodeableConcept>>,
}

/// The specimen's container.
///
/// The specimen's container, including its material, type, cap color,
/// capacity, minimum volume, associated additives, and any special
/// preparation applied to the container for this specimen type.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTestedContainer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The material type used for the container
    pub material: Option<types::CodeableConcept>,

    /// Kind of container associated with the kind of specimen
    pub r#type: Option<types::CodeableConcept>,

    /// Color of container cap
    pub cap: Option<types::CodeableConcept>,

    /// The description of the kind of container
    pub description: Option<types::Markdown>,

    /// The capacity of this kind of container
    pub capacity: Option<types::Quantity>,

    /// Minimum volume
    pub minimum_volume_quantity: Option<types::Quantity>,

    /// Minimum volume
    pub minimum_volume_string: Option<types::String>,

    /// Additive associated with container
    pub additive: Option<Vec<SpecimenDefinitionTypeTestedContainerAdditive>>,

    /// Special processing applied to the container for this specimen type
    pub preparation: Option<types::Markdown>,
}

/// Additive associated with container.
///
/// Substance introduced in the kind of container to preserve, maintain
/// or enhance the specimen, referenced either as a coded concept or as
/// a reference to a Substance resource.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Additive associated with container
    pub additive_codeable_concept: Option<types::CodeableConcept>,

    /// Additive associated with container
    pub additive_reference: Option<types::Reference>,
}

/// Specimen handling before testing.
///
/// Set of instructions for preservation and handling of a specimen at a
/// given temperature interval, including the maximum preservation time
/// for the specimen under those conditions.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTestedHandling {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Qualifies the interval of temperature
    pub temperature_qualifier: Option<types::CodeableConcept>,

    /// Temperature range for these handling instructions
    pub temperature_range: Option<types::Range>,

    /// Maximum preservation time
    pub max_duration: Option<types::Duration>,

    /// Preservation instruction
    pub instruction: Option<types::Markdown>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SpecimenDefinition;

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
