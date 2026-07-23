# Introduction

`fhir` is a Rust implementation of the **HL7 FHIR®** data model, together with a
spec-driven code generator that produces it from the official FHIR
specification JSON. Two releases are modelled: **R5 (5.0.0)** under `fhir::r5`
and **R4 (4.0.1)** under `fhir::r4`.

Fast Healthcare Interoperability Resources (FHIR, pronounced "fire") is the HL7
standard for exchanging electronic health records. This crate lets you **build,
parse, validate, and round-trip** FHIR resources in idiomatic Rust with `serde`.

## What you get

For each release:

- **Every resource** (Patient, Observation, Encounter, …) as a Rust struct,
  plus a polymorphic `Resource` enum tagged by `resourceType` — 158 in R5,
  146 in R4.
- **Every complex datatype** and **primitive newtype**, serializing
  transparently to its JSON form.
- **400+ code systems** as type-safe enums; `required`-binding fields are typed
  as those enums via [`Coded`](terminology-and-codes.md).
- **`value[x]` choice elements as enums** — exactly one type at compile time.
- **Recursive validation** (`Validate`): primitive formats, cardinality,
  required-binding membership, and a subset of FHIR invariants.
- **Ergonomics**: builders, a prelude, extension helpers, Bundle utilities,
  summary serialization, and an async REST client (feature `client`).

## How to read this guide

Each chapter is task-oriented and standalone. Start with
[Getting started](getting-started.md), then read
[FHIR releases](fhir-releases.md) if you need R4 or both releases at once, then
dip into whichever topic you need.

Examples throughout use R5 paths. Every one of them works for R4 by changing
`r5` to `r4`.
For the full API, run `cargo doc --open`; for the normative rules, see the
[`spec/`](https://github.com/joelparkerhenderson/fhir-rust-crate/tree/main/spec)
directory in the repository.

> FHIR® is a registered trademark of Health Level Seven International. This crate
> is not affiliated with or endorsed by HL7.
