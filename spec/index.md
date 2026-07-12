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
- Each spec ends with **Acceptance criteria** — objective checks that decide
  whether the requirement is met. The green gate (`cargo build`, `cargo test`,
  `cargo clippy --all-targets`) enforces most of them mechanically.
- Specs are numbered by layer, from the smallest units up to the pipeline that
  produces them.

## The specifications

| # | Spec | Scope |
| --- | --- | --- |
| 01 | [Overview](01-overview.md) | Purpose, scope, FHIR R5, crate identity, goals |
| 02 | [Primitive types](02-primitive-types.md) | The 21 FHIR primitive datatypes |
| 03 | [Complex datatypes](03-complex-datatypes.md) | The 50 complex datatypes |
| 04 | [Resources](04-resources.md) | The 158 resources + the `Resource` enum |
| 05 | [Code systems](05-code-systems.md) | The 419 code-system enums |
| 06 | [Serialization](06-serialization.md) | JSON mapping, serde, choice `[x]`, cardinality |
| 07 | [Validation](07-validation.md) | The `Validate` trait and `#[derive(Validate)]` |
| 08 | [Code generation](08-code-generation.md) | The spec-JSON → Rust generator |
| 09 | [Primitive extensions](09-primitive-extensions.md) | The `_field` sibling representation |
| 10 | [Invariant coverage](10-invariants-coverage.md) | Which FHIR constraints are enforced |
| 11 | [Choice types](11-choice-types.md) | `value[x]` choice elements as enums |

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
5. **Spec authority.** The FHIR R5 definition JSON in
   `doc/fhir-specifications/r5/fhir-definitions-json/` is the upstream source;
   these specs interpret it for this crate.

## Status

The crate currently satisfies specs 01–08 for the shipped model: 21 primitives,
50 datatypes, 158 resources, 419 code enums, recursive validation. Open
improvements are recorded as **Future work** sections within the relevant spec.
