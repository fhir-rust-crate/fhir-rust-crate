# Tasks — `fhir` improvement plan

Executable task list for `plan.md`. Ordered; respect the `Depends` column.
Each task is sized for one focused Claude (Opus) session unless marked EPIC
(multi-session; has sub-tasks).

Conventions for the executing session:

- **Verify** (every task): `cargo build && cargo test && cargo test --doc &&
  cargo clippy --all-targets` clean; `cargo doc --no-deps` zero warnings.
  After T2 lands, also: `cargo test --test roundtrip_official_examples`.
- **Branch + commit** a baseline before any mass edit; commit again after
  verification. Never leave large uncommitted work while agents run.
- **Mass edits across `src/r5/{types,resources}`** must use Read+Edit-only
  agents (no Bash) or generator output — see memory note
  `parallel-file-edit-agents-no-bash`.
- Breaking changes land only in the phase's designated version bump.

---

## Phase 0 — Infrastructure & trust

### T1. GitHub Actions CI
- **Do:** `.github/workflows/ci.yml`: jobs for (a) `cargo build --all-targets`,
  (b) `cargo test` + `cargo test --doc`, (c) `cargo clippy --all-targets -- -D warnings`,
  (d) `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`,
  (e) `cargo publish --dry-run -p fhir-derive-macros` and `--dry-run` for `fhir`
  (allow the parent dry-run to be non-blocking until fhir-derive-macros is on
  crates.io — it fails registry resolution until then). Cache cargo. Add badge
  to README.
- **Accept:** workflow file lints (`act` optional); README badge present.
- **Depends:** —

### T2. Official-examples round-trip test suite  ⭐ highest value
- **Do:** Fetch FHIR R5 `examples-json.zip`
  (https://hl7.org/fhir/R5/examples-json.zip) into
  `doc/fhir-specifications/r5/fhir-examples-json/` (git-LFS or a
  `cargo xtask fetch-examples` script + .gitignore, decide by repo-size
  budget; ~100MB unzipped → prefer fetch-script + CI download).
  Add `tests/roundtrip_official_examples.rs`: for each `*.json`, parse to
  `fhir::r5::resources::Resource`, re-serialize, assert
  `serde_json::Value` equality. Provide an `#[ignore]`-by-default full run +
  a committed curated subset (~50 diverse files) always-on.
- **Accept:** curated subset passes; full-run failures are enumerated in
  `tasks-roundtrip-failures.md` (this becomes the Phase 1/2 burn-down list).
- **Depends:** —

### T3. Publish hygiene
- **Do:** Fix root `Cargo.toml` `include` (remove or create `llms.txt`,
  `llms.json`; keep `LICENSE.md`, README). Add `[package.metadata.docs.rs]`
  (all-features). Create `CHANGELOG.md` (0.1.0 entry summarizing current
  state) and `CONTRIBUTING.md` (build/test commands, generator workflow,
  doc conventions from AGENTS.md). Confirm `cargo publish --dry-run
  -p fhir-derive-macros` green.
- **Accept:** dry-run green; no missing-file include warnings.
- **Depends:** —

### T4. Property-based round-trip tests
- **Do:** dev-dep `proptest`. Arbitrary-ish generators for 5 representative
  types (Patient, Observation, Bundle, CodeableConcept, Timing) producing
  populated values; assert serde round-trip equality.
- **Accept:** `cargo test proptest_` passes, ≥5 types covered.
- **Depends:** —

### T5. MSRV + toolchain policy
- **Do:** Determine minimum supported Rust (edition 2024 ⇒ likely 1.85+);
  set `rust-version` in both Cargo.tomls; add CI job on that toolchain.
- **Accept:** CI green on MSRV.
- **Depends:** T1

---

## Phase 1 — Primitive extensions (`_field`)  → version 0.2

### T6. Prototype `_field` siblings on 3 resources  (decision task)
- **Do:** On Patient, Observation, Questionnaire: add
  `pub _birth_date: Option<types::Element>`-style siblings (serde rename
  `_birthDate`) for scalar primitives, and `Option<Vec<Option<Element>>>` for
  repeating primitives (FHIR aligns arrays with nulls). Also prototype the
  wrapper-type alternative (`FhirField<T>`) on a branch for comparison.
  Run curated round-trip subset incl. examples with `_field` data.
- **Accept:** written comparison in `spec/09-primitive-extensions.md` with a
  decision; chosen prototype passes round-trip on `_field` examples.
- **Depends:** T2

### T7. EPIC: roll `_field` across the model
- **Do:** Apply the T6 decision to all ~208 structs. Preferred route:
  extend the generator to emit the siblings and use it to produce a
  mechanical edit list; else Read+Edit-only agent fan-out with an exact
  per-file spec. Update `#[derive(Validate)]` to recurse `_field` elements.
- **Accept:** full official-examples run: zero failures attributable to
  dropped `_field`; CHANGELOG 0.2 entry; version bump.
- **Depends:** T6
- **Sub-tasks:** T7a types (~50), T7b resources A–M, T7c resources N–Z,
  T7d validate-derive update, T7e docs+example `examples/primitive_extensions.rs`.

---

## Phase 2 — Type safety  → version 0.3

### T8. Generator: extract element metadata (bindings, choice lists, targetProfiles)
- **Do:** In `src/r5/parse/`, surface per-element: binding strength +
  valueSet URL, `value[x]` type lists, reference targetProfiles, isSummary,
  cardinality. Emit as a queryable table (JSON in `tmp/out/` + a
  `src/r5/meta.rs` lookup) — foundation for T9–T12 and Phase 3.
- **Accept:** unit tests assert known facts (Patient.gender binding=required
  → administrative-gender; Observation.value[x] has 11 types;
  Observation.subject targets Patient|Group|Device|Location…).
- **Depends:** —

### T9. EPIC: choice types as enums
- **Do:** Generate `ObservationValue { Quantity(..), String(..), … }`-style
  enums (custom Serialize/Deserialize keyed by `value<Type>` field names, incl.
  paired `_value<Type>`), replace flattened fields. Keep
  `#[deprecated]` accessor methods mirroring old field names where cheap.
- **Accept:** official-examples full run green; compile-time "exactly one"
  enforced; CHANGELOG migration notes; version bump 0.3.
- **Depends:** T7, T8
- **Sub-tasks:** T9a enum codegen + serde impls with unit tests;
  T9b apply to datatypes; T9c resources A–M; T9d resources N–Z;
  T9e docs/examples update (`examples/build_patient.rs` etc. compile).

### T10. Coded fields use `codes::` enums (required bindings only)
- **Do:** For elements with binding strength `required` whose CodeSystem enum
  exists in `codes.rs`: switch field type from `types::Code` to the enum.
  Add a fallback variant policy first (decision: add
  `#[serde(untagged)] Other(String)`-style variant vs closed enum → document
  in `spec/05-code-systems.md`; recommend fallback variant for wire
  compatibility).
- **Accept:** examples suite green; ≥100 fields migrated; docs updated.
- **Depends:** T8, T9 (avoid double-churn on the same structs)

### T11. Typed references
- **Do:** `Reference<T: ResourceType = Any>` newtype-with-phantom over the
  existing struct; generator picks `Reference<Patient>` where targetProfile
  is a single type, `Reference<Any>` otherwise. `Deref` to untyped;
  `.resolve(&bundle)` helper.
- **Accept:** examples suite green; doctest showing typed resolve.
- **Depends:** T8

### T12. Primitive value APIs
- **Do:** Keep `String` storage; add `Date::parse_parts() -> (year, Option<month>, Option<day>)`,
  ordering per FHIR precision rules, `DateTime`/`Instant`/`Time` equivalents;
  `Decimal` behind `serde_json/arbitrary_precision` audit (feature `precise-decimal`).
- **Accept:** unit tests for precision edge cases (e.g. `"2024"`, `"2024-03"`);
  no representation change (round-trip untouched).
- **Depends:** —

---

## Phase 3 — Validation depth  → version 0.4

### T13. Cardinality + required-binding validation
- **Do:** Using T8 metadata: `Validate` reports empty 1..* Vecs, and
  code-not-in-valueset for required bindings (via `codes.rs` enums / a
  generated membership set).
- **Accept:** unit tests: invalid Patient.gender code and empty
  `CodeSystem.concept` (1..*) each yield a pathed issue.
- **Depends:** T8, T10

### T14. Common-invariant subset
- **Do:** Generator recognizes ~5 recurring constraint shapes from
  ElementDefinition.constraint (e.g. "shall have value or children",
  xor-pairs) and emits checks; unrecognized constraints are listed (not
  silently dropped) in a generated report.
- **Accept:** ≥3 invariant classes enforced with tests; coverage report
  committed at `spec/10-invariants-coverage.md`.
- **Depends:** T13

### T15. `OperationOutcome` bridge
- **Do:** `impl From<Vec<ValidationIssue>> for OperationOutcome` + severity
  mapping; example `examples/operation_outcome.rs`.
- **Accept:** doctest + example run.
- **Depends:** T13

---

## Phase 4 — Ergonomics  → version 0.5

### T16. Builders (generated)
- **Do:** `#[derive(Builder)]`-style generated builders (own proc-macro or
  generator-emitted `impl`), required 1..1 fields enforced at `build()`.
  Start: 10 most-used resources (Patient, Observation, Encounter, Condition,
  MedicationRequest, Practitioner, Organization, Bundle, DiagnosticReport,
  AllergyIntolerance) + all general-purpose datatypes.
- **Accept:** `examples/build_patient.rs` rewritten with builder; doctests.
- **Depends:** T9 (builder API shaped by choice enums)

### T17. Prelude + extension helpers
- **Do:** `fhir::prelude` (Resource, top resources, common types, Validate,
  codes commonly used); `ExtensionExt` trait: `extension(url)`,
  `extensions(url)`, `set_extension`, modifier-extension accessor.
- **Accept:** doctests; `examples/extensions.rs`.
- **Depends:** —

### T18. Bundle utilities + typed `contained`
- **Do:** `Bundle::resources::<T>() -> impl Iterator<&T>`, transaction/batch
  builder, `next`-link paging helper; change `contained` fields to
  `Option<Vec<Resource>>` across resources (generator/agent fan-out) with
  local-reference resolution helper.
- **Accept:** examples suite green (contained is exercised heavily there);
  `examples/transaction_bundle.rs`.
- **Depends:** T2 (oracle), T9 landed (avoid churn)

---

## Phase 5 — Interop  → version 0.6

### T19. REST client (feature `client`)
- **Do:** `fhir::client::Client` (reqwest, tokio): read/vread/create/update/
  delete/search + capability fetch; error → `OperationOutcome`; generated
  search-parameter builder from bundled `search-parameters.json` (typed
  params per resource). Integration tests against the public HAPI test
  server behind `#[ignore]`.
- **Accept:** `examples/client_crud.rs` (feature-gated) runs against HAPI;
  unit tests with a mock server (wiremock).
- **Depends:** T8 (search params), T15

### T20. Summary serialization
- **Do:** isSummary metadata (T8) → `to_summary_value(&self)` or a serializer
  wrapper emitting only summary elements + mandatory ones.
- **Accept:** Patient summary matches spec's `_summary` semantics on
  examples; doctest.
- **Depends:** T8

### T21. EPIC: XML support (feature `xml`)  — gate behind milestone review
- **Do:** quick-xml-based `to_xml`/`from_xml` driven by generator metadata;
  validate against official XML examples (`examples.zip` XML variant).
- **Accept:** curated XML round-trip subset green.
- **Depends:** T8; review after Phase 4 whether demand justifies it

---

## Phase 6 — Multi-version (0.7+)

### T22. EPIC: R4B model
- **Do:** Point generator at R4B definitions → `src/r4b/`; feature flags
  `r5` (default) / `r4b`; shared primitives where identical.
- **Accept:** R4B examples round-trip subset green; compile-time measured
  and documented.
- **Depends:** generator hardening from T7–T9

---

## Documentation / tutorials / examples track (interleave; one per phase)

### T23. mdBook guide
- **Do:** `book/` with chapters: Getting started; Model mapping; JSON
  serialization deep-dive (incl. `_field` once T7 lands); Validation;
  Terminology & codes; Extensions; Bundles; Code generator internals. CI job
  builds it (`mdbook build`), deploy to GitHub Pages.
- **Accept:** `mdbook build` green in CI; linked from README.
- **Depends:** T1 (CI); content updated at the end of each phase

### T24. Example set expansion
- **Do (rolling):** `extensions.rs` (T17), `primitive_extensions.rs` (T7),
  `transaction_bundle.rs` + `search_response.rs` (T18),
  `operation_outcome.rs` (T15), `typed_references.rs` (T11),
  `client_crud.rs` (T19). Every example: header comment tutorial style
  (match the existing four), runs cleanly, and is listed in README +
  `lib.rs` "More examples".
- **Accept:** `cargo build --examples` green; each prints sensible output.

### T25. `llms.txt` / `llms.json`
- **Do:** Author AI-readable crate summaries (crate purpose, module map, key
  types, examples index); restore them to `Cargo.toml` `include`; add a CI
  check that they mention every top-level module.
- **Accept:** files exist, included in package, dry-run green.
- **Depends:** T3

---

## Suggested execution order (first five sessions)

1. **T2** (examples oracle) + **T1** (CI) — everything else gets safer.
2. **T3 + T5 + T25** (publish hygiene bundle) — then actually publish 0.1:
   `cargo publish -p fhir-derive-macros && cargo publish`.
3. **T8** (generator metadata) — unlocks most of Phases 2–5.
4. **T6** (prototype `_field`, make the representation decision).
5. **T7** (EPIC `_field` rollout) → ship 0.2.
