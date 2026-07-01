use async_trait::async_trait;
use serde::Deserialize;

use crate::providers::{BalanceInfo, BalanceProvider, UsageData, UsageProvider};
use crate::http::{auth_headers, handle_response_status};

/// 去掉 base URL 末尾的所有连续 `/`。
pub fn normalize_base_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}

pub struct CustomProvider {
    api_base_url: String,
}

impl CustomProvider {
    pub fn new() -> Self {
        Self {
            api_base_url: "https://api.custom.com".to_string(),
        }
    }
    
    pub fn with_url(api_base_url: String) -> Self {
        Self { api_base_url }
    }
}

#[derive(Deserialize)]
struct CustomBalanceResponse {
    #[serde(default)]
    balance: f64,
    #[serde(default)]
    total_balance: f64,
    #[serde(default)]
    used: f64,
    #[serde(default)]
    remaining: f64,
}

/// Derive `BalanceInfo` from a custom endpoint's fields, tolerating partial data:
///   total      = total_balance when > 0, otherwise fall back to `balance`
///   remaining  = remaining when > 0, otherwise fall back to `total - used`
///   percentage = used / total * 100  (0 when total is 0 to avoid div-by-zero)
/// No clamping of over-usage.
fn compute_balance(balance: f64, total_balance: f64, used: f64, remaining: f64) -> BalanceInfo {
    let total = if total_balance > 0.0 { total_balance } else { balance };
    let used = used;
    let remaining = if remaining > 0.0 { remaining } else { total - used };
    let percentage = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
    BalanceInfo {
        total,
        used,
        remaining,
        currency: "USD".to_string(),
        percentage_used: percentage,
    }
}

#[async_trait]
impl BalanceProvider for CustomProvider {
    async fn get_balance(
        &self,
        api_key: &str,
        client: &reqwest::Client,
    ) -> crate::error::Result<BalanceInfo> {
        let url = format!("{}/user/balance", normalize_base_url(&self.api_base_url));
        let response = client
            .get(&url)
            .headers(auth_headers(api_key)?)
            .send()
            .await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response.text().await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;
        handle_response_status(status, &body)?;

        let data: CustomBalanceResponse = serde_json::from_str(&body)
            .map_err(|e| crate::error::AppError::Unknown(format!("Parse error: {}", e)))?;

        Ok(compute_balance(
            data.balance,
            data.total_balance,
            data.used,
            data.remaining,
        ))
    }

    fn provider_name(&self) -> &str {
        "Custom"
    }
}

#[async_trait]
impl UsageProvider for CustomProvider {
    async fn get_usage(
        &self,
        _api_key: &str,
        period: &str,
        _client: &reqwest::Client,
    ) -> crate::error::Result<UsageData> {
        Ok(UsageData {
            points: vec![],
            total_cost: 0.0,
            total_tokens_input: 0,
            total_tokens_output: 0,
            total_requests: 0,
            period: period.to_string(),
        })
    }

    fn provider_name(&self) -> &str {
        "Custom"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_response_parsing_with_total_balance() {
        // total_balance > 0 → use it; remaining > 0 → use it.
        let info = compute_balance(0.0, 100.0, 30.0, 70.0);
        assert_eq!(info.total, 100.0);
        assert_eq!(info.used, 30.0);
        assert_eq!(info.remaining, 70.0);
        assert_eq!(info.percentage_used, 30.0);
        assert_eq!(info.currency, "USD");
    }

    #[test]
    fn balance_response_parsing_fallback_to_balance() {
        // total_balance == 0 → fall back to `balance`; remaining == 0 → fall back to total-used.
        let info = compute_balance(50.0, 0.0, 20.0, 0.0);
        assert_eq!(info.total, 50.0);
        assert_eq!(info.remaining, 30.0); // 50 - 20
        assert_eq!(info.percentage_used, 40.0); // 20 / 50 * 100
    }

    #[test]
    fn compute_balance_remaining_preferred_over_derived() {
        // Both remaining and total_balance present → use the explicit values.
        let info = compute_balance(999.0, 100.0, 10.0, 90.0);
        assert_eq!(info.remaining, 90.0);
        assert_eq!(info.percentage_used, 10.0);
    }

    #[test]
    fn compute_balance_all_zero_avoids_div_by_zero() {
        // No fields at all (empty endpoint body) → zeros, no panic.
        let info = compute_balance(0.0, 0.0, 0.0, 0.0);
        assert_eq!(info.total, 0.0);
        assert_eq!(info.used, 0.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 0.0);
    }

    #[test]
    fn compute_balance_over_usage_not_clamped() {
        // used > total → percentage > 100, no clamping.
        let info = compute_balance(0.0, 100.0, 150.0, 0.0);
        assert_eq!(info.used, 150.0);
        assert_eq!(info.remaining, -50.0); // total - used
        assert_eq!(info.percentage_used, 150.0);
    }

    #[test]
    fn balance_response_parsing_defaults() {
        let json = r#"{}"#;
        let data: CustomBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.balance, 0.0);
        assert_eq!(data.total_balance, 0.0);
        assert_eq!(data.used, 0.0);
        assert_eq!(data.remaining, 0.0);
    }

    #[test]
    fn provider_with_url() {
        let provider = CustomProvider::with_url("https://custom.example.com".to_string());
        assert_eq!(BalanceProvider::provider_name(&provider), "Custom");
    }

    #[test]
    fn provider_default_url() {
        let provider = CustomProvider::new();
        assert_eq!(BalanceProvider::provider_name(&provider), "Custom");
    }

    #[test]
    fn normalize_no_trailing_slash() {
        assert_eq!(normalize_base_url("https://x.com"), "https://x.com");
    }

    #[test]
    fn normalize_single_trailing_slash() {
        assert_eq!(normalize_base_url("https://x.com/"), "https://x.com");
    }

    #[test]
    fn normalize_multiple_trailing_slashes() {
        assert_eq!(normalize_base_url("https://x.com///"), "https://x.com");
    }

    #[test]
    fn normalize_empty() {
        assert_eq!(normalize_base_url(""), "");
    }

    #[test]
    fn normalize_internal_slash_preserved() {
        assert_eq!(
            normalize_base_url("https://api.example.com/v1/"),
            "https://api.example.com/v1"
        );
    }
}
