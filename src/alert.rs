use crate::event::{Event, Severity};
use crate::config::Thresholds;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::path::PathBuf;

pub struct AlertEngine {
    thresholds: Thresholds,
    cooldowns: HashMap<String, Instant>,
    cooldown_duration: Duration,
}

impl AlertEngine {
    pub fn new(thresholds: Thresholds) -> Self {
        let cooldown_duration = Duration::from_secs(thresholds.alert_cooldown_secs);
        Self {
            thresholds,
            cooldowns: HashMap::new(),
            cooldown_duration,
        }
    }

    pub fn process(&mut self, event: &Event) -> Option<(String, Severity)> {
        let (key, message, severity) = match event {
            Event::SysHealth { cpu_pct, ram_pct, .. } => {
                let mut msgs = Vec::new();
                let mut sev = Severity::Info;
                
                if *cpu_pct > self.thresholds.cpu_pct { 
                    msgs.push(format!("CPU high: {:.1}% (limit: {:.1}%)", cpu_pct, self.thresholds.cpu_pct)); 
                    sev = Severity::Warning; 
                }
                if *ram_pct > self.thresholds.ram_pct { 
                    msgs.push(format!("RAM high: {:.1}% (limit: {:.1}%)", ram_pct, self.thresholds.ram_pct)); 
                    sev = Severity::Warning; 
                }
                
                if msgs.is_empty() { return None; }
                ("syshealth".to_string(), msgs.join(", "), sev)
            }
            Event::DangerousCommand { raw, .. } => {
                ("dangerous_cmd".to_string(), format!("Dangerous command detected: {}", raw), Severity::Critical)
            }
            Event::BuildFailure { tool, log_tail, .. } => {
                let mut msg = format!("Build failed ({}): {}", tool, log_tail);
                if let Some(suggestion) = self.oracle_diagnose(log_tail) {
                    msg.push_str(&format!("\n\n🔮 Oracle Suggestion: {}", suggestion));
                }
                ("build_failure".to_string(), msg, Severity::Critical)
            }
            Event::Custom { watcher, message, severity } => {
                (watcher.clone(), message.clone(), severity.clone())
            }
            Event::ProcessCrash { name, .. } => {
                ("process_crash".to_string(), format!("Bodyguard Alert: {} is missing from duty! Checking the perimeter.", name), Severity::Warning)
            }
            _ => return None,
        };

        let now = Instant::now();
        if let Some(last) = self.cooldowns.get(&key) {
            if now.duration_since(*last) < self.cooldown_duration {
                return None;
            }
        }

        self.cooldowns.insert(key, now);
        Some((message, severity))
    }

    fn oracle_diagnose(&self, log: &str) -> Option<String> {
        let rules = [
            ("unresolved import", "Check your imports or Cargo.toml for missing dependencies."),
            ("cannot find value", "Is the variable defined in this scope? Check for typos."),
            ("expected", "Type mismatch detected. Use `.into()` or check function signatures."),
            ("failed to resolve", "Run `cargo update` or check if the dependency is in Cargo.toml."),
        ];

        for (pattern, suggestion) in rules {
            if log.contains(pattern) {
                return Some(suggestion.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_thresholds(cpu: f32, ram: f32) -> Thresholds {
        Thresholds {
            cpu_pct: cpu,
            ram_pct: ram,
            disk_pct: 90.0,
            alert_cooldown_secs: 0, // No cooldown for tests
        }
    }

    #[test]
    fn test_alert_on_high_cpu() {
        let mut engine = AlertEngine::new(mock_thresholds(10.0, 90.0));
        let event = Event::SysHealth { cpu_pct: 15.0, ram_pct: 50.0, disk_pct: 0.0 };
        let result = engine.process(&event);
        assert!(result.is_some());
        assert!(result.unwrap().0.contains("CPU high"));
    }

    #[test]
    fn test_no_alert_on_low_usage() {
        let mut engine = AlertEngine::new(mock_thresholds(80.0, 90.0));
        let event = Event::SysHealth { cpu_pct: 10.0, ram_pct: 10.0, disk_pct: 0.0 };
        let result = engine.process(&event);
        assert!(result.is_none());
    }

    #[test]
    fn test_alert_on_dangerous_command() {
        let mut engine = AlertEngine::new(mock_thresholds(80.0, 90.0));
        let event = Event::DangerousCommand { 
            raw: "rm -rf /".to_string(), 
            cwd: PathBuf::from("/"), 
            blocked: false 
        };
        let result = engine.process(&event);
        assert!(result.is_some());
        let (msg, sev) = result.unwrap();
        assert!(msg.contains("Dangerous command detected"));
        assert_eq!(sev, Severity::Critical);
    }

    #[test]
    fn test_alert_on_build_failure() {
        let mut engine = AlertEngine::new(mock_thresholds(80.0, 90.0));
        let event = Event::BuildFailure { 
            tool: "cargo".to_string(), 
            exit_code: 1, 
            log_tail: "error[E0425]: cannot find value `x`".to_string() 
        };
        let result = engine.process(&event);
        assert!(result.is_some());
        let (msg, sev) = result.unwrap();
        assert!(msg.contains("Build failed (cargo)"));
        assert!(msg.contains("cannot find value `x`"));
        assert_eq!(sev, Severity::Critical);
    }

    #[test]
    fn test_alert_on_sentinel_event() {
        let mut engine = AlertEngine::new(mock_thresholds(80.0, 90.0));
        let event = Event::Custom {
            watcher: "sentinel".to_string(),
            message: "Sentinel Alert: New connection from 1.2.3.4".to_string(),
            severity: Severity::Info,
        };
        let result = engine.process(&event);
        assert!(result.is_some());
        let (msg, sev) = result.unwrap();
        assert!(msg.contains("Sentinel Alert"));
        assert_eq!(sev, Severity::Info);
    }

    #[test]
    fn test_alert_on_process_crash() {
        let mut engine = AlertEngine::new(mock_thresholds(80.0, 90.0));
        let event = Event::ProcessCrash {
            name: "postgres".to_string(),
            pid: 0,
            exit_code: None,
        };
        let result = engine.process(&event);
        assert!(result.is_some());
        let (msg, sev) = result.unwrap();
        assert!(msg.contains("Bodyguard Alert"));
        assert!(msg.contains("postgres"));
        assert_eq!(sev, Severity::Warning);
    }

    #[test]
    fn test_oracle_suggestion() {
        let mut engine = AlertEngine::new(mock_thresholds(80.0, 90.0));
        let event = Event::BuildFailure {
            tool: "cargo".to_string(),
            exit_code: 101,
            log_tail: "error[E0432]: unresolved import `abc`".to_string(),
        };
        let result = engine.process(&event);
        assert!(result.is_some());
        let (msg, _) = result.unwrap();
        assert!(msg.contains("Oracle Suggestion"));
        assert!(msg.contains("Check your imports"));
    }
}
