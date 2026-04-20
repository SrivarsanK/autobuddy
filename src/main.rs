mod config;
mod event;
mod alert;
mod telegram;
mod watchers;

use config::Config;
use alert::AlertEngine;
use telegram::TelegramBot;
use watchers::syshealth::SysHealthWatcher;
use watchers::terminal::TerminalWatcher;
use watchers::build::BuildWatcher;
use watchers::Watcher;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("autobuddy.toml")?;
    
    let (tx, mut rx) = mpsc::channel(100);
    let mut alert_engine = AlertEngine::new(config.thresholds.clone());
    let bot = TelegramBot::new(&config.telegram.bot_token, config.telegram.chat_id);

    println!("autobuddy daemon starting...");

    // Spawn SysHealth watcher
    if config.watchers.syshealth {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let watcher = SysHealthWatcher;
            if let Err(e) = watcher.run(tx_clone).await {
                eprintln!("SysHealthWatcher failed: {}", e);
            }
        });
    }

    // Spawn Terminal watcher
    if config.watchers.terminal {
        let tx_clone = tx.clone();
        let rules = config.rules.clone();
        tokio::spawn(async move {
            let watcher = TerminalWatcher { rules };
            if let Err(e) = watcher.run(tx_clone).await {
                eprintln!("TerminalWatcher failed: {}", e);
            }
        });
    }

    // Spawn Build watcher
    if config.watchers.build {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let watcher = BuildWatcher;
            if let Err(e) = watcher.run(tx_clone).await {
                eprintln!("BuildWatcher failed: {}", e);
            }
        });
    }

    // Event loop with graceful shutdown
    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                if let Some((msg, severity)) = alert_engine.process(&event) {
                    println!("[{:?}] {}", severity, msg);
                    if let Err(e) = bot.send_alert(&msg, severity).await {
                        eprintln!("Failed to send Telegram alert: {}", e);
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("autobuddy daemon shutting down...");
                break;
            }
        }
    }

    Ok(())
}
