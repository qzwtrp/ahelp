use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::providers::ProviderName;

const CONFIG_DIR: &str = ".config/ahelp";
const FILE: &str = "config.toml";

fn config_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(CONFIG_DIR).join(FILE)
}

fn ensure_dir() {
    let dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let cfg = dir.join(CONFIG_DIR);
    fs::create_dir_all(&cfg).expect("Failed to create config dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&cfg).unwrap().permissions();
        perms.set_mode(0o700);
        fs::set_permissions(&cfg, perms).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Keys {
    pub gemini: Option<String>,
    pub openai: Option<String>,
    pub anthropic: Option<String>,
    pub openrouter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default_provider: Option<ProviderName>,
    #[serde(default)]
    pub keys: Keys,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_provider: Some(ProviderName::Gemini),
            keys: Keys::default(),
        }
    }
}

fn load() -> Config {
    let path = config_path();
    if !path.exists() {
        let default = Config::default();
        save(&default);
        return default;
    }
    let text = fs::read_to_string(&path).unwrap_or_default();
    toml::from_str(&text).unwrap_or_default()
}

fn save(cfg: &Config) {
    ensure_dir();
    let text = toml::to_string_pretty(cfg).unwrap();
    let path = config_path();
    fs::write(&path, text).expect("Failed to write config");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&path, perms).unwrap();
    }
}

pub fn set_key(provider: ProviderName, key: &str, make_default: bool) {
    let mut cfg = load();
    match provider {
        ProviderName::Gemini => cfg.keys.gemini = Some(key.into()),
        ProviderName::OpenAi => cfg.keys.openai = Some(key.into()),
        ProviderName::Anthropic => cfg.keys.anthropic = Some(key.into()),
        ProviderName::OpenRouter => cfg.keys.openrouter = Some(key.into()),
    }
    if make_default {
        cfg.default_provider = Some(provider);
    }
    save(&cfg);
}

pub fn get_key(provider: ProviderName) -> Option<String> {
    let cfg = load();
    let key = match provider {
        ProviderName::Gemini => cfg.keys.gemini,
        ProviderName::OpenAi => cfg.keys.openai,
        ProviderName::Anthropic => cfg.keys.anthropic,
        ProviderName::OpenRouter => cfg.keys.openrouter,
    };
    if key.is_some() {
        return key;
    }
    let env = match provider {
        ProviderName::Gemini => "GEMINI_API_KEY",
        ProviderName::OpenAi => "OPENAI_API_KEY",
        ProviderName::Anthropic => "ANTHROPIC_API_KEY",
        ProviderName::OpenRouter => "OPENROUTER_API_KEY",
    };
    std::env::var(env).ok()
}

pub fn default_provider() -> ProviderName {
    let cfg = load();
    cfg.default_provider.unwrap_or(ProviderName::Gemini)
}

pub fn list_keys() -> String {
    let cfg = load();
    let dp = cfg.default_provider.map(|p| format!("{}", p)).unwrap_or_else(|| "gemini (fallback)".into());
    format!(r#"Stored keys:
  gemini:     {}
  openai:     {}
  anthropic:  {}
  openrouter: {}

Default: {}
"#,
        if cfg.keys.gemini.is_some() { "yes" } else { "no" },
        if cfg.keys.openai.is_some() { "yes" } else { "no" },
        if cfg.keys.anthropic.is_some() { "yes" } else { "no" },
        if cfg.keys.openrouter.is_some() { "yes" } else { "no" },
        dp
    )
}
