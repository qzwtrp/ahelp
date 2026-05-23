use crate::providers::{Provider, ProviderName};
use reqwest::blocking::Client;
use serde_json::json;

pub struct AnthropicProvider {
    api_key: String,
    client: Client,
}

const MODEL: &str = "claude-3-opus-20240229";
const BASE: &str = "https://api.anthropic.com/v1/messages";

impl AnthropicProvider {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap(),
        }
    }
}

impl Provider for AnthropicProvider {
    fn name(&self) -> ProviderName { ProviderName::Anthropic }

    fn generate(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        let body = json!({
            "model": MODEL,
            "max_tokens": 4096,
            "system": system_prompt,
            "messages": [{"role": "user", "content": user_prompt}]
        });

        let resp = self.client.post(BASE)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| format!("Network error: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().unwrap_or_default();
            return Err(format!("API error ({}): {}", status, text));
        }

        let json: serde_json::Value = resp.json().map_err(|e| format!("JSON parse error: {}", e))?;
        let text = json.get("content")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("(empty response)")
            .to_string();

        Ok(text)
    }
}
