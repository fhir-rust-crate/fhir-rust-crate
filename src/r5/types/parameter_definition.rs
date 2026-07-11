//! ParameterDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ParameterDefinition
//!
//! Version: 5.0.0
//!
//! ParameterDefinition Type: The parameters to the module.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The parameters to the module. This collection specifies both the input and
/// output parameters. Input parameters are provided by the caller as part of
/// the $evaluate operation. Output parameters are included in the
/// GuidanceResponse. ParameterDefinition is typically used within knowledge
/// artifacts such as Library and PlanDefinition to describe the interface of a
/// module.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::parameter_definition::ParameterDefinition;
///
/// let value = ParameterDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ParameterDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<types::Extension>>,

    /// Name used to access the parameter value
    pub name: Option<types::Code>,

    /// in | out
    pub r#use: types::Code,

    /// Minimum cardinality
    pub min: Option<types::Integer>,

    /// Maximum cardinality (a number of *)
    pub max: Option<types::String>,

    /// A brief description of the parameter
    pub documentation: Option<types::String>,

    /// What type of value
    pub r#type: types::Code,

    /// What profile the value is expected to be
    pub profile: Option<types::Canonical>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ParameterDefinition;

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
