# DeepSeek Balance / DeepSeek 余额显示

> 实时 DeepSeek API 余额，直接显示在系统托盘 / 菜单栏。

[English](README.md)

**macOS** — 数字直接显示在菜单栏。　**Windows** — 文字渲染到托盘图标。

两者均在 5 MB 内存以内，每 5 分钟轮询，加密存储 API Key，开机自启。

## 效果示意

```
 macOS 菜单栏:                  Windows 系统托盘:
 [ ¥12.34 ]  🔋  📶  ⏰          [¥12.34]  🔉  🌐  ⏰
```

## 快速开始

### macOS

```bash
cd DeepSeekBalance
./install.sh         # 编译 → 安装 → 开机自启 → 启动
```

需要 macOS 13+ (arm64)。使用 Xcode Command Line Tools 编译，无额外依赖。

### Windows

```bat
cd deepseek-balance-win
build.bat            # cargo build --release
.\target\release\deepseek-balance.exe
```

需要 Windows 10+ 和 [Rust](https://rustup.rs)。首次启动自动弹出设置窗口，输入 API Key。

## 工作原理

- 使用你的 API Key 调用 `GET https://api.deepseek.com/user/balance`
- 显示 `¥12.34`（优先 CNY，回退 USD）
- 定时轮询（默认 5 分钟，可配置 1 分钟 – 1 小时）
- 右键 / 点击余额 → 菜单：详情、手动刷新、设置、退出

## 安全性

| 平台     | 存储方式 |
|----------|---------|
| macOS    | macOS Keychain (`SecItemAdd`) |
| Windows  | DPAPI (`CryptProtectData`) + 注册表 |

API Key 不会以明文形式写入磁盘。

## 项目结构

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
