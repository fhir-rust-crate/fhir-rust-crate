# 02 — Primitive types

Defines how the 21 FHIR R5 primitive datatypes are represented in Rust.

## Background

FHIR primitives are single scalar values with a lowercase initial letter. In
FHIR JSON they serialize as bare scalars (a JSON string, number, or boolean) —
**not** as objects.

## Requirements

- **R2.1** Each primitive MUST be a Rust **newtype** wrapping the smallest
  faithful inner type, so it serializes transparently as a bare scalar:

  | FHIR primitive | Rust |
  | --- | --- |
  | `string`, `code`, `id`, `markdown`, `uri`, `url`, `canonical`, `oid`, `uuid`, `base64Binary`, `xhtml` | `struct X(pub String)` |
  | `date`, `dateTime`, `instant`, `time` | `struct X(pub String)` |
  | `boolean` | `struct Boolean(pub bool)` |
  | `integer` | `struct Integer(pub i32)` |
  | `positiveInt`, `unsignedInt` | `struct X(pub u32)` |
  | `integer64` | `struct Integer64(pub i64)` — serialized as a JSON **string** |
  | `decimal` | `struct Decimal(pub serde_json::Number)` |

- **R2.2** `decimal` MUST preserve numeric precision and satisfy `Eq`; using
  `serde_json::Number` achieves both. `Default` is `Number::from(0)`.
- **R2.3** `integer64` MUST serialize and deserialize as a JSON **string**
  (FHIR encodes 64-bit integers as strings to avoid precision loss). Implemented
  via `serde_with`'s `DisplayFromStr`.
- **R2.4** Every primitive MUST derive `Debug, Default, Clone, PartialEq, Eq`
  and be `serde` (de)serializable. No primitive may contain `f64`/`f32`.
- **R2.5** Each primitive lives in `src/r5/types/<snake>.rs` and is re-exported
  from `src/r5/types.rs` as `pub use <snake>::<Pascal>;`.
- **R2.6** Each primitive MUST implement `Validate` (spec 07) with its FHIR
  format constraint where one exists (`code`, `id`, `oid`, `uuid`, `uri`,
  `canonical`, `url`); the rest are structurally valid by construction.

## Representation notes

- A single-field tuple struct is serialized by serde **as its inner value**, so
  no `#[serde(transparent)]` attribute is required.
- Inside `string.rs`, refer to the standard library type as
  `std::string::String` to avoid shadowing by the newtype.

## Rationale

Empty-struct stubs (the previous approach) serialized every primitive as `{}`,
which is not valid FHIR and cannot carry a value. Newtypes make primitives able
to hold real data and serialize as canonical FHIR scalars.

## Future work

- **Primitive extensions**: FHIR allows a sibling `_field` object carrying `id`
  and `extension` for any primitive element. Not yet modeled.
- Format validation currently covers a subset; date/time/base64 format checks
  MAY be added under spec 07.

## Acceptance criteria

- [ ] All 21 primitives exist as newtypes per R2.1 and re-export cleanly.
- [ ] `Decimal` round-trips `3.5` as JSON `3.5`; `Default` is `0`.
- [ ] `Integer64` round-trips `9007199254740993` as the JSON string
      `"9007199254740993"`.
- [ ] `Code("bad  code")` and `Id("bad id!")` are reported invalid by `Validate`.
- [ ] The primitive modules build and pass their round-trip tests.
