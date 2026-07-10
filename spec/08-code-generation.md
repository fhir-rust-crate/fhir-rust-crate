# 08 — Code generation

Defines the spec-JSON → Rust generator: the engine that makes this project
spec-driven. Operational how-to is in
[`../AGENTS/code-generation.md`](../AGENTS/code-generation.md).

## Background

The FHIR R5 specification is published as StructureDefinition JSON bundles. The
generator reads those bundles and emits Rust source, so the model can be
(re)derived from the upstream truth rather than hand-maintained.

## Inputs

- **R8.1** The generator MUST read from
  `doc/fhir-specifications/r5/fhir-definitions-json/`:
  `profiles-types.json` (datatypes), `profiles-resources.json` (resources),
  `valuesets.json` (code systems), and the supporting bundles
  (`conceptmaps.json`, `search-parameters.json`, `dataelements.json`,
  `profiles-others.json`).

## Parse layer

- **R8.2** Each bundle MUST have a `r5::parse::<bundle>` module whose structs
  (`Bundle`, `Entry`, `Resource`, `Element`, `Snapshot`, …) deserialize the
  spec JSON faithfully. A `test_serde_json_from_reader` test MUST deserialize
  the real bundle; a missing field (serde `unknown field`) is a defect to fix.
- **R8.3** `resource_into_rust(&Resource)` MUST emit a Rust source file for a
  StructureDefinition; output goes to `tmp/out/<snake>.rs` (tracked in git).

## Element → field mapping

- **R8.4** `element_into_rust_struct_attribute(&Element)` MUST implement the
  serialization rules of spec 06:
  1. Skip the root element (path without `.`).
  2. Map FHIR type code → Rust type: FHIRPath system primitive
     `http://hl7.org/fhirpath/System.X` → `types::X`; otherwise
     `types::Pascal(code)`.
  3. Apply cardinality (spec 06, R6.6).
  4. Expand `[x]` choices (spec 06, R6.10).
  5. snake_case names and escape Rust keywords (spec 06, R6.9).

## Determinism

- **R8.5** Generation MUST be deterministic: identical input yields byte-identical
  output. No dependence on hash-map iteration order, timestamps, or randomness.

## Known limitation

- **R8.6** The field mapper currently **flattens** nested backbone elements
  (yielding duplicate `id`/`extension` fields). The model requires named nested
  structs instead (spec 03 R3.4, spec 04 R4.3). Therefore the shipped model was
  authored using the field mapper as a starting point but with correct nested
  backbone structs. Making the generator emit nested structs directly is the
  primary generator improvement.

## Code-system generation

- **R8.7** `r5::codes` MUST be generated from `valuesets.json` per spec 05
  (complete CodeSystems, sanitized variant identifiers, serde renames, wrapped
  doc URLs).

## Future work

- Emit nested backbone structs from the generator (removes R8.6).
- Consolidate the four near-duplicate generator copies.
- Generate the `Resource` enum and the module-tree wiring.

## Acceptance criteria

- [ ] `cargo run` regenerates `tmp/out/*.rs` without error.
- [ ] Every `r5::parse::<bundle>` deserializes its real bundle
      (`test_serde_json_from_reader` passes).
- [ ] Generated field types, cardinalities, choice expansions, and keyword
      escaping match spec 06.
- [ ] Regeneration is deterministic (a second run produces no diff).
