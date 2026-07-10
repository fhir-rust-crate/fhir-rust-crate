# 04 — Resources

Defines how the 158 FHIR R5 resources are represented, and the polymorphic
`Resource` enum.

## Background

A resource is a top-level FHIR entity that can be exchanged on its own
(`Patient`, `Observation`, `Encounter`, `Bundle`, …). Most inherit from
`DomainResource`, which adds `text`, `contained`, `extension`, and
`modifierExtension`; all inherit `id`, `meta`, `implicitRules`, and `language`.

## Requirements

- **R4.1** Each resource MUST be a Rust struct following the canonical
  conventions (spec 06), including `#[derive(… , Validate)]`.
- **R4.2** Fields MUST use the cardinality mapping of spec 06 and reference
  `types::X`, a nested backbone struct, or `::serde_json::Value`.
- **R4.3** **Nested backbone elements** MUST be modeled as named nested structs
  named by concatenating PascalCase path segments (e.g. `Patient.contact` →
  `PatientContact`, `Claim.item.detail.subDetail` → `ClaimItemDetailSubDetail`),
  recursing to any depth. This is the same rule as datatypes (spec 03, R3.4).
- **R4.4** The `contained` element and any element whose FHIR type is
  `Resource`/`DomainResource` MUST be represented as `::serde_json::Value`
  (polymorphic; there is no per-field resource typing yet).
- **R4.5** Each resource lives in `src/r5/resources/<snake>.rs`, declared in
  `src/r5/resources.rs` (`pub mod` + `pub use <Pascal>`).
- **R4.6** `src/r5/resources.rs` MUST define a **polymorphic `Resource` enum**:

  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
  #[serde(tag = "resourceType")]
  pub enum Resource {
      Patient(Box<Patient>),
      Observation(Box<Observation>),
      // … one Box<T> variant per resource …
  }
  ```

  Variants MUST be `Box`ed (to keep the enum small) and the serde tag MUST be
  `resourceType`, so a JSON object `{"resourceType":"Patient", …}` deserializes
  to `Resource::Patient(..)`.

## Documentation

- Module header with FHIR name, URL, Version, one-line description, FHIR/UML
  links.
- A struct-level `# Examples` doctest that round-trips the default value.
- One-line `///` per public field from the FHIR `short`.

## Notes and future work

- Individual resource structs do **not** currently carry a `resourceType`
  field; the discriminator is handled by the `Resource` enum. Adding a
  per-struct `resourceType` for standalone serialization is future work.
- Typed references (`Reference<T>` constrained to allowed targets) are future
  work; today `Reference` is untyped.

## Acceptance criteria

- [ ] All 158 resource modules exist and re-export from `resources.rs`.
- [ ] The `Resource` enum has one `Box<T>` variant per resource and
      deserializes by `resourceType`.
- [ ] Backbone-heavy resources (ExplanationOfBenefit, Claim, CapabilityStatement,
      …) define all required nested structs with no duplicate fields.
- [ ] Every resource round-trips its default value through JSON and derives
      `Validate`.
- [ ] Build, tests, doctests, and pedantic clippy are clean.
