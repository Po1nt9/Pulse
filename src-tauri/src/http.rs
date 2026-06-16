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
            message: format!("Client error: {}", body.chars().take(200).collect::<String>()),
        }),
        500..=599 => Err(crate::error::AppError::Api {
            status,
            message: format!("Server error: {}", body.chars().take(200).collect::<String>()),
        }),
        _ => Err(crate::error::AppError::Unknown(format!("Unexpected status: {}", status))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;

    #[test]
    fn handle_response_status_success_200() {
        let result = handle_response_status(200, "ok");
        assert!(result.is_ok());
    }

    #[test]
    fn handle_response_status_success_201() {
        let result = handle_response_status(201, "created");
        assert!(result.is_ok());
    }

    #[test]
    fn handle_response_status_success_204() {
        let result = handle_response_status(204, "");
        assert!(result.is_ok());
    }

    #[test]
    fn handle_response_status_unauthorized_401() {
        let result = handle_response_status(401, "unauthorized");
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[test]
    fn handle_response_status_rate_limited_429() {
        let result = handle_response_status(429, "too many requests");
        assert!(matches!(result, Err(AppError::RateLimited)));
    }

    #[test]
    fn handle_response_status_client_error_400() {
        let result = handle_response_status(400, "bad request");
        assert!(matches!(result, Err(AppError::Api { status: 400, .. })));
        if let Err(AppError::Api { status, message }) = result {
            assert_eq!(status, 400);
            assert_eq!(message, "Client error: bad request");
        }
    }

    #[test]
    fn handle_response_status_client_error_404() {
        let result = handle_response_status(404, "not found");
        assert!(matches!(result, Err(AppError::Api { status: 404, .. })));
        if let Err(AppError::Api { status, message }) = result {
            assert_eq!(status, 404);
            assert_eq!(message, "Client error: not found");
        }
    }

    #[test]
    fn handle_response_status_server_error_500() {
        let result = handle_response_status(500, "internal server error");
        assert!(matches!(result, Err(AppError::Api { status: 500, .. })));
        if let Err(AppError::Api { status, message }) = result {
            assert_eq!(status, 500);
            assert_eq!(message, "Server error: internal server error");
        }
    }

    #[test]
    fn handle_response_status_server_error_503() {
        let result = handle_response_status(503, "service unavailable");
        assert!(matches!(result, Err(AppError::Api { status: 503, .. })));
        if let Err(AppError::Api { status, message }) = result {
            assert_eq!(status, 503);
            assert_eq!(message, "Server error: service unavailable");
        }
    }

    #[test]
    fn handle_response_status_unexpected_100() {
        let result = handle_response_status(100, "continue");
        assert!(matches!(result, Err(AppError::Unknown(_))));
        if let Err(AppError::Unknown(message)) = result {
            assert_eq!(message, "Unexpected status: 100");
        }
    }

    #[test]
    fn handle_response_status_unexpected_300() {
        let result = handle_response_status(300, "redirect");
        assert!(matches!(result, Err(AppError::Unknown(_))));
        if let Err(AppError::Unknown(message)) = result {
            assert_eq!(message, "Unexpected status: 300");
        }
    }

    #[test]
    fn auth_headers_valid_token() {
        let result = auth_headers("test_token_123");
        assert!(result.is_ok());
        let headers = result.unwrap();
        assert!(headers.contains_key(reqwest::header::AUTHORIZATION));
        assert!(headers.contains_key(reqwest::header::CONTENT_TYPE));
        let auth_value = headers.get(reqwest::header::AUTHORIZATION).unwrap();
        assert_eq!(auth_value.to_str().unwrap(), "Bearer test_token_123");
    }

    #[test]
    fn auth_headers_empty_token() {
        let result = auth_headers("");
        assert!(result.is_ok());
        let headers = result.unwrap();
        let auth_value = headers.get(reqwest::header::AUTHORIZATION).unwrap();
        assert_eq!(auth_value.to_str().unwrap(), "Bearer ");
    }

    #[test]
    fn auth_headers_special_characters() {
        let result = auth_headers("token-with-special!@#$%");
        assert!(result.is_ok());
    }

    #[test]
    fn create_client_returns_client() {
        let client = create_client();
        // Just verify it doesn't panic and returns a client
        let _ = client;
    }
}
