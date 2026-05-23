use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::header::{self, HeaderMap, HeaderValue};

const MODEL: &str = "gemini-2.5-flash";
const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct RequestBody {
    contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    systemInstruction: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generationConfig: Option<GenerationConfig>,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    maxOutputTokens: i32,
}

#[derive(Deserialize)]
struct ResponseBody {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<ContentResp>,
    finishReason: Option<String>,
}

#[derive(Deserialize)]
struct ContentResp {
    parts: Option<Vec<PartResp>>,
}

#[derive(Deserialize)]
struct PartResp {
    text: Option<String>,
}

pub fn generate(api_key: &str, system_prompt: &str, user_prompt: &str) -> String {
    let url = format!("{}/models/{}:generateContent?key={}", BASE_URL, MODEL, api_key);

    let body = json!({
        "contents": [{
            "parts": [{"text": user_prompt}]
        }],
        "systemInstruction": {
            "parts": [{"text": system_prompt}]
        },
        "generationConfig": {
            "temperature": 0.3,
            "maxOutputTokens": 4096
        }
    });

    let json_bytes = serde_json::to_vec(&body).expect("serialize request");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("build client");

    let resp = client
        .post(&url)
        .headers(headers)
        .body(json_bytes)
        .send();

    match resp {
        Ok(r) => {
            let status = r.status();
            let text = r.text().unwrap_or_default();
            if !status.is_success() {
                return format!("API error ({}): {}", status, text);
            }
            match serde_json::from_str::<ResponseBody>(&text) {
                Ok(parsed) => {
                    parsed
                        .candidates
                        .and_then(|cands| cands.into_iter().next())
                        .and_then(|cand| cand.content)
                        .and_then(|content| content.parts)
                        .and_then(|parts| parts.into_iter().next())
                        .and_then(|part| part.text)
                        .unwrap_or_else(|| "(empty response)".into())
                }
                Err(e) => format!("JSON parse error: {} — body was: {}", e, text),
            }
        }
        Err(e) => format!("Network error: {}", e),
    }
}
