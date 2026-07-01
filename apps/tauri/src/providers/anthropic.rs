use async_trait::async_trait;
use serde::Deserialize;

use crate::providers::{BalanceInfo, BalanceProvider, UsageData, UsageProvider};
use crate::http::{auth_headers, handle_response_status};

pub struct AnthropicProvider;

impl AnthropicProvider {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize)]
struct AnthropicBalanceResponse {
    #[serde(default)]
    credits_remaining: f64,
    #[serde(default)]
    credits_used: f64,
}

/// Derive `BalanceInfo` from Anthropic's credit fields.
///   total      = remaining + used
///   percentage = used / total * 100  (0 when total is 0 to avoid div-by-zero)
fn compute_balance(credits_remaining: f64, credits_used: f64) -> BalanceInfo {
    let remaining = credits_remaining;
    let used = credits_used;
    let total = remaining + used;
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
impl BalanceProvider for AnthropicProvider {
    async fn get_balance(
        &self,
        api_key: &str,
        client: &reqwest::Client,
    ) -> crate::error::Result<BalanceInfo> {
        let response = client
            .get("https://api.anthropic.com/v1/account")
            .headers(auth_headers(api_key)?)
            .send()
            .await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response.text().await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;
        handle_response_status(status, &body)?;

        let data: AnthropicBalanceResponse = serde_json::from_str(&body)
            .map_err(|e| crate::error::AppError::Unknown(format!("Parse error: {}", e)))?;

        Ok(compute_balance(data.credits_remaining, data.credits_used))
    }

    fn provider_name(&self) -> &str {
        "Anthropic"
    }
}

#[async_trait]
impl UsageProvider for AnthropicProvider {
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
        "Anthropic"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_response_parsing_valid() {
        let json = r#"{"credits_remaining": 50.0, "credits_used": 50.0}"#;
        let data: AnthropicBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.credits_remaining, 50.0);
        assert_eq!(data.credits_used, 50.0);
    }

    #[test]
    fn balance_response_parsing_defaults() {
        let json = r#"{}"#;
        let data: AnthropicBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.credits_remaining, 0.0);
        assert_eq!(data.credits_used, 0.0);
    }

    #[test]
    fn compute_balance_half_used() {
        let info = compute_balance(50.0, 50.0);
        assert_eq!(info.total, 100.0);
        assert_eq!(info.used, 50.0);
        assert_eq!(info.remaining, 50.0);
        assert_eq!(info.percentage_used, 50.0);
        assert_eq!(info.currency, "USD");
    }

    #[test]
    fn compute_balance_zero_total_avoids_div_by_zero() {
        // both credits 0 → total 0, must not divide by zero.
        let info = compute_balance(0.0, 0.0);
        assert_eq!(info.total, 0.0);
        assert_eq!(info.percentage_used, 0.0);
    }

    #[test]
    fn compute_balance_full_usage() {
        let info = compute_balance(0.0, 100.0);
        assert_eq!(info.total, 100.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 100.0);
    }

    #[test]
    fn compute_balance_no_usage() {
        let info = compute_balance(100.0, 0.0);
        assert_eq!(info.percentage_used, 0.0);
        assert_eq!(info.remaining, 100.0);
    }
}
