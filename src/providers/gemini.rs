use crate::providers::{Provider, ProviderName};
use reqwest::blocking::Client;
use serde_json::json;

pub struct GeminiProvider {
    api_key: String,
    client: Client,
}

const MODEL: &str = "gemini-2.5-flash";
const BASE: &str = "https://generativelanguage.googleapis.com/v1beta";

impl GeminiProvider {
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

impl Provider for GeminiProvider {
    fn name(&self) -> ProviderName { ProviderName::Gemini }

    fn generate(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        let url = format!("{}/models/{}:generateContent?key={}", BASE, MODEL, self.api_key);
        let body = json!({
            "contents": [{"parts": [{"text": user_prompt}]}],
            "systemInstruction": {"parts": [{"text": system_prompt}]},
            "generationConfig": {"temperature": 0.3, "maxOutputTokens": 4096}
        });

        let resp = self.client.post(&url)
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
        let text = json.get("candidates")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("content"))
            .and_then(|c| c.get("parts"))
            .and_then(|p| p.as_array())
            .and_then(|arr| arr.first())
            .and_then(|p| p.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("(empty response)")
            .to_string();

        Ok(text)
    }
}
