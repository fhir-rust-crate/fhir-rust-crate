# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-07-11

### Added
- **Primitive extensions (`_field`).** Every FHIR primitive element now has a
  sibling field `<field>_ext` (serde-renamed to the `_field` key) of type
  `Element` — `Option<Element>` for scalars, `Option<Vec<Option<Element>>>` for
  repeating primitives — so `id`/`extension` on primitive values round-trip
  instead of being dropped. Applied across all datatypes and resources via a new
  metadata-driven generator (`src/r5/parse/siblings.rs`). See
  `spec/09-primitive-extensions.md`. `#[derive(Validate)]` recurses these
  siblings automatically. New example `examples/primitive_extensions.rs`.
- **Element metadata table** (`fhir::r5::meta`): a compile-time, path-keyed table
  of per-element cardinality, coded-value bindings, `value[x]` choice types,
  reference target profiles, and summary membership, generated from the spec
  (`src/r5/parse/meta.rs`).
- GitHub Actions CI: build, test + doctest, `clippy -D warnings`,
  `doc -D warnings`, MSRV (1.88), llms sync check, and `cargo publish --dry-run`.
- Official-examples round-trip test suite (`tests/roundtrip_official_examples.rs`)
  with a committed curated subset and an `#[ignore]` full-set run, plus
  `bin/fetch-examples`. Full-set round-trip improved from 2760 to 2780 passing.
- `CHANGELOG.md`, `CONTRIBUTING.md`, and AI-readable `llms.txt` / `llms.json`.
- `rust-version` (MSRV) declared as `1.88` on both packages.

### Note

- The `_field` rollout adds many optional fields; construct resources with
  `..Default::default()` (as the examples do) to stay forward-compatible.

## [0.1.0] - 2026-07-11

Initial release: the complete FHIR R5 (5.0.0) data model in idiomatic,
`serde`-serializable Rust, generated from the official specification JSON.

### Added
- **158 R5 resources** as Rust structs under `fhir::r5::resources`, each
  round-tripping to and from canonical FHIR JSON via `serde`, plus a polymorphic
  `Resource` enum tagged by `resourceType`.
- **21 primitive datatypes** (transparent newtypes such as `Code`, `Id`,
  `DateTime`) and **50 complex datatypes** (`Period`, `HumanName`,
  `CodeableConcept`, …) under `fhir::r5::types`.
- **400+ FHIR `CodeSystem`s** as type-safe enums under `fhir::r5::codes`.
- **Lightweight validation** via the `Validate` trait and `#[derive(Validate)]`
  (`fhir-derive-macros`), reporting pathed `ValidationIssue`s and checking
  primitive format constraints recursively.
- **A spec-driven code generator** under `fhir::r5::parse` that reads the FHIR
  specification JSON shipped in `DEFINITIONS_DIR` and emits the Rust model.
- Runnable examples: `build_patient`, `validate_resource`, `read_bundle`,
  `code_systems`.

[Unreleased]: https://github.com/joelparkerhenderson/fhir-rust-crate/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/joelparkerhenderson/fhir-rust-crate/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/joelparkerhenderson/fhir-rust-crate/releases/tag/v0.1.0
