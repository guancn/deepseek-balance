#!/bin/bash
set -euo pipefail

APP_NAME="DeepSeekBalance"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/.build"
APP_PATH="$BUILD_DIR/$APP_NAME.app"
DEST="/Applications/$APP_NAME.app"
LAUNCH_AGENT_DIR="$HOME/Library/LaunchAgents"
PLIST="$LAUNCH_AGENT_DIR/com.deepseek.balance.plist"

# ── Build if not already built ──────────────────────────────────────
if [ ! -d "$APP_PATH" ]; then
    echo "==> App not built yet. Running build.sh first..."
    "$SCRIPT_DIR/build.sh"
fi

# ── Install app bundle ──────────────────────────────────────────────
echo "==> Installing to /Applications..."
if [ -d "$DEST" ]; then
    echo "    Removing existing installation..."
    rm -rf "$DEST"
fi
cp -R "$APP_PATH" "$DEST"

# ── Create LaunchAgent for auto-start on login ──────────────────────
echo "==> Creating LaunchAgent for auto-start on login..."
mkdir -p "$LAUNCH_AGENT_DIR"

cat > "$PLIST" << 'PLISTEOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.deepseek.balance</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Applications/DeepSeekBalance.app/Contents/MacOS/DeepSeekBalance</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>ProcessType</key>
    <string>Background</string>
</dict>
</plist>
PLISTEOF

# Reload LaunchAgent
launchctl unload "$PLIST" 2>/dev/null || true
launchctl load "$PLIST"

# ── Launch now ──────────────────────────────────────────────────────
echo "==> Launching DeepSeek Balance..."
open "$DEST"

echo ""
echo "✅ DeepSeek Balance installed and running."
echo "   It will auto-start on login."
echo ""
echo "   Look for the balance in your menu bar (top-right)."
echo "   On first launch, the Settings window will appear — enter your API key."
