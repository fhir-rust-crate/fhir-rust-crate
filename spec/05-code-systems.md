# 05 — Code systems

Defines the type-safe code-system enums in `r5::codes`.

## Background

FHIR `code` values are drawn from **CodeSystems**. Many are small, closed
enumerations (e.g. `administrative-gender` → male | female | other | unknown).
Representing these as Rust enums gives compile-time safety and exhaustive
matching.

## Requirements

- **R5.1** For every `CodeSystem` in `valuesets.json` whose `content` is
  `complete` and which has **≥ 2** concepts, `r5::codes` MUST provide a Rust
  `enum` whose variants are its codes.
- **R5.2** Each enum MUST derive `Debug, Clone, Serialize, Deserialize,
  PartialEq, Eq, Default`; the first concept is the `#[default]` variant.
- **R5.3** Each variant MUST serialize to its exact FHIR code string via
  `#[serde(rename = "<code>")]`, so `to_value`/`from_value` use the canonical
  codes, not the Rust identifiers.
- **R5.4** Variant identifiers MUST be sanitized deterministically:
  - PascalCase of the code, non-alphanumeric separators removed.
  - A leading digit is prefixed with `N`.
  - Rust reserved words in PascalCase form (e.g. `Self`) get a trailing `_`.
  - Collisions within an enum are de-duplicated with a numeric suffix.
- **R5.5** Enum type names MUST be the PascalCase of the CodeSystem id, with the
  same collision handling across the module.
- **R5.6** `r5::codes` MUST be generated deterministically from the spec (same
  input → identical file), and MUST carry doc comments (system URL wrapped in
  `<...>` so rustdoc treats it as a link; concept displays as field docs).

## Relationship to the model

`r5::codes` is currently **standalone**: model structs use `types::Code` (a
string newtype) for coded fields, not these enums. Wiring specific
required-binding fields to their enum (e.g. `Patient.gender` →
`AdministrativeGender`) is **future work**, because it requires the binding
strength and value-set membership from the spec.

## Future work

- Wire `required`-binding fields to their code enums.
- Expose value-set membership and binding metadata.
- Provide `FromStr`/`Display` conveniences alongside serde.

## Acceptance criteria

- [ ] `r5::codes` contains an enum for every complete CodeSystem with ≥ 2
      concepts.
- [ ] `AdministrativeGender::Female` serializes to `"female"`; `"male"`
      deserializes to `AdministrativeGender::Male`.
- [ ] Generation is deterministic and the module compiles with zero clippy
      warnings (no bare URLs, no keyword/identifier errors).

## Typing coded fields (T10)

Status: **decided and rolled out.** Elements with a `required` binding whose
value set maps to an existing code enum are typed as that enum rather than the
opaque `types::Code`, so the compiler enforces the value set.

**Fallback policy — decision.** A closed enum would reject any code outside the
value set, which is unacceptable for a data-exchange library (data carries newer
codes, local extensions, or simply invalid values, and read must not fail).
Rather than add an `Other(String)` variant to every one of the ~419 generated
enums, the field type is wrapped:

```rust
pub enum Coded<E> {           // fhir::r5::coded
    Known(E),                 // a recognized code from the value set
    Unknown(String),          // any other code, preserved verbatim
}
```

`Coded<E>` is `#[serde(untagged)]`: deserialization tries `E` first and falls
back to `Unknown`; serialization emits the code string either way. This keeps
`codes.rs` untouched, makes the retype **round-trip-safe** (a code outside the
set — or a mis-mapped enum — simply lands in `Unknown`), and localizes the
policy in one type. A required-binding field is therefore
`Option<Coded<AdministrativeGender>>` (or `Coded<…>` for `1..1`).

Rollout: `src/r5/parse/coded_gen.rs` retyped 343 fields (27 datatypes + 316
resources) whose value set maps to a code enum. Membership-of-value-set checking
for the `Unknown` fallback is deferred to the validation-depth work (T13).

### Acceptance criteria (T10)

- [x] ≥100 required-binding fields retyped (343 done).
- [x] `Coded<E>` round-trips known and unknown codes; official-examples run
      unchanged (2822/2824).
- [x] Examples and doctests migrated to the enum API; full gate green.
