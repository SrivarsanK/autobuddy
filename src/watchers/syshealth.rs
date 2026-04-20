use crate::event::Event;
use crate::watchers::Watcher;
use async_trait::async_trait;
use sysinfo::System;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

pub struct SysHealthWatcher;

#[async_trait]
impl Watcher for SysHealthWatcher {
    fn name(&self) -> &'static str {
        "syshealth"
    }

    async fn run(&self, tx: mpsc::Sender<Event>) -> anyhow::Result<()> {
        let mut sys = System::new_all();
        let mut interval = time::interval(Duration::from_secs(30));

        loop {
            interval.tick().await;
            sys.refresh_all();

            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let ram_usage = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
            
            let event = Event::SysHealth {
                cpu_pct: cpu_usage,
                ram_pct: ram_usage,
                disk_pct: 0.0,
            };

            if let Err(e) = tx.send(event).await {
                eprintln!("Failed to send SysHealth event: {}", e);
                break;
            }
        }

        Ok(())
    }
}
