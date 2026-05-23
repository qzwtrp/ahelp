use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".config/ahelp";
const KEY_FILE: &str = "api_key";

fn config_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(CONFIG_DIR)
}

pub fn set_key(value: String) {
    let dir = config_dir();
    fs::create_dir_all(&dir).expect("Failed to create config dir");
    let path = dir.join(KEY_FILE);
    fs::write(&path, value.trim()).expect("Failed to write API key");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&path, perms).unwrap();
    }
}

pub fn get_key() -> Option<String> {
    let path = config_dir().join(KEY_FILE);
    if path.exists() {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    } else {
        std::env::var("GEMINI_API_KEY").ok()
    }
}
