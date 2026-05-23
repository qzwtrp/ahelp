use crate::providers::{Provider, ProviderName};
use reqwest::blocking::Client;
use serde_json::json;

pub struct OpenRouterProvider {
    api_key: String,
    client: Client,
}

const MODEL: &str = "anthropic/claude-3.5-sonnet";
const BASE: &str = "https://openrouter.ai/api/v1/chat/completions";

impl OpenRouterProvider {
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

impl Provider for OpenRouterProvider {
    fn name(&self) -> ProviderName { ProviderName::OpenRouter }

    fn generate(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        let body = json!({
            "model": MODEL,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
            ],
            "temperature": 0.3,
            "max_tokens": 4096
        });

        let resp = self.client.post(BASE)
            .bearer_auth(&self.api_key)
            .header("HTTP-Referer", "https://github.com/qzwtrp/ahelp")
            .header("X-Title", "ahelp")
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
        let text = json.get("choices")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|t| t.as_str())
            .unwrap_or("(empty response)")
            .to_string();

        Ok(text)
    }
}
