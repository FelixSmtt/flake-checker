use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub flake_lock_path: String,
    pub allowed_inputs: Vec<String>,
    #[serde(default = "default_notification_format")]
    pub notification_format: String,
}

fn default_notification_format() -> String {
    "• <b>{name}</b> ({target_ref})\n  {local_rev} → {remote_rev}\n".into()
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}
