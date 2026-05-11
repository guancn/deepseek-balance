# DeepSeek Balance — Windows / Windows 版

> 极简 Windows 系统托盘应用，实时显示 DeepSeek API 余额。

[English](README.md)

## 特性

- **零点击查看余额**：文字直接渲染在托盘图标上，始终可见。
- **极致轻量**：~3 MB 二进制，空闲内存 ~5 MB。
- **安全**：通过 Windows DPAPI 加密 API Key，存储于注册表。
- **自动刷新**：每 5 分钟轮询（可配置 1 分钟 – 1 小时）。
- **开机自启**：注册表 Run 键，登录即启动。
- **暗色模式适配**：托盘图标颜色自适应 Windows 主题。
- **错误提示**：网络/API 错误时显示 ⚠️，自动恢复。
- **多币种**：优先 CNY，回退 USD。

## 快速开始

### 前置条件

- Windows 10 及以上
- [Rust](https://rustup.rs)（推荐 MSVC 工具链以获得最小二进制）

### 构建与运行

```bat
# 1. 在此目录打开命令提示符或 PowerShell
# 2. 构建
build.bat

# 3. 运行
.\target\release\deepseek-balance.exe
```

首次启动弹出设置窗口 — 粘贴 DeepSeek API Key 并点击 Save。

未配置 Key 时托盘图标显示 `🔑`。

## 使用方式

- **余额**：直接显示在系统托盘图标上，如 `¥12.34`。
- **右键图标** → 弹出菜单：
  - 余额详情
  - 立即刷新
  - 设置...
  - 退出
- **左键图标** → 显示完整余额提示。
- **设置窗口**：
  - API Key（支持显示/隐藏切换）
  - 刷新间隔下拉选择
  - 保存时启用开机自启

## 卸载

1. 右键托盘图标 → 退出
2. 删除 `deepseek-balance.exe`
3. 移除开机自启：
   ```bat
   reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v DeepSeekBalance /f
   ```
4. （可选）清除存储的数据：
   ```bat
   reg delete "HKCU\Software\DeepSeekBalance" /f
   ```

## 架构

```
deepseek-balance-win/
├── Cargo.toml
├── src/
│   ├── main.rs      # 入口，隐藏窗口，消息循环
│   ├── tray.rs       # 托盘图标 + GDI 文字渲染
│   ├── balance.rs    # API 客户端
│   ├── storage.rs    # DPAPI 加密 + 注册表存储
│   └── settings.rs   # 设置对话框
├── build.bat
└── README.md
```

依赖：`windows` (Win32 API)、`ureq` (HTTP)、`serde`/`serde_json` (JSON)。
无需运行时 — 单文件静态二进制。
