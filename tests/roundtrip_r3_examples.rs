//! Round-trip the official FHIR R3 example resources through the data model.
//!
//! For every example `*.json` file we:
//!
//! 1. Parse the raw JSON into a [`serde_json::Value`] (the oracle).
//! 2. Deserialize that value into the polymorphic
//!    [`Resource`](fhir::r3::resources::Resource) enum.
//! 3. Re-serialize the `Resource` back to a [`serde_json::Value`].
//! 4. Assert the re-serialized value equals the original.
//!
//! # Two entry points
//!
//! - [`roundtrip_curated_subset`] — always runs. It scans a small, curated set
//!   of diverse example files committed under
//!   `tests/data/roundtrip_examples_r4/`. These are chosen to pass today, so
//!   this test guards against regressions.
//!
//! - [`roundtrip_full_official_examples`] — `#[ignore]` by default. It scans the
//!   complete official example set, which is *not* committed. Populate it first
//!   with `bin/fetch-examples r3`, then run:
//!
//!   ```sh
//!   cargo test --features r3 --test roundtrip_r4_examples -- --ignored --nocapture
//!   ```
//!
//!   Point it at an alternate directory with `FHIR_ROUNDTRIP_DIR_R3`.
//!   The full run prints a per-file failure report rather than panicking on the
//!   first mismatch.

#![cfg(feature = "r3")]

mod common;

use fhir::r3::resources::Resource;
use serde_json::Value;

/// Deserialize into the R3 `Resource` enum and serialize straight back.
fn roundtrip(original: Value) -> Result<Value, String> {
    let resource: Resource = serde_json::from_value(original).map_err(|e| e.to_string())?;
    serde_json::to_value(&resource).map_err(|e| format!("re-serialize error: {e}"))
}

#[test]
fn roundtrip_curated_subset() {
    common::with_large_stack(|| {
        common::assert_all_roundtrip(&common::curated_dir("r3"), roundtrip, "R3");
    });
}

#[test]
#[ignore = "requires the full official example set; run bin/fetch-examples first"]
fn roundtrip_full_official_examples() {
    common::with_large_stack(|| {
        common::report_all_roundtrip(
            &common::full_dir("r3", "FHIR_ROUNDTRIP_DIR_R3"),
            roundtrip,
            "R3",
        );
    });
}
