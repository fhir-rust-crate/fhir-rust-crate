//! Address
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Address
//!
//! Version: 5.0.0
//!
//! Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world. The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "use")]
    pub use1: Option<types::Code>, // « AddressUse! »

    #[serde(rename = "type")]
    pub r#type: Option<types::Code>, // « AddressType! »

    /// # text
    /// 
    /// ## Description
    /// 
    /// The `text` attribute provides a human-readable narrative summary of a
    /// FHIR resource's content in XHTML format. This narrative serves as a
    /// fallback representation that ensures the essential information remains
    /// accessible even when systems cannot process all the structured data
    /// elements. The text element is particularly important for clinical
    /// safety, regulatory compliance, and systems interoperability where human
    /// readability is required.
    /// 
    /// ## Purpose
    /// 
    /// The `text` exists to:
    /// 
    /// - Provide human-readable summaries of structured resource content
    /// - Ensure clinical information remains accessible when structured data
    ///   cannot be processed
    /// - Support regulatory requirements for human-readable clinical documents
    /// - Enable fallback display when rendering systems have limited
    ///   capabilities
    /// - Provide narrative context that complements structured data
    /// - Support clinical safety by ensuring critical information is always
    ///   readable
    /// - Enable content review and validation by healthcare professionals
    /// 
    /// ## Usage
    /// 
    /// Use the `text` attribute when:
    /// 
    /// - Creating clinical resources that require human-readable summaries
    /// - Supporting regulatory compliance for clinical documentation
    /// - Ensuring accessibility across diverse healthcare systems
    /// - Providing narrative context for complex structured data
    /// - Creating resources for patient-facing applications
    /// - Supporting clinical review workflows that need readable content
    /// - Implementing systems that require both structured and narrative
    ///   representations
    /// 
    /// The narrative should accurately summarize the key information from the
    /// structured elements.
    /// 
    /// ## Data Type
    /// 
    /// **Narrative** - A complex structure containing:
    /// 
    /// - `status` (code): The generation status of the narrative
    ///   (generated|extensions|additional|empty)
    /// - `div` (xhtml): The XHTML content of the narrative
    /// 
    /// **Status Values:**
    /// 
    /// - `generated`: Generated from structured data, no additional information
    /// - `extensions`: Generated from structured data with additional extension
    ///   content
    /// - `additional`: Contains additional information not in structured data
    /// - `empty`: No narrative content provided
    /// 
    /// ## Constraints
    /// 
    /// - **Required**: Optional but strongly recommended for most clinical
    ///   resources
    /// - **Cardinality**: 0..1 (at most one narrative per resource)
    /// - **XHTML Format**: The div element must contain valid XHTML content
    /// - **Safety**: Should include all critical information from structured
    ///   data
    /// - **Consistency**: Should accurately reflect the structured data content
    /// - **Language**: Should match the language specified in the resource
    /// - **Security**: XHTML content must be safe and not contain executable
    ///   scripts
    /// 
    /// ## Examples
    /// 
    /// See the accompanying `example.json` file for complete resources
    /// demonstrating text narratives for different resource types including
    /// clinical observations, medications, and patient information.
    /// 
    /// ## Related Keys
    /// 
    /// - `div` - The XHTML content portion of the narrative
    /// - `status` - Indicates how the narrative was generated and its
    ///   relationship to structured data
    /// - `language` - Language code that may affect narrative content
    /// - `meta` - Resource metadata that may influence narrative generation
    /// - `contained` - Inline resources that may be referenced in the narrative
    /// - `extension` - Extensions that may be included in "extensions" status
    ///   narratives
    /// 
    /// ## Specification Reference
    /// 
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Narrative data type and narrative
    /// generation requirements.
    /// 
    pub text: Option<types::String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<types::String>,

    pub city: Option<types::String>,

    pub district: Option<types::String>,

    pub state: Option<types::String>,

    pub postal_code: Option<types::String>,

    pub country: Option<types::String>,

    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Address;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            use1: None,
            r#type: None,
            text: None,
            line: vec![],
            city: None,
            district: None,
            state: None,
            postal_code: None,
            country: None,
            period: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "line": []
                }
            );
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
