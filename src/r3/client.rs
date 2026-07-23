//! An async FHIR R3 REST client (feature `client`).
//!
//! The REST protocol does not vary by release, so the implementation lives once
//! in [`crate::client`]; this module pins it to R3.
//!
//! ```no_run
//! # async fn demo() -> Result<(), fhir::r3::client::ClientError> {
//! use fhir::r3::client::Client;
//!
//! let client = Client::new("https://hapi.fhir.org/baseR4");
//! let patient = client.read("Patient", "example").await?;
//! println!("{patient:?}");
//! # Ok(()) }
//! ```

/// An async FHIR R3 REST client.
pub type Client = crate::client::ReleaseClient<super::R3>;

/// An error from a FHIR R3 REST interaction.
pub type ClientError = crate::client::ReleaseClientError<super::R3>;
