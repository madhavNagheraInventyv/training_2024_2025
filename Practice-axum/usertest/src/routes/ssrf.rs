use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post};
use hyper::Client;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};
use url::Url;

#[derive(Deserialize)]
pub struct FetchRequest {
    url: String,
}

// Allowlist of safe domains
const ALLOWED_DOMAINS: &[&str] = &["https://example.com", "https://api.github.com"];

// Function to check if an IP is private
fn is_private_ip(host: &str) -> bool {
    match host.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => ip.is_private() || ip == Ipv4Addr::LOCALHOST || ip == Ipv4Addr::BROADCAST,
        Ok(IpAddr::V6(_)) => true, // Block all IPv6 addresses for simplicity
        Err(_) => false,
    }
}

pub async fn fetch_external(Json(payload): Json<FetchRequest>) -> impl IntoResponse {
    let client = Client::new();

    match Url::parse(&payload.url) {
        Ok(url) => {
            // Check if URL is in allowlist
            if !ALLOWED_DOMAINS.iter().any(|&allowed| url.as_str().starts_with(allowed)) {
                return (StatusCode::FORBIDDEN, "SSRF Blocked: URL not allowed").into_response();
            }

            // Prevent private IP access
            if let Some(host) = url.host_str() {
                if is_private_ip(host) {
                    return (StatusCode::FORBIDDEN, "SSRF Blocked: Private IP").into_response();
                }
            }

            // Make safe request
            match client.get(payload.url.parse().unwrap()).await {
                Ok(_) => (StatusCode::OK, "Request successful").into_response(),
                Err(_) => (StatusCode::BAD_REQUEST, "Invalid request").into_response(),
            }
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Invalid URL").into_response(),
    }
}

// Routes for SSRF protection
pub fn ssrf_routes() -> axum::Router {
    axum::Router::new().route("/fetch", post(fetch_external))
}
