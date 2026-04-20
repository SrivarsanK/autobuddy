mod config;
mod event;
mod alert;
mod telegram;
mod watchers;

use config::Config;
use alert::AlertEngine;
use telegram::TelegramBot;
use telegram::Command;
use tokio::sync::mpsc;
use teloxide::repls::CommandReplExt;
use log::{info, warn, error};

#[cfg(target_os = "linux")]
use sd_notify::NotifyState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    systemd_journal_logger::JournalLog::new()?.install()?;
    
    #[cfg(not(target_os = "linux"))]
    simple_logger::init_with_level(log::Level::Info).ok();

    let mut current_config = Config::load("autobuddy.toml")?;
    let config_path = "autobuddy.toml";
    
    let (tx, mut rx) = mpsc::channel(100);
    let mut alert_engine = AlertEngine::new(current_config.thresholds.clone(), current_config.buddy_mode);
    let bot = TelegramBot::new(&current_config.telegram.bot_token, current_config.telegram.chat_id);

    // Shared state for Telegram commands
    let tx_for_bot = tx.clone();
    let master_pin = current_config.telegram.master_pin.clone();

    info!("autobuddy daemon starting...");

    // Start heartbeat task (Linux only)
    #[cfg(target_os = "linux")]
    {
        tokio::spawn(async move {
            info!("Starting systemd heartbeat...");
            loop {
                let _ = sd_notify::notify(true, &[NotifyState::Watchdog]);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    }

    // Spawn SysHealth watcher
    if current_config.watchers.syshealth {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let watcher = watchers::syshealth::SysHealthWatcher;
            if let Err(e) = watchers::Watcher::run(&watcher, tx_clone).await {
                error!("SysHealthWatcher failed: {}", e);
            }
        });
    }

    // Spawn Terminal watcher
    if current_config.watchers.terminal {
        let tx_clone = tx.clone();
        let rules = current_config.rules.clone();
        tokio::spawn(async move {
            let watcher = watchers::terminal::TerminalWatcher { rules };
            if let Err(e) = watchers::Watcher::run(&watcher, tx_clone).await {
                error!("TerminalWatcher failed: {}", e);
            }
        });
    }

    // Spawn Sentinel watcher
    if current_config.watchers.sentinel {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let watcher = watchers::sentinel::SentinelWatcher;
            if let Err(e) = watchers::Watcher::run(&watcher, tx_clone).await {
                error!("SentinelWatcher failed: {}", e);
            }
        });
    }

    // Spawn Build watcher
    if current_config.watchers.build {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let watcher = watchers::build::BuildWatcher;
            if let Err(e) = watchers::Watcher::run(&watcher, tx_clone).await {
                error!("BuildWatcher failed: {}", e);
            }
        });
    }

    // Spawn Process watcher
    if current_config.watchers.process {
        let tx_clone = tx.clone();
        let critical_processes = current_config.watchers.critical_processes.clone();
        tokio::spawn(async move {
            let watcher = watchers::process::ProcessWatcher { critical_processes };
            if let Err(e) = watchers::Watcher::run(&watcher, tx_clone).await {
                error!("ProcessWatcher failed: {}", e);
            }
        });
    }

    // Spawn Telegram command listener
    let bot_clone = bot.bot();
    tokio::spawn(async move {
        Command::repl(bot_clone, move |bot, msg, cmd| {
            let tx = tx_for_bot.clone();
            let pin = master_pin.clone();
            async move {
                crate::telegram::TelegramBot::handle_command(bot, msg, cmd, &tx, &pin).await
            }
        }).await;
    });

    #[cfg(target_os = "linux")]
    let _ = sd_notify::notify(true, &[NotifyState::Ready]);

    // Event loop with graceful shutdown
    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                match event {
                    crate::event::Event::ModeChange { name } => {
                        let new_mode = match name.to_lowercase().as_str() {
                            "silent" => crate::config::BuddyMode::Silent,
                            "normal" => crate::config::BuddyMode::Normal,
                            "chatty" => crate::config::BuddyMode::Chatty,
                            _ => {
                                warn!("Unknown mode requested: {}", name);
                                continue;
                            }
                        };
                        
                        info!("Switching buddy mode to: {:?}", new_mode);
                        alert_engine.set_mode(new_mode);
                        current_config.buddy_mode = new_mode;
                        
                        if let Err(e) = current_config.save(config_path) {
                            error!("Failed to persist config change: {}", e);
                            let _ = bot.send_alert(&format!("Failed to save config: {}", e), crate::event::Severity::Warning).await;
                        } else {
                            let _ = bot.send_alert(&format!("Mode persisted: {:?}", new_mode), crate::event::Severity::Info).await;
                        }
                    }
                    crate::event::Event::ProcessCrash { ref name, .. } => {
                        let mut alert_msg = None;
                        if let Some((msg, severity)) = alert_engine.process(&event) {
                            alert_msg = Some((msg, severity));
                        }
                        
                        if current_config.watchers.auto_heal {
                            info!("⚠️ Detected crash of critical process: {}. Attempting auto-heal...", name);
                            let status = std::process::Command::new("systemctl")
                                .arg("restart")
                                .arg(name)
                                .status();
                            
                            match status {
                                Ok(s) if s.success() => {
                                    info!("✅ Successfully restarted {}", name);
                                    let _ = bot.send_alert(&format!("Buddy Intervention: I've successfully restarted {} for you. Watching its recovery...", name), crate::event::Severity::Info).await;
                                    
                                    // Spawn verification
                                    let bot_clone = bot.clone();
                                    let name_clone = name.clone();
                                    tokio::spawn(async move {
                                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                        let check = std::process::Command::new("systemctl")
                                            .arg("is-active")
                                            .arg(&name_clone)
                                            .output();
                                        
                                        match check {
                                            Ok(out) if String::from_utf8_lossy(&out.stdout).trim() == "active" => {
                                                info!("🛡️ Recovery Verified: {} is stable.", name_clone);
                                                let _ = bot_clone.send_alert(&format!("🛡️ Recovery Verified: {} is stable and active.", name_clone), crate::event::Severity::Info).await;
                                            }
                                            _ => {
                                                error!("🚨 Recovery Failed: {} did not stay active.", name_clone);
                                                let _ = bot_clone.send_alert(&format!("🚨 Recovery Failed: {} did not stay active. It might be in a crash loop!", name_clone), crate::event::Severity::Critical).await;
                                            }
                                        }
                                    });
                                }
                                Ok(s) => {
                                    error!("❌ Failed to restart {}: exit code {:?}", name, s.code());
                                    let _ = bot.send_alert(&format!("Buddy Warning: I tried to restart {}, but the system refused. Manual intervention required!", name), crate::event::Severity::Critical).await;
                                }
                                Err(e) => {
                                    error!("❌ Detailed failure restarting {}: {}", name, e);
                                    let _ = bot.send_alert(&format!("Buddy Alert: My attempt to heal {} failed due to a system error: {}", name, e), crate::event::Severity::Critical).await;
                                }
                            }
                        } else if let Some((msg, severity)) = alert_msg {
                            let detailed_msg = format!("{}\n\n💡 Use `/heal [pin] {}` to authorize a restart.", msg, name);
                            if let Err(e) = bot.send_alert(&detailed_msg, severity).await {
                                error!("Failed to send Telegram alert: {}", e);
                            }
                        }
                    }
                    crate::event::Event::HealRequest { ref name } => {
                        info!("💊 Manual heal requested for: {}. Attempting...", name);
                        let status = std::process::Command::new("systemctl")
                            .arg("restart")
                            .arg(name)
                            .status();
                        
                        match status {
                            Ok(s) if s.success() => {
                                info!("✅ Successfully manually restarted {}", name);
                                let _ = bot.send_alert(&format!("Buddy Intervention: Manual heal successful for {}. Service is coming back online.", name), crate::event::Severity::Info).await;
                                
                                // Spawn verification
                                let bot_clone = bot.clone();
                                let name_clone = name.clone();
                                tokio::spawn(async move {
                                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                    let check = std::process::Command::new("systemctl")
                                        .arg("is-active")
                                        .arg(&name_clone)
                                        .output();
                                    
                                    match check {
                                        Ok(out) if String::from_utf8_lossy(&out.stdout).trim() == "active" => {
                                            info!("🛡️ Recovery Verified (Manual): {} is stable.", name_clone);
                                            let _ = bot_clone.send_alert(&format!("🛡️ Recovery Verified: {} has been successfully recovered manually.", name_clone), crate::event::Severity::Info).await;
                                        }
                                        _ => {
                                            error!("🚨 Manual Recovery Failed: {} did not stay active.", name_clone);
                                            let _ = bot_clone.send_alert(&format!("🚨 Manual Recovery Failed: {} failed to stabilize. Further investigation required.", name_clone), crate::event::Severity::Critical).await;
                                        }
                                    }
                                });
                            }
                            Ok(s) => {
                                error!("❌ Failed manual restart of {}: exit code {:?}", name, s.code());
                                let _ = bot.send_alert(&format!("Buddy Warning: Manual heal for {} failed (Code: {:?}). Check system logs.", name, s.code()), crate::event::Severity::Critical).await;
                            }
                            Err(e) => {
                                error!("❌ System error during manual heal of {}: {}", name, e);
                                let _ = bot.send_alert(&format!("Buddy Alert: Manual heal for {} triggered a system error: {}", name, e), crate::event::Severity::Critical).await;
                            }
                        }
                    }
                    _ => {
                        if let Some((msg, severity)) = alert_engine.process(&event) {
                            if let Err(e) = bot.send_alert(&msg, severity).await {
                                error!("Failed to send Telegram alert: {}", e);
                            }
                        }
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Shutting down autobuddy...");
                #[cfg(target_os = "linux")]
                let _ = sd_notify::notify(true, &[NotifyState::Stopping]);
                break;
            }
        }
    }

    Ok(())
}
