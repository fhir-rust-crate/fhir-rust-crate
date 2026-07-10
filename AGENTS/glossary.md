# Glossary

Shared vocabulary for FHIR and this project, so the specs, the code, and agents
all use terms the same way.

## FHIR concepts

- **FHIR** — Fast Healthcare Interoperability Resources; the HL7 standard for
  exchanging electronic health records.
- **R5** — FHIR Release 5 (version 5.0.0), the version this crate implements.
- **Resource** — a top-level FHIR entity that can stand alone and be exchanged
  (e.g. `Patient`, `Observation`, `Encounter`). Carries a `resourceType`.
- **DomainResource** — the base for most resources; adds `text` (narrative),
  `contained`, `extension`, `modifierExtension`.
- **Datatype** — a reusable value structure used inside resources. Split into:
  - **Primitive type** — a single scalar value (`string`, `boolean`, `code`,
    `dateTime`, `decimal`, …). Lowercase initial in FHIR.
  - **Complex type** — a structured value (`CodeableConcept`, `Reference`,
    `Period`, `Quantity`, …). Uppercase initial in FHIR.
- **Element** — the unit an `ElementDefinition` describes; every field of every
  resource/datatype is an element with a path, cardinality, and type(s).
- **Backbone element** — an anonymous nested structure inside a resource (e.g.
  `Patient.contact`). Modeled here as a named nested struct.
- **Cardinality** — `min..max` occurrence count, e.g. `0..1`, `1..1`, `0..*`.
- **Choice element `[x]`** — an element that may be one of several types, e.g.
  `Observation.value[x]` → `valueQuantity` | `valueString` | ….
- **CodeSystem** — a set of coded concepts (codes) with meanings.
- **ValueSet** — a selected set of codes drawn from one or more CodeSystems.
- **ConceptMap** — a mapping between codes in different systems.
- **Canonical URL** — the stable, version-independent identifier of a
  conformance resource (StructureDefinition, ValueSet, …).
- **StructureDefinition** — the FHIR metadata resource that defines a
  resource/datatype/profile; the generator reads these.
- **Snapshot / Differential** — the fully-resolved vs. delta list of elements
  in a StructureDefinition. The generator uses the snapshot.
- **Narrative** — human-readable XHTML summary of a resource (`text`).
- **Extension** — the FHIR mechanism for adding data not in the base
  definition; every element may carry extensions.

## Project terms

- **The model** — the generated/authored Rust types under `r5::types`,
  `r5::resources`, `r5::codes`.
- **The generator / parse layer** — `r5::parse`, which turns spec JSON into
  Rust (`tmp/out/*.rs`).
- **The green gate** — build + tests + doctests + `clippy --all-targets` all
  clean; the release bar for every change.
- **Newtype primitive** — a primitive represented as `struct X(pub Inner)` that
  serializes transparently as its inner scalar.
- **Round-trip-of-default** — the preferred test/doctest pattern: serialize a
  `Default` value and deserialize it back to an equal value.
- **Nested backbone struct** — a Rust struct named `<Parent><Field>` (e.g.
  `TimingRepeat`) representing a FHIR backbone element.
- **Spec-driven development** — behaviour is written in `spec/*` first, then
  implemented; the specs are the source of truth.

## Crate identity

- Package/crate name: **`fhir`** (import as `use fhir::…`).
- Workspace member: **`fhir-derive`** (proc-macro crate).
