# Specifications — index

This directory holds the **living specifications** for the `fhir` crate. It is
the source of truth for spec-driven development: behaviour is defined here
first, then implemented and verified. When code and spec disagree, reconcile
them — do not let them drift.

Operational guidance for agents (commands, conventions, how-to) lives in
[`../AGENTS.md`](../AGENTS.md) and [`../AGENTS/`](../AGENTS/architecture.md);
this directory defines **what must be true**, not how to work.

## How to read these specs

- Requirement levels use **MUST / SHOULD / MAY** (RFC 2119 sense).
- Requirements are numbered `R<spec>.<n>` and referred to by that number from
  the code and from the other specs, so a rule has exactly one home.
- Each spec ends with **Acceptance criteria** — objective checks that decide
  whether the requirement is met. The green gate (`cargo build`, `cargo test`,
  `cargo clippy --all-targets`) enforces most of them mechanically.
- Specs are numbered by layer, from the smallest units up to the pipeline that
  produces them.
- Specs state what is true **now**, in the present tense. Decisions that were
  weighed and rejected are kept only where the reasoning still guides future
  change; the history of how the code got here belongs in
  [`../CHANGELOG.md`](../CHANGELOG.md) and in git.

## The specifications

| # | Spec | Scope |
| --- | --- | --- |
| 01 | [Overview](01-overview.md) | Purpose, scope, crate identity, goals |
| 02 | [Primitive types](02-primitive-types.md) | The FHIR primitive datatypes as newtypes |
| 03 | [Complex datatypes](03-complex-datatypes.md) | The complex datatypes as structs |
| 04 | [Resources](04-resources.md) | The resources + the `Resource` enum |
| 05 | [Code systems](05-code-systems.md) | `CodeSystem`s as enums, and `Coded<E>` |
| 06 | [Serialization](06-serialization.md) | JSON mapping, serde, choice `[x]`, cardinality |
| 07 | [Validation](07-validation.md) | The `Validate` trait and `#[derive(Validate)]` |
| 08 | [Code generation](08-code-generation.md) | The spec-JSON → Rust generator |
| 09 | [Primitive extensions](09-primitive-extensions.md) | The `_field` sibling representation |
| 10 | [Invariant coverage](10-invariants-coverage.md) | Which FHIR constraints are enforced |
| 11 | [Choice types](11-choice-types.md) | `value[x]` choice elements as enums |
| 12 | [FHIR releases](12-fhir-releases.md) | Modelling R3, R4 and R5 side by side |

Specs 02–11 define one release's model and apply to every release
independently; spec 12 defines how the releases coexist. Where the releases
differ in scale, each spec states both figures:

| | R5 (5.0.0) | R4 (4.0.1) | R3 (3.0.2) |
| --- | ---: | ---: | ---: |
| Primitive datatypes | 21 | 20 | 18 |
| Complex datatypes | 50 | 43 | 36 |
| Resources | 158 | 146 | 117 |
| Code-system enums | 419 | 486 | 386 |
| Choice elements | 261 | 186 | 133 |
| Invariant keys | 314 | 240 | 187 |

## Cross-cutting invariants

These hold across every spec and are non-negotiable:

1. **Green gate.** The crate MUST build, pass all unit tests and doctests, and
   produce zero `cargo clippy --all-targets` warnings with `clippy::pedantic`
   enabled.
2. **Round-trip fidelity.** Any value MUST serialize to JSON and deserialize
   back to an equal value (`assert_eq!(v, from(to(v)))`).
3. **Canonical FHIR JSON.** Serialized output MUST use FHIR's camelCase keys and
   omit absent optional fields.
4. **Uniformity.** Every datatype/resource MUST follow the same struct
   conventions (see spec 06); no bespoke shapes.
5. **Spec authority.** The FHIR definition JSON in
   `doc/fhir-specifications/<release>/fhir-definitions-json/` is the upstream
   source; these specs interpret it for this crate.
6. **Release fidelity.** Each modelled release stands on its own. Specs 02–11
   apply to every release independently; spec 12 governs how they coexist.

## Status

The crate satisfies specs 01–12 for all three shipped releases. Each release has the
full model above, recursive validation (07), `_field` primitive extensions (09),
invariant coverage (10), and `value[x]` choice enums (11). Cardinality maps
exactly — `0..1`→`Option<T>`, `1..1`→`T`, `0..*`→`Vec<T>`,
`1..*`→`vec1::Vec1<T>` — and required-binding codes are `Coded<E>`.

Open improvements are recorded as **Future work** sections within the relevant
spec. The largest of them are a FHIRPath evaluator (unlocking most of spec 10),
typed `Reference<T>` rollout (spec 04), and the next release models, R4B and
R6 (spec 12).
