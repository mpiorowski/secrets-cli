use anyhow::{Context, Result};
use std::{
    env::var,
    fs::{metadata, read_to_string, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub config_path: PathBuf,
    pub secrets_path: PathBuf,
    pub clipboard_command: String,
}

impl Config {
    pub fn create() -> Result<Self> {
        // Get config path
        let config_path = var("XDG_CONFIG_HOME")
            .or_else(|_| var("HOME").map(|v| v + "/.config"))
            .context("Config not set")?;
        let mut config_path = PathBuf::from(config_path);
        config_path.push("secrets-cli.json");

        // If config file doesn't exist, create it
        if metadata(&config_path).is_err() {
            let mut file = File::create(&config_path).context("Config file not created")?;
            file.write_all(b"{\"secrets_path\": \"~/secrets\"}")
                .context("Config file not written")?;
            return Ok(Config {
                config_path,
                secrets_path: PathBuf::from("~/secrets"),
                clipboard_command: "xclip".to_string(),
            });
        }
        // If config file exists, read it
        let config_str = read_to_string(&config_path).context("Config not found")?;
        let config_json: serde_json::Value =
            serde_json::from_str(&config_str).context("Config not valid")?;
        let secrets_path = config_json
            .get("secrets_path")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .context("Secrets folder path not found in config")?;
        let clipboard_command = config_json
            .get("clipboard_command")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .unwrap_or("xclip".to_string());
        Ok(Config {
            config_path,
            secrets_path,
            clipboard_command,
        })
    }
}
