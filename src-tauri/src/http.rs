use reqwest::{Client, header};
use std::time::Duration;
use once_cell::sync::Lazy;

pub static SHARED_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .pool_idle_timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client")
});

pub fn create_client() -> Client {
    SHARED_CLIENT.clone()
}

pub fn auth_headers(token: &str) -> crate::error::Result<header::HeaderMap> {
    let mut headers = header::HeaderMap::new();
    let auth_value = header::HeaderValue::from_str(&format!("Bearer {}", token))
        .map_err(|e| crate::error::AppError::Unknown(format!("Invalid token format: {}", e)))?;
    headers.insert(header::AUTHORIZATION, auth_value);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    Ok(headers)
}

pub fn handle_response_status(status: u16, body: &str) -> crate::error::Result<()> {
    match status {
        200..=299 => Ok(()),
        401 => Err(crate::error::AppError::Unauthorized),
        429 => Err(crate::error::AppError::RateLimited),
        400..=499 => Err(crate::error::AppError::Api {
            status,
            message: format!("Client error: {}", body),
        }),
        500..=599 => Err(crate::error::AppError::Api {
            status,
            message: format!("Server error: {}", body),
        }),
        _ => Err(crate::error::AppError::Unknown(format!("Unexpected status: {}", status))),
    }
}
