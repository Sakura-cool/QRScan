# QRScan

PC 端批量二维码识别工具（Tauri 2 桌面 App）。

一次上传一张或多张图片，自动识别每张图上的**全部**二维码（一张图可包含多个二维码），识别结果以列表展示，支持单条复制与全部复制。图片仅在本地识别，不会上传。

## 跨平台

| 平台 | 架构 | 产物 |
|------|------|------|
| macOS | x86_64 / ARM64（universal） | `.app` / `.dmg` |
| Windows | x86_64 / ARM64 | `.exe`（NSIS）/ `.msi` |
| Ubuntu | x86_64 / ARM64 | `.deb` / `.AppImage` |

## 快速使用

- **普通用户**：下载对应平台的安装包（见 Releases），安装后双击打开即用。
- **开发者**：见下方「本地构建」。

## 本地构建

前置要求：Rust（stable）、系统 WebView 依赖。

```bash
# 安装 Tauri CLI
cargo install tauri-cli --locked

# 以调试模式运行
cargo tauri dev

# 构建发布安装包
cargo tauri build
```

### 各平台系统依赖

- **macOS**：Xcode Command Line Tools。
- **Windows**：Microsoft C++ Build Tools + WebView2 Runtime（Win10/11 一般已内置）。
- **Ubuntu**：`sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`

### 三端三架构 CI

仓库已提供 `.github/workflows/build.yml`，push tag 后自动在 macOS / Windows / Ubuntu 三个平台构建安装包并发布 Release。

## 目录结构

```
├── index.html                # 应用本体：HTML + 内联 CSS + 内联 JS（上传/识别/列表/复制）
├── lib/
│   ├── zxing_reader.js       # zxing-wasm 读者端 JS 绑定（IIFE，全局 ZXingWASM）
│   └── zxing_reader.wasm     # zxing-wasm WebAssembly 引擎（本地文件）
├── src-tauri/                # Tauri 2 壳工程
└── .github/workflows/build.yml
```

## 技术要点

- 识别引擎：zxing-wasm（官方 ZXing WASM），一次调用识别单图全部二维码。
- 隐私：全程本地 WebView 处理，无任何网络请求。
- 轻量：安装包 ~5-10MB（系统 WebView，不内置 Chromium）。
