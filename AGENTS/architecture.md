# Architecture

How the `fhir` crate is put together. Read this before making structural
changes. It harmonizes with the specifications in [`../spec/`](../spec/index.md);
where this describes *code layout*, the specs describe *required behaviour*.

## Two artifacts in one crate

`fhir` is simultaneously:

1. **A data model** — Rust types under `r5::types`, `r5::resources`,
   `r5::codes` that serialize to and from canonical FHIR JSON via `serde`.
2. **A code generator** — under `r5::parse` — that reads the official FHIR R5
   specification JSON and emits Rust source. This is the spec-driven engine.

## Workspace

```text
fhir (root package)          library + binary
└── fhir-derive-macros              proc-macro crate: #[derive(Validate, FhirChoice, Builder)]
```

The root package depends on `fhir-derive-macros` by path. `fhir-derive-macros` uses `syn`,
`quote`, and `proc-macro2`; it is the only place procedural macros live. It
provides three derives: `Validate` (recursive + cardinality + invariant checks),
`FhirChoice` (`value[x]` enums), and `Builder` (required-field enforcement).

## Library vs binary

- `src/lib.rs` is the **library root**. It owns the module tree, the
  crate-level lint configuration (`#![warn(clippy::pedantic)]` plus documented
  allows), the `DEFINITIONS_DIR` static (path to the spec JSON), and the
  `SourceCodeString` type alias.
- `src/main.rs` is a **thin binary** that calls the generator.

The crate must remain a library (not binary-only): doctests only run for a
library target, and this crate has ~200 of them. Import path is `fhir::…`.

## The `r5` module tree

```text
r5::
  types::        primitives (21) + complex datatypes (50)
  resources::    resources (158) + `Resource` enum (tagged by resourceType)
  codes::        code-system enums (419)
  coded::        `Coded<E>` wrapper for required-binding codes
  choice::       `Primitive<T>` + the `value[x]` choice-enum support
  validate::     `Validate` trait, `ValidationIssue`, primitive checks
  meta::         per-element metadata table (cardinality, bindings, targets, …)
  temporal::     precision-aware date/time parsing
  summary::      `_summary=true` view
  xml::          FHIR XML bridge (feature `xml`)
  parse::        the generator (one submodule per spec bundle)
  abstract_types:: base/abstract FHIR interfaces (e.g. CanonicalResource)
  properties, resource, todo:: legacy/WIP scaffolding
```

### Layering (dependency direction)

```text
resources  ──▶ types ──▶ (primitives)
    │            │
    └──▶ codes   └──▶ validate (Validate impls for every type)
resources ──▶ serde_json::Value   (for `contained` / polymorphic slots)
```

- **Primitives** depend on nothing (newtypes over `String`/`bool`/numbers).
- **Complex datatypes** depend on primitives and each other.
- **Resources** depend on datatypes, on nested backbone structs defined in the
  same file, and on `serde_json::Value` for polymorphic `contained` fields.
- **`codes`** is wired into the model: required-binding fields are typed as
  their `codes` enum via `r5::coded::Coded<E>` (`Known | Unknown` fallback). See
  [`../spec/05-code-systems.md`](../spec/05-code-systems.md).
- **`validate`** is cross-cutting: `#[derive(Validate)]` on every struct/enum
  produces a recursive implementation.

## Data flow: spec JSON → Rust

```text
doc/…/fhir-definitions-json/*.json
        │  (serde_json::from_reader into parse::*::Bundle)
        ▼
r5::parse::<bundle>::resource_into_rust(&resource)
        │  (element_into_rust_struct_attribute maps each element)
        ▼
tmp/out/<snake>.rs   (generated Rust source, tracked in git)
```

The generator is described in
[`code-generation.md`](code-generation.md) and
[`../spec/08-code-generation.md`](../spec/08-code-generation.md).

## Nested backbone elements

FHIR "backbone elements" are anonymous nested structures (e.g.
`Timing.repeat`, `Patient.contact`). They become **named nested structs** in the
same module, named by concatenating the PascalCase path segments:

- `Timing.repeat` → `struct TimingRepeat`
- `Patient.contact` → `struct PatientContact`
- `Claim.item.detail` → `struct ClaimItemDetail`

The parent field references the nested struct directly (e.g.
`pub repeat: Option<TimingRepeat>`). This is a hard rule; see
[`conventions.md`](conventions.md).

## Where the spec JSON comes from

`doc/fhir-specifications/r5/fhir-definitions-json/` holds the official bundles:
`profiles-types.json` (datatypes), `profiles-resources.json` (resources),
`valuesets.json` (code systems), `conceptmaps.json`, `search-parameters.json`,
`dataelements.json`, `profiles-others.json`. These are the ground truth the
specs and the generator both draw from.
