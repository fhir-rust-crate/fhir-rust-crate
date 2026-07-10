//! SubstanceSourceMaterial
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceSourceMaterial
//!
//! Version: 5.0.0
//!
//! SubstanceSourceMaterial Resource: Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can be
/// modified to form a substance. This set of data elements shall be used to
/// define polymer substances isolated from biological matrices, describing the
/// taxonomic and anatomical origins using a controlled vocabulary. It supports
/// the characterization of complex substances derived from plants, animals, and
/// other biological sources in FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_source_material::SubstanceSourceMaterial;
///
/// let value = SubstanceSourceMaterial::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SubstanceSourceMaterial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterial {
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

    /// General high level classification of the source material specific to the origin of the material
    pub source_material_class: Option<types::CodeableConcept>,

    /// The type of the source material shall be specified based on a controlled vocabulary. For vaccines, this subclause refers to the class of infectious agent
    pub source_material_type: Option<types::CodeableConcept>,

    /// The state of the source material when extracted
    pub source_material_state: Option<types::CodeableConcept>,

    /// The unique identifier associated with the source material parent organism shall be specified
    pub organism_id: Option<types::Identifier>,

    /// The organism accepted Scientific name shall be provided based on the organism taxonomy
    pub organism_name: Option<types::String>,

    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant)
    pub parent_substance_id: Option<Vec<types::Identifier>>,

    /// The parent substance of the Herbal Drug, or Herbal preparation
    pub parent_substance_name: Option<Vec<types::String>>,

    /// The country where the plant material is harvested or the countries where the plasma is sourced from as laid down in accordance with the Plasma Master File
    pub country_of_origin: Option<Vec<types::CodeableConcept>>,

    /// The place/region where the plant is harvested or the places/regions where the animal source material has its habitat
    pub geographical_location: Option<Vec<types::String>>,

    /// Stage of life for animals, plants, insects and microorganisms. This information shall be provided only when the substance is significantly different in these stages (e.g. foetal bovine serum)
    pub development_stage: Option<types::CodeableConcept>,

    /// Many complex materials are fractions of parts of plants, animals, or minerals
    pub fraction_description: Option<Vec<SubstanceSourceMaterialFractionDescription>>,

    /// This subclause describes the organism which the substance is derived from
    pub organism: Option<SubstanceSourceMaterialOrganism>,

    /// To do
    pub part_description: Option<Vec<SubstanceSourceMaterialPartDescription>>,
}

/// Many complex materials are fractions of parts of plants, animals, or
/// minerals. Fraction elements are often necessary to define both Substances
/// and Specified Group 1 Substances.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialFractionDescription {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// This element is capturing information about the fraction of a plant part, or human plasma for fractionation
    pub fraction: Option<types::String>,

    /// The specific type of the material constituting the component. For Herbal preparations the particulars of the extracts (liquid/dry) is described in Specified Substance Group 1
    pub material_type: Option<types::CodeableConcept>,
}

/// This subclause describes the organism which the substance is derived from.
/// For vaccines, the parent organism shall be specified based on these
/// subclause elements.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganism {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The family of an organism shall be specified
    pub family: Option<types::CodeableConcept>,

    /// The genus of an organism shall be specified; refers to the Latin epithet of the genus element of the plant/animal scientific name
    pub genus: Option<types::CodeableConcept>,

    /// The species of an organism shall be specified; refers to the Latin epithet of the species of the plant/animal
    pub species: Option<types::CodeableConcept>,

    /// The Intraspecific type of an organism shall be specified
    pub intraspecific_type: Option<types::CodeableConcept>,

    /// The intraspecific description of an organism shall be specified based on a controlled vocabulary
    pub intraspecific_description: Option<types::String>,

    /// 4.9.13.6.1 Author type (Conditional)
    pub author: Option<Vec<SubstanceSourceMaterialOrganismAuthor>>,

    /// 4.9.13.8.1 Hybrid species maternal organism ID (Optional)
    pub hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,

    /// 4.9.13.7.1 Kingdom (Conditional)
    pub organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
}

/// The author of an organism species. The parenthetical author refers to the
/// first author who published the plant/animal name; the primary author refers
/// to the first author(s) who validly published the name.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganismAuthor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of author of an organism species shall be specified
    pub author_type: Option<types::CodeableConcept>,

    /// The author of an organism species shall be specified. The author year of an organism shall also be specified when applicable
    pub author_description: Option<types::String>,
}

/// 4.9.13.8.1 Hybrid species maternal organism ID (Optional). Describes the
/// maternal and paternal species constituting a hybrid organism.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganismHybrid {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The identifier of the maternal species constituting the hybrid organism shall be specified based on a controlled vocabulary
    pub maternal_organism_id: Option<types::String>,

    /// The name of the maternal species constituting the hybrid organism shall be specified
    pub maternal_organism_name: Option<types::String>,

    /// The identifier of the paternal species constituting the hybrid organism shall be specified based on a controlled vocabulary
    pub paternal_organism_id: Option<types::String>,

    /// The name of the paternal species constituting the hybrid organism shall be specified
    pub paternal_organism_name: Option<types::String>,

    /// The hybrid type of an organism shall be specified
    pub hybrid_type: Option<types::CodeableConcept>,
}

/// 4.9.13.7.1 Kingdom (Conditional). Describes the general taxonomic
/// classification of the organism.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The kingdom of an organism shall be specified
    pub kingdom: Option<types::CodeableConcept>,

    /// The phylum of an organism shall be specified
    pub phylum: Option<types::CodeableConcept>,

    /// The class of an organism shall be specified
    pub class: Option<types::CodeableConcept>,

    /// The order of an organism shall be specified
    pub order: Option<types::CodeableConcept>,
}

/// Anatomical origin of the source material within an organism, together with
/// the detailed anatomic location when the part can be extracted from different
/// anatomical locations.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialPartDescription {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Entity of anatomical origin of source material within an organism
    pub part: Option<types::CodeableConcept>,

    /// The detailed anatomic location when the part can be extracted from different anatomical locations of the organism. Multiple alternative locations may apply
    pub part_location: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceSourceMaterial;

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
