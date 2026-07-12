# 03 — Complex datatypes

Defines how the 50 FHIR R5 complex datatypes are represented in Rust.

## Background

Complex datatypes are structured values reused inside resources (e.g.
`CodeableConcept`, `Reference`, `Period`, `Quantity`, `Identifier`, `Timing`,
`Dosage`, `ElementDefinition`). They have an uppercase initial letter in FHIR.

## Requirements

- **R3.1** Each complex datatype MUST be a Rust struct following the canonical
  conventions of spec 06: the standard derive set (including `Validate`),
  `#[serde_with::skip_serializing_none]`, and `#[serde(rename_all = "camelCase")]`.
- **R3.2** Fields MUST use the cardinality mapping of spec 06
  (`0..1`→`Option<T>`, `1..1`→`T`, `0..*`→`Vec<T>`, `1..*`→`vec1::Vec1<T>`).
- **R3.3** Field types MUST reference `types::X` (another datatype or a
  primitive), a nested backbone struct in the same file, or
  `::serde_json::Value` where the FHIR type is `Element`/`BackboneElement`
  with no children.
- **R3.4** **Nested backbone elements** MUST be modeled as named nested structs
  in the same module, named `<Parent><Field>` in PascalCase (e.g.
  `Timing.repeat` → `TimingRepeat`, `Dosage.doseAndRate` → `DosageDoseAndRate`).
  The parent field references the nested struct directly. Nesting recurses to
  any depth. Each nested struct carries the full derive set and serde
  attributes.
- **R3.5** Choice elements `[x]` MUST expand per spec 06.
- **R3.6** Each datatype lives in `src/r5/types/<snake>.rs`, declared in
  `src/r5/types.rs` (`pub mod` + `pub use <Pascal>`).
- **R3.7** Recursive references (e.g. `Extension.extension:
  Vec<types::Extension>`) are permitted via `Vec` (heap-allocated) with
  no `Box`; direct non-`Vec` self-reference MUST be `Box`ed.
- **R3.8** Bare `Vec` fields using `skip_serializing_if = "Vec::is_empty"`
  MUST also declare `#[serde(default)]` (round-trip requirement, spec 06).

## Coverage

The 50 complex datatypes include the general-purpose types (Address,
Annotation, Attachment, CodeableConcept, CodeableReference, Coding,
ContactPoint, HumanName, Identifier, Period, Quantity, Range, Ratio, Reference,
SampledData, Signature, Timing, …), the metadata types (ContactDetail,
Contributor, DataRequirement, Expression, ParameterDefinition,
RelatedArtifact, TriggerDefinition, UsageContext, …), the special-purpose types
(Dosage, ElementDefinition, Extension, Meta, Narrative, …), and the abstract
bases (Base, Element, BackboneElement, DataType, PrimitiveType).

## Documentation

- Module header with FHIR name, URL, Version, one-line description, FHIR/UML
  links (spec 06).
- A struct-level `# Examples` doctest that round-trips the default value
  (marked `ignore` for the few datatypes with a `1..*`/`Vec1` field, which have
  no `Default`).
- One-line `///` per public field from the FHIR `short`.

## Acceptance criteria

- [ ] All 50 complex datatype modules exist and re-export from `types.rs`.
- [ ] Backbone-bearing types (Timing, Dosage, ElementDefinition,
      DataRequirement, Availability, …) define the required nested structs with
      no duplicate fields.
- [ ] Every datatype round-trips its default value through JSON.
- [ ] Every datatype derives `Validate` and participates in recursive
      validation (spec 07).
- [ ] Build, tests, doctests, and pedantic clippy are clean.
