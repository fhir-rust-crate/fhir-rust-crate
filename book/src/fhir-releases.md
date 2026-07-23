# FHIR releases

This crate models two FHIR releases:

| Release | Module | Cargo feature | Resources | Datatypes | Primitives | Code enums |
| --- | --- | --- | --- | --- | --- | --- |
| R5 (5.0.0) | `fhir::r5` | `r5` (default) | 158 | 50 | 21 | 419 |
| R4 (4.0.1) | `fhir::r4` | `r4` | 146 | 43 | 20 | 486 |

## Enabling a release

Each release is a complete model of roughly 135,000 lines of generated Rust, so
you compile only what you use. `r5` is on by default:

```toml
[dependencies]
# R5 only (the default)
fhir = "1"

# R5 and R4
# fhir = { version = "1", features = ["r4"] }

# R4 only
# fhir = { version = "1", default-features = false, features = ["r4"] }
```

Enabling both roughly doubles build time, which is why neither is implied by the
other.

## The two models are the same shape

Every module a release exposes has a counterpart in the other, so moving code
between releases means changing one path segment:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::codes::AdministrativeGender;
```

```rust
use fhir::r4::resources::Patient;
use fhir::r4::codes::AdministrativeGender;
```

Everything in this guide — builders, validation, choice enums, `Coded<E>`,
extensions, bundles, summary serialization — works identically in both.

## …but they are not the same types

`fhir::r4::resources::Patient` and `fhir::r5::resources::Patient` are distinct
Rust types, and there is deliberately no conversion between them.

The releases genuinely disagree:

- `Observation.value[x]` admits 11 types in R4 and 13 in R5 (R5 added
  `Attachment` and `Reference`).
- `MedicationRequest.medication[x]` is a choice element in R4 and a
  `CodeableReference` in R5.
- R4 has no `integer64`, `CodeableReference`, or `RatioRange` datatype. R5 has
  no `Contributor`, `Population`, or `SubstanceAmount`.
- `Bundle.link.relation` is a free `string` in R4 and a bound code in R5.

A single Rust type standing for both would have to be either the union of the
two (accepting data that is invalid in *both* releases, and letting an R5-only
element be written to an R4 server) or the intersection (silently dropping data
that is valid in the release it came from). Both failures are silent, and both
corrupt health records. Distinct types turn the mismatch into a compile error.

## Converting between releases

Go through JSON, and let serde tell you what the target release will not accept:

```rust
let json = serde_json::to_value(&r4_patient)?;
let r5_patient: fhir::r5::resources::Patient = serde_json::from_value(json)?;
```

For a `Patient` this succeeds, because the resource's shape is stable across the
two releases. For resources where it does not, the error names the element that
does not carry over, and you decide what to do about it — drop it, map it to the
target release's equivalent, or refuse the conversion. Nothing is discarded
behind your back.

The `meta` tables tell you where the releases differ, without guessing:

```rust
let r4_value = fhir::r4::meta::element("Observation.value[x]").unwrap();
let r5_value = fhir::r5::meta::element("Observation.value[x]").unwrap();

let added: Vec<&str> = r5_value
    .type_codes()
    .filter(|code| !r4_value.type_codes().any(|c| c == *code))
    .collect();
assert_eq!(added, ["Attachment", "Reference"]);
```

Run the full worked version with:

```sh
cargo run --example r4_and_r5_side_by_side --features "r4 r5"
```

## What the releases share

Anything that does not name a release's types is defined once at the crate root
and re-exported by both, so `fhir::r4::validate::Validate` and
`fhir::r5::validate::Validate` are the **same trait**:

| Crate root | Purpose |
| --- | --- |
| `fhir::validate` | The `Validate` trait and `ValidationIssue` |
| `fhir::coded` | `Coded<E>`, the required-binding wrapper |
| `fhir::builder` | `BuilderError` |
| `fhir::meta` | The element-metadata table types and lookups |
| `fhir::temporal` | Date/time parsing and precision-aware comparison |
| `fhir::summary` | `_summary=true` pruning |
| `fhir::xml` | The FHIR XML bridge (feature `xml`) |
| `fhir::client` | The async REST client (feature `client`) |
| `fhir::release` | The `Release` trait |

Because the trait is shared, one function can validate values of either release:

```rust
use fhir::validate::Validate;

fn check<T: Validate>(value: &T) -> bool {
    value.validate().is_empty()
}
```

The REST client is generic over `Release` for the same reason. `fhir::r4::client::Client`
and `fhir::r5::client::Client` are two aliases for one implementation, each
returning its own release's `Resource`, `Bundle`, and `OperationOutcome`.
