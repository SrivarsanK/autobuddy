use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use crate::event::Severity;
use sysinfo::System;
use tokio::sync::mpsc;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "check if buddy is awake.")]
    Ping,
    #[command(description = "get system status and uptime.")]
    Status,
    #[command(description = "change buddy personality mode.", parse_with = "split")]
    Mode { pin: String, name: String },
    #[command(description = "manually authorize a service restart.", parse_with = "split")]
    Heal { pin: String, name: String },
}

#[derive(Clone)]
pub struct TelegramBot {
    bot: Bot,
    chat_id: ChatId,
}

impl TelegramBot {
    pub fn new(token: &str, chat_id: i64) -> Self {
        Self {
            bot: Bot::new(token),
            chat_id: ChatId(chat_id),
        }
    }

    pub fn bot(&self) -> Bot {
        self.bot.clone()
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id
    }

    pub async fn send_alert(&self, message: &str, severity: Severity) -> anyhow::Result<()> {
        let prefix = match severity {
            Severity::Critical => "🚨 CRITICAL: ",
            Severity::Warning => "⚠️ WARNING: ",
            Severity::Info => "ℹ️ INFO: ",
        };

        let full_msg = format!("{}{}", prefix, message);
        self.bot.send_message(self.chat_id, full_msg).await?;
        Ok(())
    }

    pub async fn handle_command(
        bot: Bot, 
        msg: Message, 
        cmd: Command, 
        tx: &mpsc::Sender<crate::event::Event>,
        master_pin: &str,
    ) -> ResponseResult<()> {
        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Command::Ping => {
                bot.send_message(msg.chat.id, "I'm awake and watching your back! 🫡").await?;
            }
            Command::Status => {
                let mut sys = System::new_all();
                sys.refresh_all();
                
                let uptime_secs = System::uptime();
                let days = uptime_secs / 86400;
                let hours = (uptime_secs % 86400) / 3600;
                let minutes = (uptime_secs % 3600) / 60;
                
                let status_msg = format!(
                    "📊 *System Status*\n\n\
                    ⏱ *Uptime:* {}d {}h {}m\n\
                    🧠 *RAM:* {}/{} MB",
                    days, hours, minutes,
                    sys.used_memory() / 1024 / 1024,
                    sys.total_memory() / 1024 / 1024
                );
                
                bot.send_message(msg.chat.id, status_msg).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
            }
            Command::Mode { pin, name } => {
                if pin != master_pin {
                    bot.send_message(msg.chat.id, "⛔ *Invalid Master PIN\\.*\nAccess denied\\.").parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                } else {
                    let _ = tx.send(crate::event::Event::ModeChange { name: name.clone() }).await;
                    bot.send_message(msg.chat.id, format!("Buddy personality mode update requested: *{}*\\.\\.\\.", name))
                       .parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                }
            }
            Command::Heal { pin, name } => {
                if pin != master_pin {
                    bot.send_message(msg.chat.id, "⛔ *Invalid Master PIN\\.*\nHealing authority denied\\.").parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                } else {
                    let _ = tx.send(crate::event::Event::HealRequest { name: name.clone() }).await;
                    bot.send_message(msg.chat.id, format!("Buddy Intervention: Healing authorization received for *{}*\\.", name))
                       .parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                }
            }
        };
        Ok(())
    }
}
