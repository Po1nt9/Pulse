# Changelog

所有 Pulse 项目的显著变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，版本号遵循 [SemVer](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 新增

- ESLint 9 flat config 与 Prettier 配置
- CI lint job、Release workflow、Dependabot
- Issue/PR 模板、CODEOWNERS、LICENSE、.editorconfig
- AGENTS.md 扩充项目开发规范

### 修复

- NotificationToast 重复清理与孤儿条目泄漏
- useSettings 冗余的 invalidateQueries 调用
- window.rs async 路径中的 blocking_read
- error.rs serde 序列化简化
- package.json lint 脚本 ESLint 9 兼容性
- CODEOWNERS/dependabot/模板中的仓库 owner 引用（pulse-app → Po1nt9）

### 变更

- refresh_all_balances 改用 JoinSet 实现并发
- React Query 默认参数针对 Tauri 调优
- ci.yml test job 不再安装未使用的 npm 依赖

## [0.1.0] - 2026-06-16

### 新增

- 初始版本发布
- 支持 DeepSeek、OpenAI、Anthropic、OpenRouter、自定义 OpenAI-compatible provider 余额查询
- 支持多供应商用量趋势图表展示
- 系统托盘入口，支持 Windows/macOS/Linux
- 深色毛玻璃风格 UI，小尺寸高信息密度设计
- API Key 通过 OS keychain 安全存储
- 可配置的自动刷新间隔与余额预警阈值
- 余额不足时推送系统通知
- 供应商启用/禁用管理
- 设置面板支持全局配置调整

### 技术实现

- Tauri v2 桌面应用框架
- Rust 后端：tokio + reqwest + serde + keyring
- React 18 + TypeScript + Vite 前端
- Zustand 管理 UI state，TanStack Query 管理 server state
- Tailwind CSS + Recharts + lucide-react 构建 UI
- 统一 provider trait 抽象不同供应商 API
