use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Authentication failed — check your API key")]
    Unauthorized,

    #[error("Rate limited — please wait before retrying")]
    RateLimited,

    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("Duplicate provider: {0}")]
    DuplicateProvider(String),

    #[error("Keychain error: {0}")]
    Keychain(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// ── Tauri command integration ─────────────────────────────────────────
// AppError implements Serialize above, so it can be returned directly
// from #[tauri::command] functions as a Result<T, AppError>.

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_serde_roundtrip_network() {
        let err = AppError::Network("timeout".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::Network(s) if s == "timeout"));
    }

    #[test]
    fn error_serde_roundtrip_api() {
        let err = AppError::Api { status: 429, message: "rate limited".to_string() };
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::Api { status: 429, message: ref m } if m == "rate limited"));
    }

    #[test]
    fn error_serde_roundtrip_unauthorized() {
        let err = AppError::Unauthorized;
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::Unauthorized));
    }

    #[test]
    fn error_serde_roundtrip_rate_limited() {
        let err = AppError::RateLimited;
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::RateLimited));
    }

    #[test]
    fn error_serde_roundtrip_provider_not_found() {
        let err = AppError::ProviderNotFound("deepseek".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::ProviderNotFound(s) if s == "deepseek"));
    }

    #[test]
    fn error_serde_roundtrip_duplicate_provider() {
        let err = AppError::DuplicateProvider("openai".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::DuplicateProvider(s) if s == "openai"));
    }

    #[test]
    fn error_serde_roundtrip_keychain() {
        let err = AppError::Keychain("not found".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::Keychain(s) if s == "not found"));
    }

    #[test]
    fn error_serde_roundtrip_config() {
        let err = AppError::Config("invalid json".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::Config(s) if s == "invalid json"));
    }

    #[test]
    fn error_serde_roundtrip_unknown() {
        let err = AppError::Unknown("panic".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let de: AppError = serde_json::from_str(&json).unwrap();
        assert!(matches!(de, AppError::Unknown(s) if s == "panic"));
    }

    #[test]
    fn error_display_network() {
        let err = AppError::Network("timeout".to_string());
        assert_eq!(err.to_string(), "Network error: timeout");
    }

    #[test]
    fn error_display_api() {
        let err = AppError::Api { status: 500, message: "server error".to_string() };
        assert_eq!(err.to_string(), "API error: 500 - server error");
    }

    #[test]
    fn error_deserialize_unknown_variant_fails() {
        let json = r#"{"type":"NonExistent"}"#;
        let result: std::result::Result<AppError, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
