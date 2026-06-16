use crate::commands::keychain;
use crate::error::Result;

/// Resolve the API key for a provider. Returns `Err(AppError::Keychain)` if the
/// keychain itself fails, and `Ok(None)` only if the key is genuinely missing.
pub async fn resolve_api_key(provider_id: &str) -> Result<Option<String>> {
    match keychain::has(provider_id).await {
        Ok(true) => keychain::retrieve(provider_id).await.map(Some),
        Ok(false) => Ok(None),
        Err(e) => Err(e),
    }
}
