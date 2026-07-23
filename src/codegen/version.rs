//! The FHIR releases this crate can model, and where each one's inputs and
//! outputs live.
//!
//! Everything the generator does that differs between releases is reachable
//! from [`Version`]: which definition bundles to read, which `src/<module>`
//! tree to write, and which release the emitted code names in its paths and
//! documentation.

use std::path::PathBuf;

/// A FHIR release that the generator can produce a model for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// FHIR Release 3 (3.0.2), also published as STU3.
    R3,
    /// FHIR Release 4 (4.0.1).
    R4,
    /// FHIR Release 5 (5.0.0).
    R5,
}

impl Version {
    /// Every release the generator knows about.
    pub const ALL: [Version; 3] = [Version::R3, Version::R4, Version::R5];

    /// Parse a release token such as `"r4"` (case-insensitive, `R4`/`4.0.1`
    /// also accepted).
    #[must_use]
    pub fn parse(token: &str) -> Option<Self> {
        match token.to_ascii_lowercase().as_str() {
            "r3" | "stu3" | "3" | "3.0" | "3.0.1" | "3.0.2" => Some(Version::R3),
            "r4" | "4" | "4.0" | "4.0.1" => Some(Version::R4),
            "r5" | "5" | "5.0" | "5.0.0" => Some(Version::R5),
            _ => None,
        }
    }

    /// The Rust module name for this release's model, e.g. `"r4"`.
    #[must_use]
    pub fn module(self) -> &'static str {
        match self {
            Version::R3 => "r3",
            Version::R4 => "r4",
            Version::R5 => "r5",
        }
    }

    /// The uppercase release label used in prose, e.g. `"R4"`.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Version::R3 => "R3",
            Version::R4 => "R4",
            Version::R5 => "R5",
        }
    }

    /// The full FHIR version string, e.g. `"4.0.1"`.
    #[must_use]
    pub fn version_string(self) -> &'static str {
        match self {
            Version::R3 => "3.0.2",
            Version::R4 => "4.0.1",
            Version::R5 => "5.0.0",
        }
    }

    /// The published specification base URL, e.g. `https://hl7.org/fhir/R4/`.
    #[must_use]
    pub fn spec_url(self) -> &'static str {
        match self {
            // R3 is published under its STU3 name.
            Version::R3 => "https://hl7.org/fhir/STU3/",
            Version::R4 => "https://hl7.org/fhir/R4/",
            Version::R5 => "https://hl7.org/fhir/R5/",
        }
    }

    /// The directory holding this release's official definition JSON bundles.
    #[must_use]
    pub fn definitions_dir(self) -> PathBuf {
        crate_root()
            .join("doc")
            .join("fhir-specifications")
            .join(self.module())
            .join("fhir-definitions-json")
    }

    /// The `src/<module>` directory this release's model is written to.
    #[must_use]
    pub fn source_dir(self) -> PathBuf {
        crate_root().join("src").join(self.module())
    }

    /// The definition bundles that describe primitive and complex datatypes.
    #[must_use]
    pub fn types_bundle(self) -> PathBuf {
        self.definitions_dir().join("profiles-types.json")
    }

    /// The definition bundle that describes resources.
    #[must_use]
    pub fn resources_bundle(self) -> PathBuf {
        self.definitions_dir().join("profiles-resources.json")
    }

    /// The definition bundles that carry `CodeSystem`s for the `codes` module.
    ///
    /// R4 ships the HL7 v2 and v3 code systems as separate bundles; R5 folds
    /// everything it defines into `valuesets.json`.
    #[must_use]
    pub fn code_system_bundles(self) -> Vec<PathBuf> {
        // Both releases publish their `CodeSystem`s in `valuesets.json`. R4
        // additionally ships `v2-tables.json` and `v3-codesystems.json`, which
        // this crate deliberately leaves out: they are external HL7 v2/v3
        // terminologies, not FHIR-defined ones, and would add thousands of
        // enum variants that no FHIR element has a `required` binding to.
        vec![self.definitions_dir().join("valuesets.json")]
    }
}

/// The crate's root directory (where `Cargo.toml` lives).
pub(crate) fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        assert_eq!(Version::parse("r3"), Some(Version::R3));
        assert_eq!(Version::parse("STU3"), Some(Version::R3));
        assert_eq!(Version::parse("r4"), Some(Version::R4));
        assert_eq!(Version::parse("R5"), Some(Version::R5));
        assert_eq!(Version::parse("4.0.1"), Some(Version::R4));
        assert_eq!(Version::parse("r6"), None);
    }

    #[test]
    fn paths_are_release_scoped() {
        for version in Version::ALL {
            assert!(version.definitions_dir().ends_with("fhir-definitions-json"));
            assert!(version.source_dir().ends_with(version.module()));
            assert!(version.types_bundle().ends_with("profiles-types.json"));
        }
    }
}
