# 11 — Choice types (`value[x]`)

Status: **implemented (T9a–d, shipped 0.3).** This spec defines how
the crate will represent FHIR `value[x]` *choice* elements as Rust enums, records
the serde design validated by the T9a prototype
(`tests/choice_type_prototype.rs`), and its findings.

## Background: how FHIR JSON represents a choice element

A choice element `value[x]` appears on the parent object as exactly one key,
named by suffixing the chosen type to the base: `valueQuantity`, `valueString`,
`valueBoolean`, … A *primitive* choice may additionally carry the paired
`_value<Type>` extension sibling (per spec 09):

```json
{ "valueString": "positive", "_valueString": { "extension": [ … ] } }
```

There are **261** choice elements in R5. Most allow 2–13 types; ten (e.g.
`Extension.value[x]`) allow all 54 datatypes.

## Current representation (to be replaced)

The model flattens each choice into one `Option` field per type, e.g.
`Observation` has `value_quantity`, `value_string`, `value_boolean`, … Nothing
enforces that at most one is set, the field list is verbose, and the paired
`_value<Type>` primitive extensions are not modelled.

## Decision: one enum per choice, flattened onto the parent

Replace the flattened fields with a single field holding a generated enum:

```rust
pub enum ObservationValue {
    Quantity(Quantity),
    CodeableConcept(CodeableConcept),
    String(Primitive<FhirString>),   // primitive variant carries its `_value` ext
    Boolean(Primitive<Boolean>),
    // … one variant per allowed type
}

pub struct Observation {
    // …
    #[serde(flatten)]
    pub value: Option<ObservationValue>,
}
```

- **`Primitive<T> { value: T, extension: Option<Element> }`** carries a primitive
  variant's value together with its paired `_value<Type>` extension. Complex
  variants hold the type directly (they already model `id`/`extension`).
- The enum has a **hand-written `Serialize`/`Deserialize`** (generated per enum):
  - *Serialize* emits a map with the single `value<Type>` entry (plus
    `_value<Type>` when a primitive variant has an extension), so
    `#[serde(flatten)]` merges it onto the parent object.
  - *Deserialize* visits the parent's (flattened) map, matches the `value<Type>`
    / `_value<Type>` keys, ignores all others, and requires exactly one value
    variant.
- **Backward compatibility:** keep `#[deprecated]` accessor methods mirroring the
  old field names (`fn value_quantity(&self) -> Option<&Quantity>`, …) where
  cheap, so downstream code migrates gradually.

## Prototype findings (T9a)

The prototype (`tests/choice_type_prototype.rs`, 7 tests, green) validated:

1. **Complex and primitive variants round-trip** (`valueQuantity`,
   `valueCodeableConcept`, `valueString`, `valueBoolean`).
2. **The paired `_value<Type>` extension round-trips** with its value.
3. **The enum flattens onto the parent** via `#[serde(flatten)]`.
4. **Absent choice → `None`**, and serialization of a set choice **emits exactly
   one** value key.

Key limitation found:

- **`#[serde(flatten)]` on `Option<Enum>` swallows the field's deserialize errors
  into `None`.** So deserialize-time strictness — rejecting an extension-only
  choice (`_valueString` with no `valueString`) or two value keys — is **not**
  enforceable through flatten. "Exactly one" is instead guaranteed at the **type
  level** (the enum can hold only one variant) and on **serialize**. Deserialize
  is intentionally *lenient* (a malformed choice becomes `None`). Strict
  rejection would require a hand-written parent `Deserialize` for every resource
  — rejected as far too costly. This lenient behaviour also means an
  extension-only choice value is dropped; this accounts for the last few
  official-example round-trip mismatches (`_valueCode` on `Extension.value[x]`),
  which are accepted as out of scope.

## Requirements

- **R1.** Each `value[x]` element becomes one generated enum with a variant per
  allowed type, replacing the flattened `value_<type>` fields.
- **R2.** The field is `Option<Enum>` with `#[serde(flatten)]`.
- **R3.** Primitive variants use `Primitive<T>` to carry the paired
  `_value<Type>` extension; complex variants hold the type directly.
- **R4.** Serialization emits exactly one `value<Type>` key (plus `_value<Type>`
  when present). Deserialization matches those keys, ignores others, and is
  lenient (malformed → `None`).
- **R5.** `#[derive(Validate)]` must validate the active variant.
- **R6.** Provide `#[deprecated]` accessors mirroring the old field names where
  cheap.

## Acceptance criteria

- `tests/choice_type_prototype.rs` is green (the design proof).
- After the rollout (T9b–e): the official-examples full run stays green, "at most
  one" is compile-time enforced, and CHANGELOG documents the migration; version
  bump 0.3.

## Rollout (done)

- **T9b** — `FhirChoice` derive + `Primitive<T>`; first conversion (Annotation).
- **T9c** — `src/r5/parse/choice_gen.rs` generator; 17 datatype conversions.
- **T9d** — 241 resource conversions (258 total).
- **T9e** — 0.3 ship; examples/tests migrated to the enum API.

Deferred: crate-level re-exports of the choice enums (they are reachable via
their type's module today), and `#[deprecated]` accessor shims for every old
field (R6 "where cheap" — provided on the Annotation demonstrator; skipped in the
bulk to keep the generated surface manageable for a pre-1.0 breaking release).
