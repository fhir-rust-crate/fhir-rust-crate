//! AdministrableProductDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition
//!
//! Version: 5.0.0
//!
//! AdministrableProductDefinition Resource: A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

/// AdministrableProductDefinition
///
/// A medicinal product in the final form which is suitable for administering to a
/// patient (after any mixing of multiple components, dissolution etc. has been
/// performed). It describes the administrable form of a product, including its
/// dose form, unit of presentation, ingredients, and the routes by which it may
/// be given. This resource links back to the overall medicinal product and to the
/// manufactured items from which the administrable form is prepared.
///
/// # Examples
///
/// ```
/// use fhir_specifications_parser::r5::resources::administrable_product_definition::AdministrableProductDefinition;
///
/// let value = AdministrableProductDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AdministrableProductDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinition {
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

    /// An identifier for the administrable product
    pub identifier: Option<Vec<types::Identifier>>,

    /// draft | active | retired | unknown
    pub status: types::Code,

    /// References a product from which one or more of the constituent parts of that product can be prepared and used as described by this administrable product
    pub form_of: Option<Vec<types::Reference>>,

    /// The dose form of the final product after necessary reconstitution or processing
    pub administrable_dose_form: Option<types::CodeableConcept>,

    /// The presentation type in which this item is given to a patient. e.g. for a spray - 'puff'
    pub unit_of_presentation: Option<types::CodeableConcept>,

    /// Indicates the specific manufactured items that are part of the 'formOf' product that are used in the preparation of this specific administrable form
    pub produced_from: Option<Vec<types::Reference>>,

    /// The ingredients of this administrable medicinal product. This is only needed if the ingredients are not specified either using ManufacturedItemDefiniton, or using by incoming references from the Ingredient resource
    pub ingredient: Option<Vec<types::CodeableConcept>>,

    /// A device that is integral to the medicinal product, in effect being considered as an "ingredient" of the medicinal product
    pub device: Option<types::Reference>,

    /// A general description of the product, when in its final form, suitable for administration e.g. effervescent blue liquid, to be swallowed
    pub description: Option<types::Markdown>,

    /// Characteristics e.g. a product's onset of action
    pub property: Option<Vec<AdministrableProductDefinitionProperty>>,

    /// The path by which the product is taken into or makes contact with the body
    pub route_of_administration: Vec<AdministrableProductDefinitionRouteOfAdministration>,
}

/// AdministrableProductDefinitionProperty
///
/// Characteristics e.g. a product's onset of action.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// A code expressing the type of characteristic
    pub r#type: types::CodeableConcept,

    /// A value for the characteristic
    pub value_codeable_concept: Option<types::CodeableConcept>,

    /// A value for the characteristic
    pub value_quantity: Option<types::Quantity>,

    /// A value for the characteristic
    pub value_date: Option<types::Date>,

    /// A value for the characteristic
    pub value_boolean: Option<types::Boolean>,

    /// A value for the characteristic
    pub value_markdown: Option<types::Markdown>,

    /// A value for the characteristic
    pub value_attachment: Option<types::Attachment>,

    /// A value for the characteristic
    pub value_reference: Option<types::Reference>,

    /// The status of characteristic e.g. assigned or pending
    pub status: Option<types::CodeableConcept>,
}

/// AdministrableProductDefinitionRouteOfAdministration
///
/// The path by which the product is taken into or makes contact with the body.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Coded expression for the route
    pub code: types::CodeableConcept,

    /// The first dose (dose quantity) administered can be specified for the product
    pub first_dose: Option<types::Quantity>,

    /// The maximum single dose that can be administered
    pub max_single_dose: Option<types::Quantity>,

    /// The maximum dose quantity to be administered in any one 24-h period
    pub max_dose_per_day: Option<types::Quantity>,

    /// The maximum dose per treatment period that can be administered
    pub max_dose_per_treatment_period: Option<types::Ratio>,

    /// The maximum treatment period during which the product can be administered
    pub max_treatment_period: Option<types::Duration>,

    /// A species for which this route applies
    pub target_species: Option<Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies>>,
}

/// AdministrableProductDefinitionRouteOfAdministrationTargetSpecies
///
/// A species for which this route applies.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// Coded expression for the species
    pub code: types::CodeableConcept,

    /// A species specific time during which consumption of animal product is not appropriate
    pub withdrawal_period:
        Option<Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod>>,
}

/// AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod
///
/// A species specific time during which consumption of animal product is not
/// appropriate.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    pub modifier_extension: Option<Vec<types::Extension>>,

    /// The type of tissue for which the withdrawal period applies, e.g. meat, milk
    pub tissue: types::CodeableConcept,

    /// A value for the time
    pub value: types::Quantity,

    /// Extra information about the withdrawal period
    pub supporting_information: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AdministrableProductDefinition;

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
