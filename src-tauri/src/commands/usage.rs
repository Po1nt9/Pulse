use tauri::State;
use serde::Serialize;

use crate::AppState;
use crate::providers::{create_usage_provider, UsageData};
use crate::commands::keychain;
use crate::error::Result;

#[derive(Serialize)]
pub struct ProviderUsage {
    pub provider_id: String,
    pub provider_name: String,
    pub usage: Option<UsageData>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn get_usage(
    state: State<'_, AppState>,
    provider_id: String,
    period: String,
) -> Result<ProviderUsage> {
    let provider = {
        let config = state.config.read().await;
        config.providers.iter()
            .find(|p| p.id == provider_id)
            .cloned()
            .ok_or_else(|| crate::error::AppError::ProviderNotFound(provider_id.clone()))?
    };

    let api_key = keychain::retrieve(&provider_id).await.ok();

    if let Some(key) = api_key {
        let adapter = create_usage_provider(&provider.provider_type, &provider.api_base_url);
        match adapter.get_usage(&key, &period, &state.http_client).await {
            Ok(usage) => Ok(ProviderUsage {
                provider_id: provider.id.clone(),
                provider_name: provider.name.clone(),
                usage: Some(usage),
                error: None,
            }),
            Err(e) => Ok(ProviderUsage {
                provider_id: provider.id.clone(),
                provider_name: provider.name.clone(),
                usage: None,
                error: Some(e.to_string()),
            }),
        }
    } else {
        Ok(ProviderUsage {
            provider_id: provider.id.clone(),
            provider_name: provider.name.clone(),
            usage: None,
            error: Some("API key not configured".to_string()),
        })
    }
}
