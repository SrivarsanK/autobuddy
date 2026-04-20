use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub thresholds: Thresholds,
    pub watchers: Watchers,
    pub rules: Rules,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: i64,
    pub master_pin: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Thresholds {
    pub cpu_pct: f32,
    pub ram_pct: f32,
    pub disk_pct: f32,
    pub alert_cooldown_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Watchers {
    pub syshealth: bool,
    pub process: bool,
    pub terminal: bool,
    pub build: bool,
    pub sentinel: bool,
    pub critical_processes: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rules {
    pub dangerous_commands: Vec<String>,
    pub block_on_match: bool,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
