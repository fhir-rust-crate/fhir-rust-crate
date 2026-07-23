# Architecture

How the `fhir` crate is put together. Read this before making structural
changes. It harmonizes with the specifications in [`../spec/`](../spec/index.md);
where this describes *code layout*, the specs describe *required behaviour*.

## Two artifacts in one crate

`fhir` is simultaneously:

1. **A data model** — Rust types under `<release>::types`, `<release>::resources`,
   `<release>::codes` that serialize to and from canonical FHIR JSON via `serde`.
2. **A code generator** — under `codegen` — that reads the official FHIR
   specification JSON for a release and emits that release's Rust source. This
   is the spec-driven engine.

## Three layers

```text
crate root          release-independent: the traits, the wrappers, the generator
  ├── r5            the FHIR R5 model (feature `r5`, default)
  └── r4            the FHIR R4 model (feature `r4`)
```

**A release module owns only what the specification made release-specific**: the
types, the code enums, the metadata table, and the thin bindings that attach
shared machinery to them. Everything else lives at the crate root and is
re-exported, so `fhir::r5::validate::Validate` and `fhir::r4::validate::Validate`
are the *same trait* — one `#[derive(Validate)]` serves both, and a generic
function can validate either.

| Crate root | What each release adds |
| --- | --- |
| `validate` — `Validate`, `ValidationIssue`, container impls, format checks | its own primitive `impl`s and `OperationOutcome` bridge |
| `coded` — `Coded<E>` | nothing; re-exported as-is |
| `builder` — `BuilderError` | nothing; re-exported as-is |
| `meta` — `ElementMeta`, `TypeRef`, `BindingMeta`, lookups | its generated `ELEMENTS` table |
| `temporal` — `DateParts`, `TimeParts`, precision comparison | `impl`s on its own `Date`/`DateTime`/`Instant`/`Time` |
| `summary` — the pruning algorithm | binds it to its own table |
| `xml` — the FHIR XML bridge | binds it to its own table |
| `client` — `ReleaseClient<R>` | a `Client` type alias for its `Release` |
| `release` — the `Release` trait | a marker type (`R4`, `R5`) implementing it |
| `choice` | **not shared**: `Primitive<T>` holds that release's `Element` |

The rule of thumb: if the code names a release's types, it belongs in that
release's module; otherwise it belongs at the root.

## Why the releases are separate types

An R4 `Patient` and an R5 `Patient` model different things. `Observation.value[x]`
allows 11 types in R4 and 13 in R5. `MedicationRequest.medication[x]` is a choice
element in R4 and a `CodeableReference` in R5. R4 has no `integer64`,
`CodeableReference`, or `RatioRange` datatype; R5 has no `Contributor`,
`Population`, or `SubstanceAmount`. Sharing one Rust type across releases would
mean either a union of everything (accepting data that is invalid in both) or an
intersection (silently dropping data that is valid in one). Callers convert
explicitly, through JSON, and decide what to do with what does not carry over.

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

## A release module tree

Both releases have the same shape; the counts differ.

```text
<release>::
  types::        primitives + complex datatypes       R5: 21+50   R4: 20+43
  resources::    resources + `Resource` enum          R5: 158     R4: 146
  codes::        code-system enums                    R5: 419     R4: 486
  coded::        re-export of `crate::coded`
  choice::       `Primitive<T>` + `value[x]` choice-enum support
  validate::     that release's primitive-format constraints
  builder::      re-export of `crate::builder`
  meta::         its generated per-element metadata table
  temporal::     date/time parsing bound to its primitives
  summary::      `_summary=true` view over its table
  extension_ext:: `ExtensionExt` accessors for its types
  bundle_util::  `Bundle` iteration, paging, transaction building
  prelude::      the common imports
  client::       `Client` type alias (feature `client`)
  xml::          FHIR XML bridge over its table (feature `xml`)
  R4 / R5        the `Release` marker type
```

`r5` additionally keeps `parse::` (the original R5-only generator and its
splicing generators) and the `abstract_types`, `properties`, `resource`, `todo`
scaffolding. `r4` has none of that: it is generated in one pass.

### Layering (dependency direction)

```text
resources  ──▶ types ──▶ (primitives)
    │            │
    └──▶ codes   └──▶ validate (Validate impls for every type)
resources ──▶ serde_json::Value   (for `contained` / polymorphic slots)

every release ──▶ crate root (validate, coded, builder, meta, temporal, …)
```

Nothing points the other way: the crate root never names a release's types, so
either release can be compiled out.

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
doc/fhir-specifications/<release>/fhir-definitions-json/*.json
        │  codegen::spec  (permissive views of StructureDefinition / CodeSystem)
        ▼
codegen::codes_gen::plan   ──┐   (the enum names field typing depends on)
        │                    ▼
codegen::plan::plan_type ──▶ break_type_cycles ──▶ resolve_defaults
        │
        ▼  codegen::render / primitives / codes_gen / meta_gen / extension_ext_gen
src/<release>/{types,resources,codes.rs,extension_ext.rs,meta/generated.rs}
```

`Version` is the only thing that knows a release apart: which bundles to read,
which `src/` tree to write, and which release name the emitted code uses. Adding
a release should be a matter of extending `Version`.

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

`doc/fhir-specifications/<release>/fhir-definitions-json/` holds the official
bundles: `profiles-types.json` (datatypes), `profiles-resources.json`
(resources), `valuesets.json` (code systems), `conceptmaps.json`,
`search-parameters.json`, `dataelements.json`, `profiles-others.json`. These are
the ground truth the specs and the generator both draw from, and they are
committed so that generation is reproducible from a clean clone.

R4 additionally ships `extension-definitions.json`, `v2-tables.json`, and
`v3-codesystems.json`. The generator deliberately ignores the last two: they are
external HL7 v2/v3 terminologies rather than FHIR-defined ones, and no FHIR
element has a `required` binding into them.
