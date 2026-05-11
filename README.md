# DeepSeek Balance / DeepSeek 余额显示

> Real-time DeepSeek API balance, shown directly in your system tray / menu bar.
> 实时 DeepSeek API 余额，直接显示在系统托盘 / 菜单栏。

**macOS** — number in the menu bar.　**Windows** — text rendered on the tray icon.
**macOS** — 数字直接显示在菜单栏。　**Windows** — 文字渲染到托盘图标。

Both are under 5 MB RAM, poll every 5 min, store the API key encrypted, and auto-start on login.
两者均在 5 MB 内存以内，每 5 分钟轮询，加密存储 API Key，开机自启。

## Screenshots / 效果示意

```
 macOS menu bar / 菜单栏:         Windows system tray / 系统托盘:
 [ ¥12.34 ]  🔋  📶  ⏰          [¥12.34]  🔉  🌐  ⏰
```

## Quick Start / 快速开始

### macOS

```bash
cd DeepSeekBalance
./install.sh         # build → install → auto-start → launch
                     # 编译 → 安装 → 开机自启 → 启动
```

Requires macOS 13+ (arm64). Builds with Xcode Command Line Tools — no extra deps.
需要 macOS 13+ (arm64)。使用 Xcode Command Line Tools 编译，无额外依赖。

### Windows

```bat
cd deepseek-balance-win
build.bat            # cargo build --release
.\target\release\deepseek-balance.exe
```

Requires Windows 10+ and [Rust](https://rustup.rs). First launch opens the Settings window for your API key.
需要 Windows 10+ 和 [Rust](https://rustup.rs)。首次启动自动弹出设置窗口，输入 API Key。

## How It Works / 工作原理

- Fetches `GET https://api.deepseek.com/user/balance` with your API key
  使用你的 API Key 调用 `GET https://api.deepseek.com/user/balance`
- Displays `¥12.34` (prefers CNY; falls back to USD)
  显示 `¥12.34`（优先 CNY，回退 USD）
- Polls on a timer (default 5 min; configurable 1 min – 1 hour)
  定时轮询（默认 5 分钟，可配置 1 分钟 – 1 小时）
- Right-click / click the balance → menu: detail, manual refresh, settings, quit
  右键 / 点击余额 → 菜单：详情、手动刷新、设置、退出

## Security / 安全性

| Platform / 平台 | Storage / 存储方式 |
|----------|---------|
| macOS    | macOS Keychain (`SecItemAdd`) |
| Windows  | DPAPI (`CryptProtectData`) + registry / 注册表 |

The API key never touches a plaintext file on disk.
API Key 不会以明文形式写入磁盘。

## Project Structure / 项目结构

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
