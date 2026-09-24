# QRScan

PC 端批量二维码识别工具（Tauri 2 桌面 App）。

一次上传一张或多张图片，自动识别每张图上的**全部**二维码（一张图可包含多个二维码），识别结果以列表展示，支持单条复制与全部复制。图片仅在本地识别，不会上传。

## 跨平台与产物

| 平台 | 架构 | 产物 | 产出方式 |
|------|------|------|---------|
| macOS | x86_64 + ARM64（universal） | `.dmg` / `.app.zip` | 本机构建 + CI |
| Windows | x86_64 | `.msi` / `.exe`（NSIS） | CI（GitHub Actions） |
| Ubuntu | x86_64 | `.deb` / `.AppImage` | CI（GitHub Actions） |

### 产物获取

1. **GitHub Releases**（推荐，全平台）：push `v*` tag 自动三端构建并发版 → https://github.com/Sakura-cool/QRScan/releases
2. **本地 .package 目录**（macOS 双架构）：`workspace/QRScan/.package/{arm64,x86_64}/`，命名 `QRScan-{arch}.dmg` / `.app.zip`（规范同 BatKill）

> ARM64 版 Windows/Ubuntu 安装包：macOS 交叉打包不可行（Tauri 需各平台原生工具链），如需请在三端 CI runner 上补充矩阵项。

## 快速使用

- **普通用户**：下载对应平台安装包（GitHub Releases），macOS 也可直接用 `.package/` 里的 `.app.zip` 解压即用。
- **开发者**：见下方「本地构建」。

## 本地构建

前置要求：Rust（stable）、系统 WebView 依赖、Tauri CLI。

```bash
# 安装 Tauri CLI
cargo install tauri-cli --locked

# 以调试模式运行
cargo tauri dev

# 构建当前平台发布安装包
cargo tauri build

# macOS 双架构（universal）
cargo tauri build --target universal-apple-darwin

# 产物落位到 .package/{arch}/（按 BatKill 规范）
cp -r src-tauri/target/{arch}/release/bundle/*/* .
```

### 各平台系统依赖

- **macOS**：Xcode Command Line Tools。
- **Windows**：Microsoft C++ Build Tools + WebView2 Runtime（Win10/11 一般已内置）。
- **Ubuntu**：`sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`

### 三端 CI

`.github/workflows/build.yml`：push `v*` tag 自动执行完整流水线（`main` 分支推送不触发，仅 tag 触发）：

1. **build**：macOS universal / Windows x64 / Ubuntu x64 三端构建安装包并发布 GitHub Release（Node 24 运行时）
2. **semgrep**：三通道扫描（自定义规则集 `.github/semgrep/qrscan-rules.yml` + `security-audit` + `rust`），命中即失败，SARIF 上传 GitHub Code Scanning
3. **sonarqube**：配置 `SONAR_TOKEN` + `SONAR_HOST_URL` secrets 后自动启用（内网 SonarQube 需配置可达地址），未配置时跳过

## 静态扫描

| 工具 | 配置 | 说明 |
|------|------|------|
| Semgrep | `.github/semgrep/qrscan-rules.yml` + `p/security-audit` + `p/rust` | 三通道，CI 命中即失败；本地复扫：`semgrep scan --config .github/semgrep/qrscan-rules.yml --metrics off .` |
| SonarQube | `sonar-project.properties`（本地容器 `sonarqube_se` :9001） | 本地手动：`docker-compose run --rm sonar-scanner-cli -Dsonar.projectBaseDir=/opt/scan -Dsonar.login=<token>` |

## Git 双远端（同 BatKill）

```
gitea    http://localhost:3002/admin/qrscan.git          # 内网主仓库
origin   git@github.com:Sakura-cool/QRScan.git           # 对外 + CI
```

## 更新签名密钥

> 本仓库为**开源**（MIT，无保密要求），签名密钥已随仓库公开，**仅本地签名打包时需要**；CI 构建无需手动处理密钥。

| 项 | 说明 |
|----|------|
| 存放位置 | 仓库 `keys/`（`qrscan.key` 为 age 加密格式，无密码不可用） |
| 文件 | `keys/qrscan.key`（签名私钥，age 加密）+ `keys/qrscan.key.pub`（公钥，已内嵌 `tauri.conf.json` pubkey） |
| 本地签名构建 | `bash scripts/fetch-keys.sh` 后 `TAURI_SIGNING_PRIVATE_KEY=$(cat /tmp/qrscan-keys/qrscan.key) cargo tauri build`（解开 age 密钥的密码联系仓库维护者获取） |
| CI | GitHub Secrets：`TAURI_SIGNING_PRIVATE_KEY` / `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`（自动化签名，协作者无需接触） |

## 目录结构

```
├── ui/
│   ├── index.html            # 应用本体：HTML + 内联 CSS + 内联 JS（上传/识别/列表/复制）
│   └── lib/
│       ├── zxing_reader.js   # zxing-wasm 读者端 JS 绑定（IIFE，全局 ZXingWASM）
│       └── zxing_reader.wasm # zxing-wasm WebAssembly 引擎（本地文件）
├── src-tauri/                # Tauri 2 壳工程
├── keys/                     # 更新签名密钥（公开，本地打包才需要）
├── scripts/                  # fetch-keys.sh 等辅助脚本
├── .package/                 # macOS 本地打包产物（{arm64,x86_64}/）
└── .github/workflows/build.yml
```

## 技术要点

- 识别引擎：zxing-wasm（官方 ZXing WASM），一次调用识别单图全部二维码。
- 隐私：全程本地 WebView 处理，无任何网络请求。
- 轻量：安装包 ~2-4MB（系统 WebView，不内置 Chromium）。
