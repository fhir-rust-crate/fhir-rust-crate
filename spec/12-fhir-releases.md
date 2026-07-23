# 12 — FHIR releases

Defines how this crate models more than one FHIR release at once: where each
release lives, what it may and may not share, and what a caller is guaranteed
when moving data between them.

## Background

FHIR is versioned, and its releases are not compatible with one another. R4
(4.0.1) and R5 (5.0.0) disagree about which elements exist, what they are
called, which types a choice element admits, and which datatypes exist at all.
Real deployments are split across both: R4 remains the most widely implemented
release while R5 is the current normative one, so a Rust FHIR library that
models only one of them is unusable to half its potential callers.

The design question is whether to model the union of the releases in one set of
types or to model each release separately. This crate models them separately.

## Modelling one release per module

- **R12.1** Each supported release MUST have its own top-level module named for
  the release in lowercase: `fhir::r3`, `fhir::r4`, `fhir::r5`.
- **R12.2** A release module MUST expose the same public shape as every other,
  so that porting code between releases is a matter of changing one path
  segment: `types`, `resources`, `codes`, `validate`, `meta`, `choice`, `coded`,
  `builder`, `temporal`, `summary`, `extension_ext`, `bundle_util`, `prelude`,
  and, under the matching features, `client` and `xml`.
- **R12.3** Each release module MUST also export a marker type named for the
  release (`R3`, `R4`, `R5`) implementing [`Release`](../src/release.rs), so
  that code can be written generically over a release without naming its types.
- **R12.4** The releases MUST NOT share model types. An R3, R4 and R5 `Patient`
  are three distinct Rust types, and no `From`/`Into` conversion between them is
  provided.

### Why R12.4

A shared type would have to be either a union of the releases' elements or an
intersection of them. A union accepts data that is invalid in every release and
silently permits writing an R5-only element to an R3 server. An intersection
silently drops data that is valid in the release it came from. Both failure
modes are silent, and both corrupt health records. Distinct types make the
mismatch a compile error instead.

The disagreements are not marginal. `Observation.value[x]` admits eleven types
in R3 and eleven in R4 — but not the same eleven: R3 allows `Attachment` and not
`integer`, and R4 reversed both. R5 then allows all of them plus `Reference`.
A resource's `id` is typed `id` in R3 and `string` in R4/R5. `Extension.url` is
a `uri` in R3 and a `string` afterwards. R3 has no `canonical` or `url`
primitive at all.

Callers that must convert do so explicitly, through JSON, and decide what to do
with whatever does not carry over. Serde reports what the target release will
not accept rather than discarding it.

## Compiling only what you use

- **R12.5** Each release MUST be behind a cargo feature named for it (`r3`,
  `r4`, `r5`). A release that is not enabled MUST NOT be compiled.
- **R12.6** The default feature set MUST be `["r5"]`, so that callers who
  predate multi-release support are unaffected and pay nothing for R4.
- **R12.7** Every release feature MUST stand alone: the crate MUST build, test,
  and document with any non-empty subset of the release features, including
  `--no-default-features --features r3`.

A release model is ~135,000 lines of generated Rust. Compiling both roughly
doubles build time, which is why this is opt-in rather than always-on.

## Sharing what does not vary

- **R12.8** Code that does not name a release's types MUST live once at the
  crate root and be re-exported by each release module, not copied per release.
  This covers at least: the `Validate` trait and `ValidationIssue`; `Coded<E>`;
  `BuilderError`; the `meta` table types and lookups; date/time parsing;
  summary pruning; the XML bridge; and the REST client.
- **R12.9** A re-export MUST preserve the release-scoped path. `fhir::r5::validate::Validate`
  MUST continue to resolve, and MUST be the same trait as
  `fhir::r4::validate::Validate`.
- **R12.10** `choice::Primitive<T>` is exempt from R12.8: it holds that
  release's `Element`, so it is defined per release.

### Consequence

Because R12.9 makes `Validate` one trait rather than two, a single
`#[derive(Validate)]` implementation serves every release, and a caller can
write `fn check<T: Validate>(v: &T)` that accepts values of every release
alike.

## Naming a release in generated code

- **R12.11** The derive macros MUST resolve release-specific paths (the `meta`
  table, `types::Element`, `choice::Primitive`) through a
  `#[fhir_version("<release>")]` attribute on the type.
- **R12.12** The attribute MUST default to `r5` when absent, so that existing R5
  code needs no change.
- **R12.13** An unknown release in that attribute MUST be a compile error, not a
  silent fallback.

## Generation

- **R12.14** Everything the generator does that differs between releases MUST be
  reachable from `codegen::Version`: the definition directory, the output
  directory, the module name, the release label, and the specification URL.
- **R12.15** `src/r3` and `src/r4` MUST be fully generated and MUST NOT be
  hand-edited. (Their hand-written support modules, which bind shared machinery
  to the generated types, are exempt and listed in `src/<release>.rs`.)
- **R12.16** `src/r5` carries hand-written documentation and MUST NOT be
  regenerated over. The generator MUST refuse to write there without an explicit
  output directory.

The asymmetry in R12.15/R12.16 is historical, not principled: R5 was authored
before the generator could emit a finished model. It is recorded here because
the two trees must be edited differently.

## Acceptance criteria

1. `cargo build`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`
   are clean for the default (`r5`), for every release enabled together, and for
   each release on its own (`--no-default-features --features r3`).
2. `fhir::r3`, `fhir::r4` and `fhir::r5` each export every module listed in
   R12.2.
3. Every release's `validate::Validate` resolves to the same trait (a generic
   function bounded on it accepts values of all of them).
4. No release module duplicates a crate-root implementation rather than
   re-exporting it.
5. `cargo run -- r5` without `--out` exits non-zero without writing to `src/r5`.
6. `cargo run -- r3` and `cargo run -- r4` are idempotent: running either twice
   leaves `git status` clean.
7. The curated official examples for every release round-trip exactly.

## Reading releases the specification spells differently

- **R12.17** The generator MUST tolerate the ways older releases write the same
  fact, and normalize them at the input boundary rather than downstream:
  - `ElementDefinition.type.targetProfile` is a single string in R3 and a list
    in R4/R5.
  - A binding's value set is `valueSet` (canonical) in R4/R5, and
    `valueSetReference` (a `Reference`) or `valueSetUri` in R3.
  - `ElementDefinition.type.code` is absent on a primitive's own `value`
    element in R3, where the type is carried only by an extension.
  - R4/R5 mark FHIR infrastructure elements (`<Type>.id`, `Extension.url`) with
    a FHIRPath system type; R3 types them as ordinary primitives. Which elements
    are infrastructure MUST therefore be decided structurally, not from the type
    code, so that none of them sprouts a spurious `_field` sibling (spec 09,
    R9.5).
- **R12.18** A definition that fails to parse MUST stop generation, not be
  skipped. Skipping one silently removes a whole resource from the model.

## Future work

- R6 when it is published, and R4B, which the release table would accommodate
  without structural change.
- A `Release`-generic façade over the common resources, for callers that handle
  several releases and only need the elements they agree on.
- Machine-checked conversion between releases, driven by the official
  cross-version extension maps rather than by hand-written rules.
