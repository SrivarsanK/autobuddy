use crate::event::{Event, Severity};
use crate::watchers::Watcher;
use async_trait::async_trait;
use tokio::sync::mpsc;
use std::time::Duration;
use std::path::PathBuf;

pub struct SentinelWatcher;

#[async_trait]
impl Watcher for SentinelWatcher {
    fn name(&self) -> &'static str {
        "sentinel"
    }

    async fn run(&self, tx: mpsc::Sender<Event>) -> anyhow::Result<()> {
        println!("SentinelWatcher active. Monitoring auth.log...");
        let log_path = PathBuf::from("auth.log");

        loop {
            if log_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&log_path) {
                    for line in content.lines() {
                        if line.contains("Accepted password") {
                            // Format: Accepted password for [USER] from [IP]
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 9 {
                                let user = parts[8];
                                let ip = parts[10];
                                let _ = tx.send(Event::Custom {
                                    watcher: "sentinel".to_string(),
                                    message: format!("Sentinel Alert: New connection from {} for user {}. Should I keep an eye on them?", ip, user),
                                    severity: Severity::Info,
                                }).await;
                            }
                        }
                    }
                }
                // Cleanup for demo
                let _ = std::fs::remove_file(&log_path);
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
