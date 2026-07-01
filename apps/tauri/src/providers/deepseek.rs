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

/// Derive the user-facing `BalanceInfo` from DeepSeek's raw balance fields.
///
/// DeepSeek reports `total_balance` as the *remaining* balance, so:
///   total      = granted + topped_up
///   used       = total - remaining
///   percentage = used / total * 100  (0 when total is 0 to avoid div-by-zero)
///
/// Values are reported as-is (no clamping), so over-usage surfaces as a
/// `used` greater than `total` and a percentage above 100.
fn compute_balance(granted: f64, topped_up: f64, remaining: f64, currency: String) -> BalanceInfo {
    let total = granted + topped_up;
    let used = total - remaining;
    let percentage = if total > 0.0 {
        (used / total) * 100.0
    } else {
        0.0
    };
    BalanceInfo {
        total,
        used,
        remaining,
        currency,
        percentage_used: percentage,
    }
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

        // DeepSeek's `total_balance` is the *remaining* balance, not the grant total.
        let remaining: f64 = info.total_balance.parse()
            .map_err(|_| crate::error::AppError::Unknown("Invalid total_balance format".to_string()))?;
        let granted: f64 = info.granted_balance.parse()
            .map_err(|_| crate::error::AppError::Unknown("Invalid granted_balance format".to_string()))?;
        let topped_up: f64 = info.topped_up_balance.parse()
            .map_err(|_| crate::error::AppError::Unknown("Invalid topped_up_balance format".to_string()))?;

        Ok(compute_balance(granted, topped_up, remaining, info.currency.clone()))
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
    fn compute_balance_normal_usage() {
        // granted=200, topped_up=50, remaining=100 → total=250, used=150, 60%
        let info = compute_balance(200.0, 50.0, 100.0, "CNY".to_string());
        assert_eq!(info.total, 250.0);
        assert_eq!(info.used, 150.0);
        assert_eq!(info.remaining, 100.0);
        assert_eq!(info.percentage_used, 60.0);
        assert_eq!(info.currency, "CNY");
    }

    #[test]
    fn compute_balance_zero_grant_avoids_div_by_zero() {
        // No grant at all — must not divide by zero; percentage clamps to 0.
        let info = compute_balance(0.0, 0.0, 0.0, "CNY".to_string());
        assert_eq!(info.total, 0.0);
        assert_eq!(info.used, 0.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 0.0);
    }

    #[test]
    fn compute_balance_full_usage() {
        // granted=100, topped_up=0, remaining=0 → fully consumed, 100%
        let info = compute_balance(100.0, 0.0, 0.0, "CNY".to_string());
        assert_eq!(info.total, 100.0);
        assert_eq!(info.used, 100.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 100.0);
    }

    #[test]
    fn compute_balance_over_usage_is_not_clamped() {
        // remaining went negative (e.g. refund reversal or API reporting lag):
        // used exceeds total and percentage exceeds 100 — contract is "no clamping".
        // (values chosen so the percentage is exactly representable in f64)
        let info = compute_balance(100.0, 0.0, -50.0, "CNY".to_string());
        assert_eq!(info.used, 150.0);
        assert_eq!(info.percentage_used, 150.0);
        assert_eq!(info.remaining, -50.0);
    }

    #[test]
    fn compute_balance_remaining_exceeds_grant() {
        // remaining > granted+topped (credit added beyond grant) → negative used.
        let info = compute_balance(100.0, 0.0, 150.0, "CNY".to_string());
        assert_eq!(info.used, -50.0);
        assert_eq!(info.percentage_used, -50.0);
    }
}
