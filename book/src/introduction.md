# Introduction

`fhir` is a Rust implementation of the **HL7 FHIR® Release 5 (R5)** data model,
together with a spec-driven code generator that produces it from the official
FHIR specification JSON.

Fast Healthcare Interoperability Resources (FHIR, pronounced "fire") is the HL7
standard for exchanging electronic health records. This crate lets you **build,
parse, validate, and round-trip** FHIR resources in idiomatic Rust with `serde`.

## What you get

- **158 R5 resources** (Patient, Observation, Encounter, …) as Rust structs,
  plus a polymorphic `Resource` enum tagged by `resourceType`.
- **~50 complex datatypes** and **21 primitive newtypes** that serialize
  transparently to their JSON form.
- **400+ code systems** as type-safe enums; `required`-binding fields are typed
  as those enums via [`Coded`](terminology-and-codes.md).
- **`value[x]` choice elements as enums** — exactly one type at compile time.
- **Recursive validation** (`Validate`): primitive formats, cardinality,
  required-binding membership, and a subset of FHIR invariants.
- **Ergonomics**: builders, a prelude, extension helpers, Bundle utilities,
  summary serialization, and an async REST client (feature `client`).

## How to read this guide

Each chapter is task-oriented and standalone. Start with
[Getting started](getting-started.md), then dip into whichever topic you need.
For the full API, run `cargo doc --open`; for the normative rules, see the
[`spec/`](https://github.com/joelparkerhenderson/fhir-rust-crate/tree/main/spec)
directory in the repository.

> FHIR® is a registered trademark of Health Level Seven International. This crate
> is not affiliated with or endorsed by HL7.
