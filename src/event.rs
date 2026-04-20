use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Event {
    SysHealth { cpu_pct: f32, ram_pct: f32, disk_pct: f32 },
    ProcessCrash { name: String, pid: u32, exit_code: Option<i32> },
    DangerousCommand { raw: String, cwd: PathBuf, blocked: bool },
    BuildFailure { tool: String, exit_code: i32, log_tail: String },
    Custom { watcher: String, message: String, severity: Severity },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity { 
    Info, 
    Warning, 
    Critical 
}
