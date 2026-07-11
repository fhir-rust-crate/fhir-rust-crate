# 09 — Primitive extensions (`_field`)

Status: **decided** (T6). This spec defines how the crate represents FHIR
*primitive extensions* — the `_field` siblings that carry `id` and `extension`
for a primitive value. It records the two representations prototyped, the
comparison, and the chosen approach that the T7 rollout applies to every struct.

## Background: how FHIR JSON represents primitive extensions

In FHIR, every element — including a *primitive* such as `birthDate` or
`gender` — may carry an `id` and `extension`s. JSON has no place to hang those
on a bare scalar, so FHIR splits them into a **sibling property** named with a
leading underscore
([FHIR R5 §JSON representation of primitive elements](https://hl7.org/fhir/R5/json.html#primitive)):

```json
{
  "birthDate": "1970-03-25",
  "_birthDate": {
    "extension": [{
      "url": "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
      "valueDateTime": "1970-03-25T14:35:00-05:00"
    }]
  }
}
```

Rules that any representation MUST honour:

1. The value (`birthDate`) and its extension (`_birthDate`) are **two sibling
   keys** on the *parent* object, not a nested object.
2. Either may appear **without** the other: a value with no extension, or an
   extension with no value (e.g. a data-absent-reason).
3. For a **repeating** primitive, the two arrays are **positionally aligned**
   and JSON `null` fills the gaps:

   ```json
   { "given": ["Peter", "James"], "_given": [null, {"id": "n2"}] }
   ```

Before this spec, the model dropped every `_field`: the official-examples
round-trip (see `tasks-roundtrip-failures.md`, category A) lost `_birthDate`,
`_gender`, `_family`, `_given`, `_event`, `_profile`, and more.

## Prototypes

Both were exercised against crafted Patient / Observation / Questionnaire
fixtures covering the scalar, extension-only, required, and repeating cases
(`tests/primitive_extensions_prototype.rs`).

### Approach A — `_field` siblings (chosen)

Next to a primitive field `x`, add a sibling field renamed to the FHIR `_x`
key, typed by the existing [`Element`](../src/r5/types/element.rs)
(`{ id, extension }`):

```rust
pub birth_date: Option<types::Date>,
#[serde(rename = "_birthDate")]
pub birth_date_ext: Option<types::Element>,          // scalar

pub subject_type: Option<Vec<types::Code>>,
#[serde(rename = "_subjectType")]
pub subject_type_ext: Option<Vec<Option<types::Element>>>, // repeating (null-aligned)
```

- The Rust field is named `<field>_ext`.
- It MUST carry an explicit `#[serde(rename = "_<fhirName>")]`. The struct-level
  `rename_all = "camelCase"` would otherwise map `birth_date_ext` to
  `birthDateExt`, and — worse — would map a naive `_birth_date` to `birthDate`,
  colliding with the value field. The explicit rename is mandatory, not
  cosmetic.
- `skip_serializing_none` already drops the sibling when it is `None`, so
  extension-free data serializes unchanged.
- Repeating primitives use `Option<Vec<Option<Element>>>`: the outer `Option`
  distinguishes "no `_field` at all" from "present"; the inner `Option`
  is the per-position `null`.

This works with the crate's existing `#[derive(Serialize, Deserialize)]` +
`skip_serializing_none` + `rename_all` recipe, with **zero** custom serde.

### Approach B — `FhirField<T>` wrapper (rejected)

A single field holding value + id + extension together:

```rust
pub struct FhirField<T> { pub value: Option<T>, pub id: Option<String>, pub extension: Option<Vec<Extension>> }
pub birth_date: FhirField<Date>,
```

This is cohesive in Rust, but it **cannot** serialize with derive: `serde`
maps one struct field to one JSON key, whereas FHIR requires `birth_date` to
become **two** sibling keys (`birthDate` and `_birthDate`) on the parent. Making
B produce valid FHIR JSON requires one of:

- a hand-written `Serialize`/`Deserialize` for **every** container that splits
  each `FhirField` into two parent keys — abandoning the uniform derive recipe
  across ~208 structs; or
- a bespoke proc-macro that re-implements serde's field handling for the
  `x` / `_x` split — high complexity, and a second code-generation surface to
  maintain.

It is also a **breaking** change to every primitive field's type, and it buries
the common case (a plain value) inside a wrapper.

## Comparison

| Criterion | A — siblings | B — `FhirField<T>` |
| --- | --- | --- |
| Valid FHIR JSON with plain `#[derive]` | ✅ yes | ❌ needs custom serde everywhere |
| Round-trips scalar / repeating / extension-only | ✅ (tests pass) | ✅ only after custom serde |
| Generator effort (T7) | Low: emit one sibling field per primitive | High: emit custom serde / new macro |
| Backward compatibility | ✅ additive optional fields | ❌ changes every primitive field type |
| Common-case ergonomics (value only) | ✅ untouched | ⚠️ wrapped |
| Extension-access ergonomics | ⚠️ second field | ✅ co-located |
| Consistency with existing code | ✅ same recipe | ❌ diverges |

## Decision

**Adopt Approach A (`_field` siblings).** It is the only option that preserves
the crate's derive-based uniformity, is a purely additive (non-breaking) change,
and is mechanically generatable. The ergonomic cost — a separate `<field>_ext`
field — is small and MAY later be smoothed with accessor helpers
(e.g. an `ExtensionExt`-style trait, see T17) without changing the wire model.

## Requirements

- **R1.** For every primitive-typed element `x` (including primitive `value[x]`
  choice variants such as `deceasedBoolean`), the owning struct MUST have a
  sibling field `x_ext` with `#[serde(rename = "_<fhirName>")]`.
- **R2.** Scalar primitives use `Option<Element>`; repeating primitives use
  `Option<Vec<Option<Element>>>`.
- **R3.** The sibling MUST NOT be emitted when absent (`skip_serializing_none`).
- **R4.** `#[derive(Validate)]` MUST recurse into `_field` `Element`s so their
  nested extensions are validated (T7d).
- **R5.** Non-primitive elements are unaffected: complex datatypes already model
  `id`/`extension` on the type itself. (The separate gap where some complex
  datatypes lack `id`/`extension` — round-trip category B — is out of scope
  here and tracked in `tasks-roundtrip-failures.md`.)

## Acceptance criteria

- The prototype fields on `Patient`, `Observation`, and `Questionnaire` compile
  and pass the green gate.
- `tests/primitive_extensions_prototype.rs` round-trips (via the `Resource`
  enum) the scalar, extension-only, required-primitive, and repeating
  (null-aligned) cases with exact `serde_json::Value` equality.
- This document records the decision (Approach A) with the rationale above.

## Scope of the T6 prototype vs the T7 rollout

T6 (this spec) prototypes the representation on three resources' own primitive
fields. It does **not** yet cover primitives nested in complex datatypes
(e.g. `HumanName.family` / `HumanName.given`), so a full official example such
as `patient-example.json` — which also has `contact[0].name._family` — is not
expected to round-trip until **T7** rolls the siblings across all ~208 structs
(datatypes included) and updates the `Validate` derive.
