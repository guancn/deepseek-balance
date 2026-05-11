# DeepSeek Balance — Windows / Windows 版

> A minimal Windows system tray app that displays your DeepSeek API balance in real time.
> 极简 Windows 系统托盘应用，实时显示 DeepSeek API 余额。

## Features / 特性

- **Zero-click balance / 零点击查看余额**: Text rendered directly on the tray icon — always visible. 文字直接渲染在托盘图标上，始终可见。
- **Ultra-light / 极致轻量**: ~3 MB binary, ~5 MB RAM idle. ~3 MB 二进制，空闲内存 ~5 MB。
- **Secure / 安全**: API key encrypted via Windows DPAPI, stored in registry. 通过 Windows DPAPI 加密，存储于注册表。
- **Auto-refresh / 自动刷新**: Polls every 5 minutes (configurable: 1 min – 1 hour). 每 5 分钟轮询（可配置 1 分钟 – 1 小时）。
- **Auto-start / 开机自启**: Registry Run key — starts on every login. 注册表 Run 键，登录即启动。
- **Dark mode aware / 暗色模式适配**: Tray icon colors adapt to Windows theme. 托盘图标颜色自适应 Windows 主题。
- **Error indicator / 错误提示**: ⚠️ appears on network/API errors; auto-recovers. 网络/API 错误时显示 ⚠️，自动恢复。
- **Multi-currency / 多币种**: Prefers CNY, falls back to USD or first available. 优先 CNY，回退 USD。

## Quick Start / 快速开始

### Prerequisites / 前置条件

- Windows 10 or later / Windows 10 及以上
- [Rust](https://rustup.rs) (MSVC toolchain recommended for smallest binary / 推荐 MSVC 工具链以获得最小二进制)

### Build & Run / 构建与运行

```bat
# 1. Open Command Prompt or PowerShell in this directory
#    在此目录打开命令提示符或 PowerShell
# 2. Build / 构建
build.bat

# 3. Run / 运行
.\target\release\deepseek-balance.exe
```

On first launch, the Settings window will appear — paste your DeepSeek API key and click Save.
首次启动弹出设置窗口 — 粘贴 DeepSeek API Key 并点击 Save。

The tray icon will show `🔑` until a key is configured.
未配置 Key 时托盘图标显示 `🔑`。

## Usage / 使用方式

- **Balance / 余额**: Displayed directly on the system tray icon, e.g. `¥12.34`。直接显示在系统托盘图标上。
- **Right-click the icon / 右键图标** → context menu with / 弹出菜单:
  - Balance detail line / 余额详情
  - Refresh Now / 立即刷新
  - Settings... / 设置...
  - Quit / 退出
- **Left-click the icon / 左键图标** → shows tooltip with full balance. 显示完整余额提示。
- **Settings window / 设置窗口**:
  - API Key (with Show/Hide toggle / 支持显示/隐藏切换)
  - Refresh interval dropdown / 刷新间隔下拉选择
  - Enables auto-start on save / 保存时启用开机自启

## Uninstall / 卸载

1. Right-click tray icon → Quit / 右键托盘图标 → 退出
2. Delete `deepseek-balance.exe` / 删除 `deepseek-balance.exe`
3. Remove auto-start key / 移除开机自启:
   ```bat
   reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v DeepSeekBalance /f
   ```
4. (Optional / 可选) Remove stored data / 清除存储的数据:
   ```bat
   reg delete "HKCU\Software\DeepSeekBalance" /f
   ```

## Architecture / 架构

```
deepseek-balance-win/
├── Cargo.toml
├── src/
│   ├── main.rs      # Entry point, hidden window, message loop / 入口，隐藏窗口，消息循环
│   ├── tray.rs       # System tray icon + text-to-icon GDI rendering / 托盘图标 + GDI 文字渲染
│   ├── balance.rs    # DeepSeek API client (ureq) / API 客户端
│   ├── storage.rs    # DPAPI encryption + registry key/value storage / DPAPI 加密 + 注册表存储
│   └── settings.rs   # Settings dialog (Win32) / 设置对话框
├── build.bat
└── README.md
```

Dependencies / 依赖: `windows` (Win32 API), `ureq` (HTTP), `serde`/`serde_json` (JSON).
No runtime required — single static binary. 无需运行时 — 单文件静态二进制。
