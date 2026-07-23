//! An async FHIR REST client (feature `client`).
//!
//! [`Client`] wraps a [`reqwest`] client and speaks the FHIR RESTful API:
//! `read`, `vread`, `create`, `update`, `delete`, `search`, and `capabilities`.
//! Non-success responses are surfaced as [`ClientError`], parsing the server's
//! `OperationOutcome` when present.
//!
//! ```no_run
//! # async fn demo() -> Result<(), fhir::client::ClientError> {
//! use fhir::client::Client;
//!
//! let client = Client::new("https://hapi.fhir.org/baseR5");
//! let patient = client.read("Patient", "example").await?;
//! println!("{patient:?}");
//! # Ok(()) }
//! ```
//!
//! # Choosing a release
//!
//! The wire protocol is the same for every FHIR release; only the resource
//! types differ. [`ReleaseClient<R>`] is therefore generic over a
//! [`Release`](crate::release::Release), and each release module exposes an
//! alias for it: [`Client`] here (and [`r5::client::Client`](crate::r5::client))
//! for R5, [`r4::client::Client`](crate::r4::client) for R4.
//!
//! ```no_run
//! # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
//! let r4 = fhir::r4::client::Client::new("https://hapi.fhir.org/baseR4");
//! let bundle = r4.search("Patient", &[("name", "chalmers")]).await?;
//! # Ok(()) }
//! ```

use ::serde::Serialize;

use crate::release::Release;

/// FHIR JSON media type.
const FHIR_JSON: &str = "application/fhir+json";

/// An error from a FHIR REST interaction with release `R`.
///
/// Most code wants the release-specific alias — [`ClientError`] for R5, or
/// [`r4::client::ClientError`](crate::r4::client) for R4.
pub enum ReleaseClientError<R: Release> {
    /// A transport or (de)serialization error from `reqwest`.
    Http(reqwest::Error),
    /// The server returned an error status with an `OperationOutcome` body.
    Outcome {
        /// HTTP status code.
        status: u16,
        /// The parsed outcome.
        outcome: Box<R::OperationOutcome>,
    },
    /// The server returned an error status without a parseable `OperationOutcome`.
    Status {
        /// HTTP status code.
        status: u16,
        /// The raw response body.
        body: String,
    },
}

// Written by hand rather than derived: `#[derive(Debug)]` would demand
// `R: Debug` of the release marker, which says nothing about whether the error
// can be printed.
impl<R: Release> std::fmt::Debug for ReleaseClientError<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleaseClientError::Http(e) => f.debug_tuple("Http").field(e).finish(),
            ReleaseClientError::Outcome { status, outcome } => f
                .debug_struct("Outcome")
                .field("status", status)
                .field("outcome", outcome)
                .finish(),
            ReleaseClientError::Status { status, body } => f
                .debug_struct("Status")
                .field("status", status)
                .field("body", body)
                .finish(),
        }
    }
}

impl<R: Release> std::fmt::Display for ReleaseClientError<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleaseClientError::Http(e) => write!(f, "HTTP error: {e}"),
            ReleaseClientError::Outcome { status, .. } => {
                write!(f, "FHIR error status {status} (OperationOutcome)")
            }
            ReleaseClientError::Status { status, .. } => write!(f, "error status {status}"),
        }
    }
}

impl<R: Release> std::error::Error for ReleaseClientError<R> {}

impl<R: Release> From<reqwest::Error> for ReleaseClientError<R> {
    fn from(e: reqwest::Error) -> Self {
        ReleaseClientError::Http(e)
    }
}

/// An async FHIR REST client for a single service base URL, speaking release `R`.
///
/// Most code wants the release-specific alias — [`Client`] for R5, or
/// [`r4::client::Client`](crate::r4::client) for R4.
#[derive(Clone)]
pub struct ReleaseClient<R: Release> {
    base_url: String,
    http: reqwest::Client,
    release: std::marker::PhantomData<R>,
}

// As with the error type, deriving would impose a needless `R: Debug`.
impl<R: Release> std::fmt::Debug for ReleaseClient<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReleaseClient")
            .field("base_url", &self.base_url)
            .field("http", &self.http)
            .field("release", &R::LABEL)
            .finish()
    }
}

impl<R: Release> ReleaseClient<R> {
    /// A client for the given service base URL (e.g. `https://.../baseR5`).
    #[must_use]
    pub fn new(base_url: impl Into<String>) -> Self {
        Self::with_http(base_url, reqwest::Client::new())
    }

    /// A client using a caller-provided `reqwest::Client` (for custom TLS,
    /// timeouts, auth headers, …).
    #[must_use]
    pub fn with_http(base_url: impl Into<String>, http: reqwest::Client) -> Self {
        let base_url = base_url.into().trim_end_matches('/').to_string();
        Self { base_url, http, release: std::marker::PhantomData }
    }

    /// Send a request, returning the response on success or a
    /// [`ReleaseClientError`] (parsing an `OperationOutcome` from an error body
    /// when possible).
    async fn send(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, ReleaseClientError<R>> {
        let resp = req.header(reqwest::header::ACCEPT, FHIR_JSON).send().await?;
        if resp.status().is_success() {
            return Ok(resp);
        }
        let status = resp.status().as_u16();
        let body = resp.text().await?;
        match ::serde_json::from_str::<R::OperationOutcome>(&body) {
            Ok(outcome) => Err(ReleaseClientError::Outcome { status, outcome: Box::new(outcome) }),
            Err(_) => Err(ReleaseClientError::Status { status, body }),
        }
    }

    /// `GET [base]/[type]/[id]` — read the current version of a resource.
    pub async fn read(
        &self,
        resource_type: &str,
        id: &str,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = format!("{}/{}/{}", self.base_url, resource_type, id);
        Ok(self.send(self.http.get(url)).await?.json().await?)
    }

    /// `GET [base]/[type]/[id]/_history/[vid]` — read a specific version.
    pub async fn vread(
        &self,
        resource_type: &str,
        id: &str,
        version_id: &str,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = format!("{}/{}/{}/_history/{}", self.base_url, resource_type, id, version_id);
        Ok(self.send(self.http.get(url)).await?.json().await?)
    }

    /// `POST [base]/[type]` — create a resource; returns the server's copy.
    pub async fn create<T: Serialize>(
        &self,
        resource_type: &str,
        resource: &T,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = format!("{}/{}", self.base_url, resource_type);
        Ok(self.send(self.http.post(url).json(resource)).await?.json().await?)
    }

    /// `PUT [base]/[type]/[id]` — update (or create) a resource at a known id.
    pub async fn update<T: Serialize>(
        &self,
        resource_type: &str,
        id: &str,
        resource: &T,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = format!("{}/{}/{}", self.base_url, resource_type, id);
        Ok(self.send(self.http.put(url).json(resource)).await?.json().await?)
    }

    /// `DELETE [base]/[type]/[id]`.
    pub async fn delete(&self, resource_type: &str, id: &str) -> Result<(), ReleaseClientError<R>> {
        let url = format!("{}/{}/{}", self.base_url, resource_type, id);
        self.send(self.http.delete(url)).await?;
        Ok(())
    }

    /// `GET [base]/[type]?[params]` — search, returning a `Bundle`.
    pub async fn search(
        &self,
        resource_type: &str,
        params: &[(&str, &str)],
    ) -> Result<R::Bundle, ReleaseClientError<R>> {
        let url = format!("{}/{}", self.base_url, resource_type);
        Ok(self.send(self.http.get(url).query(params)).await?.json().await?)
    }

    /// `GET [base]/metadata` — the server's `CapabilityStatement`.
    pub async fn capabilities(&self) -> Result<R::CapabilityStatement, ReleaseClientError<R>> {
        let url = format!("{}/metadata", self.base_url);
        Ok(self.send(self.http.get(url)).await?.json().await?)
    }
}

/// An async FHIR R5 REST client.
#[cfg(feature = "r5")]
pub type Client = ReleaseClient<crate::r5::R5>;

/// An error from a FHIR R5 REST interaction.
#[cfg(feature = "r5")]
pub type ClientError = ReleaseClientError<crate::r5::R5>;

#[cfg(test)]
#[cfg(feature = "r5")]
mod tests {
    use super::*;
    use crate::r5::resources::Resource;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn read_returns_resource() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/Patient/pat-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(::serde_json::json!({
                "resourceType": "Patient", "id": "pat-1", "active": true
            })))
            .mount(&server)
            .await;

        let client = Client::new(server.uri());
        let resource = client.read("Patient", "pat-1").await.unwrap();
        match resource {
            Resource::Patient(p) => assert_eq!(p.id.unwrap().0, "pat-1"),
            other => panic!("expected Patient, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn search_returns_bundle() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/Patient"))
            .respond_with(ResponseTemplate::new(200).set_body_json(::serde_json::json!({
                "resourceType": "Bundle", "type": "searchset",
                "entry": [{ "resource": { "resourceType": "Patient", "id": "a" } }]
            })))
            .mount(&server)
            .await;

        let client = Client::new(server.uri());
        let bundle = client.search("Patient", &[("name", "chalmers")]).await.unwrap();
        assert_eq!(bundle.iter_resources().count(), 1);
    }

    #[tokio::test]
    async fn error_status_parses_operation_outcome() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/Patient/missing"))
            .respond_with(ResponseTemplate::new(404).set_body_json(::serde_json::json!({
                "resourceType": "OperationOutcome",
                "issue": [{ "severity": "error", "code": "not-found",
                            "diagnostics": "no such Patient" }]
            })))
            .mount(&server)
            .await;

        let client = Client::new(server.uri());
        let err = client.read("Patient", "missing").await.unwrap_err();
        match err {
            ClientError::Outcome { status, outcome } => {
                assert_eq!(status, 404);
                assert_eq!(outcome.issue.len(), 1);
            }
            other => panic!("expected Outcome, got {other:?}"),
        }
    }
}
