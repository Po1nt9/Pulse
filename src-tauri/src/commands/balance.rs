use tauri::State;
use serde::Serialize;

use crate::AppState;
use crate::providers::{create_balance_provider, BalanceInfo};
use crate::commands::provider_key;
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

    match provider_key::resolve_api_key(&provider_id).await? {
        Some(key) => {
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
        }
        None => Ok(ProviderBalance {
            provider_id: provider.id.clone(),
            provider_name: provider.name.clone(),
            balance: None,
            error: Some("API key not configured".to_string()),
            last_updated: None,
        }),
    }
}

#[tauri::command]
pub async fn refresh_all_balances(state: State<'_, AppState>) -> Result<Vec<ProviderBalance>> {
    // Read providers list, drop lock
    let providers = {
        let config = state.config.read().await;
        config.providers.clone()
    };

    // Filter enabled providers
    let enabled_providers: Vec<_> = providers.into_iter()
        .filter(|p| p.enabled)
        .collect();

    // Use JoinSet to run all balance requests concurrently with error isolation
    let mut join_set = tokio::task::JoinSet::new();

    for provider in enabled_providers {
        let http_client = state.http_client.clone();
        join_set.spawn(async move {
            match provider_key::resolve_api_key(&provider.id).await {
                Ok(Some(key)) => {
                    let adapter = create_balance_provider(&provider.provider_type, &provider.api_base_url);
                    match adapter.get_balance(&key, &http_client).await {
                        Ok(balance) => ProviderBalance {
                            provider_id: provider.id,
                            provider_name: provider.name,
                            balance: Some(balance),
                            error: None,
                            last_updated: Some(chrono::Local::now().to_rfc3339()),
                        },
                        Err(e) => ProviderBalance {
                            provider_id: provider.id,
                            provider_name: provider.name,
                            balance: None,
                            error: Some(e.to_string()),
                            last_updated: None,
                        },
                    }
                }
                Ok(None) => ProviderBalance {
                    provider_id: provider.id,
                    provider_name: provider.name,
                    balance: None,
                    error: Some("API key not configured".to_string()),
                    last_updated: None,
                },
                // Keychain failures for one provider must not abort the whole batch.
                Err(e) => ProviderBalance {
                    provider_id: provider.id,
                    provider_name: provider.name,
                    balance: None,
                    error: Some(e.to_string()),
                    last_updated: None,
                },
            }
        });
    }

    // Collect results; individual task panics are silently skipped.
    let mut results = Vec::new();
    while let Some(task_result) = join_set.join_next().await {
        if let Ok(provider_balance) = task_result {
            results.push(provider_balance);
        }
    }

    Ok(results)
}
