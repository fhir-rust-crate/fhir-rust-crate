# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Common invariants (T14): `Validate` enforces `ext-1` (an extension has a value
  xor nested extensions) and `dom-2`/`dom-4` (rules on contained resources). A
  generated coverage report of all 314 constraint keys is committed at
  `spec/10-invariants-coverage.md`.
- `OperationOutcome` bridge (T15): `From<Vec<ValidationIssue>>` +
  `examples/operation_outcome.rs`.
- Deeper validation (T13): `Validate` now reports empty `1..*` elements (read
  from `meta` at runtime, since bare `Vec` is also used for some `0..*`) and
  required-binding codes outside the value set (a `Coded::Unknown`). See
  `spec/07-validation.md`.

### Changed (breaking)
- Coded fields with a `required` binding are now typed as their `codes` enum via
  the new `fhir::r5::coded::Coded<E>` wrapper (`Known(E)` | `Unknown(String)`
  fallback for wire compatibility), instead of the opaque `types::Code`. 343
  fields retyped. `Coded::code()` returns the wire string; `Coded::known()` the
  enum. See `spec/05-code-systems.md`.

### Added
- Typed references: `Reference` is now `Reference<T = Any>`, a phantom-typed
  newtype over the same wire form (`Reference<Any>` = the old untyped reference,
  so existing code is unaffected). Adds the `ResourceType` marker trait, the
  `Any` target, `.cast()`/`.into_any()`, and `.resolve(&bundle)` to look a
  reference up in a `Bundle`. (Machinery only; typing individual reference fields
  from `targetProfile` is a follow-up rollout.)
- `fhir::r5::temporal`: precision-aware parsing for the date/time primitives —
  `Date::parse_parts`/`DateTime::date_parts`/`Instant::date_parts`/`Time::parse_parts`,
  a `DateParts` type with FHIR precision ordering (`"2024"` vs `"2024-03"` is
  indeterminate), and `TimeParts`. Storage is unchanged (still `String`).
- `precise-decimal` feature: back `serde_json::Number` with arbitrary precision
  to preserve exact `decimal` values.

## [0.3.0] - 2026-07-11

### Changed (breaking)
- **Choice types (`value[x]`) are now enums.** Every FHIR choice element is
  modelled as a generated `#[derive(FhirChoice)]` enum (one variant per allowed
  type), held via `#[serde(flatten)]`, replacing the flattened `value_<type>`
  fields (e.g. `Observation.value_quantity`/`value_string`/… → `value:
  Option<ObservationValue>`). This makes "at most one" a compile-time property
  and models the paired `_value<Type>` primitive extensions. 258 choice elements
  across all datatypes and resources were converted by the `choice_gen`
  generator. See `spec/11-choice-types.md`.
  - Primitive choice variants use `fhir::r5::choice::Primitive<T>` to carry the
    `_value<Type>` extension; complex variants hold `Box<T>`.
  - The choice enums live in their type's module (e.g.
    `resources::observation::ObservationValue`,
    `types::extension::ExtensionValue`).
  - Deserialization is lenient (a malformed choice → `None`); see the spec for
    why strict rejection isn't possible under `flatten`.

### Added
- `fhir::r5::choice` module with `Primitive<T>` and the `FhirChoice` derive
  (`fhir-derive-macros`).

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

[Unreleased]: https://github.com/joelparkerhenderson/fhir-rust-crate/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/joelparkerhenderson/fhir-rust-crate/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/joelparkerhenderson/fhir-rust-crate/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/joelparkerhenderson/fhir-rust-crate/releases/tag/v0.1.0
