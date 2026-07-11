# 01 — Overview

## Purpose

`fhir` is a Rust crate that provides the **FHIR Release 5 (R5, version 5.0.0)**
data model as strongly-typed, `serde`-serializable Rust, together with a
spec-driven code generator that derives that model from the official FHIR
specification JSON.

FHIR (Fast Healthcare Interoperability Resources) is the HL7 standard for
representing and exchanging electronic health records.

## Scope

In scope:

- The FHIR R5 **primitive datatypes** (spec 02).
- The FHIR R5 **complex datatypes** (spec 03).
- The FHIR R5 **resources** (spec 04).
- The FHIR R5 **code systems** as enums (spec 05).
- **JSON serialization** to/from canonical FHIR JSON (spec 06).
- **Structural validation** of the model (spec 07).
- The **code generator** that produces the model from spec JSON (spec 08).

Out of scope (for now; see each spec's Future work):

- FHIR XML and Turtle representations.
- A running FHIR REST server or client.
- FHIRPath evaluation and full invariant (constraint) checking.
- FHIR versions other than R5.

## Goals

1. **Correctness.** Types mirror the FHIR R5 StructureDefinitions; JSON
   round-trips losslessly for supported representations.
2. **Ergonomics.** Idiomatic Rust: `Default`, `Clone`, `PartialEq`, `serde`,
   and rich rustdoc with runnable examples.
3. **Uniformity.** Every type is built the same way, so the model is
   predictable and machine-generable.
4. **Spec-driven.** The specifications in this directory define behaviour; the
   implementation follows and is verified against them.
5. **Leanness.** Few dependencies; fast compile and serialization.

## Non-goals

- Hiding FHIR's shape behind a "friendlier" abstraction. This crate exposes
  FHIR faithfully.
- Runtime reflection or dynamic typing beyond the `Resource` enum and the
  `serde_json::Value` used for polymorphic `contained` slots.

## Crate identity

- Package and crate name: **`fhir`**. Import as `use fhir::…`.
- Cargo **workspace** with one extra member, `fhir-derive-macros` (proc-macros).
- Library **and** binary: the library is the model + generator API; the binary
  runs the generator. A library target is required so doctests execute.

## Success definition

The crate meets this overview when:

- The model covers the R5 primitive types, complex datatypes, resources, and
  code systems.
- Every value round-trips through JSON.
- The green gate passes (build, tests, doctests, pedantic clippy).

## Acceptance criteria

- [ ] `cargo build` succeeds for the workspace.
- [ ] The public API exposes `fhir::r5::{types, resources, codes, validate}`.
- [ ] `cargo test` and `cargo clippy --all-targets` are clean.
- [ ] Each downstream spec (02–08) has its own acceptance criteria met.
