use tauri::State;

use crate::AppState;
use crate::config::{AppSettings, write_config};
use crate::error::Result;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings> {
    let config = state.config.read().await;
    Ok(config.settings.clone())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<AppSettings> {
    let mut config = state.config.write().await;
    config.settings = settings.clone();
    write_config(&config).await?;
    Ok(settings)
}
