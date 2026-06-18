use serde::{Deserialize, Serialize};
use async_trait::async_trait;

pub mod deepseek;
pub mod openai;
pub mod anthropic;
pub mod openrouter;
pub mod custom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceInfo {
    pub total: f64,
    pub used: f64,
    pub remaining: f64,
    pub currency: String,
    pub percentage_used: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePoint {
    pub timestamp: String,
    pub cost: f64,
    pub tokens_input: u64,
    pub tokens_output: u64,
    pub requests: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    pub points: Vec<UsagePoint>,
    pub total_cost: f64,
    pub total_tokens_input: u64,
    pub total_tokens_output: u64,
    pub total_requests: u64,
    pub period: String,
}

#[async_trait]
pub trait BalanceProvider: Send + Sync {
    async fn get_balance(
        &self,
        api_key: &str,
        client: &reqwest::Client,
    ) -> crate::error::Result<BalanceInfo>;
    fn provider_name(&self) -> &str;
}

#[async_trait]
pub trait UsageProvider: Send + Sync {
    async fn get_usage(
        &self,
        api_key: &str,
        period: &str,
        client: &reqwest::Client,
    ) -> crate::error::Result<UsageData>;
    fn provider_name(&self) -> &str;
}

pub fn create_balance_provider(
    provider_type: &crate::config::ProviderType,
    api_base_url: &str,
) -> Box<dyn BalanceProvider> {
    match provider_type {
        crate::config::ProviderType::DeepSeek => Box::new(deepseek::DeepSeekProvider::new()),
        crate::config::ProviderType::OpenAi => Box::new(openai::OpenAiProvider::new()),
        crate::config::ProviderType::Anthropic => Box::new(anthropic::AnthropicProvider::new()),
        crate::config::ProviderType::OpenRouter => Box::new(openrouter::OpenRouterProvider::new()),
        crate::config::ProviderType::Custom => Box::new(custom::CustomProvider::with_url(api_base_url.to_string())),
    }
}

pub fn create_usage_provider(
    provider_type: &crate::config::ProviderType,
    api_base_url: &str,
) -> Box<dyn UsageProvider> {
    match provider_type {
        crate::config::ProviderType::DeepSeek => Box::new(deepseek::DeepSeekProvider::new()),
        crate::config::ProviderType::OpenAi => Box::new(openai::OpenAiProvider::new()),
        crate::config::ProviderType::Anthropic => Box::new(anthropic::AnthropicProvider::new()),
        crate::config::ProviderType::OpenRouter => Box::new(openrouter::OpenRouterProvider::new()),
        crate::config::ProviderType::Custom => Box::new(custom::CustomProvider::with_url(api_base_url.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProviderType;

    #[test]
    fn create_balance_provider_deepseek() {
        let provider = create_balance_provider(&ProviderType::DeepSeek, "https://api.deepseek.com");
        assert_eq!(provider.provider_name(), "DeepSeek");
    }

    #[test]
    fn create_balance_provider_openai() {
        let provider = create_balance_provider(&ProviderType::OpenAi, "https://api.openai.com");
        assert_eq!(provider.provider_name(), "OpenAI");
    }

    #[test]
    fn create_balance_provider_anthropic() {
        let provider = create_balance_provider(&ProviderType::Anthropic, "https://api.anthropic.com");
        assert_eq!(provider.provider_name(), "Anthropic");
    }

    #[test]
    fn create_balance_provider_openrouter() {
        let provider = create_balance_provider(&ProviderType::OpenRouter, "https://openrouter.ai/api");
        assert_eq!(provider.provider_name(), "OpenRouter");
    }

    #[test]
    fn create_balance_provider_custom() {
        let provider = create_balance_provider(&ProviderType::Custom, "https://custom.example.com");
        assert_eq!(provider.provider_name(), "Custom");
    }

    #[test]
    fn create_usage_provider_deepseek() {
        let provider = create_usage_provider(&ProviderType::DeepSeek, "https://api.deepseek.com");
        assert_eq!(provider.provider_name(), "DeepSeek");
    }

    #[test]
    fn create_usage_provider_openai() {
        let provider = create_usage_provider(&ProviderType::OpenAi, "https://api.openai.com");
        assert_eq!(provider.provider_name(), "OpenAI");
    }

    #[test]
    fn create_usage_provider_anthropic() {
        let provider = create_usage_provider(&ProviderType::Anthropic, "https://api.anthropic.com");
        assert_eq!(provider.provider_name(), "Anthropic");
    }

    #[test]
    fn create_usage_provider_openrouter() {
        let provider = create_usage_provider(&ProviderType::OpenRouter, "https://openrouter.ai/api");
        assert_eq!(provider.provider_name(), "OpenRouter");
    }

    #[test]
    fn create_usage_provider_custom() {
        let provider = create_usage_provider(&ProviderType::Custom, "https://custom.example.com");
        assert_eq!(provider.provider_name(), "Custom");
    }

    #[test]
    fn create_balance_provider_default_variant_is_custom() {
        // ProviderType::default() is Custom — the factory must handle it,
        // otherwise loading a config with a missing/unknown type would panic.
        let default_type = ProviderType::default();
        let provider = create_balance_provider(&default_type, "https://fallback.example.com");
        assert_eq!(provider.provider_name(), "Custom");
    }

    #[test]
    fn create_usage_provider_default_variant_is_custom() {
        let default_type = ProviderType::default();
        let provider = create_usage_provider(&default_type, "https://fallback.example.com");
        assert_eq!(provider.provider_name(), "Custom");
    }

    #[test]
    fn balance_info_serde_roundtrip() {
        let info = BalanceInfo {
            total: 100.0,
            used: 30.0,
            remaining: 70.0,
            currency: "USD".to_string(),
            percentage_used: 30.0,
        };
        let json = serde_json::to_string(&info).unwrap();
        let restored: BalanceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.total, info.total);
        assert_eq!(restored.used, info.used);
        assert_eq!(restored.remaining, info.remaining);
        assert_eq!(restored.currency, info.currency);
        assert_eq!(restored.percentage_used, info.percentage_used);
    }

    #[test]
    fn usage_data_serde_roundtrip() {
        let data = UsageData {
            points: vec![UsagePoint {
                timestamp: "2026-06-18T10:00:00Z".to_string(),
                cost: 0.05,
                tokens_input: 1000,
                tokens_output: 500,
                requests: 2,
            }],
            total_cost: 0.05,
            total_tokens_input: 1000,
            total_tokens_output: 500,
            total_requests: 2,
            period: "today".to_string(),
        };
        let json = serde_json::to_string(&data).unwrap();
        let restored: UsageData = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.points.len(), 1);
        assert_eq!(restored.total_cost, data.total_cost);
        assert_eq!(restored.period, data.period);
        assert_eq!(restored.points[0].tokens_input, 1000);
    }

    #[test]
    fn usage_data_empty_points_serde_roundtrip() {
        let data = UsageData {
            points: vec![],
            total_cost: 0.0,
            total_tokens_input: 0,
            total_tokens_output: 0,
            total_requests: 0,
            period: "week".to_string(),
        };
        let json = serde_json::to_string(&data).unwrap();
        let restored: UsageData = serde_json::from_str(&json).unwrap();
        assert!(restored.points.is_empty());
        assert_eq!(restored.period, "week");
    }
}
