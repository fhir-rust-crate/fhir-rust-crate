# 02 — Primitive types

Defines how the FHIR primitive datatypes are represented in Rust.

Applies to every modelled release. Where a release differs, the difference is
stated here rather than in the code.

| Release | Primitives |
| --- | --- |
| R5 | 21 |
| R4 | 20 (no `integer64`) |
| R3 | 18 (no `integer64`, `canonical` or `url`) |

## Background

FHIR primitives are single scalar values with a lowercase initial letter. In
FHIR JSON they serialize as bare scalars — a JSON string, number, or boolean —
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
  | `integer64` (R5 only) | `struct Integer64(pub i64)` — serialized as a JSON **string** |

  `canonical` and `url` arrived in R4; `integer64` in R5. A release simply does
  not generate a primitive it does not define.
  | `decimal` | `struct Decimal(pub serde_json::Number)` |

- **R2.2** `decimal` MUST preserve numeric precision and satisfy `Eq`;
  `serde_json::Number` achieves both. Its `Default` is `Number::from(0)`, which
  is the one primitive whose `Default` cannot be derived.
- **R2.3** `integer64` MUST serialize and deserialize as a JSON **string**
  (FHIR encodes 64-bit integers as strings so they survive consumers whose
  numbers are 64-bit floats). Implemented with `serde_with`'s `DisplayFromStr`.
- **R2.4** Every primitive MUST derive `Debug, Default, Clone, PartialEq, Eq`
  and be `serde` (de)serializable. No primitive may contain `f64`/`f32`.
- **R2.5** Each primitive lives in `src/<release>/types/<snake>.rs` and is
  re-exported from `src/<release>/types.rs` as `pub use <snake>::<Pascal>;`.
- **R2.6** Each primitive MUST implement `Validate` (spec 07) with its FHIR
  format constraint where one exists (`code`, `id`, `oid`, `uuid`, `uri`,
  `canonical`, `url`); the rest are structurally valid by construction.
- **R2.7** The Rust representation is a design decision the specification JSON
  does not state, so it MUST live in one table
  (`codegen::primitives::PRIMITIVES`) shared by every release. A release that
  defines a primitive absent from that table MUST fail generation loudly rather
  than be guessed at.

## Representation notes

- A single-field tuple struct is serialized by serde **as its inner value**, so
  no `#[serde(transparent)]` attribute is required.
- Inside `string.rs`, refer to the standard library type as
  `std::string::String` to avoid shadowing by the newtype.

## Rationale

Newtypes rather than type aliases give each primitive its own `Validate` impl
and prevent a `Code` being passed where an `Id` is meant, at no runtime cost —
the wire form is identical to the bare scalar.

## Future work

- Format validation covers a subset of the primitives; date/time and base64
  format checks MAY be added under spec 07.

## Acceptance criteria

1. Every primitive its release defines exists as a newtype per R2.1 and
   re-exports from `types.rs`.
2. `Decimal` round-trips `3.5` as JSON `3.5`; its `Default` is `0`.
3. `Integer64` (R5) round-trips `9007199254740993` as the JSON string
   `"9007199254740993"`.
4. `Code("bad  code")` and `Id("bad id!")` are reported invalid by `Validate`.
5. Every primitive module passes its generated round-trip test.
