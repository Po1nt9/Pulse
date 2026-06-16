use tauri::State;
use serde::Serialize;

use crate::AppState;
use crate::providers::{create_balance_provider, BalanceInfo};
use crate::commands::keychain;
use crate::error::Result;

#[derive(Serialize)]
pub struct ProviderBalance {
    pub provider_id: String,
    pub provider_name: String,
    pub balance: Option<BalanceInfo>,
    pub error: Option<String>,
    pub last_updated: Option<String>,
}

#[tauri::command]
pub async fn get_balance(state: State<'_, AppState>, provider_id: String) -> Result<ProviderBalance> {
    // Read config and clone needed data, then drop lock
    let provider = {
        let config = state.config.read().await;
        config.providers.iter()
            .find(|p| p.id == provider_id)
            .cloned()
            .ok_or_else(|| crate::error::AppError::ProviderNotFound(provider_id.clone()))?
    };

    let api_key = keychain::retrieve(&provider_id).await.ok();

    if let Some(key) = api_key {
        let adapter = create_balance_provider(&provider.provider_type, &provider.api_base_url);
        match adapter.get_balance(&key, &state.http_client).await {
            Ok(balance) => Ok(ProviderBalance {
                provider_id: provider.id.clone(),
                provider_name: provider.name.clone(),
                balance: Some(balance),
                error: None,
                last_updated: Some(chrono::Local::now().to_rfc3339()),
            }),
            Err(e) => Ok(ProviderBalance {
                provider_id: provider.id.clone(),
                provider_name: provider.name.clone(),
                balance: None,
                error: Some(e.to_string()),
                last_updated: None,
            }),
        }
    } else {
        Ok(ProviderBalance {
            provider_id: provider.id.clone(),
            provider_name: provider.name.clone(),
            balance: None,
            error: Some("API key not configured".to_string()),
            last_updated: None,
        })
    }
}

#[tauri::command]
pub async fn refresh_all_balances(state: State<'_, AppState>) -> Result<Vec<ProviderBalance>> {
    // Read providers list, drop lock
    let providers = {
        let config = state.config.read().await;
        config.providers.clone()
    };

    let mut results = Vec::new();

    for provider in providers {
        if !provider.enabled {
            continue;
        }
        let api_key = keychain::retrieve(&provider.id).await.ok();

        let result = if let Some(key) = api_key {
            let adapter = create_balance_provider(&provider.provider_type, &provider.api_base_url);
            match adapter.get_balance(&key, &state.http_client).await {
                Ok(balance) => ProviderBalance {
                    provider_id: provider.id.clone(),
                    provider_name: provider.name.clone(),
                    balance: Some(balance),
                    error: None,
                    last_updated: Some(chrono::Local::now().to_rfc3339()),
                },
                Err(e) => ProviderBalance {
                    provider_id: provider.id.clone(),
                    provider_name: provider.name.clone(),
                    balance: None,
                    error: Some(e.to_string()),
                    last_updated: None,
                },
            }
        } else {
            ProviderBalance {
                provider_id: provider.id.clone(),
                provider_name: provider.name.clone(),
                balance: None,
                error: Some("API key not configured".to_string()),
                last_updated: None,
            }
        };
        results.push(result);
    }

    Ok(results)
}
