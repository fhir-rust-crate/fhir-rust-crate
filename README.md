# FHIR for Rust

[![CI](https://github.com/joelparkerhenderson/fhir-rust-crate/actions/workflows/ci.yml/badge.svg)](https://github.com/joelparkerhenderson/fhir-rust-crate/actions/workflows/ci.yml)

A Rust implementation of the **HL7 FHIR®** data model, plus a code generator
that produces it from the official FHIR specification JSON files. Two releases
are modelled: **R5 (5.0.0)** and **R4 (4.0.1)**.

Fast Healthcare Interoperability Resources (FHIR, pronounced "fire") is the HL7
standard for exchanging electronic health records. This crate lets you build,
parse, validate, and round-trip FHIR resources in idiomatic Rust with `serde`.

> **Status:** stable (1.0). Both data models (resources, datatypes, primitives,
> code systems, validation) are implemented and green, and the API follows
> semantic versioning.

> FHIR® is a registered trademark of Health Level Seven International. This crate
> is not affiliated with or endorsed by HL7.

## Features

Per release, under `fhir::r5` and `fhir::r4`:

- **Every resource** (Patient, Observation, Encounter, …) as a Rust struct,
  round-tripping to and from canonical FHIR JSON via `serde` — 158 in R5,
  146 in R4.
- **Every complex datatype** (Period, HumanName, CodeableConcept, …) and every
  **primitive newtype** (`Code`, `Id`, `DateTime`, …), serializing transparently.
- **400+ code systems** as type-safe enums that serialize to their canonical
  FHIR code strings.
- **A polymorphic `Resource` enum**, tagged by `resourceType`, for reading a
  resource whose type you do not know ahead of time.
- **Lightweight validation** via a `Validate` trait and `#[derive(Validate)]`
  that walks every field recursively.
- **Builders, a prelude, extension helpers, `Bundle` utilities**, and summary
  serialization.
- Optional **async REST client** (`client`) and **FHIR XML** (`xml`).

And a **code generator** that reads the bundled specification JSON for a release
and emits that release's Rust model.

## Installation

Each release is a complete model of ~135,000 lines of Rust, so they are cargo
features: you compile only what you use. `r5` is on by default.

```toml
[dependencies]
# R5 only (the default)
fhir = "1"

# R5 and R4
# fhir = { version = "1", features = ["r4"] }

# R4 only
# fhir = { version = "1", default-features = false, features = ["r4"] }

serde_json = "1" # or any other serde data format
```

## Choosing a release

An R4 `Patient` and an R5 `Patient` are **different Rust types**, on purpose.
The releases genuinely disagree — `Observation.value[x]` admits 11 types in R4
and 13 in R5, `MedicationRequest.medication[x]` is a choice element in R4 but a
`CodeableReference` in R5, and R4 has no `integer64`, `CodeableReference`, or
`RatioRange` at all. A single type standing for both would either accept data
that is invalid in both releases or silently drop data that is valid in one.

The two modules are otherwise identical in shape, so porting code between
releases is a matter of changing one path segment:

```rust
use fhir::r4::resources::Patient;   // instead of fhir::r5::resources::Patient
use fhir::r4::codes::AdministrativeGender;
```

To move data between releases, go through JSON and decide explicitly what to do
with whatever does not carry over — serde will tell you what the target release
will not accept, rather than discarding it:

```rust
let json = serde_json::to_value(&r4_patient)?;
let r5_patient: fhir::r5::resources::Patient = serde_json::from_value(json)?;
```

See the `r4_and_r5_side_by_side` example for a worked version.

What the releases *share* lives at the crate root and is re-exported by both, so
`fhir::r4::validate::Validate` and `fhir::r5::validate::Validate` are the same
trait: validation, `Coded<E>`, builders, the element metadata table, date/time
parsing, and the REST client.

## Quick start

Build a `Patient`, serialize to canonical FHIR JSON, and parse it back:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::coded::Coded;
use fhir::r5::codes::AdministrativeGender;
use fhir::r5::types::{Boolean, HumanName, String as FhirString};

let patient = Patient {
    id: Some(FhirString("pat-1".to_string())),
    active: Some(Boolean(true)),
    gender: Some(Coded::Known(AdministrativeGender::Male)),
    name: vec![HumanName {
        family: Some(FhirString("Chalmers".to_string())),
        given: vec![FhirString("Peter".to_string())],
        ..Default::default()
    }],
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
  | `0..*`           | `Vec<T>`         |
  | `1..*`           | `Vec1<T>`        |

- **`value[x]` choice elements** are one generated enum per element (e.g.
  `Observation.value` is `Option<ObservationValue>` with a variant per allowed
  type), so exactly one type is set at compile time.
- **Required-binding coded fields** are their `codes::` enum wrapped in
  `Coded<E>` (a `Known(E)` | `Unknown(String)` fallback for wire compatibility).
- **Builders**: `Type::builder()…build()` enforces required `1..1` fields; a
  `fhir::prelude` re-exports the common items.
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
cargo run --example primitive_extensions  # _field primitive extensions
cargo run --example operation_outcome     # validation → OperationOutcome
cargo run --example extensions            # ExtensionExt: get/set extensions
cargo run --example transaction_bundle    # build/read a transaction Bundle
cargo run --example client_crud --features client  # REST CRUD vs HAPI

cargo run --example r4_patient --features r4              # the same, in R4
cargo run --example r4_and_r5_side_by_side --features "r4 r5"  # both at once
```

The R5 examples all work for R4 by changing `r5` to `r4` in the imports.

## Crate layout

```txt
src/
  lib.rs            Crate root and guide (see `cargo doc --open`)

  # Shared by every release, and re-exported from each.
  validate.rs       `Validate` trait, `ValidationIssue`, primitive checks
  coded.rs          `Coded<E>` for required-binding codes
  builder.rs        `BuilderError`
  meta.rs           The shape of the per-element metadata table
  temporal.rs       Date/time parsing and precision-aware comparison
  summary.rs        `_summary=true` pruning
  xml.rs            FHIR XML bridge (feature `xml`)
  client.rs         Async REST client, generic over a release (feature `client`)
  release.rs        The `Release` trait
  codegen/          The generator: specification JSON -> a release's Rust

  # One tree per release, identical in shape.
  r5/               158 resources, 50 datatypes, 21 primitives, 419 code enums
  r4/               146 resources, 43 datatypes, 20 primitives, 486 code enums
    resources/      Resource structs + the polymorphic `Resource` enum
    types/          Complex datatypes + primitive newtypes
    codes.rs        FHIR CodeSystems as enums
    validate.rs     That release's primitive constraints
    meta/           That release's generated element metadata

fhir-derive-macros/ Proc-macro crate providing `#[derive(Validate)]`
doc/                Bundled FHIR specification JSON, one directory per release
examples/           Runnable example programs
```

## Documentation

- **The guide** — a task-oriented mdBook in [`book/`](book/) (getting started,
  model mapping, JSON serialization, validation, terminology, extensions,
  bundles, and generator internals). Build it with `mdbook build book`.
- **API reference** — build and open the full API docs, including the crate
  guide and every resource/datatype:

  ```sh
  cargo doc --open
  ```

## The code generator

Each release's `types`, `resources`, `codes`, and `meta` modules are derived
from that release's official specification JSON in
`doc/fhir-specifications/<release>/fhir-definitions-json/`. The generator lives
under `src/codegen`; the binary in `src/main.rs` drives it:

```sh
cargo run -- r4                    # rewrite src/r4 from the R4 definitions
cargo run -- r5 --out tmp/out/r5   # emit R5 elsewhere, to compare
```

`src/r4` is entirely generated and safe to rewrite. `src/r5` is not: it carries
hand-written prose on top of generated shapes, so `cargo run -- r5` refuses to
write there without an explicit `--out`. See [`AGENTS.md`](AGENTS.md) and
[`spec/`](spec/) for the generator's design and conventions.

---

## FHIR specification reference

The remainder of this document is background reference on the FHIR
specification files the generator consumes (described for R5; R4 publishes the
same bundles). It is useful when working on the generator itself.

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

## FHIR®

FHIR® is the registered trademark of HL7 and is used with the permission of HL7. Use of the FHIR trademark does not constitute endorsement of this library by HL7.

## License

Licensed under any of:

- MIT
- Apache License 2.0
- BSD 3-Clause
- GPL 2.0 only
- GPL 3.0 only

at your option.
