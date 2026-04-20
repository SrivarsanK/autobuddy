use crate::event::Event;
use crate::watchers::Watcher;
use async_trait::async_trait;
use tokio::sync::mpsc;
use std::time::Duration;
use std::path::PathBuf;

pub struct BuildWatcher;

#[async_trait]
impl Watcher for BuildWatcher {
    fn name(&self) -> &'static str {
        "build"
    }

    async fn run(&self, tx: mpsc::Sender<Event>) -> anyhow::Result<()> {
        println!("BuildWatcher active. Monitoring build.log...");
        let log_path = PathBuf::from("build.log");

        loop {
            if log_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&log_path) {
                    let content_lower = content.to_lowercase();
                    if content_lower.contains("error") || content_lower.contains("fail") {
                        let _ = tx.send(Event::BuildFailure {
                            tool: "compiler".to_string(),
                            exit_code: 1,
                            log_tail: content.lines().last().unwrap_or("No logs").to_string(),
                        }).await;
                    }
                }
                // For demo simplicity, we delete the log after reading
                let _ = std::fs::remove_file(&log_path);
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
