//! Annotation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Annotation
//!
//! Version: 5.0.0
//!
//! Annotation Type: A  text note which also  contains information about who made the statement and when.
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
pub struct Annotation {
    /// # author
    /// 
    /// ## Description
    /// 
    /// The `author` property identifies the individual, organization, or system responsible for creating, authoring, or originating a resource. It provides attribution and accountability for the content or data within the resource.
    /// 
    /// ## Purpose
    /// 
    /// - Establish accountability and responsibility for resource content
    /// - Support attribution requirements for clinical and research data
    /// - Enable contact and communication with content creators
    /// - Provide audit trail for resource authorship
    /// - Support workflow and approval processes
    /// 
    /// ## Usage
    /// 
    /// The `author` property appears in various FHIR resources including clinical documents, knowledge artifacts, and data collection resources. It typically references a Practitioner, Organization, Device, or Patient who created or is responsible for the content.
    /// 
    /// ## Data Type
    /// 
    /// **Reference** to Practitioner | PractitionerRole | Organization | Device | Patient | RelatedPerson
    /// 
    /// May also appear as **ContactDetail** in metadata resources
    /// 
    /// ## Constraints
    /// 
    /// - Must reference a valid FHIR resource of the appropriate type
    /// - Should be resolvable if provided as a Reference
    /// - Multiple authors are typically supported through arrays
    /// - Should represent the actual author, not just a data entry person
    /// 
    /// ## Examples
    /// 
    /// ### Clinical Document Author
    /// ```json
    /// {
    ///   "author": [
    ///     {
    ///       "reference": "Practitioner/dr-johnson",
    ///       "display": "Dr. Sarah Johnson, MD"
    ///     }
    ///   ]
    /// }
    /// ```
    /// 
    /// ### Knowledge Resource with Multiple Authors
    /// ```json
    /// {
    ///   "author": [
    ///     {
    ///       "name": "Clinical Guidelines Committee",
    ///       "telecom": [
    ///         {
    ///           "system": "email",
    ///           "value": "guidelines@hospital.org"
    ///         }
    ///       ]
    ///     },
    ///     {
    ///       "name": "Dr. Michael Smith",
    ///       "telecom": [
    ///         {
    ///           "system": "email", 
    ///           "value": "msmith@hospital.org"
    ///         }
    ///       ]
    ///     }
    ///   ]
    /// }
    /// ```
    /// 
    /// ## Related Keys
    /// 
    /// - `subject` - The focus or subject of the resource
    /// - `performer` - Who performed an action or procedure
    /// - `contact` - Contact information for the resource
    /// - `publisher` - Organization responsible for publishing
    /// - `editor` - Those who edited or reviewed the content
    /// 
    /// ## Specification Reference
    /// 
    /// FHIR R5: [Resource Attribution](http://hl7.org/fhir/R5/) (varies by specific resource type)
    ///
    /// This is the `Reference` variant of the `author[x]` choice element,
    /// serialized as `authorReference`. Cardinality 0..1.
    ///
    /// « Reference( Practitioner | PractitionerRole | Patient | RelatedPerson | Organization ) »
    ///
    pub author_reference: Option<types::Reference>,

    /// # authorString
    ///
    /// The `string` variant of the `author[x]` choice element, serialized as
    /// `authorString`. Used when the author is named as free text rather than
    /// referenced as a resource. Cardinality 0..1.
    ///
    /// See [`Self::author_reference`] for the full description of authorship.
    ///
    pub author_string: Option<types::String>,

    /// # time
    ///
    /// The `time` element records when this annotation was made. Cardinality
    /// 0..1, data type `dateTime`.
    ///
    pub time: Option<types::DateTime>,

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
    pub text: types::Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Annotation;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            author_reference: None,
            author_string: None,
            time: None,
            text: types::Markdown::default(),
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
