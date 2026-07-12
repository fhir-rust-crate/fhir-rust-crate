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

use ::serde::Serialize;

use crate::r5::resources::{Bundle, CapabilityStatement, OperationOutcome, Resource};

/// FHIR JSON media type.
const FHIR_JSON: &str = "application/fhir+json";

/// An error from a FHIR REST interaction.
#[derive(Debug)]
pub enum ClientError {
    /// A transport or (de)serialization error from `reqwest`.
    Http(reqwest::Error),
    /// The server returned an error status with an `OperationOutcome` body.
    Outcome {
        /// HTTP status code.
        status: u16,
        /// The parsed outcome.
        outcome: Box<OperationOutcome>,
    },
    /// The server returned an error status without a parseable `OperationOutcome`.
    Status {
        /// HTTP status code.
        status: u16,
        /// The raw response body.
        body: String,
    },
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Http(e) => write!(f, "HTTP error: {e}"),
            ClientError::Outcome { status, .. } => {
                write!(f, "FHIR error status {status} (OperationOutcome)")
            }
            ClientError::Status { status, .. } => write!(f, "error status {status}"),
        }
    }
}

impl std::error::Error for ClientError {}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        ClientError::Http(e)
    }
}

/// An async FHIR REST client for a single service base URL.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
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
        Self { base_url, http }
    }

    /// Send a request, returning the response on success or a [`ClientError`]
    /// (parsing an `OperationOutcome` from an error body when possible).
    async fn send(&self, req: reqwest::RequestBuilder) -> Result<reqwest::Response, ClientError> {
        let resp = req.header(reqwest::header::ACCEPT, FHIR_JSON).send().await?;
        if resp.status().is_success() {
            return Ok(resp);
        }
        let status = resp.status().as_u16();
        let body = resp.text().await?;
        match ::serde_json::from_str::<OperationOutcome>(&body) {
            Ok(outcome) => Err(ClientError::Outcome { status, outcome: Box::new(outcome) }),
            Err(_) => Err(ClientError::Status { status, body }),
        }
    }

    /// `GET [base]/[type]/[id]` — read the current version of a resource.
    pub async fn read(&self, resource_type: &str, id: &str) -> Result<Resource, ClientError> {
        let url = format!("{}/{}/{}", self.base_url, resource_type, id);
        Ok(self.send(self.http.get(url)).await?.json().await?)
    }

    /// `GET [base]/[type]/[id]/_history/[vid]` — read a specific version.
    pub async fn vread(
        &self,
        resource_type: &str,
        id: &str,
        version_id: &str,
    ) -> Result<Resource, ClientError> {
        let url = format!("{}/{}/{}/_history/{}", self.base_url, resource_type, id, version_id);
        Ok(self.send(self.http.get(url)).await?.json().await?)
    }

    /// `POST [base]/[type]` — create a resource; returns the server's copy.
    pub async fn create<T: Serialize>(
        &self,
        resource_type: &str,
        resource: &T,
    ) -> Result<Resource, ClientError> {
        let url = format!("{}/{}", self.base_url, resource_type);
        Ok(self.send(self.http.post(url).json(resource)).await?.json().await?)
    }

    /// `PUT [base]/[type]/[id]` — update (or create) a resource at a known id.
    pub async fn update<T: Serialize>(
        &self,
        resource_type: &str,
        id: &str,
        resource: &T,
    ) -> Result<Resource, ClientError> {
        let url = format!("{}/{}/{}", self.base_url, resource_type, id);
        Ok(self.send(self.http.put(url).json(resource)).await?.json().await?)
    }

    /// `DELETE [base]/[type]/[id]`.
    pub async fn delete(&self, resource_type: &str, id: &str) -> Result<(), ClientError> {
        let url = format!("{}/{}/{}", self.base_url, resource_type, id);
        self.send(self.http.delete(url)).await?;
        Ok(())
    }

    /// `GET [base]/[type]?[params]` — search, returning a `Bundle`.
    pub async fn search(
        &self,
        resource_type: &str,
        params: &[(&str, &str)],
    ) -> Result<Bundle, ClientError> {
        let url = format!("{}/{}", self.base_url, resource_type);
        Ok(self.send(self.http.get(url).query(params)).await?.json().await?)
    }

    /// `GET [base]/metadata` — the server's `CapabilityStatement`.
    pub async fn capabilities(&self) -> Result<CapabilityStatement, ClientError> {
        let url = format!("{}/metadata", self.base_url);
        Ok(self.send(self.http.get(url)).await?.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
