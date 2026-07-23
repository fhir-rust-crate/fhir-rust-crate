//! Generating the `codes` module: FHIR `CodeSystem`s as Rust enums.
//!
//! Only a `complete` code system can become an exhaustive enum — a `fragment`
//! or `example` system does not list all its codes, so a closed Rust enum would
//! reject valid data. Everything else stays an opaque `types::Code`.
//!
//! Even for complete systems the wire format stays authoritative: each variant
//! carries `#[serde(rename = "…")]` with the original code, and fields bound to
//! a system are wrapped in [`Coded`](crate::coded::Coded) so an unrecognized
//! code still round-trips. See `spec/05-code-systems.md`.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use super::naming;
use super::spec::CodeSystem;
use super::version::Version;

/// One code system, planned as a Rust enum.
#[derive(Debug, Clone)]
pub struct CodeEnum {
    /// The Rust enum name, e.g. `AdministrativeGender`.
    pub name: String,
    /// The code system's canonical URL.
    pub url: String,
    /// The system's prose description.
    pub description: String,
    /// The variants, in specification order.
    pub variants: Vec<CodeVariant>,
}

/// One variant of a generated code enum.
#[derive(Debug, Clone)]
pub struct CodeVariant {
    /// The Rust variant name, e.g. `EnteredInError`.
    pub name: String,
    /// The code as it appears on the wire, e.g. `entered-in-error`.
    pub code: String,
    /// The human-readable label, used as the variant's doc comment.
    pub display: String,
}

/// Plan an enum for every code system that can become one.
///
/// The enum is named after the *last segment of the system's URL*, not its
/// `name`, because that is what a value-set binding refers to: a field bound to
/// `…/ValueSet/observation-status` resolves to `ObservationStatus`.
/// Systems are considered in bundle order and the first claim on a name wins,
/// so the result does not depend on iteration order.
#[must_use]
pub fn plan(systems: &[CodeSystem]) -> Vec<CodeEnum> {
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut out = Vec::new();

    for system in systems {
        if system.content.as_deref() != Some("complete") {
            continue;
        }
        let Some(url) = system.url.as_deref() else { continue };
        let Some(segment) = url.rsplit('/').next() else { continue };
        let name = naming::enum_name(segment);
        if !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
            continue;
        }
        if !taken.insert(name.clone()) {
            continue;
        }

        let concepts = system.codes();
        if concepts.is_empty() {
            continue;
        }
        let raw: Vec<String> =
            concepts.iter().map(|c| naming::sanitize_variant(&c.code)).collect();
        let variants = naming::dedupe(&raw)
            .into_iter()
            .zip(concepts.iter())
            .map(|(variant_name, concept)| CodeVariant {
                name: variant_name,
                code: concept.code.clone(),
                display: concept
                    .display
                    .clone()
                    .or_else(|| concept.definition.clone())
                    .unwrap_or_else(|| concept.code.clone()),
            })
            .collect();

        out.push(CodeEnum {
            name,
            url: url.to_string(),
            description: system.description.clone().unwrap_or_default(),
            variants,
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Render the whole `codes` module.
#[must_use]
pub fn render(enums: &[CodeEnum], version: Version) -> String {
    let label = version.label();
    let module = version.module();
    let example = enums.iter().find(|e| e.name == "AdministrativeGender").or_else(|| enums.first());

    let mut out = format!(
        "//! FHIR {label} code systems as type-safe Rust enums.\n\
         //!\n\
         //! Each enum corresponds to a complete FHIR {label} `CodeSystem`. Variants\n\
         //! serialize to and from their canonical FHIR code strings via serde renames.\n\
         //!\n"
    );
    if let Some(example) = example
        && let Some(variant) = example.variants.first()
    {
        let _ = write!(
            out,
            "//! # Examples\n\
             //!\n\
             //! ```\n\
             //! use fhir::{module}::codes::{};\n\
             //!\n\
             //! let code = {}::{};\n\
             //! assert_eq!(::serde_json::to_value(&code).unwrap(), ::serde_json::json!({:?}));\n\
             //! ```\n",
            example.name, example.name, variant.name, variant.code,
        );
    }
    out.push_str("\nuse ::serde::{Deserialize, Serialize};\n");

    for code_enum in enums {
        out.push('\n');
        out.push_str(&render_enum(code_enum));
    }
    out
}

/// Render one code enum.
fn render_enum(code_enum: &CodeEnum) -> String {
    let mut out = String::new();
    if code_enum.description.is_empty() {
        out.push_str(&naming::doc_comment(&format!("The `{}` code system.", code_enum.url), ""));
    } else {
        out.push_str(&naming::doc_comment(&code_enum.description, ""));
    }
    out.push_str("///\n");
    let _ = writeln!(out, "/// System: <{}>", code_enum.url);
    out.push_str("#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]\n");
    let _ = writeln!(out, "pub enum {} {{", code_enum.name);
    for (index, variant) in code_enum.variants.iter().enumerate() {
        out.push_str(&naming::doc_comment(&variant.display, "    "));
        if index == 0 {
            // An enum needs a default so that `Coded<E>` and every struct
            // holding one can derive `Default`; the first code is as good a
            // choice as any and is stable across regeneration.
            out.push_str("    #[default]\n");
        }
        let _ = writeln!(out, "    #[serde(rename = {:?})]", variant.code);
        let _ = writeln!(out, "    {},", variant.name);
    }
    out.push_str("}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn system(json: ::serde_json::Value) -> CodeSystem {
        ::serde_json::from_value(json).unwrap()
    }

    #[test]
    fn complete_systems_become_enums() {
        let enums = plan(&[system(::serde_json::json!({
            "resourceType": "CodeSystem",
            "name": "ObservationStatus",
            "url": "http://hl7.org/fhir/observation-status",
            "description": "Codes providing the status of an observation.",
            "content": "complete",
            "concept": [
                { "code": "registered", "display": "Registered" },
                { "code": "entered-in-error", "display": "Entered in Error" }
            ]
        }))]);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].name, "ObservationStatus");
        assert_eq!(enums[0].variants[1].name, "EnteredInError");
        assert_eq!(enums[0].variants[1].code, "entered-in-error");
    }

    #[test]
    fn incomplete_systems_are_left_as_opaque_codes() {
        let enums = plan(&[system(::serde_json::json!({
            "resourceType": "CodeSystem", "url": "http://example.org/partial",
            "content": "fragment",
            "concept": [{ "code": "a" }]
        }))]);
        assert!(enums.is_empty());
    }

    #[test]
    fn empty_systems_are_skipped() {
        let enums = plan(&[system(::serde_json::json!({
            "resourceType": "CodeSystem", "url": "http://example.org/empty",
            "content": "complete"
        }))]);
        assert!(enums.is_empty());
    }

    #[test]
    fn the_first_claim_on_a_name_wins() {
        let enums = plan(&[
            system(::serde_json::json!({
                "resourceType": "CodeSystem", "url": "http://a.example/thing",
                "content": "complete", "concept": [{ "code": "first" }]
            })),
            system(::serde_json::json!({
                "resourceType": "CodeSystem", "url": "http://b.example/thing",
                "content": "complete", "concept": [{ "code": "second" }]
            })),
        ]);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].variants[0].code, "first");
    }

    #[test]
    fn colliding_variant_names_are_numbered() {
        let enums = plan(&[system(::serde_json::json!({
            "resourceType": "CodeSystem", "url": "http://example.org/case",
            "content": "complete",
            "concept": [{ "code": "OP" }, { "code": "op" }]
        }))]);
        let names: Vec<&str> = enums[0].variants.iter().map(|v| v.name.as_str()).collect();
        assert_eq!(names, ["Op", "Op2"]);
        // Both still round-trip to their own code.
        assert_eq!(enums[0].variants[0].code, "OP");
        assert_eq!(enums[0].variants[1].code, "op");
    }

    #[test]
    fn rendering_marks_a_default_and_renames_every_variant() {
        let enums = plan(&[system(::serde_json::json!({
            "resourceType": "CodeSystem", "url": "http://hl7.org/fhir/administrative-gender",
            "description": "The gender of a person.",
            "content": "complete",
            "concept": [{ "code": "male", "display": "Male" }]
        }))]);
        let out = render(&enums, Version::R4);
        assert!(out.contains("pub enum AdministrativeGender {"));
        assert!(out.contains("    #[default]\n"));
        assert!(out.contains("#[serde(rename = \"male\")]"));
        assert!(out.contains("/// System: <http://hl7.org/fhir/administrative-gender>"));
        assert!(out.contains("use fhir::r4::codes::AdministrativeGender;"));
    }
}
