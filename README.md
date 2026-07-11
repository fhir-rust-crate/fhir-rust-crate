# FHIR R5 for Rust

[![CI](https://github.com/joelparkerhenderson/fhir-rust-crate/actions/workflows/ci.yml/badge.svg)](https://github.com/joelparkerhenderson/fhir-rust-crate/actions/workflows/ci.yml)

A Rust implementation of the **HL7 FHIR® Release 5 (R5)** data model, plus a
code generator that produces it from the official FHIR specification JSON files.

Fast Healthcare Interoperability Resources (FHIR, pronounced "fire") is the HL7
standard for exchanging electronic health records. This crate lets you build,
parse, validate, and round-trip FHIR resources in idiomatic Rust with `serde`.

> **Status:** work in progress. The R5 data model (resources, datatypes,
> primitives, code systems, validation) is implemented and green; APIs may still
> change before 1.0.

> FHIR® is a registered trademark of Health Level Seven International. This crate
> is not affiliated with or endorsed by HL7.

## Features

- **158 R5 resources** (Patient, Observation, Encounter, …) as Rust structs,
  each round-tripping to and from canonical FHIR JSON via `serde`.
- **~50 complex datatypes** (Period, HumanName, CodeableConcept, …) and **21
  primitive newtypes** (`Code`, `Id`, `DateTime`, …) that serialize
  transparently.
- **400+ code systems** as type-safe enums that serialize to their canonical
  FHIR code strings.
- **A polymorphic `Resource` enum**, tagged by `resourceType`, for reading a
  resource whose type you do not know ahead of time.
- **Lightweight validation** via a `Validate` trait and `#[derive(Validate)]`
  that walks every field recursively.
- **A code generator** that reads the bundled FHIR R5 spec JSON and emits Rust.

## Installation

```toml
[dependencies]
fhir = "0.1"
serde_json = "1" # or any other serde data format
```

## Quick start

Build a `Patient`, serialize to canonical FHIR JSON, and parse it back:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::types::{Boolean, Code, HumanName, String as FhirString};

let patient = Patient {
    id: Some(FhirString("pat-1".to_string())),
    active: Some(Boolean(true)),
    gender: Some(Code("male".to_string())),
    name: Some(vec![HumanName {
        family: Some(FhirString("Chalmers".to_string())),
        given: vec![FhirString("Peter".to_string())],
        ..Default::default()
    }]),
    ..Default::default()
};

let json = serde_json::to_string_pretty(&patient).unwrap();
let parsed: Patient = serde_json::from_str(&json).unwrap();
assert_eq!(parsed, patient);
```

## How the model maps to Rust

Everything derives `serde::Serialize` and `serde::Deserialize`, so you work
through `serde_json` (or any serde format).

- **Primitives are transparent newtypes.** `Code("final")` serializes to the
  JSON string `"final"` — no wrapper object. (`integer64` is the FHIR-mandated
  exception: it serializes as a JSON *string*.)
- **Element cardinality maps directly:**

  | FHIR cardinality | Rust type        |
  |------------------|------------------|
  | `0..1`           | `Option<T>`      |
  | `1..1`           | `T`              |
  | `0..*`           | `Option<Vec<T>>` |
  | `1..*`           | `Vec<T>`         |

- **`value[x]` choice elements** are flattened into one field per allowed type,
  named `value_<type>` (e.g. `Observation` has `value_quantity`,
  `value_string`, `value_boolean`, …); set exactly one.
- **Nested backbone elements** become nested structs named `<Parent><Field>`
  (e.g. `PatientContact`, `BundleEntry`).
- **Unset optional fields are omitted** from the JSON (`skip_serializing_none`).

## Validation

`Validate` reports every problem as a `ValidationIssue { path, message }`.
Primitive types check their FHIR regex constraints; `#[derive(Validate)]` makes
complex types and resources validate recursively, prefixing each nested issue's
`path` with the field name.

```rust
use fhir::r5::types::Id;
use fhir::r5::validate::Validate;

assert!(Id("patient-1".to_string()).is_valid());
assert!(!Id("has spaces".to_string()).is_valid());
```

## Code systems

```rust
use fhir::r5::codes::AdministrativeGender;

let gender = AdministrativeGender::Female;
assert_eq!(serde_json::to_value(&gender).unwrap(), "female");
```

## Reading a resource of unknown type

```rust
use fhir::r5::resources::Resource;

let json = serde_json::json!({ "resourceType": "Patient", "id": "pat-1" });
match serde_json::from_value(json).unwrap() {
    Resource::Patient(patient) => assert_eq!(patient.id.unwrap().0, "pat-1"),
    _ => unreachable!(),
}
```

## Runnable examples

Programs in the [`examples/`](examples/) directory demonstrate common tasks:

```sh
cargo run --example build_patient      # build a resource and print its JSON
cargo run --example validate_resource  # recursive validation and issue paths
cargo run --example read_bundle        # dispatch on each entry's resourceType
cargo run --example code_systems       # code-system enums
```

## Crate layout

```txt
src/
  lib.rs            Crate root and guide (see `cargo doc --open`)
  r5/
    resources/      158 resource structs + the polymorphic `Resource` enum
    types/          ~50 complex datatypes + 21 primitive newtypes
    codes.rs        FHIR CodeSystems as enums
    validate.rs     `Validate` trait + primitive constraints
    parse/          Code generator that reads the spec JSON
fhir-derive-macros/ Proc-macro crate providing `#[derive(Validate)]`
doc/                Bundled FHIR R5 specification JSON files
examples/           Runnable example programs
```

## Documentation

Build and open the full API documentation, including the crate guide and every
resource/datatype:

```sh
cargo doc --open
```

## The code generator

The types under `src/r5/types` and `src/r5/resources` are derived from the
official FHIR R5 specification JSON in
`doc/fhir-specifications/r5/fhir-definitions-json/` (exposed at runtime as
`fhir::DEFINITIONS_DIR`). The generator lives under `src/r5/parse`; the binary
in `src/main.rs` drives it. See [`AGENTS.md`](AGENTS.md) and
[`spec/`](spec/) for the generator's design and conventions.

---

## FHIR specification reference

The remainder of this document is background reference on the FHIR R5
specification files the generator consumes. It is useful when working on the
generator itself.

### Datatype categories

FHIR R5 datatypes live in `profiles-types.json`, which distinguishes primitive
types (lowercase names) from complex types (uppercase names).

**Primitive types:** `base64Binary`, `boolean`, `canonical`, `code`, `date`,
`dateTime`, `decimal`, `id`, `instant`, `integer`, `integer64`, `markdown`,
`oid`, `positiveInt`, `string`, `time`, `unsignedInt`, `uri`, `url`, `uuid`.

**General-purpose complex types:** Address, Age, Annotation, Attachment,
CodeableConcept, Coding, ContactPoint, Count, Distance, Duration, HumanName,
Identifier, Money, MoneyQuantity, Period, Quantity, Range, Ratio, RatioRange,
SampledData, Signature, SimpleQuantity, Timing.

**Metadata complex types:** Availability, ContactDetail, Contributor,
DataRequirement, Expression, ExtendedContactDetail, MonetaryComponent,
ParameterDefinition, RelatedArtifact, TriggerDefinition, UsageContext,
VirtualServiceDetail.

**Special-purpose complex types:** BackboneType, CodeableReference, Dosage,
ElementDefinition, Extension, Meta, Narrative, Reference, xhtml.

You can list the ids straight from the spec with `jq`:

```sh
<profiles-types.json jq -r '.entry | map(select(.resource.kind == "primitive-type")) | map(.resource.id)[]'
<profiles-types.json jq -r '.entry | map(select(.resource.kind == "complex-type"))   | map(.resource.id)[]'
```

### Element extension URLs

Any element defined in any version of FHIR is automatically assigned an
extension URL that uniquely identifies it:

```txt
http://hl7.org/fhir/[version]/StructureDefinition/extension-[Path]
```

### Snapshot view versus differential view

A FHIR profile offers two views of a profiled resource:

- **Snapshot** — the complete, final structure after applying all changes from
  the differential to the base resource. Self-contained; useful when you do not
  have the base resource at hand.
- **Differential** — only the differences (added, modified, or removed elements)
  the profile introduces relative to its base. Useful for understanding what a
  profile customizes.

### FHIR documentation links

- Datatypes: <https://build.fhir.org/datatypes.html>
- JSON representation: <https://build.fhir.org/json.html>
- UML: <https://build.fhir.org/uml.html>
- References: <https://build.fhir.org/references.html>
- Extensibility: <https://build.fhir.org/extensibility.html>
- Narrative: <https://build.fhir.org/narrative.html>
- Resource: <https://build.fhir.org/resource.html>
- Versions / standards process: <https://build.fhir.org/versions.html#std-process>

## License

Licensed under any of:

- MIT
- Apache License 2.0
- BSD 3-Clause
- GPL 2.0 only
- GPL 3.0 only

at your option.
