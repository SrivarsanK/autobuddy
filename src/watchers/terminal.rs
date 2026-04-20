use crate::event::Event;
use crate::watchers::Watcher;
use crate::config::Rules;
use async_trait::async_trait;
use tokio::sync::mpsc;
use std::time::Duration;
use std::path::PathBuf;

pub struct TerminalWatcher {
    pub rules: Rules,
}

#[async_trait]
impl Watcher for TerminalWatcher {
    fn name(&self) -> &'static str {
        "terminal"
    }

    async fn run(&self, tx: mpsc::Sender<Event>) -> anyhow::Result<()> {
        println!("TerminalWatcher active. Monitoring dangerous commands...");
        
        // For the demo/viral aspect, we'll check for a simulated command file or history.
        // In a real production app, this would use a shell hook.
        let trigger_path = PathBuf::from(".autobuddy_trigger");

        loop {
            if trigger_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&trigger_path) {
                    let cmd = content.trim();
                    if !cmd.is_empty() {
                        for danger in &self.rules.dangerous_commands {
                            if cmd.contains(danger) {
                                let _ = tx.send(Event::DangerousCommand {
                                    raw: cmd.to_string(),
                                    cwd: std::env::current_dir().unwrap_or_default(),
                                    blocked: self.rules.block_on_match,
                                }).await;
                            }
                        }
                    }
                }
                let _ = std::fs::remove_file(&trigger_path);
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
}
