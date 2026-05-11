# DeepSeek Balance — Windows

> A minimal Windows system tray app that displays your DeepSeek API balance in real time.

[简体中文](README.zh-CN.md)

## Features

- **Zero-click balance**: Text rendered directly on the tray icon — always visible.
- **Ultra-light**: ~3 MB binary, ~5 MB RAM idle — negligible resource usage.
- **Secure**: API key encrypted via Windows DPAPI, stored in registry.
- **Auto-refresh**: Polls every 5 minutes (configurable: 1 min – 1 hour).
- **Auto-start**: Registry Run key — starts on every login.
- **Dark mode aware**: Tray icon colors adapt to Windows theme.
- **Error indicator**: ⚠️ appears on network/API errors; auto-recovers.
- **Multi-currency**: Prefers CNY, falls back to USD or first available.

## Quick Start

### Prerequisites

- Windows 10 or later
- [Rust](https://rustup.rs) (MSVC toolchain recommended for smallest binary)

### Build & Run

```bat
# 1. Open Command Prompt or PowerShell in this directory
# 2. Build
build.bat

# 3. Run
.\target\release\deepseek-balance.exe
```

On first launch, the Settings window will appear — paste your DeepSeek API key and click Save.

The tray icon will show `🔑` until a key is configured.

## Usage

- **Balance**: Displayed directly on the system tray icon, e.g. `¥12.34`.
- **Right-click the icon** → context menu with:
  - Balance detail line
  - Refresh Now
  - Settings...
  - Quit
- **Left-click the icon** → shows tooltip with full balance.
- **Settings window**:
  - API Key (with Show/Hide toggle)
  - Refresh interval dropdown
  - Enables auto-start on save

## Uninstall

1. Right-click tray icon → Quit
2. Delete `deepseek-balance.exe`
3. Remove auto-start key:
   ```bat
   reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v DeepSeekBalance /f
   ```
4. (Optional) Remove stored data:
   ```bat
   reg delete "HKCU\Software\DeepSeekBalance" /f
   ```

## Architecture

```
deepseek-balance-win/
├── Cargo.toml
├── src/
│   ├── main.rs      # Entry point, hidden window, message loop
│   ├── tray.rs       # System tray icon + text-to-icon GDI rendering
│   ├── balance.rs    # DeepSeek API client (ureq)
│   ├── storage.rs    # DPAPI encryption + registry key/value storage
│   └── settings.rs   # Settings dialog (Win32)
├── build.bat
└── README.md
```

Dependencies: `windows` (Win32 API), `ureq` (HTTP), `serde`/`serde_json` (JSON).
No runtime required — single static binary.
