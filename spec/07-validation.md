# 07 — Validation

Defines the `Validate` trait, its primitive checks, and the
`#[derive(Validate)]` procedural macro.

## Background

Beyond Rust's type system, FHIR imposes constraints (primitive regexes,
cardinality, invariants). This crate provides a lightweight, dependency-free
validation layer that walks a value and reports issues.

## Requirements

### The trait

- **R7.1** `r5::validate` MUST define:

  ```rust
  pub struct ValidationIssue { pub path: String, pub message: String }
  pub trait Validate {
      fn validate(&self) -> Vec<ValidationIssue>;
      fn is_valid(&self) -> bool { self.validate().is_empty() }
  }
  ```

  An empty result means valid.

- **R7.2** Blanket impls MUST cover `Option<T>`, `Vec<T>`, and `Box<T>` for
  `T: Validate`, and `::serde_json::Value` and `String` (both structurally
  valid).

### Primitive checks

- **R7.3** Primitives MUST implement `Validate` with their FHIR format
  constraint where one exists:
  - `code`: non-empty, trimmed, single internal spaces
    (`[^\s]+(\s[^\s]+)*`).
  - `id`: 1..=64 chars from `[A-Za-z0-9-.]`.
  - `oid`: begins `urn:oid:`; `uuid`: begins `urn:uuid:`.
  - `uri`, `canonical`, `url`: non-empty and not surrounded by whitespace.
  - Other primitives are valid by construction (no extra check yet).

### The derive macro

- **R7.4** The `fhir-derive` crate MUST provide `#[derive(Validate)]` that
  generates a recursive `crate::r5::validate::Validate` implementation:
  - For a **struct**: validate every field; prefix each returned issue's `path`
    with the field name, dot-separated (e.g. a bad `Coding.code` yields path
    `code.code`).
  - For an **enum**: match the active variant and validate its data; unit
    variants produce no issues.
- **R7.5** Every complex datatype, resource, nested backbone struct, and the
  `Resource` enum MUST `#[derive(Validate)]`. Primitives implement it by hand
  (R7.3), so they MUST NOT derive it.
- **R7.6** Validation MUST NOT allocate or fail for valid values beyond the
  returned `Vec` (no panics, no I/O).

## Future work

- A `#[derive(Validate)]` field attribute for cardinality/invariant metadata.
- FHIRPath-based invariant (`constraint`) evaluation.
- Date/time/base64 format checks for the remaining primitives.

## Acceptance criteria

- [ ] `Id("patient-1").is_valid()` is true; `Id("bad id!").is_valid()` is false.
- [ ] A `Coding` with `code = "bad  code"` yields exactly one issue with path
      `code.code`.
- [ ] A default-constructed resource validates with no panics.
- [ ] `fhir-derive` builds and the whole model derives `Validate` without
      clippy warnings.
