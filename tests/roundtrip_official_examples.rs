//! Round-trip the official FHIR R5 example resources through the data model.
//!
//! For every example `*.json` file we:
//!
//! 1. Parse the raw JSON into a [`serde_json::Value`] (the oracle).
//! 2. Deserialize that value into the polymorphic
//!    [`Resource`](fhir::r5::resources::Resource) enum.
//! 3. Re-serialize the `Resource` back to a [`serde_json::Value`].
//! 4. Assert the re-serialized value equals the original.
//!
//! `serde_json::Value` equality is order-independent for objects, so a passing
//! round-trip means the model preserves every field and value byte-for-byte in
//! semantic terms — nothing is silently dropped or altered.
//!
//! # Two entry points
//!
//! - [`roundtrip_curated_subset`] — always runs. It scans a small, curated set
//!   of diverse example files committed under `tests/data/roundtrip_examples/`.
//!   These are chosen to pass today, so this test guards against regressions.
//!
//! - [`roundtrip_full_official_examples`] — `#[ignore]` by default. It scans the
//!   complete official example set, which is *not* committed (it is ~190 MB).
//!   Populate it first with `bin/fetch-examples`, then run:
//!
//!   ```sh
//!   cargo test --test roundtrip_official_examples -- --ignored --nocapture
//!   ```
//!
//!   Point it at an alternate directory with `FHIR_ROUNDTRIP_DIR=/path/to/json`.
//!   The full run prints a per-file failure report rather than panicking on the
//!   first mismatch, so it can be used to burn down `tasks-roundtrip-failures.md`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use fhir::r5::resources::Resource;
use serde_json::Value;

/// Directory holding the curated, always-on subset committed to the repo.
fn curated_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("roundtrip_examples")
}

/// Directory holding the full official example set. Not committed; populated by
/// `bin/fetch-examples`. Overridable with `FHIR_ROUNDTRIP_DIR`.
fn full_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("FHIR_ROUNDTRIP_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("doc")
        .join("fhir-specifications")
        .join("r5")
        .join("fhir-examples-json")
}

/// The outcome of round-tripping a single file.
enum Outcome {
    /// Round-tripped with an exact semantic match.
    Ok,
    /// Not a FHIR resource file (no string `resourceType`); skipped.
    NotAResource,
    /// Failed to deserialize into the `Resource` enum.
    Deserialize(String),
    /// Deserialized, but re-serialization did not match the original.
    Mismatch(Mismatch),
}

/// A structured description of the first differences between original and
/// re-serialized JSON, for reporting.
struct Mismatch {
    /// JSON pointer paths that differ, with a short reason. Capped for brevity.
    diffs: Vec<String>,
}

fn roundtrip_file(path: &Path) -> Outcome {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => return Outcome::Deserialize(format!("read error: {e}")),
    };
    let original: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => return Outcome::Deserialize(format!("not valid JSON: {e}")),
    };
    // Only top-level FHIR resources have a string `resourceType`.
    if original
        .get("resourceType")
        .and_then(Value::as_str)
        .is_none()
    {
        return Outcome::NotAResource;
    }
    let resource: Resource = match serde_json::from_value(original.clone()) {
        Ok(r) => r,
        Err(e) => return Outcome::Deserialize(e.to_string()),
    };
    let reserialized = match serde_json::to_value(&resource) {
        Ok(v) => v,
        Err(e) => return Outcome::Deserialize(format!("re-serialize error: {e}")),
    };
    if reserialized == original {
        Outcome::Ok
    } else {
        let mut diffs = Vec::new();
        diff_json("", &original, &reserialized, &mut diffs);
        diffs.truncate(8);
        Outcome::Mismatch(Mismatch { diffs })
    }
}

/// Recursively collect the paths where `orig` and `got` differ.
fn diff_json(path: &str, orig: &Value, got: &Value, out: &mut Vec<String>) {
    if out.len() >= 8 {
        return;
    }
    match (orig, got) {
        (Value::Object(a), Value::Object(b)) => {
            let keys: BTreeMap<&String, ()> =
                a.keys().chain(b.keys()).map(|k| (k, ())).collect();
            for (k, ()) in keys {
                let child = format!("{path}/{k}");
                match (a.get(k), b.get(k)) {
                    (Some(av), Some(bv)) => diff_json(&child, av, bv, out),
                    (Some(_), None) => out.push(format!("{child}: dropped")),
                    (None, Some(_)) => out.push(format!("{child}: added")),
                    (None, None) => {}
                }
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                out.push(format!(
                    "{path}: array len {} -> {}",
                    a.len(),
                    b.len()
                ));
                return;
            }
            for (i, (av, bv)) in a.iter().zip(b.iter()).enumerate() {
                diff_json(&format!("{path}/{i}"), av, bv, out);
            }
        }
        _ => {
            if orig != got {
                out.push(format!("{path}: {orig} -> {got}"));
            }
        }
    }
}

/// List `*.json` files in a directory (non-recursive), sorted.
fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = match fs::read_dir(dir) {
        Ok(rd) => rd
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect(),
        Err(_) => Vec::new(),
    };
    files.sort();
    files
}

#[test]
fn roundtrip_curated_subset() {
    let dir = curated_dir();
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "no curated example files found in {}",
        dir.display()
    );

    let mut failures = Vec::new();
    let mut checked = 0usize;
    for file in &files {
        match roundtrip_file(file) {
            Outcome::Ok => checked += 1,
            Outcome::NotAResource => {}
            Outcome::Deserialize(msg) => failures.push(format!(
                "{}: deserialize failed: {msg}",
                file.file_name().unwrap().to_string_lossy()
            )),
            Outcome::Mismatch(m) => failures.push(format!(
                "{}: mismatch:\n    {}",
                file.file_name().unwrap().to_string_lossy(),
                m.diffs.join("\n    ")
            )),
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} curated examples failed round-trip:\n{}",
        failures.len(),
        files.len(),
        failures.join("\n")
    );
    assert!(checked > 0, "no resources were actually round-tripped");
}

#[test]
#[ignore = "requires the full official example set; run bin/fetch-examples first"]
fn roundtrip_full_official_examples() {
    let dir = full_dir();
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "no example files found in {} — run bin/fetch-examples first \
         (or set FHIR_ROUNDTRIP_DIR)",
        dir.display()
    );

    let mut ok = 0usize;
    let mut skipped = 0usize;
    let mut deserialize_failures: Vec<String> = Vec::new();
    let mut mismatches: Vec<String> = Vec::new();

    for file in &files {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        match roundtrip_file(file) {
            Outcome::Ok => ok += 1,
            Outcome::NotAResource => skipped += 1,
            Outcome::Deserialize(msg) => {
                deserialize_failures.push(format!("{name}\n    {msg}"));
            }
            Outcome::Mismatch(m) => {
                mismatches.push(format!("{name}\n    {}", m.diffs.join("\n    ")));
            }
        }
    }

    let total = files.len();
    println!("\n=== Full official-examples round-trip ===");
    println!("files scanned:        {total}");
    println!("skipped (non-resource): {skipped}");
    println!("passed:               {ok}");
    println!("deserialize failures: {}", deserialize_failures.len());
    println!("mismatches:           {}", mismatches.len());

    if !deserialize_failures.is_empty() {
        println!("\n--- Deserialize failures ---");
        for f in &deserialize_failures {
            println!("{f}");
        }
    }
    if !mismatches.is_empty() {
        println!("\n--- Round-trip mismatches ---");
        for f in &mismatches {
            println!("{f}");
        }
    }

    assert!(
        deserialize_failures.is_empty() && mismatches.is_empty(),
        "{} deserialize failures + {} mismatches out of {} files (see report above)",
        deserialize_failures.len(),
        mismatches.len(),
        total
    );
}
