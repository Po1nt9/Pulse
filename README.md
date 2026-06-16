# Pulse

> 一个轻量级的桌面托盘应用，用于监控 LLM API 账户余额与用量趋势。

![version](https://img.shields.io/badge/version-0.1.0-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2-24C8D8?logo=tauri)
![React](https://img.shields.io/badge/React-18-61DAFB?logo=react)
![License](https://img.shields.io/badge/license-MIT-green)

## 功能特性

- **多供应商支持**：DeepSeek、OpenAI、Anthropic、OpenRouter、自定义 OpenAI-compatible provider
- **实时余额监控**：托盘图标实时展示账户余额状态
- **用量趋势图表**：基于 Recharts 的用量可视化分析
- **安全密钥存储**：API Key 通过操作系统 keychain 安全存储，不在配置文件中明文保存
- **深色毛玻璃 UI**：小尺寸、高信息密度的现代化界面
- **自动刷新**：可配置的自动刷新间隔，保持数据实时性
- **通知提醒**：余额低于阈值时自动推送系统通知

## 技术栈

- **桌面壳**：Tauri v2
- **后端**：Rust、tokio、reqwest、serde、thiserror、keyring
- **前端**：React 18、TypeScript、Vite
- **状态管理**：Zustand（UI state）+ TanStack Query（server state）
- **UI 组件**：Tailwind CSS、Recharts、lucide-react
- **安全**：OS keychain 存储 API Key

## 安装

### 前提条件

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- Windows / macOS / Linux

### 开发环境

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI
npm install -g @tauri-apps/cli

# 启动开发服务器
npm run tauri:dev
```

### 构建生产版本

```bash
# 构建前端 + Tauri 应用
npm run tauri:build
```

构建产物位于 `src-tauri/target/release/` 目录。

## 项目结构

```
.
├── src/                          # 前端源码
│   ├── components/               # React 组件
│   ├── hooks/                    # React Query Hooks
│   ├── store/                    # Zustand Store
│   ├── types/                    # TypeScript 类型定义
│   ├── utils/                    # 工具函数
│   ├── App.tsx                   # 应用根组件
│   └── main.tsx                  # 入口文件
├── src-tauri/                    # Tauri / Rust 后端
│   ├── src/
│   │   ├── commands/             # Tauri Commands
│   │   ├── providers/            # Provider Adapters
│   │   ├── error.rs              # 错误处理
│   │   ├── config.rs             # 配置管理
│   │   ├── http.rs               # HTTP Client
│   │   ├── keychain.rs           # Keychain 存储
│   │   ├── tray.rs               # 托盘图标管理
│   │   ├── window.rs             # 窗口管理
│   │   └── lib.rs                # 库入口
│   ├── icons/                    # 应用图标
│   └── Cargo.toml                # Rust 依赖
├── docs/                         # 文档
└── package.json                  # 前端配置
```

## 配置说明

首次启动时，Pulse 会在系统配置目录创建默认配置文件。配置通过前端设置面板修改，API Key 通过系统 keychain 安全存储。

### 支持的供应商

| 供应商 | 余额 API | 用量 API |
|--------|----------|----------|
| DeepSeek | ✅ | ✅ |
| OpenAI | ✅ | ✅ |
| Anthropic | ✅ | ✅ |
| OpenRouter | ✅ | ✅ |
| Custom | ✅ | ✅ |

## 开发规范

- 代码标识符、文件名、模块名使用英文
- UI 文案、错误提示、用户文档使用中文
- API Key 不写入配置文件、日志或测试数据
- 使用 Tauri v2 API，不引入 v1 写法
- async Rust 中避免持锁跨 `.await`

## 安全与隐私

- API Key 仅通过 OS keychain 存取
- 日志、错误提示中不暴露密钥
- 不添加遥测、埋点或远程上报
- 外部请求仅访问用户配置的供应商 API

## 许可证

[MIT License](LICENSE)

## 更新日志

详见 [CHANGELOG.md](CHANGELOG.md)
