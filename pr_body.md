## 变更内容

本次 PR 完成了项目的全面优化重构和 GitHub 仓库整理：

### 项目重构
- **Rust 后端优化**
  - 移除 `async_trait` 依赖，使用原生 async fn in trait
  - 优化 `refresh_all_balances` 并发性能
  - 简化 `error.rs` 的 serde 实现
  - 修复 `window.rs` 的 `blocking_read` 问题
  - 新增 23 个单元测试（共 70 个测试全部通过）

- **前端优化**
  - 修复 `NotificationToast` 双重清理逻辑
  - 修复 `useSettings` 冗余的 `invalidateQueries` 调用
  - 优化 React Query 配置

### GitHub 仓库整理
- 创建完整的 Issue 模板（Bug 报告、功能请求）
- 创建 PR 模板
- 配置 12 个标签（类型、状态、优先级）
- 配置 Dependabot 自动依赖更新
- 优化 CI/CD workflow（添加 lint job、并行执行）
- 创建 Release workflow
- 完善 AGENTS.md 规范文档（从 181 行扩展至 1180 行）
- 添加项目配置文件（LICENSE、.editorconfig、.prettierrc、.dockerignore 等）

## 关联 Issue

Closes #1

## 变更类型

- [x] 🐛 Bug 修复（修复问题的非破坏性更改）
- [x] ✨ 新功能（添加功能的非破坏性更改）
- [ ] 💥 破坏性更改（会导致功能不兼容的修复或功能）
- [x] 📚 文档更新
- [x] ♻️ 重构（不改变功能的代码更改）
- [x] ⚡️ 性能优化
- [x] 🧪 测试相关
- [x] 🔧 构建/工具链更新
- [ ] 📦 其他（请描述）

## 验证结果

### Rust 后端
```bash
cargo check: ✓ 通过
cargo test: ✓ 70 passed; 0 failed
```

### 前端
```bash
npm run lint: ✓ 通过（0 warnings）
npm run build: ✓ 构建成功（7.58s）
```

### GitHub 仓库
- ✓ 12 个标签已同步到 GitHub
- ✓ CI workflow 已优化
- ✓ Release workflow 已创建
- ✓ Dependabot 已配置

## 影响范围

### 前端
- 组件：NotificationToast、SettingsPanel
- Hooks：useSettings
- 配置：main.tsx（React Query 配置优化）

### 后端
- 模块：providers、commands、error、window
- 依赖：移除 async-trait
- 测试：新增 23 个单元测试

### 工程化
- CI/CD：添加 lint job、优化缓存策略
- 规范：完善 AGENTS.md、添加代码审查清单
- 配置：添加 ESLint、Prettier、EditorConfig

## Checklist

- [x] 我的代码遵循项目的代码风格
- [x] 我已经添加了必要的测试
- [x] 所有新测试和现有测试都通过了
- [x] 我已经更新了相关文档
- [x] 我的更改没有生成新的警告
- [ ] 我已经更新了 CHANGELOG.md（如果适用）

## 补充说明

本次重构遵循了项目的 AGENTS.md 规范，所有改动都经过了充分的测试验证。GitHub 仓库的配置已完善，为后续的协作开发奠定了基础。
