//! Pterodactyl Application API implementation, for all endpoints under `api/application`

use crate::http::EmptyBody;
use reqwest::Method;
use std::sync::RwLock;

/// Server-related endpoints for the application API
pub mod servers;
/// Data structures for the application API
pub mod structs;

/// The rate limits of the API key
#[derive(Debug, Copy, Clone)]
pub struct RateLimits {
    /// The request limit per minute
    pub limit: u32,
    /// The number of requests remaining in this minute
    pub limit_remaining: u32,
}

/// A Pterodactyl application client, to make requests to the Pterodactyl application API
#[derive(Debug)]
pub struct Client {
    pub(crate) url: String,
    pub(crate) client: reqwest::Client,
    pub(crate) api_key: String,
    pub(crate) rate_limits: RwLock<Option<RateLimits>>,
}

impl Client {
    /// Gets the rate limit information after the previous request
    pub fn get_rate_limits(&self) -> Option<RateLimits> {
        *self.rate_limits.read().unwrap()
    }

    /// Makes a request to the Pterodactyl application API
    pub(crate) async fn request<Response: crate::http::ResponseBody>(
        &self,
        method: Method,
        endpoint: &str,
    ) -> crate::Result<Response> {
        self.request_with_body::<Response, _>(method, endpoint, EmptyBody)
            .await
    }

    /// Makes a request with a body to the Pterodactyl application API
    pub(crate) async fn request_with_body<Response: crate::http::ResponseBody, Body: crate::http::RequestBody>(
        &self,
        method: Method,
        endpoint: &str,
        body: Body,
    ) -> crate::Result<Response> {
        Response::decode(
            self.get_response::<_, crate::http::NullErrorHandler>(method, endpoint, body)
                .await?,
        )
        .await
    }

    /// Gets a response from the Pterodactyl application API
    pub(crate) async fn get_response<Body: crate::http::RequestBody, EHandler: crate::http::ErrorHandler>(
        &self,
        method: Method,
        endpoint: &str,
        body: Body,
    ) -> crate::Result<reqwest::Response> {
        let request = self
            .client
            .request(method, format!("{}{}", self.url, endpoint))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key));
        let request = body.encode(request)?;
        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            if let Some(err) = EHandler::get_error(response).await {
                return Err(err);
            }
            return Err(Self::translate_error(status));
        }

        if let Some(limit) = response
            .headers()
            .get("x-ratelimit-limit")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.parse().ok())
        {
            if let Some(limit_remaining) = response
                .headers()
                .get("x-ratelimit-remaining")
                .and_then(|header| header.to_str().ok())
                .and_then(|header| header.parse().ok())
            {
                *self.rate_limits.write().unwrap() = Some(RateLimits {
                    limit,
                    limit_remaining,
                });
            }
        }

        Ok(response)
    }

    /// Translates an HTTP status code to an error
    fn translate_error(status: reqwest::StatusCode) -> crate::Error {
        match status {
            reqwest::StatusCode::FORBIDDEN => crate::Error::PermissionError,
            reqwest::StatusCode::NOT_FOUND => crate::Error::ResourceNotFound,
            reqwest::StatusCode::TOO_MANY_REQUESTS => crate::Error::RateLimit,
            status => crate::Error::Http(status),
        }
    }
}

/// A builder for an application client
#[derive(Debug)]
pub struct ClientBuilder {
    url: String,
    client: Option<reqwest::Client>,
    api_key: String,
}

impl ClientBuilder {
    /// Creates a new application client builder, connecting to the given URL where a Pterodactyl server is
    /// hosted, using the given API key for authentication
    pub fn new(url: impl Into<String>, api_key: impl Into<String>) -> Self {
        let mut url = url.into();
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str("api/application/");
        Self {
            url,
            client: None,
            api_key: api_key.into(),
        }
    }

    /// Uses the specified [`reqwest::Client`] for requests instead of making a default one
    pub fn with_client(self, client: reqwest::Client) -> Self {
        Self {
            client: Some(client),
            ..self
        }
    }

    /// Builds a client
    pub fn build(self) -> Client {
        Client {
            url: self.url,
            client: self.client.unwrap_or_default(),
            api_key: self.api_key,
            rate_limits: RwLock::new(None),
        }
    }
} 