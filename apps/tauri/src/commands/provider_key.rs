use crate::commands::keychain;
use crate::error::Result;

/// Resolve the API key for a provider. Returns `Err(AppError::Keychain)` if the
/// keychain itself fails, and `Ok(None)` only if the key is genuinely missing.
/// Uses a single keychain lookup via `try_retrieve` to avoid a double query and
/// the TOCTOU race between `has` and `retrieve`.
pub async fn resolve_api_key(provider_id: &str) -> Result<Option<String>> {
    keychain::try_retrieve(provider_id).await
}
