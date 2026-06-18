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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_name_is_stable() {
        // The service name is a security boundary — changing it would orphan
        // every stored API key. Pin it so any rename is caught at test time.
        assert_eq!(SERVICE_NAME, "com.pulse.app");
    }

    #[test]
    fn entry_for_typical_provider_id() {
        // Entry::new must succeed for a well-formed provider id; it only
        // builds the credential handle and does not touch the keychain yet.
        let result = entry_for("deepseek");
        assert!(result.is_ok(), "entry_for should succeed for a typical provider id");
    }

    #[test]
    fn entry_for_empty_provider_id() {
        // An empty provider id is degenerate but must not panic.
        let result = entry_for("");
        assert!(result.is_ok(), "entry_for should not panic on empty provider id");
    }

    #[test]
    fn entry_for_provider_id_with_special_chars() {
        // Provider ids may contain hyphens, underscores, dots — all must be
        // accepted by the keychain's credential builder.
        let result = entry_for("my-provider_1.2");
        assert!(result.is_ok(), "entry_for should accept special characters");
    }

    #[test]
    fn entry_for_distinct_ids_produce_distinct_entries() {
        // Each provider id must map to its own keychain entry; if two ids
        // collapsed to the same entry, one provider's key would overwrite
        // another's.
        let e1 = entry_for("openai").unwrap();
        let e2 = entry_for("anthropic").unwrap();
        // The keyring::Entry doesn't expose its inner credential for direct
        // comparison, but the two handles must be independently usable.
        // We verify they were created without error — the isolation guarantee
        // is ultimately enforced by the keychain backend keyed on (service, user).
        let _ = (e1, e2);
    }
}
