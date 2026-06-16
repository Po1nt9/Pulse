use keyring::Entry;

use crate::error::{AppError, Result};

const SERVICE_NAME: &str = "com.pulse.app";

// ── Core functions (callable from other Rust modules) ────────────────

fn entry_for(provider_id: &str) -> Result<Entry> {
    Entry::new(SERVICE_NAME, provider_id)
        .map_err(|e| AppError::Keychain(e.to_string()))
}

pub async fn store(provider_id: &str, api_key: &str) -> Result<()> {
    let entry = entry_for(provider_id)?;
    entry.set_password(api_key)
        .map_err(|e| AppError::Keychain(e.to_string()))?;
    Ok(())
}

pub async fn retrieve(provider_id: &str) -> Result<String> {
    let entry = entry_for(provider_id)?;
    entry.get_password()
        .map_err(|e| AppError::Keychain(e.to_string()))
}

pub async fn delete_key(provider_id: &str) -> Result<()> {
    let entry = entry_for(provider_id)?;
    entry.delete_password()
        .map_err(|e| AppError::Keychain(e.to_string()))?;
    Ok(())
}

pub async fn has(provider_id: &str) -> Result<bool> {
    let entry = entry_for(provider_id)?;
    match entry.get_password() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(e) => Err(AppError::Keychain(e.to_string())),
    }
}

/// Retrieve the API key, returning `Ok(None)` when the key is genuinely missing
/// (NoEntry) and `Err` only when the keychain itself fails. Avoids the double
/// query + TOCTOU race of calling `has` then `retrieve`.
pub async fn try_retrieve(provider_id: &str) -> Result<Option<String>> {
    let entry = entry_for(provider_id)?;
    match entry.get_password() {
        Ok(p) => Ok(Some(p)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AppError::Keychain(e.to_string())),
    }
}

// ── Tauri command wrappers ───────────────────────────────────────────

#[tauri::command]
pub async fn store_api_key(provider_id: String, api_key: String) -> Result<()> {
    store(&provider_id, &api_key).await
}

#[tauri::command]
pub async fn retrieve_api_key(provider_id: String) -> Result<String> {
    retrieve(&provider_id).await
}

#[tauri::command]
pub async fn delete_api_key(provider_id: String) -> Result<()> {
    delete_key(&provider_id).await
}

#[tauri::command]
pub async fn has_api_key(provider_id: String) -> Result<bool> {
    has(&provider_id).await
}
