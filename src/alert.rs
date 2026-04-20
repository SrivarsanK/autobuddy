use crate::event::{Event, Severity};
use crate::config::Thresholds;
use std::collections::HashMap;
use std::time::{Duration, Instant};

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
}
