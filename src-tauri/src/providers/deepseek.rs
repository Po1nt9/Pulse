use async_trait::async_trait;
use serde::Deserialize;

use crate::providers::{BalanceInfo, BalanceProvider, UsageData, UsageProvider};
use crate::http::{auth_headers, handle_response_status};

pub struct DeepSeekProvider;

impl DeepSeekProvider {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize)]
struct DeepSeekBalanceResponse {
    balance_infos: Vec<BalanceInfoItem>,
    #[serde(default)]
    #[allow(dead_code)]
    is_available: bool,
}

#[derive(Deserialize)]
struct BalanceInfoItem {
    currency: String,
    total_balance: String,
    granted_balance: String,
    topped_up_balance: String,
}

#[async_trait]
impl BalanceProvider for DeepSeekProvider {
    async fn get_balance(
        &self,
        api_key: &str,
        client: &reqwest::Client,
    ) -> crate::error::Result<BalanceInfo> {
        let response = client
            .get("https://api.deepseek.com/user/balance")
            .headers(auth_headers(api_key)?)
            .send()
            .await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response.text().await
            .map_err(|e| crate::error::AppError::Network(e.to_string()))?;
        handle_response_status(status, &body)?;

        let data: DeepSeekBalanceResponse = serde_json::from_str(&body)
            .map_err(|e| crate::error::AppError::Unknown(format!("Parse error: {}", e)))?;

        let info = data.balance_infos.first()
            .ok_or_else(|| crate::error::AppError::Unknown("No balance info".to_string()))?;

        let total: f64 = info.total_balance.parse()
            .map_err(|_| crate::error::AppError::Unknown("Invalid total_balance format".to_string()))?;
        let granted: f64 = info.granted_balance.parse()
            .map_err(|_| crate::error::AppError::Unknown("Invalid granted_balance format".to_string()))?;
        let topped_up: f64 = info.topped_up_balance.parse()
            .map_err(|_| crate::error::AppError::Unknown("Invalid topped_up_balance format".to_string()))?;
        
        // DeepSeek: total_balance is remaining balance
        // used = granted + topped_up - total_balance (remaining)
        let used = granted + topped_up - total;
        let percentage = if (granted + topped_up) > 0.0 {
            (used / (granted + topped_up)) * 100.0
        } else {
            0.0
        };

        Ok(BalanceInfo {
            total: granted + topped_up,
            used,
            remaining: total,
            currency: info.currency.clone(),
            percentage_used: percentage,
        })
    }

    fn provider_name(&self) -> &str {
        "DeepSeek"
    }
}

#[async_trait]
impl UsageProvider for DeepSeekProvider {
    async fn get_usage(
        &self,
        _api_key: &str,
        _period: &str,
        _client: &reqwest::Client,
    ) -> crate::error::Result<UsageData> {
        // DeepSeek usage API requires platform token — return empty for now
        // Will be implemented with platform token support in future
        Ok(UsageData {
            points: vec![],
            total_cost: 0.0,
            total_tokens_input: 0,
            total_tokens_output: 0,
            total_requests: 0,
            period: _period.to_string(),
        })
    }

    fn provider_name(&self) -> &str {
        "DeepSeek"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_response_parsing_valid() {
        let json = r#"{
            "balance_infos": [
                {
                    "currency": "CNY",
                    "total_balance": "100.50",
                    "granted_balance": "200.00",
                    "topped_up_balance": "50.00"
                }
            ],
            "is_available": true
        }"#;

        let response: DeepSeekBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.balance_infos.len(), 1);
        assert_eq!(response.balance_infos[0].currency, "CNY");
        assert_eq!(response.balance_infos[0].total_balance, "100.50");
        assert_eq!(response.balance_infos[0].granted_balance, "200.00");
        assert_eq!(response.balance_infos[0].topped_up_balance, "50.00");
    }

    #[test]
    fn balance_response_parsing_empty_balance_infos() {
        let json = r#"{
            "balance_infos": [],
            "is_available": true
        }"#;
        let response: DeepSeekBalanceResponse = serde_json::from_str(json).unwrap();
        assert!(response.balance_infos.is_empty());
    }

    #[test]
    fn balance_response_parsing_missing_is_available_defaults() {
        let json = r#"{
            "balance_infos": [
                {
                    "currency": "USD",
                    "total_balance": "10.00",
                    "granted_balance": "0.00",
                    "topped_up_balance": "10.00"
                }
            ]
        }"#;
        let response: DeepSeekBalanceResponse = serde_json::from_str(json).unwrap();
        assert!(!response.is_available); // serde(default)
    }

    #[test]
    fn balance_info_calculation() {
        // granted=200, topped_up=50, total=100 (remaining)
        // used = 200 + 50 - 100 = 150
        // percentage = 150 / 250 * 100 = 60%
        let total: f64 = "100.0".parse().unwrap();
        let granted: f64 = "200.0".parse().unwrap();
        let topped_up: f64 = "50.0".parse().unwrap();
        let used = granted + topped_up - total;
        let percentage = used / (granted + topped_up) * 100.0;
        
        assert_eq!(used, 150.0);
        assert_eq!(percentage, 60.0);
    }

    #[test]
    fn balance_info_calculation_zero_granted() {
        let total: f64 = "0.0".parse().unwrap();
        let granted: f64 = "0.0".parse().unwrap();
        let topped_up: f64 = "0.0".parse().unwrap();
        let used = granted + topped_up - total;
        let percentage = if (granted + topped_up) > 0.0 {
            (used / (granted + topped_up)) * 100.0
        } else {
            0.0
        };
        
        assert_eq!(used, 0.0);
        assert_eq!(percentage, 0.0);
    }

    #[test]
    fn balance_info_calculation_full_usage() {
        // granted=100, topped_up=0, total=0 (remaining)
        // used = 100, percentage = 100%
        let total: f64 = "0.0".parse().unwrap();
        let granted: f64 = "100.0".parse().unwrap();
        let topped_up: f64 = "0.0".parse().unwrap();
        let used = granted + topped_up - total;
        let percentage = (used / (granted + topped_up)) * 100.0;
        
        assert_eq!(used, 100.0);
        assert_eq!(percentage, 100.0);
    }
}
