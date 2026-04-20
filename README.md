# 🤖 autobuddy

![autobuddy Hero](assets/readme/hero.png)

> **The Autonomous System Guardian.**  
> Never let your production services stay down again. `autobuddy` is a high-performance Rust daemon that monitors your system and heals it when things go wrong—all while keeping you in the loop via Telegram.

---

## ✨ Features

### 🛡️ Real-Time Monitoring
Continuously tracks CPU, RAM, Disk usage, and the health of critical processes. Built for speed and reliability using Rust.

### 💊 Autonomous Healing
![Self-Healing Illustration](assets/readme/healing.png)
When a critical service crashes, `autobuddy` doesn't just alert you—it **fixes it**. It automatically attempts to restart the service and verifies its stability in the background.

### 📲 Remote Governance
Manage your machine from anywhere.
- **Interactive Alerts**: Get notified on Telegram with actionable choices.
- **Manual Oversight**: Use `/heal` to authorize restarts for specific services.
- **Live Status**: Send `/status` to see real-time system metrics.

### 🔒 Secure by Design
Sensitive credentials stay in your `.env` file, and manual interventions require a **Master PIN** to prevent unauthorized access.

---

## 🚀 Quick Start

### 1. Prerequisites
- **Rust Toolchain**: `latest stable`
- **Linux Environment**: (Optimized for `systemd`)

### 2. Installation
Clone the repo and build:
```bash
git clone https://github.com/Arunavo/autobuddy.git
cd autobuddy
cargo build --release
```

### 3. Configuration
Rename `.env.example` (or create a `.env`) and add your Telegram bot token:
```env
TELEGRAM_BOT_TOKEN=your_token_here
MASTER_PIN=your_custom_pin
```

Modify `autobuddy.toml` to define your critical processes:
```toml
[watchers]
critical_processes = ["postgres", "nginx"]
auto_heal = true
```

---

## 🛠️ Architecture

`autobuddy` is architected for modularity:
- **AlertEngine**: Smart filtering and cooldowns to prevent alert fatigue.
- **Watchers**: Modular system for process, disk, and CPU monitoring.
- **TelegramBot**: An interactive command-and-control interface.

---

## 📈 Roadmap

- [x] **Milestone 1**: Core Engine & Resource Monitoring
- [x] **Milestone 2**: Process Watchers
- [x] **Milestone 3**: Systemd Integration
- [x] **Milestone 4**: Remote Commands (/ping, /status)
- [x] **Milestone 5**: The Healer (Auto-healing & Manual Interventions)
- [ ] **Milestone 6**: History & Analytics Dashboard

---

## 📄 License
This project is licensed under the MIT License.
