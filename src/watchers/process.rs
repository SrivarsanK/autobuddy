use crate::event::{Event, Severity};
use crate::watchers::Watcher;
use async_trait::async_trait;
use tokio::sync::mpsc;
use sysinfo::System;
use std::time::Duration;

pub struct ProcessWatcher {
    pub critical_processes: Vec<String>,
}

#[async_trait]
impl Watcher for ProcessWatcher {
    fn name(&self) -> &'static str {
        "process"
    }

    async fn run(&self, tx: mpsc::Sender<Event>) -> anyhow::Result<()> {
        println!("ProcessWatcher active. Guarding: {:?}", self.critical_processes);
        let mut sys = System::new_all();

        loop {
            sys.refresh_processes();
            
            for target in &self.critical_processes {
                let is_running = sys.processes().values().any(|p| p.name() == target);
                
                if !is_running {
                    let _ = tx.send(Event::ProcessCrash {
                        name: target.clone(),
                        pid: 0,
                        exit_code: None,
                    }).await;
                }
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
