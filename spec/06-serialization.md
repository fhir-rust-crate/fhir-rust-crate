# 06 — Serialization

Defines the JSON mapping between the Rust model and canonical FHIR JSON. This is
the normative version of the conventions in
[`../AGENTS/conventions.md`](../AGENTS/conventions.md).

## Requirements

### Struct shape

- **R6.1** Every datatype/resource struct MUST derive, in order:
  `Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate`.
- **R6.2** Structs MUST carry `#[serde_with::skip_serializing_none]` so `None`
  optional fields are omitted from output.
- **R6.3** Structs MUST carry `#[serde(rename_all = "camelCase")]`. Per-field
  `#[serde(rename)]` is only allowed when camelCase cannot produce the correct
  FHIR key.
- **R6.4** Structs MUST NOT use `#[serde(deny_unknown_fields)]` (forward
  compatibility with unknown/extension fields).
- **R6.5** No field may be a bare `f64`/`f32` (breaks `Eq`); use
  `types::Decimal`.

### Cardinality mapping

- **R6.6** FHIR `(min, max)` maps to Rust as:

  | Cardinality | Rust |
  | --- | --- |
  | `0..1` | `Option<T>` |
  | `1..1` | `T` |
  | `0..*` | `Vec<T>` |
  | `1..*` | `vec1::Vec1<T>` |

- **R6.7** A `Vec` field using `skip_serializing_if = "Vec::is_empty"` MUST also
  declare `#[serde(default)]`, so an omitted array deserializes to an empty
  `Vec` instead of erroring. (Serializing to `{}`/omitted but failing to
  deserialize it is a defect.)

### Field naming

- **R6.8** Rust field names are snake_case of the FHIR element name; camelCase
  serde renaming reproduces the FHIR JSON key.
- **R6.9** Rust keywords MUST be raw identifiers (`r#type`, `r#use`, `r#ref`,
  `r#abstract`); serde still sees the unescaped name.

### Choice elements `[x]`

- **R6.10** A choice element expands to one field per allowed type, named
  `<base>_<snake(typecode)>` and typed `Option<types::Pascal(typecode)>`, e.g.
  `value[x]` with `[Quantity, string]` →
  `value_quantity: Option<types::Quantity>`, `value_string:
  Option<types::String>`. camelCase renaming yields `valueQuantity`,
  `valueString`.
- **R6.11** *(Future)* A choice element MAY instead be a single Rust `enum`
  using serde's untagged/adjacent representation to make "exactly one variant"
  unrepresentable. If adopted, it MUST still produce the same FHIR JSON keys.

### Polymorphic slots

- **R6.12** `contained` and other `Resource`/`DomainResource`-typed elements
  are `::serde_json::Value`. A top-level polymorphic resource uses the
  `Resource` enum tagged by `resourceType` (spec 04, R4.6).

### Round-trip guarantee

- **R6.13** For any value `v`, `from_value(to_value(v)) == v` MUST hold for the
  supported representations. Tests and doctests assert this via the
  round-trip-of-default pattern (spec 07, and
  [`../AGENTS/testing.md`](../AGENTS/testing.md)).

## Acceptance criteria

- [ ] A representative resource with nested backbones and choice fields
      round-trips through `serde_json`.
- [ ] Optional `None` fields and empty `Vec`s are omitted from output and
      re-read without error.
- [ ] FHIR keys in output are camelCase; keyword fields serialize to their FHIR
      names (`type`, `use`, …).
- [ ] `cargo clippy --all-targets` reports zero warnings.
