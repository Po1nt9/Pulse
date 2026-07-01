use async_trait::async_trait;
use serde::Deserialize;

use crate::providers::{BalanceInfo, BalanceProvider, UsageData, UsageProvider};
use crate::http::{auth_headers, handle_response_status};

pub struct OpenRouterProvider;

impl OpenRouterProvider {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize)]
struct OpenRouterBalanceResponse {
    #[serde(default)]
    data: OpenRouterData,
}

#[derive(Deserialize, Default)]
struct OpenRouterData {
    #[serde(default)]
    credits_remaining: f64,
    #[serde(default)]
    total_credits: f64,
}

/// Derive `BalanceInfo` from OpenRouter's credit fields.
///   used       = total - remaining
///   percentage = used / total * 100  (0 when total is 0 to avoid div-by-zero)
/// No clamping: remaining > total surfaces as negative `used`/percentage.
fn compute_balance(total_credits: f64, credits_remaining: f64) -> BalanceInfo {
    let remaining = credits_remaining;
    let total = total_credits;
    let used = total - remaining;
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
impl BalanceProvider for OpenRouterProvider {
    async fn get_balance(
        &self,
        api_key: &str,
        client: &reqwest::Client,
    ) -> crate::error::Result<BalanceInfo> {
        let response = client
            .get("https://openrouter.ai/api/v1/auth/key")
            .headers(auth_headers(api_key)?)
            .send()
            .await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response.text().await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;
        handle_response_status(status, &body)?;

        let data: OpenRouterBalanceResponse = serde_json::from_str(&body)
            .map_err(|e| crate::error::AppError::Unknown(format!("Parse error: {}", e)))?;

        Ok(compute_balance(data.data.total_credits, data.data.credits_remaining))
    }

    fn provider_name(&self) -> &str {
        "OpenRouter"
    }
}

#[async_trait]
impl UsageProvider for OpenRouterProvider {
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
        "OpenRouter"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_response_parsing_valid() {
        let json = r#"{"data": {"credits_remaining": 25.0, "total_credits": 100.0}}"#;
        let data: OpenRouterBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.data.credits_remaining, 25.0);
        assert_eq!(data.data.total_credits, 100.0);
    }

    #[test]
    fn balance_response_parsing_defaults() {
        let json = r#"{}"#;
        let data: OpenRouterBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.data.credits_remaining, 0.0);
        assert_eq!(data.data.total_credits, 0.0);
    }

    #[test]
    fn compute_balance_partial_usage() {
        let info = compute_balance(100.0, 25.0);
        assert_eq!(info.total, 100.0);
        assert_eq!(info.used, 75.0);
        assert_eq!(info.remaining, 25.0);
        assert_eq!(info.percentage_used, 75.0);
        assert_eq!(info.currency, "USD");
    }

    #[test]
    fn compute_balance_zero_total_avoids_div_by_zero() {
        let info = compute_balance(0.0, 0.0);
        assert_eq!(info.total, 0.0);
        assert_eq!(info.used, 0.0);
        assert_eq!(info.percentage_used, 0.0);
    }

    #[test]
    fn compute_balance_full_usage() {
        let info = compute_balance(100.0, 0.0);
        assert_eq!(info.used, 100.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 100.0);
    }

    #[test]
    fn compute_balance_remaining_exceeds_total_is_not_clamped() {
        // remaining > total (e.g. credit grant increased) → negative used/percentage.
        let info = compute_balance(100.0, 150.0);
        assert_eq!(info.used, -50.0);
        assert_eq!(info.percentage_used, -50.0);
    }
}
