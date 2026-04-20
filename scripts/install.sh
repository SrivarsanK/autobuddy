#!/bin/bash
set -e

# autobuddy installer
# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0;0m'

echo -e "${GREEN}🛡️ autobuddy Installer Starting...${NC}"

# 1. OS Check
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo -e "${RED}Error: autobuddy is designed for Linux systems (systemd required).${NC}"
    exit 1
fi

# 2. Build
echo "🏗️ Building release binary..."
cargo build --release

# 3. Setup directories
echo "📁 Setting up /etc/autobuddy..."
sudo mkdir -p /etc/autobuddy

# 4. Configuration
CONFIG_FILE="/etc/autobuddy/autobuddy.toml"
if [ ! -f "$CONFIG_FILE" ]; then
    echo "⚙️ Initial Configuration Required"
    read -p "Enter Telegram Bot Token: " TELEGRAM_TOKEN
    read -p "Enter Telegram Chat ID (e.g. 12345678): " TELEGRAM_CHAT_ID
    read -p "Enter Master PIN: " MASTER_PIN

    sudo tee "$CONFIG_FILE" > /dev/null <<EOF
[telegram]
bot_token = "$TELEGRAM_TOKEN"
chat_id = $TELEGRAM_CHAT_ID
master_pin = "$MASTER_PIN"

buddy_mode = "normal"

[thresholds]
cpu_pct = 80.0
ram_pct = 90.0
disk_pct = 70.0
alert_cooldown_secs = 300

[watchers]
syshealth = true
process = true
terminal = true
build = true
sentinel = true
critical_processes = ["postgres", "nginx", "docker"]

[rules]
dangerous_commands = ["rm -rf", "mkfs", "dd"]
block_on_match = false
EOF
    echo -e "${GREEN}✅ Configuration created at $CONFIG_FILE${NC}"
fi

# 5. Install binary and service
echo "🚀 Installing binary..."
sudo cp target/release/autobuddy /usr/local/bin/autobuddy
sudo chmod +x /usr/local/bin/autobuddy

echo "📋 Installing systemd unit..."
sudo cp autobuddy.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable autobuddy

echo -e "${GREEN}✨ Installation Complete!${NC}"
echo "Run 'sudo systemctl start autobuddy' to start watching your back."
echo "Use 'journalctl -u autobuddy -f' to see live logs."
