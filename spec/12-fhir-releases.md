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
  the release in lowercase: `fhir::r4`, `fhir::r5`.
- **R12.2** A release module MUST expose the same public shape as every other,
  so that porting code between releases is a matter of changing one path
  segment: `types`, `resources`, `codes`, `validate`, `meta`, `choice`, `coded`,
  `builder`, `temporal`, `summary`, `extension_ext`, `bundle_util`, `prelude`,
  and, under the matching features, `client` and `xml`.
- **R12.3** Each release module MUST also export a marker type named for the
  release (`R4`, `R5`) implementing [`Release`](../src/release.rs), so that code
  can be written generically over a release without naming its types.
- **R12.4** The releases MUST NOT share model types. An R4 `Patient` and an R5
  `Patient` are distinct Rust types, and no `From`/`Into` conversion between
  them is provided.

### Why R12.4

A shared type would have to be either a union of both releases' elements or an
intersection of them. A union accepts data that is invalid in both releases and
silently permits writing an R5-only element to an R4 server. An intersection
silently drops data that is valid in the release it came from. Both failure
modes are silent, and both corrupt health records. Distinct types make the
mismatch a compile error instead.

Callers that must convert do so explicitly, through JSON, and decide what to do
with whatever does not carry over. Serde reports what the target release will
not accept rather than discarding it.

## Compiling only what you use

- **R12.5** Each release MUST be behind a cargo feature named for it (`r4`,
  `r5`). A release that is not enabled MUST NOT be compiled.
- **R12.6** The default feature set MUST be `["r5"]`, so that callers who
  predate multi-release support are unaffected and pay nothing for R4.
- **R12.7** Every release feature MUST stand alone: the crate MUST build, test,
  and document with any non-empty subset of the release features, including
  `--no-default-features --features r4`.

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
write `fn check<T: Validate>(v: &T)` that accepts R4 and R5 values alike.

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
- **R12.15** `src/r4` MUST be fully generated and MUST NOT be hand-edited.
- **R12.16** `src/r5` carries hand-written documentation and MUST NOT be
  regenerated over. The generator MUST refuse to write there without an explicit
  output directory.

The asymmetry in R12.15/R12.16 is historical, not principled: R5 was authored
before the generator could emit a finished model. It is recorded here because
the two trees must be edited differently.

## Acceptance criteria

1. `cargo build`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`
   are clean for each of: default (`r5`), `--features r4`, and
   `--no-default-features --features r4`.
2. `fhir::r4` and `fhir::r5` each export every module listed in R12.2.
3. `fhir::r5::validate::Validate` and `fhir::r4::validate::Validate` resolve to
   the same trait (a generic function bounded on one accepts values of both).
4. No module under `src/r4` or `src/r5` duplicates a crate-root implementation
   rather than re-exporting it.
5. `cargo run -- r5` without `--out` exits non-zero without writing to `src/r5`.
6. `cargo run -- r4` is idempotent: running it twice leaves `git status` clean.
7. The curated official examples for every release round-trip exactly.

## Future work

- R6 when it is published, and R4B, which the release table would accommodate
  without structural change.
- A `Release`-generic façade over the common resources, for callers that handle
  both releases and only need the elements the releases agree on.
- Machine-checked conversion between releases, driven by the official
  cross-version extension maps rather than by hand-written rules.
