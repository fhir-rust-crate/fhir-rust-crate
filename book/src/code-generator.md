# Code generator internals

The model under `src/r5/{types,resources,codes.rs}` is **generated** from the
official FHIR specification JSON in `doc/fhir-specifications/r5/`. This chapter
sketches how, for contributors.

## The pipeline

The generator lives under `src/r5/parse/` and is driven by the thin binary in
`src/main.rs`:

```sh
cargo run   # reads the spec JSON, writes generated Rust
```

It deserializes the `StructureDefinition` bundles with serde structs mirroring
the FHIR JSON, then translates each `ElementDefinition` into a Rust struct field,
applying the uniform conventions (`rename_all = "camelCase"`,
`skip_serializing_none`, the cardinality mapping).

## The metadata table

`src/r5/parse/meta.rs` extracts a compile-time table (`fhir::r5::meta`) of per-
element facts — cardinality, coded-value bindings, `value[x]` type lists,
reference target profiles, and summary membership — keyed by FHIR path. This
table is the foundation the later layers build on: choice enums, coded fields,
validation, and summary serialization all consult it.

## Layered generators

Several focused generators splice into the already-documented model files rather
than regenerating them (which would destroy the curated prose docs). Each is
driven by an ignored test and the metadata table:

- `siblings.rs` — the `_field` primitive-extension siblings, and the `Element`
  base (`id`/`extension`) on complex datatypes.
- `choice_gen.rs` — the `value[x]` choice enums (`#[derive(FhirChoice)]`).
- `coded_gen.rs` — `required`-binding fields retyped to `Coded<Enum>`.

## The green gate

Before any change is considered done, all of these must pass:

```sh
cargo build --all-targets
cargo test          # unit + integration
cargo test --doc    # doctests
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

The living specifications in
[`spec/`](https://github.com/joelparkerhenderson/fhir-rust-crate/tree/main/spec)
are the source of truth; behaviour is defined there first, then implemented. See
[`AGENTS.md`](https://github.com/joelparkerhenderson/fhir-rust-crate/blob/main/AGENTS.md)
for the full contributor workflow.
