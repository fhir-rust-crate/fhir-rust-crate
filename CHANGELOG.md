# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI: build, test + doctest, `clippy -D warnings`,
  `doc -D warnings`, and `cargo publish --dry-run`.
- Official-examples round-trip test suite (`tests/roundtrip_official_examples.rs`)
  with a committed curated subset and an `#[ignore]` full-set run, plus
  `bin/fetch-examples` to download the full official R5 example set.
- `CHANGELOG.md`, `CONTRIBUTING.md`, and AI-readable `llms.txt` / `llms.json`
  crate summaries.
- `rust-version` (MSRV) declared as `1.88` on both packages.

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

[Unreleased]: https://github.com/joelparkerhenderson/fhir-rust-crate/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/joelparkerhenderson/fhir-rust-crate/releases/tag/v0.1.0
