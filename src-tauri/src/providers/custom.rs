use async_trait::async_trait;
use serde::Deserialize;

use crate::providers::{BalanceInfo, BalanceProvider, UsageData, UsageProvider};
use crate::http::{auth_headers, handle_response_status};

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
        let url = format!("{}/user/balance", self.api_base_url.trim_end_matches('/'));
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

        let total = if data.total_balance > 0.0 { data.total_balance } else { data.balance };
        let used = data.used;
        let remaining = if data.remaining > 0.0 { data.remaining } else { total - used };
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
        let json = r#"{"balance": 0.0, "total_balance": 100.0, "used": 30.0, "remaining": 70.0}"#;
        let data: CustomBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(data.total_balance, 100.0);
        assert_eq!(data.used, 30.0);
        assert_eq!(data.remaining, 70.0);
        
        let total = if data.total_balance > 0.0 { data.total_balance } else { data.balance };
        let used = data.used;
        let remaining = if data.remaining > 0.0 { data.remaining } else { total - used };
        let percentage = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
        
        assert_eq!(total, 100.0);
        assert_eq!(used, 30.0);
        assert_eq!(remaining, 70.0);
        assert_eq!(percentage, 30.0);
    }

    #[test]
    fn balance_response_parsing_fallback_to_balance() {
        let json = r#"{"balance": 50.0, "total_balance": 0.0, "used": 20.0, "remaining": 0.0}"#;
        let data: CustomBalanceResponse = serde_json::from_str(json).unwrap();
        
        let total = if data.total_balance > 0.0 { data.total_balance } else { data.balance };
        let used = data.used;
        let remaining = if data.remaining > 0.0 { data.remaining } else { total - used };
        
        assert_eq!(total, 50.0);
        assert_eq!(remaining, 30.0);
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
}
