//! Specimen
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Specimen
//!
//! Version: 5.0.0
//!
//! Specimen Resource: A sample to be used for analysis.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A sample to be used for analysis.
///
/// The Specimen resource describes a physical sample (such as blood, tissue,
/// urine, or an environmental sample) that is collected and used for laboratory
/// or diagnostic analysis. It captures the material type, its source subject,
/// collection details, processing steps, and the containers holding the sample.
/// It is central to laboratory and diagnostic workflows in FHIR R5.
///
/// A Specimen typically originates as the target of a service request (for
/// example an order for a lab test), is collected from a subject such as a
/// patient, may pass through one or more processing steps (centrifugation,
/// fixation, staining, and so on), and is ultimately placed into one or more
/// containers before being analyzed. The resulting observations and diagnostic
/// reports commonly reference the Specimen that was used to produce them,
/// making it a key link between a clinical order and its results.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) or other subject from
///   which the specimen was obtained.
/// - `ServiceRequest`, which may prompt the collection of a specimen.
/// - `Observation` and `DiagnosticReport`, which commonly reference the
///   specimen used to produce their results.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept), used throughout
///   this resource to represent coded concepts such as specimen type, role,
///   and condition.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::specimen::Specimen;
///
/// let value = Specimen::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Specimen = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Specimen {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<::serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// External Identifier, such as a specimen collection identifier used by the ordering system
    pub identifier: Option<Vec<types::Identifier>>,

    /// Identifier assigned by the accessioning laboratory when it receives the specimen
    pub accession_identifier: Option<types::Identifier>,

    /// The availability of the specimen: available | unavailable | unsatisfactory | entered-in-error
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::SpecimenStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of material that forms the specimen, for example whole blood or tissue
    pub r#type: Option<types::CodeableConcept>,

    /// Reference to the subject the specimen came from, typically a Patient, but may be a group, device, or location
    pub subject: Option<types::Reference>,

    /// The time when specimen is received by the testing laboratory
    pub received_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`received_time`](Self::received_time) (FHIR `_receivedTime`).
    #[serde(rename = "_receivedTime")]
    pub received_time_ext: Option<types::Element>,

    /// Specimen from which this specimen originated
    pub parent: Option<Vec<types::Reference>>,

    /// Why the specimen was collected
    pub request: Option<Vec<types::Reference>>,

    /// grouped | pooled
    pub combined: Option<crate::r5::coded::Coded<crate::r5::codes::SpecimenCombined>>,
    /// Primitive extension sibling for [`combined`](Self::combined) (FHIR `_combined`).
    #[serde(rename = "_combined")]
    pub combined_ext: Option<types::Element>,

    /// The role the specimen serves
    pub role: Option<Vec<types::CodeableConcept>>,

    /// The physical feature of a specimen
    pub feature: Option<Vec<SpecimenFeature>>,

    /// Collection details
    pub collection: Option<SpecimenCollection>,

    /// Processing and processing step details
    pub processing: Option<Vec<SpecimenProcessing>>,

    /// Direct container of specimen (tube/slide, etc.)
    pub container: Option<Vec<SpecimenContainer>>,

    /// State of the specimen
    pub condition: Option<Vec<types::CodeableConcept>>,

    /// Comments
    pub note: Option<Vec<types::Annotation>>,
}

/// The physical feature of a specimen.
///
/// A physical feature or landmark of a specimen, highlighted for particular
/// relevance to the analysis (for example a tissue orientation marker).
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenFeature {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Highlighted feature
    pub r#type: types::CodeableConcept,

    /// Information about the feature
    pub description: types::String,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Collection details.
///
/// Details concerning the specimen collection, including who collected it, when
/// and how it was collected, the quantity obtained, and the anatomical site.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenCollection {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Who collected the specimen
    pub collector: Option<types::Reference>,

    /// The `Specimen.collection.collected[x]` choice element (0..1); see [`SpecimenCollectionCollected`].
    #[serde(flatten)]
    pub collected: Option<SpecimenCollectionCollected>,

    /// How long it took to collect specimen
    pub duration: Option<types::Duration>,

    /// The quantity of specimen collected
    pub quantity: Option<types::Quantity>,

    /// Technique used to perform collection
    pub method: Option<types::CodeableConcept>,

    /// Device used to perform collection
    pub device: Option<types::CodeableReference>,

    /// The procedure that collects the specimen
    pub procedure: Option<types::Reference>,

    /// Anatomical collection site
    pub body_site: Option<types::CodeableReference>,

    /// The `Specimen.collection.fastingStatus[x]` choice element (0..1); see [`SpecimenCollectionFastingStatus`].
    #[serde(flatten)]
    pub fasting_status: Option<SpecimenCollectionFastingStatus>,
}

/// Processing and processing step details.
///
/// Details concerning the processing and processing steps applied to the
/// specimen, such as the method used, additives applied, and the time of
/// processing.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenProcessing {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Textual description of procedure
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Indicates the treatment step applied to the specimen
    pub method: Option<types::CodeableConcept>,

    /// Material used in the processing step
    pub additive: Option<Vec<types::Reference>>,

    /// The `Specimen.processing.time[x]` choice element (0..1); see [`SpecimenProcessingTime`].
    #[serde(flatten)]
    pub time: Option<SpecimenProcessingTime>,
}

/// Direct container of specimen (tube/slide, etc.).
///
/// The container holding the specimen, such as a tube, slide, or vial,
/// including its device, location, and the quantity of specimen it contains.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenContainer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Device resource for the container
    pub device: types::Reference,

    /// Where the container is
    pub location: Option<types::Reference>,

    /// Quantity of specimen within container
    pub specimen_quantity: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Specimen;

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
/// The `Specimen.collection.collected[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenCollectionCollected {
    /// `collectedDateTime` variant.
    #[fhir("collectedDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `collectedPeriod` variant.
    #[fhir("collectedPeriod")]
    Period(Box<types::Period>),
}

/// The `Specimen.collection.fastingStatus[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenCollectionFastingStatus {
    /// `fastingStatusCodeableConcept` variant.
    #[fhir("fastingStatusCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `fastingStatusDuration` variant.
    #[fhir("fastingStatusDuration")]
    Duration(Box<types::Duration>),
}

/// The `Specimen.processing.time[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenProcessingTime {
    /// `timeDateTime` variant.
    #[fhir("timeDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `timePeriod` variant.
    #[fhir("timePeriod")]
    Period(Box<types::Period>),
}
