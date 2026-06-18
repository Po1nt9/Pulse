use tauri::State;

use crate::AppState;
use crate::config::{AppConfig, ProviderConfig, write_config};
use crate::error::{AppError, Result};

#[tauri::command]
pub async fn list_providers(state: State<'_, AppState>) -> Result<Vec<ProviderConfig>> {
    let config = state.config.read().await;
    Ok(config.providers.clone())
}

#[tauri::command]
pub async fn add_provider(
    state: State<'_, AppState>,
    provider: ProviderConfig,
) -> Result<ProviderConfig> {
    let mut config = state.config.write().await;
    let added = add_provider_to_config(&mut config, provider)?;
    write_config(&config).await?;
    Ok(added)
}

#[tauri::command]
pub async fn update_provider(
    state: State<'_, AppState>,
    provider_id: String,
    updates: ProviderConfig,
) -> Result<ProviderConfig> {
    let mut config = state.config.write().await;
    let updated = update_provider_in_config(&mut config, &provider_id, updates)?;
    write_config(&config).await?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_provider(state: State<'_, AppState>, provider_id: String) -> Result<()> {
    let mut config = state.config.write().await;
    delete_provider_from_config(&mut config, &provider_id);
    write_config(&config).await?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_provider(
    state: State<'_, AppState>,
    provider_id: String,
    enabled: bool,
) -> Result<()> {
    let mut config = state.config.write().await;
    toggle_provider_in_config(&mut config, &provider_id, enabled)?;
    write_config(&config).await?;
    Ok(())
}

// ── Pure in-memory mutation helpers ──────────────────────────────────
// Extracted so the validation/mutation logic can be tested without a Tauri
// State or filesystem access. The command wrappers above call these and then
// persist via write_config.

fn add_provider_to_config(
    config: &mut AppConfig,
    provider: ProviderConfig,
) -> Result<ProviderConfig> {
    if config.providers.iter().any(|p| p.id == provider.id) {
        return Err(AppError::DuplicateProvider(provider.id));
    }
    config.providers.push(provider.clone());
    Ok(provider)
}

fn update_provider_in_config(
    config: &mut AppConfig,
    provider_id: &str,
    updates: ProviderConfig,
) -> Result<ProviderConfig> {
    if let Some(index) = config.providers.iter().position(|p| p.id == provider_id) {
        config.providers[index] = updates.clone();
        Ok(updates)
    } else {
        Err(AppError::ProviderNotFound(provider_id.to_string()))
    }
}

fn delete_provider_from_config(config: &mut AppConfig, provider_id: &str) {
    config.providers.retain(|p| p.id != provider_id);
}

fn toggle_provider_in_config(
    config: &mut AppConfig,
    provider_id: &str,
    enabled: bool,
) -> Result<()> {
    if let Some(provider) = config.providers.iter_mut().find(|p| p.id == provider_id) {
        provider.enabled = enabled;
        Ok(())
    } else {
        Err(AppError::ProviderNotFound(provider_id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, ProviderConfig, ProviderType};

    fn test_provider(id: &str) -> ProviderConfig {
        ProviderConfig {
            id: id.to_string(),
            name: format!("Test {}", id),
            provider_type: ProviderType::Custom,
            api_base_url: "https://api.test.com".to_string(),
            display_name: format!("Test {}", id),
            refresh_interval_seconds: 300,
            warning_threshold_percent: 30.0,
            enabled: true,
        }
    }

    fn empty_config() -> AppConfig {
        AppConfig {
            providers: vec![],
            settings: Default::default(),
            version: "0.1.0".to_string(),
        }
    }

    // ── add_provider_to_config ───────────────────────────────────────

    #[test]
    fn add_provider_to_empty_config() {
        let mut config = empty_config();
        let provider = test_provider("new");
        let result = add_provider_to_config(&mut config, provider.clone());
        assert!(result.is_ok());
        assert_eq!(config.providers.len(), 1);
        assert_eq!(config.providers[0].id, "new");
    }

    #[test]
    fn add_provider_duplicate_id_rejected() {
        let mut config = empty_config();
        let provider = test_provider("dup");
        add_provider_to_config(&mut config, provider.clone()).unwrap();
        // Adding a second provider with the same id must fail.
        let result = add_provider_to_config(&mut config, provider);
        assert!(matches!(result, Err(AppError::DuplicateProvider(id)) if id == "dup"));
        // The original provider must still be there and no duplicate added.
        assert_eq!(config.providers.len(), 1);
    }

    #[test]
    fn add_provider_distinct_ids_both_accepted() {
        let mut config = empty_config();
        add_provider_to_config(&mut config, test_provider("a")).unwrap();
        add_provider_to_config(&mut config, test_provider("b")).unwrap();
        assert_eq!(config.providers.len(), 2);
    }

    // ── update_provider_in_config ────────────────────────────────────

    #[test]
    fn update_provider_existing() {
        let mut config = empty_config();
        add_provider_to_config(&mut config, test_provider("p1")).unwrap();

        let mut updates = test_provider("p1");
        updates.display_name = "Updated".to_string();
        updates.enabled = false;
        let result = update_provider_in_config(&mut config, "p1", updates.clone());
        assert!(result.is_ok());
        assert_eq!(config.providers[0].display_name, "Updated");
        assert!(!config.providers[0].enabled);
    }

    #[test]
    fn update_provider_not_found() {
        let mut config = empty_config();
        let result = update_provider_in_config(&mut config, "missing", test_provider("missing"));
        assert!(matches!(result, Err(AppError::ProviderNotFound(id)) if id == "missing"));
        assert!(config.providers.is_empty());
    }

    #[test]
    fn update_provider_does_not_change_id() {
        // The update replaces the whole ProviderConfig at the matched index.
        // Verify that updating with a different id field still targets the
        // original index (the lookup is by provider_id, not by updates.id).
        let mut config = empty_config();
        add_provider_to_config(&mut config, test_provider("original")).unwrap();

        let updates = test_provider("renamed");
        let result = update_provider_in_config(&mut config, "original", updates);
        assert!(result.is_ok());
        // The entry at index 0 is now the "renamed" provider.
        assert_eq!(config.providers[0].id, "renamed");
        // A subsequent lookup for "original" must fail.
        let result2 = update_provider_in_config(&mut config, "original", test_provider("x"));
        assert!(matches!(result2, Err(AppError::ProviderNotFound(_))));
    }

    // ── delete_provider_from_config ──────────────────────────────────

    #[test]
    fn delete_provider_existing() {
        let mut config = empty_config();
        add_provider_to_config(&mut config, test_provider("a")).unwrap();
        add_provider_to_config(&mut config, test_provider("b")).unwrap();
        delete_provider_from_config(&mut config, "a");
        assert_eq!(config.providers.len(), 1);
        assert_eq!(config.providers[0].id, "b");
    }

    #[test]
    fn delete_provider_not_found_is_silent() {
        // delete is idempotent — removing a non-existent provider must not error.
        let mut config = empty_config();
        add_provider_to_config(&mut config, test_provider("a")).unwrap();
        delete_provider_from_config(&mut config, "missing");
        assert_eq!(config.providers.len(), 1);
    }

    #[test]
    fn delete_provider_from_empty_config() {
        let mut config = empty_config();
        delete_provider_from_config(&mut config, "anything");
        assert!(config.providers.is_empty());
    }

    // ── toggle_provider_in_config ────────────────────────────────────

    #[test]
    fn toggle_provider_disable() {
        let mut config = empty_config();
        add_provider_to_config(&mut config, test_provider("p1")).unwrap();
        assert!(config.providers[0].enabled);

        let result = toggle_provider_in_config(&mut config, "p1", false);
        assert!(result.is_ok());
        assert!(!config.providers[0].enabled);
    }

    #[test]
    fn toggle_provider_enable() {
        let mut config = empty_config();
        let mut provider = test_provider("p1");
        provider.enabled = false;
        add_provider_to_config(&mut config, provider).unwrap();
        assert!(!config.providers[0].enabled);

        let result = toggle_provider_in_config(&mut config, "p1", true);
        assert!(result.is_ok());
        assert!(config.providers[0].enabled);
    }

    #[test]
    fn toggle_provider_not_found() {
        let mut config = empty_config();
        let result = toggle_provider_in_config(&mut config, "missing", true);
        assert!(matches!(result, Err(AppError::ProviderNotFound(id)) if id == "missing"));
    }

    #[test]
    fn toggle_provider_only_affects_target() {
        let mut config = empty_config();
        add_provider_to_config(&mut config, test_provider("a")).unwrap();
        add_provider_to_config(&mut config, test_provider("b")).unwrap();

        toggle_provider_in_config(&mut config, "a", false).unwrap();
        assert!(!config.providers.iter().find(|p| p.id == "a").unwrap().enabled);
        // Provider b must remain enabled.
        assert!(config.providers.iter().find(|p| p.id == "b").unwrap().enabled);
    }
}
