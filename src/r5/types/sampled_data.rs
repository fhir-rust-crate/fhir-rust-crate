//! SampledData
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SampledData
//!
//! Version: 5.0.0
//!
//! SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SampledData {
    pub origin: types::Quantity,          // Quantity(SimpleQuantity) [1..1]
    pub interval: Option<types::Decimal>, // « C »
    pub interval_unit: types::Code,       // « UCUMCodes! »
    pub factor: Option<types::Decimal>,
    pub lower_limit: Option<types::Decimal>,
    pub upper_limit: Option<types::Decimal>,
    pub dimensions: types::PositiveInt,
    pub code_map: Option<types::Canonical>, // « ConceptMap »
    pub offsets: Option<types::String>,     // « C »
    pub data: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SampledData;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            origin: types::Quantity::default(),
            interval: None,
            interval_unit: types::Code::default(),
            factor: None,
            lower_limit: None,
            upper_limit: None,
            dimensions: types::PositiveInt::default(),
            code_map: None,
            offsets: None,
            data: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;

        #[test]
        fn test_serde_json_round_trip() {
            let value = T::default();
            let json = ::serde_json::to_value(&value).expect("to_value");
            let back: T = ::serde_json::from_value(json).expect("from_value");
            assert_eq!(value, back);
        }
    }
}
