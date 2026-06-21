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

    // The factory is the single dispatch point shared by `commands::balance`
    // and `commands::usage`. A swapped or missing match arm silently routes
    // every balance/usage request for a provider class to the wrong upstream
    // API, so lock the variant → provider contract for every branch.
    #[test]
    fn create_balance_provider_maps_each_variant() {
        let cases = [
            (ProviderType::DeepSeek, "DeepSeek"),
            (ProviderType::OpenAi, "OpenAI"),
            (ProviderType::Anthropic, "Anthropic"),
            (ProviderType::OpenRouter, "OpenRouter"),
            (ProviderType::Custom, "Custom"),
        ];
        for (variant, expected) in cases {
            let provider = create_balance_provider(&variant, "https://custom.example.com");
            assert_eq!(
                provider.provider_name(),
                expected,
                "balance provider mismatch for variant {:?}",
                variant
            );
        }
    }

    #[test]
    fn create_usage_provider_maps_each_variant() {
        let cases = [
            (ProviderType::DeepSeek, "DeepSeek"),
            (ProviderType::OpenAi, "OpenAI"),
            (ProviderType::Anthropic, "Anthropic"),
            (ProviderType::OpenRouter, "OpenRouter"),
            (ProviderType::Custom, "Custom"),
        ];
        for (variant, expected) in cases {
            let provider = create_usage_provider(&variant, "https://custom.example.com");
            assert_eq!(
                provider.provider_name(),
                expected,
                "usage provider mismatch for variant {:?}",
                variant
            );
        }
    }
}
