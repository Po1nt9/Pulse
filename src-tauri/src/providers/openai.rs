use async_trait::async_trait;
use serde::Deserialize;

use crate::providers::{BalanceInfo, BalanceProvider, UsageData, UsageProvider};
use crate::http::{auth_headers, handle_response_status};

pub struct OpenAiProvider;

impl OpenAiProvider {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize)]
struct OpenAiBillingResponse {
    #[serde(default)]
    total_usage: f64,
    #[serde(default)]
    total_granted: f64,
}

#[async_trait]
impl BalanceProvider for OpenAiProvider {
    async fn get_balance(
        &self,
        api_key: &str,
        client: &reqwest::Client,
    ) -> crate::error::Result<BalanceInfo> {
        let response = client
            .get("https://api.openai.com/v1/dashboard/billing/credit_grants")
            .headers(auth_headers(api_key)?)
            .send()
            .await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response.text().await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;
        handle_response_status(status, &body)?;

        let data: OpenAiBillingResponse = serde_json::from_str(&body)
            .map_err(|e| crate::error::AppError::Unknown(format!("Parse error: {}", e)))?;

        Ok(balance_from_response(&data))
    }

    fn provider_name(&self) -> &str {
        "OpenAI"
    }
}

/// Pure transformation from a parsed OpenAI billing response into `BalanceInfo`.
/// Extracted from `get_balance` so the percentage guard and remaining computation
/// are testable without the network.
fn balance_from_response(data: &OpenAiBillingResponse) -> BalanceInfo {
    let total = data.total_granted;
    let used = data.total_usage;
    let remaining = total - used;
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
impl UsageProvider for OpenAiProvider {
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
        "OpenAI"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn billing_response_parsing_valid() {
        let json = r#"{"total_usage": 50.0, "total_granted": 100.0}"#;
        let data: OpenAiBillingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.total_usage, 50.0);
        assert_eq!(data.total_granted, 100.0);
    }

    #[test]
    fn billing_response_parsing_defaults() {
        let json = r#"{}"#;
        let data: OpenAiBillingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.total_usage, 0.0);
        assert_eq!(data.total_granted, 0.0);
    }

    #[test]
    fn balance_from_response_valid() {
        let data = OpenAiBillingResponse {
            total_usage: 50.0,
            total_granted: 100.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.total, 100.0);
        assert_eq!(info.used, 50.0);
        assert_eq!(info.remaining, 50.0);
        assert_eq!(info.percentage_used, 50.0);
        assert_eq!(info.currency, "USD");
    }

    #[test]
    fn balance_from_response_zero_granted_avoids_division_by_zero() {
        // total_granted == 0 → percentage guard must yield 0.0, not NaN/inf.
        let data = OpenAiBillingResponse {
            total_usage: 0.0,
            total_granted: 0.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.total, 0.0);
        assert_eq!(info.used, 0.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 0.0);
        assert!(!info.percentage_used.is_nan());
    }

    #[test]
    fn balance_from_response_usage_exceeds_granted_yields_negative_remaining() {
        // Documents current behavior: when used > total, remaining goes negative
        // and percentage exceeds 100. This pins the contract so a future change
        // to clamping is intentional, not accidental.
        let data = OpenAiBillingResponse {
            total_usage: 120.0,
            total_granted: 100.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.used, 120.0);
        assert_eq!(info.remaining, -20.0);
        assert_eq!(info.percentage_used, 120.0);
    }

    #[test]
    fn balance_from_response_full_usage() {
        let data = OpenAiBillingResponse {
            total_usage: 100.0,
            total_granted: 100.0,
        };
        let info = balance_from_response(&data);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 100.0);
    }

    #[test]
    fn balance_from_response_from_empty_json_body() {
        // `{}` deserializes via serde defaults; computation must not panic.
        let data: OpenAiBillingResponse = serde_json::from_str("{}").unwrap();
        let info = balance_from_response(&data);
        assert_eq!(info.total, 0.0);
        assert_eq!(info.percentage_used, 0.0);
    }
}
