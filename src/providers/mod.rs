use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderName {
    Gemini,
    OpenAi,
    Anthropic,
    OpenRouter,
}

impl std::fmt::Display for ProviderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderName::Gemini => write!(f, "gemini"),
            ProviderName::OpenAi => write!(f, "openai"),
            ProviderName::Anthropic => write!(f, "anthropic"),
            ProviderName::OpenRouter => write!(f, "openrouter"),
        }
    }
}

impl std::str::FromStr for ProviderName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gemini" => Ok(ProviderName::Gemini),
            "openai" | "chatgpt" => Ok(ProviderName::OpenAi),
            "anthropic" | "claude" => Ok(ProviderName::Anthropic),
            "openrouter" => Ok(ProviderName::OpenRouter),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

pub trait Provider {
    fn name(&self) -> ProviderName;
    fn generate(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String>;
}

pub mod gemini;
pub mod openai;
pub mod anthropic;
pub mod openrouter;

use gemini::GeminiProvider;
use openai::OpenAiProvider;
use anthropic::AnthropicProvider;
use openrouter::OpenRouterProvider;

pub fn build(name: ProviderName, api_key: &str) -> Box<dyn Provider> {
    match name {
        ProviderName::Gemini => Box::new(GeminiProvider::new(api_key)),
        ProviderName::OpenAi => Box::new(OpenAiProvider::new(api_key)),
        ProviderName::Anthropic => Box::new(AnthropicProvider::new(api_key)),
        ProviderName::OpenRouter => Box::new(OpenRouterProvider::new(api_key)),
    }
}

pub fn all_names() -> Vec<ProviderName> {
    vec![ProviderName::Gemini, ProviderName::OpenAi, ProviderName::Anthropic, ProviderName::OpenRouter]
}
