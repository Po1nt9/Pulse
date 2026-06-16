# Changelog

所有 Pulse 项目的显著变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，版本号遵循 [SemVer](https://semver.org/lang/zh-CN/)。

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

## [Unreleased]

### 计划

- 历史数据持久化与导出
- 更多图表类型（饼图、折线图）
- 多语言支持
- 自动更新检查
- 快捷键支持
