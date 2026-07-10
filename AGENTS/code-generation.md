# Code generation

This crate is spec-driven: the FHIR R5 specification JSON is the input, Rust is
the output. This document explains the generator so agents can extend it rather
than hand-writing model shapes. The normative contract is
[`../spec/08-code-generation.md`](../spec/08-code-generation.md).

## Inputs

`doc/fhir-specifications/r5/fhir-definitions-json/`:

| File | Produces |
| --- | --- |
| `profiles-types.json` | primitive + complex datatypes |
| `profiles-resources.json` | resources |
| `valuesets.json` | code-system enums (`r5::codes`) |
| `conceptmaps.json`, `search-parameters.json`, `dataelements.json`, `profiles-others.json` | supporting bundles |

## The parse layer

`r5::parse::<bundle>` mirrors each StructureDefinition bundle. Every bundle
module has:

- `Bundle`, `Entry`, `Resource`, `Element`, `Snapshot`, … structs that
  deserialize the spec JSON.
- `resource_into_rust(&Resource)` → writes a Rust file to `tmp/out/<snake>.rs`.
- `element_into_rust_struct_attribute(&Element)` → maps one FHIR element to one
  (or, for `[x]` choices, several) Rust struct fields.

`src/main.rs` runs the datatypes generator; `cargo run` regenerates
`tmp/out/`.

## The element → field mapping

`element_into_rust_struct_attribute` implements the rules in
[`conventions.md`](conventions.md):

1. Skip the root element (path has no `.`).
2. Resolve the FHIR type code → Rust type:
   - FHIRPath system primitive `http://hl7.org/fhirpath/System.X` → `types::X`.
   - Otherwise `types::Pascal(code)`.
3. Apply cardinality (`Option`/`Vec`/`T`).
4. Expand `[x]` choice elements into one field per type.
5. snake_case the name and escape Rust keywords (`r#type`, …).

**Known generator limitation:** it *flattens* nested backbone elements
(producing duplicate `id`/`extension` fields). For types/resources with
backbones, the real model instead defines named nested structs (see
[`architecture.md`](architecture.md)). This is why the shipped model was
authored with the generator's field mapping as a starting point, not by blind
regeneration.

## Generating code-system enums (`r5::codes`)

`r5::codes` is generated from complete `CodeSystem` entries in
`valuesets.json`: each becomes a Rust enum whose variants are the PascalCase
codes with `#[serde(rename = "<code>")]`. Variant names are sanitized (keyword
guard for `Self`, digit prefix `N`, de-duplication). See
[`../spec/05-code-systems.md`](../spec/05-code-systems.md).

## Extending the generator

- Prefer changing `element_into_rust_struct_attribute` (it improves every
  generated type at once) over editing individual output files.
- Keep the four parallel generator copies
  (`profiles_types`, `profiles_others`, `profiles_resources`,
  `search_parameters`) consistent, or consolidate them.
- After any generator change, run the parse tests and regenerate, then diff
  `tmp/out/` to confirm the output is well-formed Rust.

## Reproducible, deterministic output

Generation must be deterministic (same input → same output) so `tmp/out/` diffs
are meaningful. Do not introduce ordering that depends on hash-map iteration,
timestamps, or randomness.
