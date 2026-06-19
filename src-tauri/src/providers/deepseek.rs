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

        balance_from_response(&data)
    }

    fn provider_name(&self) -> &str {
        "DeepSeek"
    }
}

/// Pure transformation from a parsed DeepSeek balance response into `BalanceInfo`.
///
/// Extracted from `get_balance` so the parsing/validation/percentage logic —
/// including its failure modes and boundary conditions — is testable without
/// touching the network. `get_balance` is now a thin HTTP + serde wrapper.
fn balance_from_response(data: &DeepSeekBalanceResponse) -> crate::error::Result<BalanceInfo> {
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
    fn balance_from_response_valid() {
        // granted=200, topped_up=50, total(remaining)=100
        // used = 200 + 50 - 100 = 150 ; total = 250 ; percentage = 60%
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![BalanceInfoItem {
                currency: "CNY".to_string(),
                total_balance: "100.00".to_string(),
                granted_balance: "200.00".to_string(),
                topped_up_balance: "50.00".to_string(),
            }],
            is_available: true,
        };
        let info = balance_from_response(&data).unwrap();
        assert_eq!(info.currency, "CNY");
        assert_eq!(info.remaining, 100.0);
        assert_eq!(info.total, 250.0);
        assert_eq!(info.used, 150.0);
        assert!((info.percentage_used - 60.0).abs() < 1e-9);
    }

    #[test]
    fn balance_from_response_empty_balance_infos_errors() {
        // No balance_infos entry → must surface a clear error, not panic.
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![],
            is_available: true,
        };
        let err = balance_from_response(&data).unwrap_err();
        assert!(matches!(err, crate::error::AppError::Unknown(ref m) if m == "No balance info"));
    }

    #[test]
    fn balance_from_response_non_numeric_total_errors() {
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![BalanceInfoItem {
                currency: "CNY".to_string(),
                total_balance: "not-a-number".to_string(),
                granted_balance: "200.00".to_string(),
                topped_up_balance: "50.00".to_string(),
            }],
            is_available: true,
        };
        let err = balance_from_response(&data).unwrap_err();
        assert!(matches!(err, crate::error::AppError::Unknown(ref m) if m == "Invalid total_balance format"));
    }

    #[test]
    fn balance_from_response_non_numeric_granted_errors() {
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![BalanceInfoItem {
                currency: "CNY".to_string(),
                total_balance: "100.00".to_string(),
                granted_balance: "".to_string(),
                topped_up_balance: "50.00".to_string(),
            }],
            is_available: true,
        };
        let err = balance_from_response(&data).unwrap_err();
        assert!(matches!(err, crate::error::AppError::Unknown(ref m) if m == "Invalid granted_balance format"));
    }

    #[test]
    fn balance_from_response_non_numeric_topped_up_errors() {
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![BalanceInfoItem {
                currency: "CNY".to_string(),
                total_balance: "100.00".to_string(),
                granted_balance: "200.00".to_string(),
                topped_up_balance: "abc".to_string(),
            }],
            is_available: true,
        };
        let err = balance_from_response(&data).unwrap_err();
        assert!(matches!(err, crate::error::AppError::Unknown(ref m) if m == "Invalid topped_up_balance format"));
    }

    #[test]
    fn balance_from_response_nan_string_propagates_nan() {
        // Data-validation gap (pinned, not endorsed): Rust's `f64::from_str`
        // accepts "NaN"/"inf", so a malformed API value is not rejected by the
        // `parse()` guard — it silently produces NaN in the resulting BalanceInfo.
        // This test documents the current behavior so any future sanitization
        // (e.g. rejecting non-finite values) is an intentional contract change.
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![BalanceInfoItem {
                currency: "CNY".to_string(),
                total_balance: "100.00".to_string(),
                granted_balance: "200.00".to_string(),
                topped_up_balance: "NaN".to_string(),
            }],
            is_available: true,
        };
        let info = balance_from_response(&data).unwrap();
        assert!(info.total.is_nan(), "NaN string should propagate as NaN, not be rejected");
        assert!(info.used.is_nan());
        // remaining is parsed from a valid number, so it stays finite.
        assert_eq!(info.remaining, 100.0);
    }

    #[test]
    fn balance_from_response_zero_granted_avoids_division_by_zero() {
        // granted + topped_up == 0 → percentage guard must yield 0.0, not NaN/inf.
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![BalanceInfoItem {
                currency: "CNY".to_string(),
                total_balance: "0".to_string(),
                granted_balance: "0".to_string(),
                topped_up_balance: "0".to_string(),
            }],
            is_available: false,
        };
        let info = balance_from_response(&data).unwrap();
        assert_eq!(info.total, 0.0);
        assert_eq!(info.used, 0.0);
        assert_eq!(info.remaining, 0.0);
        assert_eq!(info.percentage_used, 0.0);
        assert!(!info.percentage_used.is_nan());
    }

    #[test]
    fn balance_from_response_full_usage() {
        // granted=100, topped_up=0, remaining=0 → used=100, percentage=100%.
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![BalanceInfoItem {
                currency: "CNY".to_string(),
                total_balance: "0.0".to_string(),
                granted_balance: "100.0".to_string(),
                topped_up_balance: "0.0".to_string(),
            }],
            is_available: true,
        };
        let info = balance_from_response(&data).unwrap();
        assert_eq!(info.used, 100.0);
        assert_eq!(info.percentage_used, 100.0);
    }

    #[test]
    fn balance_from_response_uses_first_balance_info_only() {
        // API may return multiple entries; only the first must be used.
        let data = DeepSeekBalanceResponse {
            balance_infos: vec![
                BalanceInfoItem {
                    currency: "CNY".to_string(),
                    total_balance: "10.0".to_string(),
                    granted_balance: "100.0".to_string(),
                    topped_up_balance: "0.0".to_string(),
                },
                BalanceInfoItem {
                    currency: "USD".to_string(),
                    total_balance: "9999.0".to_string(),
                    granted_balance: "9999.0".to_string(),
                    topped_up_balance: "9999.0".to_string(),
                },
            ],
            is_available: true,
        };
        let info = balance_from_response(&data).unwrap();
        assert_eq!(info.currency, "CNY");
        assert_eq!(info.remaining, 10.0);
        assert_eq!(info.used, 90.0);
    }
}
