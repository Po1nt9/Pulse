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

        let remaining = data.data.credits_remaining;
        let total = data.data.total_credits;
        let used = total - remaining;
        let percentage = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };

        Ok(BalanceInfo {
            total,
            used,
            remaining,
            currency: "USD".to_string(),
            percentage_used: percentage,
        })
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
    fn balance_calculation() {
        let remaining = 25.0;
        let total = 100.0;
        let used = total - remaining;
        let percentage = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
        assert_eq!(used, 75.0);
        assert_eq!(percentage, 75.0);
    }

    #[test]
    fn balance_calculation_zero_total() {
        let total = 0.0;
        let remaining = 0.0;
        let used = total - remaining;
        let percentage = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
        assert_eq!(used, 0.0);
        assert_eq!(percentage, 0.0);
    }
}
