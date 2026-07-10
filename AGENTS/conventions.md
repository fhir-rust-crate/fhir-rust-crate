# Conventions

These conventions are **mandatory** and uniform across the whole model. The
generator emits them and hand-authored code must match them exactly, so that
every datatype and resource looks and behaves the same. The normative details
live in [`../spec/06-serialization.md`](../spec/06-serialization.md).

## The canonical struct

Every complex datatype and resource struct looks like this:

```rust
use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    /// Short description from the FHIR ElementDefinition.
    pub field: Option<types::String>,
}
```

Rules embodied above:

- **Derives, in this order:** `Debug, Default, Clone, Serialize, Deserialize,
  PartialEq, Eq, Validate`. Never use bare `f64`/`f32` — they break `Eq`.
- `#[serde_with::skip_serializing_none]` so `None` fields are omitted.
- `#[serde(rename_all = "camelCase")]` maps snake_case Rust fields to camelCase
  FHIR JSON keys. Do **not** add per-field `#[serde(rename)]` unless the JSON
  key cannot be produced by camelCase.
- Do **not** add `#[serde(deny_unknown_fields)]` on model structs.

## Field types and cardinality

Map the FHIR `ElementDefinition` `(min, max)` to Rust wrappers:

| FHIR cardinality | Rust type |
| --- | --- |
| `0..1` | `Option<T>` |
| `1..1` | `T` |
| `0..*` | `Option<Vec<T>>` |
| `1..*` | `Vec<T>` |

`T` is a `types::Pascal(code)` (e.g. `types::CodeableConcept`), a nested
backbone struct, or `::serde_json::Value` for polymorphic `Resource` slots.

**Bare `Vec<T>` fields that use `#[serde(skip_serializing_if = "Vec::is_empty")]`
must also carry `#[serde(default)]`** — otherwise they serialize to `{}` but
fail to deserialize from it (a real round-trip bug we have already hit).

## Choice elements `[x]`

A FHIR choice element such as `value[x]` currently expands to **one flattened
field per allowed type**:

```rust
pub value_quantity: Option<types::Quantity>,
pub value_string: Option<types::String>,
```

The snake_case name is `<base>_<snake(typecode)>`; camelCase serde renaming
yields `valueQuantity`, `valueString`, etc. (Converting these to a single Rust
enum is a known future improvement — see
[`../spec/06-serialization.md`](../spec/06-serialization.md).)

## Field names

- snake_case of the last path segment.
- Rust keywords become raw identifiers: `type` → `r#type`, `use` → `r#use`,
  `ref` → `r#ref`, `abstract` → `r#abstract`. `rename_all` still sees the
  unescaped name, so no explicit rename is needed.

## Primitives

Primitives are newtypes, not empty structs:

- string-like (`string`, `code`, `id`, `uri`, dates, …) → `struct X(pub String)`
- `boolean` → `struct Boolean(pub bool)`, `integer` → `i32`,
  `positiveInt`/`unsignedInt` → `u32`
- `decimal` → `struct Decimal(pub serde_json::Number)` (precision + `Eq`)
- `integer64` → `struct Integer64(pub i64)` serialized **as a JSON string**

A tuple newtype serializes transparently as its inner value. Details in
[`../spec/02-primitive-types.md`](../spec/02-primitive-types.md).

## Module wiring

- Each datatype is `src/r5/types/<snake>.rs`, declared in `src/r5/types.rs`
  with both `pub mod <snake>;` and `pub use <snake>::<Pascal>;`.
- Each resource is `src/r5/resources/<snake>.rs`, declared the same way in
  `src/r5/resources.rs`, which also defines the `Resource` enum.

## Documentation

- Module header: `//!` block with the FHIR name, URL, Version, a one-line
  description, and the FHIR/UML links.
- Struct: a `///` doc with a `# Examples` doctest that round-trips the default
  value through `serde_json` (see [`testing.md`](testing.md)).
- Every public field: a one-line `///` from its FHIR `short` text.
