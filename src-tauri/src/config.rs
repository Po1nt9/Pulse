use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub api_base_url: String,
    pub display_name: String,
    pub refresh_interval_seconds: u64,
    pub warning_threshold_percent: f64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    // Aliases keep deserialization backward-compatible with the previous
    // `snake_case` serialization (e.g. "deep_seek", "open_ai", "open_router")
    // so existing config.json files still load after the rename to `lowercase`.
    #[serde(alias = "deep_seek")]
    DeepSeek,
    #[serde(alias = "open_ai")]
    OpenAi,
    Anthropic,
    #[serde(alias = "open_router")]
    OpenRouter,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub auto_start: bool,
    pub global_refresh_interval: u64,
    pub show_notifications: bool,
    pub window_position: Option<(i32, i32)>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            auto_start: false,
            global_refresh_interval: 300,
            show_notifications: true,
            window_position: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub providers: Vec<ProviderConfig>,
    pub settings: AppSettings,
    pub version: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            providers: vec![
                ProviderConfig {
                    id: "deepseek".to_string(),
                    name: "DeepSeek".to_string(),
                    provider_type: ProviderType::DeepSeek,
                    api_base_url: "https://api.deepseek.com".to_string(),
                    display_name: "DeepSeek".to_string(),
                    refresh_interval_seconds: 300,
                    warning_threshold_percent: 30.0,
                    enabled: true,
                },
                ProviderConfig {
                    id: "openai".to_string(),
                    name: "OpenAI".to_string(),
                    provider_type: ProviderType::OpenAi,
                    api_base_url: "https://api.openai.com".to_string(),
                    display_name: "OpenAI".to_string(),
                    refresh_interval_seconds: 300,
                    warning_threshold_percent: 30.0,
                    enabled: true,
                },
                ProviderConfig {
                    id: "anthropic".to_string(),
                    name: "Anthropic".to_string(),
                    provider_type: ProviderType::Anthropic,
                    api_base_url: "https://api.anthropic.com".to_string(),
                    display_name: "Anthropic".to_string(),
                    refresh_interval_seconds: 300,
                    warning_threshold_percent: 30.0,
                    enabled: true,
                },
                ProviderConfig {
                    id: "openrouter".to_string(),
                    name: "OpenRouter".to_string(),
                    provider_type: ProviderType::OpenRouter,
                    api_base_url: "https://openrouter.ai/api".to_string(),
                    display_name: "OpenRouter".to_string(),
                    refresh_interval_seconds: 300,
                    warning_threshold_percent: 30.0,
                    enabled: true,
                },
            ],
            settings: AppSettings::default(),
            version: "0.1.0".to_string(),
        }
    }
}

pub fn config_dir() -> PathBuf {
    ProjectDirs::from("com", "pulse", "Pulse")
        .expect("Failed to get project dirs")
        .config_dir()
        .to_path_buf()
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

/// Synchronous config read (used during startup before tokio is running).
pub fn read_config_sync() -> crate::error::Result<AppConfig> {
    let path = config_path();
    if !path.exists() {
        let default = AppConfig::default();
        write_config_sync(&default)?;
        return Ok(default);
    }
    let content = std::fs::read_to_string(&path)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    let config: AppConfig = serde_json::from_str(&content)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    Ok(config)
}

/// Synchronous config write (used during startup / spawn_blocking).
pub fn write_config_sync(config: &AppConfig) -> crate::error::Result<()> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    std::fs::write(config_path(), content)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    Ok(())
}

/// Async config write — offloads blocking I/O to the tokio thread pool.
pub async fn write_config(config: &AppConfig) -> crate::error::Result<()> {
    let config_json = serde_json::to_string_pretty(config)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    let dir = config_dir();
    tokio::task::spawn_blocking(move || {
        std::fs::create_dir_all(&dir)
            .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
        std::fs::write(config_path(), &config_json)
            .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
        Ok(())
    })
    .await
    .map_err(|e| crate::error::AppError::Config(e.to_string()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_settings_default() {
        let s = AppSettings::default();
        assert_eq!(s.theme, "dark");
        assert!(!s.auto_start);
        assert_eq!(s.global_refresh_interval, 300);
        assert!(s.show_notifications);
        assert!(s.window_position.is_none());
    }

    #[test]
    fn app_config_default_has_providers() {
        let c = AppConfig::default();
        assert_eq!(c.providers.len(), 4);
        assert_eq!(c.version, "0.1.0");
        assert_eq!(c.settings.theme, "dark");
    }

    #[test]
    fn app_config_default_provider_ids() {
        let c = AppConfig::default();
        let ids: Vec<_> = c.providers.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&"deepseek"));
        assert!(ids.contains(&"openai"));
        assert!(ids.contains(&"anthropic"));
        assert!(ids.contains(&"openrouter"));
    }

    #[test]
    fn config_serde_roundtrip() {
        let original = AppConfig::default();
        let json = serde_json::to_string_pretty(&original).unwrap();
        let restored: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.providers.len(), original.providers.len());
        assert_eq!(restored.version, original.version);
        assert_eq!(restored.settings.theme, original.settings.theme);
    }

    #[test]
    fn provider_type_serde_roundtrip() {
        let types = vec![
            ProviderType::DeepSeek,
            ProviderType::OpenAi,
            ProviderType::Anthropic,
            ProviderType::OpenRouter,
            ProviderType::Custom,
        ];
        for pt in types {
            let json = serde_json::to_string(&pt).unwrap();
            let de: ProviderType = serde_json::from_str(&json).unwrap();
            assert!(std::mem::discriminant(&pt) == std::mem::discriminant(&de));
        }
    }

    #[test]
    fn provider_type_lowercase_serialization() {
        let pt = ProviderType::OpenAi;
        let json = serde_json::to_string(&pt).unwrap();
        assert_eq!(json, "\"openai\"");
    }

    #[test]
    fn provider_type_legacy_snake_case_deserializes() {
        // Old config.json files used snake_case; they must still load.
        assert!(matches!(
            serde_json::from_str::<ProviderType>("\"deep_seek\"").unwrap(),
            ProviderType::DeepSeek
        ));
        assert!(matches!(
            serde_json::from_str::<ProviderType>("\"open_ai\"").unwrap(),
            ProviderType::OpenAi
        ));
        assert!(matches!(
            serde_json::from_str::<ProviderType>("\"open_router\"").unwrap(),
            ProviderType::OpenRouter
        ));
    }

    #[test]
    fn provider_config_serde_roundtrip() {
        let config = ProviderConfig {
            id: "test".to_string(),
            name: "Test Provider".to_string(),
            provider_type: ProviderType::Custom,
            api_base_url: "https://api.test.com".to_string(),
            display_name: "Test".to_string(),
            refresh_interval_seconds: 600,
            warning_threshold_percent: 50.0,
            enabled: false,
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: ProviderConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, config.id);
        assert_eq!(restored.name, config.name);
        assert_eq!(restored.refresh_interval_seconds, config.refresh_interval_seconds);
        assert_eq!(restored.warning_threshold_percent, config.warning_threshold_percent);
        assert_eq!(restored.enabled, config.enabled);
    }

    #[test]
    fn app_settings_window_position_some() {
        let mut settings = AppSettings::default();
        settings.window_position = Some((100, 200));
        let json = serde_json::to_string(&settings).unwrap();
        let restored: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.window_position, Some((100, 200)));
    }

    #[test]
    fn app_config_empty_providers() {
        let config = AppConfig {
            providers: vec![],
            settings: AppSettings::default(),
            version: "0.1.0".to_string(),
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.providers.len(), 0);
    }

    #[test]
    fn provider_type_case_insensitive_deserialization() {
        // lowercase should work
        assert!(matches!(
            serde_json::from_str::<ProviderType>("\"deepseek\"").unwrap(),
            ProviderType::DeepSeek
        ));
        assert!(matches!(
            serde_json::from_str::<ProviderType>("\"anthropic\"").unwrap(),
            ProviderType::Anthropic
        ));
        assert!(matches!(
            serde_json::from_str::<ProviderType>("\"custom\"").unwrap(),
            ProviderType::Custom
        ));
    }

    #[test]
    fn provider_type_invalid_variant_fails() {
        let result = serde_json::from_str::<ProviderType>("\"invalid_provider\"");
        assert!(result.is_err());
    }

    #[test]
    fn app_settings_boundary_values() {
        let settings = AppSettings {
            theme: "light".to_string(),
            auto_start: true,
            global_refresh_interval: 0,
            show_notifications: false,
            window_position: Some((i32::MIN, i32::MAX)),
        };
        let json = serde_json::to_string(&settings).unwrap();
        let restored: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.global_refresh_interval, 0);
        assert_eq!(restored.window_position, Some((i32::MIN, i32::MAX)));
    }

    #[test]
    fn provider_config_boundary_threshold() {
        let config = ProviderConfig {
            id: "test".to_string(),
            name: "Test".to_string(),
            provider_type: ProviderType::DeepSeek,
            api_base_url: "https://api.test.com".to_string(),
            display_name: "Test".to_string(),
            refresh_interval_seconds: u64::MAX,
            warning_threshold_percent: 100.0,
            enabled: true,
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: ProviderConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.refresh_interval_seconds, u64::MAX);
        assert_eq!(restored.warning_threshold_percent, 100.0);
    }
}
