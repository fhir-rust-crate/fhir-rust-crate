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

- **R7.4** The `fhir-derive-macros` crate MUST provide `#[derive(Validate)]` that
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
- [ ] `fhir-derive-macros` builds and the whole model derives `Validate` without
      clippy warnings.

## Cardinality and required-binding checks (T13)

Beyond primitive formats, `Validate` now reports two metadata-driven problems,
using the [`meta`](../src/r5/meta.rs) table:

- **Empty `1..*` elements.** A required repeating element must have at least one
  entry. Because bare `Vec<T>` is used for some `0..*` fields too, cardinality is
  read from `meta` at validation time (via `meta::struct_prefix` + the field's
  FHIR name), not inferred from the Rust type. Reported at the field's path.
- **Required-binding codes.** A `required`-binding field is typed
  `Coded<Enum>`; a value that fell back to `Coded::Unknown` is, by definition,
  outside the value set, and is reported at `<field>.code`.

### Acceptance criteria (T13)

- [x] An empty `Appointment.participant` (`1..*`) yields a pathed
      `participant` issue; a `0..*` empty Vec does not.
- [x] `Patient.gender = Coded::Unknown(...)` yields a `gender.code` issue.
