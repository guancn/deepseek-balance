# DeepSeek Balance

> Real-time DeepSeek API balance, shown directly in your system tray / menu bar.

**macOS** — number in the menu bar. **Windows** — text rendered on the tray icon.
Both are under 5 MB RAM, poll every 5 min, store the API key encrypted, and auto-start on login.

## Screenshots

```
 macOS menu bar:            Windows system tray:
 [ ¥12.34 ]  🔋  📶  ⏰     [¥12.34]  🔉  🌐  ⏰
```

## Quick Start

### macOS

```bash
cd DeepSeekBalance
./install.sh         # build → install → auto-start → launch
```

Requires macOS 13+ (arm64). Builds with Xcode Command Line Tools — no extra deps.

### Windows

```bat
cd deepseek-balance-win
build.bat            # cargo build --release
.\target\release\deepseek-balance.exe
```

Requires Windows 10+ and [Rust](https://rustup.rs). First launch opens the Settings window for your API key.

## How It Works

- Fetches `GET https://api.deepseek.com/user/balance` with your API key
- Displays `¥12.34` (prefers CNY; falls back to USD)
- Polls on a timer (default 5 min; configurable 1 min – 1 hour)
- Right-click / click the balance → menu: detail, manual refresh, settings, quit

## Security

| Platform | Storage |
|----------|---------|
| macOS    | macOS Keychain (`SecItemAdd`) |
| Windows  | DPAPI (`CryptProtectData`) + registry |

The API key never touches a plaintext file on disk.

## Project Structure

```
dscost/
├── DeepSeekBalance/         # macOS (Swift + AppKit)
│   ├── Sources/
│   │   ├── main.swift
│   │   ├── AppDelegate.swift
│   │   ├── MenuBarController.swift
│   │   ├── BalanceService.swift
│   │   ├── KeychainService.swift
│   │   └── SettingsWindowController.swift
│   └── Resources/Info.plist
├── deepseek-balance-win/    # Windows (Rust + Win32)
│   ├── src/
│   │   ├── main.rs
│   │   ├── tray.rs
│   │   ├── balance.rs
│   │   ├── storage.rs
│   │   └── settings.rs
│   ├── Cargo.toml
│   └── build.bat
├── build.sh
├── install.sh
└── README.md
```
