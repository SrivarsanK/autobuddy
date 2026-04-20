pub mod syshealth;
pub mod terminal;
pub mod build;

use crate::event::Event;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait Watcher: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run(&self, tx: mpsc::Sender<Event>) -> anyhow::Result<()>;
}
