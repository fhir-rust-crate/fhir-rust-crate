//! Lightweight FHIR R5 validation.
//!
//! Provides a [`Validate`] trait and format checks for the FHIR primitive
//! datatypes (the regular-expression constraints from the specification,
//! implemented without an external regex dependency).
//!
//! Recursive validation of complex types and resources (walking every field)
//! is intended to be added via a `#[derive(Validate)]` procedural macro; until
//! then, callers validate primitive values directly.
//!
//! # Examples
//!
//! ```
//! use fhir::r5::types::Id;
//! use fhir::r5::validate::Validate;
//!
//! assert!(Id("patient-1".to_string()).validate().is_empty());
//! assert!(!Id("bad id!".to_string()).validate().is_empty());
//! ```

use crate::r5::types;

/// A single validation problem, with the location and a human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    /// A path or label identifying what failed (e.g. the datatype name).
    pub path: String,
    /// A human-readable description of the problem.
    pub message: String,
}

impl ValidationIssue {
    fn new(path: &str, message: impl Into<String>) -> Self {
        Self {
            path: path.to_string(),
            message: message.into(),
        }
    }
}

/// Types that can validate themselves against FHIR constraints.
pub trait Validate {
    /// Return all validation issues; an empty vector means the value is valid.
    fn validate(&self) -> Vec<ValidationIssue>;

    /// Convenience: `true` when [`Validate::validate`] finds no issues.
    fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        self.as_ref().map(Validate::validate).unwrap_or_default()
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        self.iter().flat_map(Validate::validate).collect()
    }
}

/// FHIR `code`: non-empty, no leading/trailing whitespace, single internal
/// spaces (regex `[^\s]+(\s[^\s]+)*`).
fn is_valid_code(s: &str) -> bool {
    !s.is_empty()
        && s == s.trim()
        && !s.split(' ').any(str::is_empty)
        && !s.chars().any(|c| c != ' ' && c.is_whitespace())
}

/// FHIR `id`: 1..=64 chars from `[A-Za-z0-9-.]`.
fn is_valid_id(s: &str) -> bool {
    (1..=64).contains(&s.len())
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
}

impl Validate for types::Code {
    fn validate(&self) -> Vec<ValidationIssue> {
        if is_valid_code(&self.0) {
            vec![]
        } else {
            vec![ValidationIssue::new("code", format!("invalid FHIR code: {:?}", self.0))]
        }
    }
}

impl Validate for types::Id {
    fn validate(&self) -> Vec<ValidationIssue> {
        if is_valid_id(&self.0) {
            vec![]
        } else {
            vec![ValidationIssue::new("id", format!("invalid FHIR id: {:?}", self.0))]
        }
    }
}

impl Validate for types::Oid {
    fn validate(&self) -> Vec<ValidationIssue> {
        if self.0.starts_with("urn:oid:") {
            vec![]
        } else {
            vec![ValidationIssue::new("oid", "FHIR oid must start with `urn:oid:`")]
        }
    }
}

impl Validate for types::Uuid {
    fn validate(&self) -> Vec<ValidationIssue> {
        if self.0.starts_with("urn:uuid:") {
            vec![]
        } else {
            vec![ValidationIssue::new("uuid", "FHIR uuid must start with `urn:uuid:`")]
        }
    }
}

impl Validate for types::Uri {
    fn validate(&self) -> Vec<ValidationIssue> {
        if self.0.trim() == self.0 && !self.0.is_empty() {
            vec![]
        } else {
            vec![ValidationIssue::new("uri", "FHIR uri must be non-empty and not surrounded by whitespace")]
        }
    }
}

impl<T: Validate> Validate for Box<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        (**self).validate()
    }
}

/// A zero-sized type marker (e.g. the phantom on `Reference<T>`) has nothing to
/// validate.
impl<T: ?Sized> Validate for ::std::marker::PhantomData<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

/// Arbitrary embedded JSON (e.g. a `contained` resource) is not structurally
/// validated here.
impl Validate for ::serde_json::Value {
    fn validate(&self) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

/// A bare `String` (used by a few fields) carries no FHIR-level constraint here.
impl Validate for ::std::string::String {
    fn validate(&self) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

impl Validate for types::Canonical {
    fn validate(&self) -> Vec<ValidationIssue> {
        if !self.0.is_empty() && self.0.trim() == self.0 {
            vec![]
        } else {
            vec![ValidationIssue::new("canonical", "must be non-empty and not surrounded by whitespace")]
        }
    }
}

impl Validate for types::Url {
    fn validate(&self) -> Vec<ValidationIssue> {
        if !self.0.is_empty() && self.0.trim() == self.0 {
            vec![]
        } else {
            vec![ValidationIssue::new("url", "must be non-empty and not surrounded by whitespace")]
        }
    }
}

/// Primitives whose Rust representation already guarantees validity, so there
/// is no further structural constraint to check here.
macro_rules! always_valid {
    ($($t:ty),* $(,)?) => {
        $(impl Validate for $t {
            fn validate(&self) -> Vec<ValidationIssue> { Vec::new() }
        })*
    };
}

always_valid!(
    types::Base64Binary,
    types::Boolean,
    types::Date,
    types::DateTime,
    types::Decimal,
    types::Instant,
    types::Integer,
    types::Integer64,
    types::Markdown,
    types::PositiveInt,
    types::String,
    types::Time,
    types::UnsignedInt,
    types::Xhtml,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_ok() {
        assert!(types::Code("final".to_string()).is_valid());
        assert!(types::Code("entered in error".to_string()).is_valid());
    }

    #[test]
    fn code_bad() {
        assert!(!types::Code(" leading".to_string()).is_valid());
        assert!(!types::Code("double  space".to_string()).is_valid());
        assert!(!types::Code(String::new()).is_valid());
    }

    #[test]
    fn id_ok_and_bad() {
        assert!(types::Id("abc-123.4".to_string()).is_valid());
        assert!(!types::Id("bad id!".to_string()).is_valid());
        assert!(!types::Id("x".repeat(65)).is_valid());
    }

    #[test]
    fn option_and_vec() {
        let good: Option<types::Code> = Some(types::Code("ok".to_string()));
        assert!(good.validate().is_empty());
        let bad = vec![types::Id("ok".to_string()), types::Id("no!".to_string())];
        assert_eq!(bad.validate().len(), 1);
    }

    #[test]
    fn derived_validate_recurses_with_dotted_path() {
        // The `#[derive(Validate)]` on Coding recurses into its `code` field,
        // whose primitive validator flags the double space.
        let coding = types::Coding {
            code: Some(types::Code("bad  code".to_string())),
            ..Default::default()
        };
        let issues = coding.validate();
        assert_eq!(issues.len(), 1, "{issues:?}");
        assert_eq!(issues[0].path, "code.code");

        // A well-formed Coding validates clean.
        let ok = types::Coding {
            code: Some(types::Code("final".to_string())),
            ..Default::default()
        };
        assert!(ok.validate().is_empty());
    }
}
