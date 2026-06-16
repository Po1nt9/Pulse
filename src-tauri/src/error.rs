use thiserror::Error;
use serde::{Serialize, Deserialize, Serializer, Deserializer, de};

#[derive(Error, Debug, Clone)]
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

// ── Manual serde impl (internally-tagged: { "type": "...", "message": "..." }) ──

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        match self {
            AppError::Network(msg) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Network")?;
                map.serialize_entry("message", msg)?;
                map.end()
            }
            AppError::Api { status, message } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("type", "Api")?;
                map.serialize_entry("status", status)?;
                map.serialize_entry("message", message)?;
                map.end()
            }
            AppError::Unauthorized => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "Unauthorized")?;
                map.end()
            }
            AppError::RateLimited => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "RateLimited")?;
                map.end()
            }
            AppError::ProviderNotFound(msg) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "ProviderNotFound")?;
                map.serialize_entry("message", msg)?;
                map.end()
            }
            AppError::Keychain(msg) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Keychain")?;
                map.serialize_entry("message", msg)?;
                map.end()
            }
            AppError::DuplicateProvider(msg) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "DuplicateProvider")?;
                map.serialize_entry("message", msg)?;
                map.end()
            }
            AppError::Config(msg) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Config")?;
                map.serialize_entry("message", msg)?;
                map.end()
            }
            AppError::Unknown(msg) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Unknown")?;
                map.serialize_entry("message", msg)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for AppError {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let err_type = value.get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| de::Error::missing_field("type"))?;
        let msg = value.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        match err_type {
            "Network" => Ok(AppError::Network(msg)),
            "Api" => {
                let status = value.get("status")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u16;
                Ok(AppError::Api { status, message: msg })
            }
            "Unauthorized" => Ok(AppError::Unauthorized),
            "RateLimited" => Ok(AppError::RateLimited),
            "ProviderNotFound" => Ok(AppError::ProviderNotFound(msg)),
            "DuplicateProvider" => Ok(AppError::DuplicateProvider(msg)),
            "Keychain" => Ok(AppError::Keychain(msg)),
            "Config" => Ok(AppError::Config(msg)),
            "Unknown" => Ok(AppError::Unknown(msg)),
            _ => Err(de::Error::unknown_variant(err_type, &[
                "Network", "Api", "Unauthorized", "RateLimited",
                "ProviderNotFound", "DuplicateProvider", "Keychain", "Config", "Unknown",
            ])),
        }
    }
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
