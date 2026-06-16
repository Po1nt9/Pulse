use tauri::State;

use crate::AppState;
use crate::config::{ProviderConfig, write_config};
use crate::error::Result;

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
    // Validate id uniqueness
    if config.providers.iter().any(|p| p.id == provider.id) {
        return Err(crate::error::AppError::DuplicateProvider(provider.id));
    }
    config.providers.push(provider.clone());
    write_config(&config).await?;
    Ok(provider)
}

#[tauri::command]
pub async fn update_provider(
    state: State<'_, AppState>,
    provider_id: String,
    updates: ProviderConfig,
) -> Result<ProviderConfig> {
    let mut config = state.config.write().await;
    if let Some(index) = config.providers.iter().position(|p| p.id == provider_id) {
        config.providers[index] = updates.clone();
        write_config(&config).await?;
        Ok(updates)
    } else {
        Err(crate::error::AppError::ProviderNotFound(provider_id))
    }
}

#[tauri::command]
pub async fn delete_provider(state: State<'_, AppState>, provider_id: String) -> Result<()> {
    let mut config = state.config.write().await;
    config.providers.retain(|p| p.id != provider_id);
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
    if let Some(provider) = config.providers.iter_mut().find(|p| p.id == provider_id) {
        provider.enabled = enabled;
        write_config(&config).await?;
        Ok(())
    } else {
        Err(crate::error::AppError::ProviderNotFound(provider_id))
    }
}
