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

    /// Every `ProviderType` must resolve to a balance adapter with the expected name.
    /// This guards the factory's match arms (including the Custom+url branch) against
    /// silent regressions when variants are added or reordered.
    #[test]
    fn create_balance_provider_maps_every_variant() {
        let cases = [
            (ProviderType::DeepSeek, "DeepSeek"),
            (ProviderType::OpenAi, "OpenAI"),
            (ProviderType::Anthropic, "Anthropic"),
            (ProviderType::OpenRouter, "OpenRouter"),
            (ProviderType::Custom, "Custom"),
        ];
        for (pt, expected_name) in cases {
            let provider = create_balance_provider(&pt, "https://custom.example.com");
            assert_eq!(
                provider.provider_name(),
                expected_name,
                "wrong balance provider for {:?}",
                pt
            );
        }
    }

    #[test]
    fn create_usage_provider_maps_every_variant() {
        let cases = [
            (ProviderType::DeepSeek, "DeepSeek"),
            (ProviderType::OpenAi, "OpenAI"),
            (ProviderType::Anthropic, "Anthropic"),
            (ProviderType::OpenRouter, "OpenRouter"),
            (ProviderType::Custom, "Custom"),
        ];
        for (pt, expected_name) in cases {
            let provider = create_usage_provider(&pt, "https://custom.example.com");
            assert_eq!(
                provider.provider_name(),
                expected_name,
                "wrong usage provider for {:?}",
                pt
            );
        }
    }

    /// The Custom arm must accept the supplied base URL without panicking — a regression
    /// here (e.g. someone dropping the `api_base_url` argument) would break every custom
    /// provider at runtime.
    #[test]
    fn create_balance_provider_custom_accepts_base_url() {
        let provider = create_balance_provider(&ProviderType::Custom, "https://api.my-llm.io/v1/");
        assert_eq!(provider.provider_name(), "Custom");
    }

    #[test]
    fn create_usage_provider_custom_accepts_base_url() {
        let provider = create_usage_provider(&ProviderType::Custom, "https://api.my-llm.io/v1/");
        assert_eq!(provider.provider_name(), "Custom");
    }
}
