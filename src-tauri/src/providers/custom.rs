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

        Ok(balance_from_response(&data))
    }

    fn provider_name(&self) -> &str {
        "Custom"
    }
}

/// Pure transformation from a parsed custom-provider balance response into
/// `BalanceInfo`. Extracted from `get_balance` so the fallback/percentage
/// branches are testable without the network.
fn balance_from_response(data: &CustomBalanceResponse) -> BalanceInfo {
    let total = if data.total_balance > 0.0 { data.total_balance } else { data.balance };
    let used = data.used;
    let remaining = if data.remaining > 0.0 { data.remaining } else { total - used };
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
    fn balance_from_response_uses_total_balance_when_positive() {
        let data = CustomBalanceResponse {
            balance: 0.0,
            total_balance: 100.0,
            used: 30.0,
            remaining: 70.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.total, 100.0);
        assert_eq!(info.used, 30.0);
        assert_eq!(info.remaining, 70.0);
        assert_eq!(info.percentage_used, 30.0);
        assert_eq!(info.currency, "USD");
    }

    #[test]
    fn balance_from_response_falls_back_to_balance_field() {
        // total_balance <= 0 → total must fall back to the `balance` field.
        let data = CustomBalanceResponse {
            balance: 50.0,
            total_balance: 0.0,
            used: 20.0,
            remaining: 0.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.total, 50.0);
        assert_eq!(info.used, 20.0);
        // remaining <= 0 → falls back to total - used.
        assert_eq!(info.remaining, 30.0);
        assert_eq!(info.percentage_used, 40.0);
    }

    #[test]
    fn balance_from_response_falls_back_when_total_balance_negative() {
        // Negative total_balance must also trigger the fallback (guard is `> 0.0`).
        let data = CustomBalanceResponse {
            balance: 80.0,
            total_balance: -5.0,
            used: 10.0,
            remaining: 70.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.total, 80.0);
    }

    #[test]
    fn balance_from_response_falls_back_when_remaining_negative() {
        // remaining <= 0 → remaining falls back to total - used.
        let data = CustomBalanceResponse {
            balance: 0.0,
            total_balance: 100.0,
            used: 40.0,
            remaining: -1.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.total, 100.0);
        assert_eq!(info.remaining, 60.0);
    }

    #[test]
    fn balance_from_response_zero_total_avoids_division_by_zero() {
        // total == 0 → percentage guard must yield 0.0, not NaN/inf.
        let data = CustomBalanceResponse {
            balance: 0.0,
            total_balance: 0.0,
            used: 0.0,
            remaining: 0.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.total, 0.0);
        assert_eq!(info.percentage_used, 0.0);
        assert!(!info.percentage_used.is_nan());
    }

    #[test]
    fn balance_from_response_all_fields_default() {
        // Empty `{}` body deserializes to all-zero; computation must not panic.
        let data: CustomBalanceResponse = serde_json::from_str("{}").unwrap();
        let info = balance_from_response(&data);
        assert_eq!(info.total, 0.0);
        assert_eq!(info.used, 0.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 0.0);
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
