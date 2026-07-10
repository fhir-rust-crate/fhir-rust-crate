# Testing

The crate must stay **green**: `cargo build`, `cargo test`, and
`cargo clippy --all-targets` all clean, with `clippy::pedantic` enabled. This
document describes the test patterns so new code matches them.

## The green gate

Run all three before finishing any task:

```sh
cargo build
cargo test                 # unit tests AND doctests
cargo clippy --all-targets # must print zero warnings
```

Current baseline: 646 unit tests + 199 doctests pass, 0 clippy warnings. A
change that reduces this is a regression.

## Unit test pattern

Every model module carries a `#[cfg(test)] mod tests` with, at minimum, a
default construction test and a serde round-trip test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    type T = Example;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
```

### Round-trip, don't hardcode JSON

Prefer the **round-trip-of-default** pattern above over asserting an exact
`json!({...})` shape. Hardcoded shapes break whenever a field's representation
changes (for example, when a required primitive field is present). Round-tripping
the default value is stable regardless of which fields are required.

## Doctests

Doctests run **only because the crate has a library target**. Keep them:

- Real and compiling. Pseudo-code examples must use a ```` ```text ```` fence,
  not ```` ```no_run ````, or they will fail to compile.
- Import via the crate name `fhir`, e.g.
  `use fhir::r5::resources::Patient;`.

Typical struct doctest:

```rust
/// # Examples
///
/// ```
/// use fhir::r5::types::period::Period;
///
/// let value = Period::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Period = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
```

## Validation tests

`#[derive(Validate)]` recurses through fields; nested issues get a dotted
`path`. Assert on that:

```rust
let coding = types::Coding {
    code: Some(types::Code("bad  code".to_string())), // double space is invalid
    ..Default::default()
};
assert_eq!(coding.validate()[0].path, "code.code");
```

See [`../spec/07-validation.md`](../spec/07-validation.md).

## Generator / parse tests

Each `r5::parse::<bundle>` module has a `test_serde_json_from_reader` test that
deserializes the real spec JSON bundle from
`doc/fhir-specifications/r5/fhir-definitions-json/`. If a spec field is missing
from a parse struct, this test fails with `unknown field …` — add the field.

## Clippy notes

- `clippy::pedantic` is enabled crate-wide in `src/lib.rs`, with a small set of
  documented `#![allow(...)]` (e.g. `doc_markdown`, `wildcard_imports`,
  `module_inception`). Do not add new blanket allows without justification.
- `cargo clippy` alone does not lint `#[cfg(test)]` code; always use
  `--all-targets` so test modules are checked too.
