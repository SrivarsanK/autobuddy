use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub buddy_mode: BuddyMode,
    pub thresholds: Thresholds,
    pub watchers: Watchers,
    pub rules: Rules,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BuddyMode {
    Silent,
    Normal,
    Chatty,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: i64,
    pub master_pin: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct Thresholds {
    pub cpu_pct: f32,
    pub ram_pct: f32,
    pub disk_pct: f32,
    pub alert_cooldown_secs: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watchers {
    pub syshealth: bool,
    pub process: bool,
    pub terminal: bool,
    pub build: bool,
    pub sentinel: bool,
    pub auto_heal: bool,
    pub critical_processes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        use std::io::Write;
        
        let path = path.as_ref();
        let dir = path.parent().unwrap_or(Path::new("."));
        
        // Use a temporary file in the same directory to ensure atomic rename
        let mut temp = tempfile::NamedTempFile::new_in(dir)?;
        let toml_string = toml::to_string_pretty(self)?;
        
        temp.write_all(toml_string.as_bytes())?;
        temp.persist(path)?;
        
        Ok(())
    }
}
