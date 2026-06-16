# AGENTS.md

本文件是 Pulse 项目的项目级 Agent 工作规范。所有在本仓库中工作的 agent 应优先阅读并遵守本文件；若用户在对话中给出更具体的要求，以用户要求为准。

## 项目定位

Pulse 是一个 GitHub 项目，也是一个 Tauri v2 托盘应用，用于监控 LLM API 账户余额与用量趋势。目标版本 V0.1.0 支持 DeepSeek、OpenAI、Anthropic、OpenRouter 和自定义 OpenAI-compatible provider。

核心体验：

- Windows/macOS/Linux 桌面托盘入口。
- 深色毛玻璃、小尺寸、高信息密度 UI。
- 安全存储 API Key，不在配置文件中明文保存密钥。
- 统一 provider trait，把不同供应商的余额/用量查询抽象成一致接口。

## 默认语言与沟通规范

- 默认使用中文与用户沟通，包括计划、说明、总结、风险提示和验证结果。
- 保留技术名词英文原文，例如 Tauri、React Query、Zustand、provider、command、hook、trait。
- 用户可见产品文案默认中文；品牌名、API 名称、协议名保持原文。
- 报告结果要明确：做了什么、改了哪些文件、验证了什么、是否失败。
- 不要声称“完成”“通过”“已修复”，除非已经运行相应验证并看到成功结果。

## 代码与文案语言规范

- 代码标识符、文件名、模块名、类型名、函数名使用英文。
- Rust/TypeScript 注释遵循周围代码密度；不要为显而易见的代码添加冗余注释。
- UI 文案、错误提示、README、CHANGELOG、用户文档优先中文。
- Tauri command 名称、前后端字段名与现有计划保持一致，优先使用 snake_case payload 与 Rust 参数对齐。
- 不把中文用于 API 字段、Rust enum variant、TypeScript type key 等程序接口，除非这是明确的用户可见文案。

## 项目技术栈

- 桌面壳：Tauri v2
- 后端：Rust、tokio、reqwest、serde、thiserror、keyring、directories
- 前端：React 18、TypeScript、Vite
- 状态：Zustand 管 UI state，TanStack Query 管 Tauri/server state
- UI：Tailwind CSS、Recharts、lucide-react、深色毛玻璃视觉系统
- 安全：OS keychain 存储 API Key

## 项目级 Skill 使用规范

本仓库的 `.agents/skills/` 目录包含项目级技术参考。处理对应领域任务时，应优先参考相关项目级 skill 的技术约束，并以本文件、实现计划和当前代码为最终依据。

- `tauri-v2`：Tauri v2 配置、capabilities、tray、window、plugin、build。
- `rust-testing`：Rust 单元测试、async 测试、serde/error/config/provider 测试。
- `typescript-best-practices`：TypeScript strict、类型建模、React hook 类型、避免 `any`。
- `react-state-management`：Zustand 与 React Query 状态边界、cache invalidation、selector。
- `ui-design-system`：design token、组件一致性、Tailwind/组件系统化。
- `github-actions`：CI/release workflow、artifact、构建流水线。
- `testing-tauri-apps`：Tauri 应用测试策略、前后端边界验证。

项目级 skill 只提供领域技术参考，不覆盖用户当前指令、本文件规范、实现计划或实际代码状态。

## 必读文档

在实现计划相关任务前，优先阅读：

- `docs/superpowers/plans/2026-06-16-pulse-v0.1.0-reviewed.md`
- `preview.html`

实现时以 reviewed plan 为主线；设计还原时参考 preview。若计划内容与本文件或用户最新指令冲突，以用户最新指令和本文件为准。

## 实现计划执行规范

- 以 `docs/superpowers/plans/2026-06-16-pulse-v0.1.0-reviewed.md` 为主要实施来源。
- 按任务顺序推进，除非用户明确要求调整顺序。
- 每完成一个主要任务，更新对应 checklist 或在回复中说明完成状态。
- 不跳过验证步骤；如果某个验证无法运行，要说明原因。
- 不在未经确认的情况下执行破坏性操作，例如删除大量文件、重置目录、强制覆盖用户已有文件。
- 若实现计划与当前实际文件冲突，先报告差异，再提出处理方案。
- 计划文档中的 git / gh 命令仅作为收尾操作参考，不构成自动执行授权；除非用户在当前上下文中明确要求，否则不得执行 commit、push、tag、创建 PR 或发布 release。

## Rust / Tauri 后端规范

- 使用 Tauri v2 API，不引入 Tauri v1 写法。
- 使用 `WebviewWindow`、`TrayIconBuilder`、v2 `capabilities` 与 `@tauri-apps/api/core` 对应的 command 边界。
- async Rust 中避免持锁跨 `.await`；读取配置后 clone 必要数据并释放锁。
- 配置读写使用 `directories` 指定项目目录；阻塞 I/O 在 async 路径中使用 `spawn_blocking`。
- API Key 只能通过 OS keychain 存取，不写入普通配置文件、日志、测试快照或文档示例。
- `AppError` 保持结构化序列化，便于前端展示中文友好错误。
- provider 实现应通过 trait 统一暴露能力，避免在 command 层写供应商特例逻辑。

## React / TypeScript 前端规范

- 启用并尊重 TypeScript strict；避免 `any`，优先使用明确 interface/type。
- Zustand 只管理 UI state：当前 panel、selected provider、modal、toast 等。
- TanStack Query 管 Tauri command/server state：providers、balance、usage、settings、keychain 状态。
- mutation 成功后必须正确更新或 invalidate query cache。
- Tauri invoke wrapper 要有类型参数与统一错误处理。
- 组件 props 要清晰；可复用 UI 单元独立成组件。
- 图表、列表、设置表单要处理 loading、empty、error 状态。
- icon-only button 必须有 `aria-label`。

## UI / 设计还原规范

- 默认视觉方向：深色、毛玻璃、小尺寸、高信息密度、柔和动效。
- 优先参考 `preview.html` 的布局、色彩、动效和层级。
- 数据字体优先使用 JetBrains Mono；正文优先 Inter / system font / PingFang SC。
- 状态颜色保持一致：正常、警告、危险分别使用明确 token。
- 进度条、状态点、图表 bar、toast、settings list 应共享设计 token。
- 不使用模板化、过度明亮或与托盘小窗不匹配的大屏后台风格。
- UI 文案简洁中文，避免长句挤占小窗口空间。

## 安全与隐私规范

- 不泄露真实 API Key、token、凭证、系统路径中的隐私信息或用户个人数据。
- 日志、错误提示、toast、README 示例不得包含真实密钥。
- 处理 API Key 时避免打印值；只允许显示已配置/未配置或掩码形式。
- 外部请求只访问实现计划中明确的供应商 API 或用户确认的自定义 endpoint。
- 不添加遥测、埋点、远程上报，除非用户明确要求并批准。

## 测试与验证规范

完成声明前，根据变更范围运行相应验证：

- TypeScript/前端：`npm run build`
- Rust 后端：`cargo check` 或 `cargo test`（在 `src-tauri` 下）
- Tauri 打包：`npm run tauri:build`
- 开发态验证：`npm run tauri:dev`（需要用户允许 GUI 运行时再执行）

如果项目尚未初始化或依赖尚未安装，说明当前无法运行的原因，并给出下一步。

测试优先覆盖：

- Rust error serde roundtrip。
- config default/read/write。
- provider balance parsing 与错误状态。
- TypeScript format/color/utils。
- Zustand store 行为。
- React Query hook command 参数与 cache invalidation。

## 代码格式化规范

### Rust 代码格式化

- 使用 `rustfmt` 进行代码格式化，项目使用默认配置（无自定义 `rustfmt.toml`）
- 提交前必须运行 `cargo fmt --all` 确保代码格式统一
- 建议在 CI 中添加 `cargo fmt -- --check` 步骤（当前 CI 尚未配置，后续应补充）
- 关键格式化规则：
  - 最大行宽 100 字符
  - 使用 4 空格缩进
  - 函数参数列表超过行宽时自动换行
  - `use` 语句按模块分组排序
  - 结构体字段对齐保持一致

示例：
```bash
# 在 src-tauri 目录下运行
cd src-tauri
cargo fmt --all
cargo fmt -- --check  # CI 检查用
```

### TypeScript/前端代码格式化

- 项目使用 ESLint 进行代码检查和格式化，配置通过 `package.json` 中的 `lint` 脚本管理
- 提交前必须运行 `npm run lint` 确保代码质量
- ESLint 配置要求：
  - 最大警告数 0（`--max-warnings 0`）
  - 报告未使用的禁用指令（`--report-unused-disable-directives`）
  - 检查范围：`src` 目录下的 `.ts` 和 `.tsx` 文件
- 建议配置编辑器保存时自动运行 ESLint 修复
- 代码风格一致性优先于个人偏好，遵循项目现有风格

示例：
```bash
npm run lint           # 检查代码
npm run lint -- --fix  # 自动修复可修复的问题
```

### 配置文件格式化

- `package.json`、`Cargo.toml`、`tsconfig.json` 等配置文件保持标准 JSON/TOML 格式
- 使用 2 空格缩进（JSON）或 4 空格缩进（TOML）
- 注释使用英文，保持简洁明了
- 配置项按逻辑分组，添加空行分隔不同类别

## Commit Message 规范

### Conventional Commits 格式

所有 commit message 必须遵循 Conventional Commits 规范，格式如下：

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type 类型

必须使用以下标准 type：

- `feat`: 新功能（feature）
- `fix`: 修复 bug
- `docs`: 文档变更（documentation）
- `style`: 代码格式调整（不影响代码运行的变动，如空格、格式化、分号等）
- `refactor`: 重构（既不是新增功能，也不是修改 bug 的代码变动）
- `perf`: 性能优化（performance）
- `test`: 测试相关（增加测试或修改现有测试）
- `build`: 构建系统或外部依赖的变动（如 Cargo.toml、package.json）
- `ci`: CI 配置或 GitHub Actions 的变动
- `chore`: 其他杂务（不修改 src 或 test 的变动，如更新依赖版本）
- `revert`: 回滚某个 commit

### Scope 范围

- scope 用于说明 commit 影响的范围，可选但建议填写
- 常用 scope：`backend`、`frontend`、`tauri`、`config`、`ui`、`docs`、`ci`、`provider`
- 示例：`feat(provider): 添加 OpenRouter 余额查询支持`

### Subject 主题

- subject 是 commit 目的的简短描述
- 使用中文短句，不超过 50 个字符
- 不以大写开头（英文）或不加句号
- 使用祈使语气（如"添加"而非"添加了"）

### Body 正文

- body 是对 commit 的详细描述，可选
- 说明代码变动的动机和与之前行为的对比
- 使用中文，每行不超过 72 个字符
- 使用空行与 subject 分隔

### Footer 脚注

- footer 用于引用 BREAKING CHANGE 或关联 issue
- 关联 issue 使用 `Closes #123` 或 `Fixes #456`
- 破坏性变更必须以 `BREAKING CHANGE:` 开头

### 示例

```
feat(provider): 添加 OpenRouter 余额查询支持

实现 OpenRouter provider 的余额查询接口，包括：
- 实现 BalanceProvider trait
- 添加 API endpoint 和认证逻辑
- 添加错误处理和重试机制

Closes #42
```

```
fix(ui): 修复托盘窗口定位问题

修复在高分辨率屏幕上托盘窗口无法正确定位到右下角的问题。
调整窗口坐标计算逻辑，考虑屏幕缩放因子。

Fixes #78
```

```
refactor(backend): 重构 provider trait 统一错误处理

将各个 provider 的错误类型统一为 AppError，简化错误处理逻辑。
这是一个破坏性变更，所有 provider 实现需要更新。

BREAKING CHANGE: ProviderError 类型已移除，统一使用 AppError
```

### 禁止事项

- 禁止无意义的 commit message，如 `update`、`fix`、`wip`
- 禁止中英文混合 type（如 `feat: 添加功能` 可以，但 `feature: 添加功能` 不行）
- 禁止在 subject 中使用句号结尾
- 禁止超过 50 字符的 subject（body 除外）

## Branch 命名规范

### 分支类型

项目使用以下分支类型：

- `master`（或 `main`）：主分支，保持稳定，随时可发布
- `develop`：开发分支（可选），集成待发布的功能
- `feature/*`：功能分支，用于开发新功能
- `fix/*`：修复分支，用于修复 bug
- `hotfix/*`：热修复分支，用于紧急修复生产环境问题
- `release/*`：发布分支，用于准备新版本发布
- `docs/*`：文档分支，用于文档更新
- `refactor/*`：重构分支，用于代码重构

### 命名格式

分支命名格式：`<type>/<description>`

- type：分支类型（feature、fix、hotfix、release、docs、refactor）
- description：简短描述，使用小写英文，单词间用连字符 `-` 分隔
- 可关联 issue 编号，如 `feature/add-openrouter-42`

### 示例

```
feature/add-anthropic-provider
fix/tray-window-position
hotfix/critical-security-fix
release/v0.1.0
docs/update-readme
refactor/provider-trait
```

### 分支生命周期

- `feature/*`、`fix/*`、`docs/*`、`refactor/*`：完成后合并到 `master` 或 `develop`，然后删除
- `release/*`：发布完成后合并到 `master` 和 `develop`（若存在），打 tag 后删除
- `hotfix/*`：修复完成后合并到 `master` 和 `develop`（若存在），打 tag 后删除
- 分支超过 30 天未活动，应重新评估是否继续

### 保护规则

- `master` 分支受保护，不允许直接 push
- 所有变更必须通过 Pull Request 合并
- PR 必须通过 CI 检查和至少 1 人 review（若团队规模允许）
- 禁止 force push 到 `master`

## PR Review 规范

### PR 创建要求

创建 Pull Request 时必须：

- 标题遵循 Conventional Commits 格式，如 `feat: 添加供应商设置面板`
- 描述包含以下中文小节：
  - **变更内容**：说明本次 PR 做了什么
  - **验证结果**：列出已运行的验证命令和结果
  - **影响范围**：说明影响的模块或功能
  - **截图/录屏**：涉及 UI 变更时必须提供
  - **关联 Issue**：使用 `Closes #123` 关联相关 issue
- 指定至少 1 名 reviewer（若团队规模允许）
- 添加合适的 label（如 `Feature`、`优先级：高`）
- 关联到对应的 GitHub Project 或 milestone（若使用）

### Review 检查清单

Reviewer 在审查 PR 时必须检查：

**代码质量**
- [ ] 代码符合 AGENTS.md 规范
- [ ] 代码风格与项目现有风格一致
- [ ] 没有引入无意义的 `any` 或关闭 strict
- [ ] 没有硬编码的魔法数字或字符串
- [ ] 错误处理完整且合理
- [ ] 没有明显的性能问题

**功能正确性**
- [ ] 实现与 PR 描述一致
- [ ] 实现与关联 issue 的需求一致
- [ ] 边界情况已考虑和处理
- [ ] 没有破坏现有功能

**测试覆盖**
- [ ] 新增功能有对应的测试
- [ ] bug 修复有回归测试
- [ ] 测试覆盖关键路径
- [ ] 测试可以稳定通过

**安全性**
- [ ] 没有泄露 API Key 或敏感信息
- [ ] 没有引入安全漏洞
- [ ] 输入验证完整
- [ ] 权限控制正确

**文档**
- [ ] 用户可见文案使用中文且语义清晰
- [ ] API 变更已更新相关文档
- [ ] 破坏性变更已在 commit message 和 PR 描述中说明
- [ ] CHANGELOG.md 已更新（若需要）

**构建和 CI**
- [ ] CI 检查全部通过
- [ ] 没有引入新的 lint 警告
- [ ] 构建产物大小合理
- [ ] 没有引入不必要的大型依赖

### Review 流程

1. **初步审查**：检查 PR 描述是否完整，label 和 reviewer 是否正确
2. **代码审查**：按照检查清单逐项检查，提出建设性意见
3. **讨论和修改**：作者根据 review 意见修改代码，reviewer 跟进确认
4. **批准合并**：所有检查项通过后，reviewer 批准 PR
5. **合并策略**：优先使用 squash merge 保持主分支历史清晰。feature/fix 分支用 squash merge；release/hotfix 分支用 merge commit（--no-ff）。

### Review 礼仪

- 评论代码时保持尊重和建设性，避免指责性语言
- 提出问题时说明原因和建议的解决方案
- 区分"必须修改"和"建议修改"，使用不同标签或前缀
- 认可优秀的代码实现，给予正面反馈
- 有争议时先私下沟通，避免在 PR 中公开争论

## 测试覆盖率目标（暂未强制，待配置 tarpaulin / vitest coverage 后启用）

### 覆盖率目标

- Rust 后端：核心模块覆盖率不低于 80%
- TypeScript 前端：工具函数和 hooks 覆盖率不低于 70%
- 整体覆盖率目标：75% 以上
- 关键路径（如 API Key 存取、provider 查询、错误处理）覆盖率不低于 90%

### 必须测试的场景

**Rust 后端**
- `AppError` 的序列化/反序列化 roundtrip
- 配置文件的默认值、读取、写入
- 每个 provider 的余额解析和错误状态处理
- HTTP 请求的 mock 测试（成功和失败场景）
- keyring 操作的 mock 测试
- async 函数的并发安全性

**TypeScript 前端**
- `format.ts`、`colors.ts`、`cn.ts` 等工具函数的各种输入
- Zustand store 的状态变更和 action
- React Query hook 的参数传递和 cache invalidation
- Tauri invoke wrapper 的类型安全和错误处理
- 组件的渲染测试（loading、empty、error 状态）

### 测试编写规范

- 测试文件名使用 `*.test.ts` 或 `*.spec.ts`（前端）
- Rust 测试放在对应模块的 `#[cfg(test)] mod tests` 中
- 测试函数命名清晰，说明测试的场景和期望结果
- 使用 `#[test]` 和 `#[tokio::test]` 标记测试
- 使用 mock 隔离外部依赖（网络、文件系统、keyring）
- 测试数据使用固定的 mock 数据，不依赖真实 API
- 避免测试之间的依赖和顺序

示例（Rust）：
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_serde_roundtrip() {
        let error = AppError::Network("连接超时".to_string());
        let json = serde_json::to_string(&error).unwrap();
        let deserialized: AppError = serde_json::from_str(&json).unwrap();
        assert_eq!(error.to_string(), deserialized.to_string());
    }

    #[tokio::test]
    async fn test_deepseek_balance_parsing() {
        let mock_response = r#"{"balance": 100.50}"#;
        let balance = parse_deepseek_balance(mock_response).unwrap();
        assert_eq!(balance.amount, 100.50);
    }
}
```

> 注：项目当前未引入 vitest，以下示例为引入测试框架后的参考写法。

示例（TypeScript）：
```typescript
import { describe, it, expect } from 'vitest';
import { formatBalance } from './format';

describe('formatBalance', () => {
  it('should format positive balance with 2 decimals', () => {
    expect(formatBalance(100.5)).toBe('100.50');
  });

  it('should handle zero balance', () => {
    expect(formatBalance(0)).toBe('0.00');
  });

  it('should handle negative balance', () => {
    expect(formatBalance(-50.25)).toBe('-50.25');
  });
});
```

### 测试运行命令

```bash
# Rust 测试
cd src-tauri
cargo test -- --test-threads=1

# 前端测试（当前等同于 `npm run lint`，前端单元测试框架尚未配置）
npm test

# 带覆盖率的测试
cargo tarpaulin --out Html  # Rust
npm run test:coverage       # 前端（若配置）
```

### CI 中的测试

- CI 会在每次 push 和 PR 时自动运行测试
- 测试失败会阻止 PR 合并
- 测试超时时间：Rust 10 分钟，前端 5 分钟
- 使用 `RUST_TEST_THREADS=1` 避免并发测试的资源竞争
- 使用 `PULSE_TEST_SKIP_NETWORK=1` 跳过网络测试（CI 环境）

## 性能优化指南

### Rust 后端性能

**异步和并发**
- 避免在 async 函数中执行阻塞操作，使用 `spawn_blocking` 包装
- 避免持锁跨 `.await`，clone 必要数据后释放锁
- 使用 `tokio::select!` 处理多个并发异步操作
- 合理设置超时时间，避免无限等待

**内存和分配**
- 复用 buffer 和连接，避免频繁分配
- 使用 `String::with_capacity` 预分配大字符串
- 避免不必要的 clone，使用引用或 `Cow`
- 大集合使用 `Box`、`Rc`、`Arc` 减少栈压力

**HTTP 请求**
- 复用 `reqwest::Client`，不要每次请求都创建
- 设置合理的连接池大小和超时时间
- 使用 gzip 压缩减少传输大小
- 实现请求重试机制，处理瞬时网络错误

**配置和缓存**
- 配置文件读取后缓存在内存中，避免频繁磁盘 I/O
- 使用 `once_cell::sync::Lazy` 初始化全局资源
- 合理使用 `#[derive(Clone)]`，避免深度拷贝

示例：
```rust
use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(5)
        .build()
        .unwrap()
});

async fn fetch_balance(url: &str) -> Result<Balance, AppError> {
    HTTP_CLIENT.get(url)
        .send()
        .await?
        .json()
        .await
        .map_err(Into::into)
}
```

### TypeScript 前端性能

**渲染优化**
- 使用 `React.memo` 包裹纯展示组件
- 使用 `useMemo` 缓存计算结果
- 使用 `useCallback` 缓存回调函数
- 避免在 render 中创建新对象或数组

**状态管理**
- Zustand 使用 selector 避免不必要的重渲染
- React Query 合理设置 `staleTime` 和 `cacheTime`
- mutation 成功后精确 invalidate 相关 query，避免全量刷新
- 避免在 Zustand 中存储可以从 React Query 派生的数据

**组件和 UI**
- 大列表使用虚拟滚动（如 `react-window`）
- 图表数据量大时考虑采样或降采样
- 避免不必要的重渲染，使用 `React DevTools Profiler` 检测
- icon-only button 使用 `aria-label` 提高可访问性

**打包和加载**
- 使用动态 import 懒加载非关键组件
- 优化图片大小，使用 WebP 格式
- 移除未使用的依赖和代码
- 使用 `vite build --mode production` 优化产物大小

示例：
```typescript
import { memo, useMemo } from 'react';
import { useBalance } from './hooks/useBalance';

export const BalanceDisplay = memo(({ providerId }: { providerId: string }) => {
  const { data: balance, isLoading } = useBalance(providerId);
  
  const formattedBalance = useMemo(() => {
    if (!balance) return null;
    return formatCurrency(balance.amount, balance.currency);
  }, [balance]);
  
  if (isLoading) return <Spinner />;
  return <div>{formattedBalance}</div>;
});
```

### 性能监控

- 使用 `console.time` 和 `console.timeEnd` 测量关键路径耗时
- 使用 React DevTools Profiler 分析组件渲染性能
- 使用 Rust `std::time::Instant` 测量后端操作耗时
- 定期使用 Lighthouse 或 WebPageTest 评估前端性能
- 记录性能基线，避免性能退化

## 安全审计清单

> 注：PR 评审中的基础安全检查项见 §16 PR Review 检查清单的"安全性"小节；本节为更完整的安全审计清单，聚焦深度安全项。

### API Key 和凭证安全

- [ ] API Key 只通过 OS keychain 存取，不写入配置文件
- [ ] 日志、错误提示、toast 中不打印真实 API Key
- [ ] README 和文档示例使用占位符，不包含真实密钥
- [ ] 测试数据使用 mock key，不包含真实凭证
- [ ] keyring 操作有适当的错误处理
- [ ] API Key 在内存中使用时，用完后及时清理（若可能）

### 网络安全

- [ ] 外部请求只访问实现计划中明确的供应商 API
- [ ] 不添加遥测、埋点、远程上报功能
- [ ] CSP（Content Security Policy）配置正确，限制 connect-src
- [ ] HTTPS 优先，避免明文 HTTP 请求
- [ ] 验证服务器证书，不跳过证书验证
- [ ] 请求超时设置合理，避免无限等待

### 输入验证

- [ ] 用户输入（如自定义 endpoint）进行验证和转义
- [ ] 防止 XSS 攻击，不直接渲染用户输入为 HTML
- [ ] 防止命令注入，不直接拼接用户输入到 shell 命令
- [ ] 文件路径输入进行规范化，防止路径遍历攻击
- [ ] API 响应数据进行验证，不信任外部数据

### 依赖安全

- [ ] 定期运行 `cargo audit` 检查 Rust 依赖漏洞
- [ ] 定期运行 `npm audit` 检查前端依赖漏洞
- [ ] 及时更新有已知漏洞的依赖
- [ ] 引入新依赖前检查其维护状态和安全历史
- [ ] 避免引入不必要的大型依赖

### 权限和隔离

- [ ] Tauri capabilities 配置最小权限原则
- [ ] 不请求不必要的系统权限
- [ ] 敏感操作（如删除文件）需要用户确认
- [ ] 不同模块之间职责隔离，避免越权访问

### 日志和监控

- [ ] 日志中不包含敏感信息（API Key、token、密码）
- [ ] 错误日志包含足够信息用于调试，但不泄露隐私
- [ ] 异常行为有告警机制（若适用）
- [ ] 定期审查日志，发现潜在安全问题

### 安全事件响应

- [ ] 发现安全漏洞时，第一时间评估影响范围
- [ ] 严重漏洞通过私密渠道报告，不在公开 issue 讨论
- [ ] 修复后发布安全公告，说明影响和修复措施
- [ ] 通知受影响的用户，提供升级建议

### 安全审计工具

```bash
# Rust 安全审计
cargo audit

# 前端安全审计
npm audit

# Rust 代码静态分析
cargo clippy -- -D warnings

# 前端代码静态分析
npm run lint
```

## 代码审查清单

### 提交前自查

（与 §16 PR Review 检查清单一致，此处不重复。）提交前至少确保：已运行 `cargo fmt` 和 `npm run lint`，本地 `cargo check`/`cargo test` 与 `npm run build` 通过。

### Review 重点

Reviewer 重点关注：

1. **架构合理性**：变更是否符合项目架构，是否引入不必要的复杂性
2. **代码可读性**：代码是否易于理解，命名是否清晰
3. **测试覆盖**：关键路径是否有测试覆盖
4. **性能影响**：是否引入性能问题
5. **安全风险**：是否引入安全漏洞
6. **向后兼容**：是否破坏现有功能或 API

### 常见 Review 意见模板

```
【必须修改】这里存在一个潜在的 null pointer 问题，建议添加空值检查。

【建议修改】这个函数可以拆分成更小的函数，提高可读性。

【问题】这里的逻辑我不太理解，能否解释一下为什么这样实现？

【赞赏】这个实现很优雅，学到了！
```

## 发布流程规范

### 发布前准备

1. **确认发布内容**
   - 确认所有计划的功能和修复已合并到 `master`
   - 确认所有关联的 issue 已关闭
   - 确认 CHANGELOG.md 已更新，记录所有显著变更

2. **质量检查**
   - CI 全部通过
   - 手动测试关键功能正常
   - 性能测试无明显退化
   - 安全审计无高危漏洞

3. **版本号确定**
   - 根据 SemVer 确定新版本号（major.minor.patch）
   - 破坏性变更：major
   - 新功能：minor
   - bug 修复：patch

### 发布步骤

1. **创建发布分支**
   ```bash
   git checkout -b release/v0.1.0 master
   ```

2. **更新版本号**
   - 更新 `package.json` 中的 `version`
   - 更新 `src-tauri/Cargo.toml` 中的 `version`
   - 更新 `src-tauri/tauri.conf.json` 中的 `version`
   - 提交：`chore: bump version to v0.1.0`

3. **更新 CHANGELOG.md**
   - 将 `[Unreleased]` 下的内容移到新版本号下
   - 添加发布日期
   - 提交：`docs: update CHANGELOG for v0.1.0`

4. **创建 tag**
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

5. **触发 CI 构建**
   - push tag 后，GitHub Actions 会自动构建各平台产物
   - 构建完成后会创建 draft release

6. **完善 Release Notes**
   - 编辑 draft release，补充详细的变更说明
   - 上传必要的额外文件或说明
   - 添加安装和使用说明

7. **发布 Release**
   - 确认所有产物正确
   - 将 draft release 改为 published
   - 通知用户新版本发布

8. **合并回 develop（若存在）**
   ```bash
   git checkout develop
   git merge --no-ff release/v0.1.0
   git push origin develop
   ```

### 发布后验证

- [ ] 下载各平台产物，验证安装和运行正常
- [ ] 检查关键功能是否正常
- [ ] 监控错误日志和用户反馈
- [ ] 更新项目文档（若需要）
- [ ] 关闭已完成的 milestone

### 热修复流程

紧急修复生产环境问题：

1. 从最近的 tag 创建 `hotfix/*` 分支
2. 修复问题并添加回归测试
3. 更新版本号为 patch 版本（如 v0.1.0 -> v0.1.1）
4. 合并到 `master` 和 `develop`（若存在）
5. 创建新 tag 并触发发布
6. 通知用户升级

## 版本管理规范

### SemVer 规范

项目版本遵循 [Semantic Versioning 2.0.0](https://semver.org/lang/zh-CN/)：

格式：`MAJOR.MINOR.PATCH`

- **MAJOR**：不兼容的 API 变更
  - 删除或重命名公开 API
  - 改变行为导致旧代码无法工作
  - 示例：`v1.0.0` -> `v2.0.0`

- **MINOR**：向后兼容的功能新增
  - 添加新功能
  - 添加新的 provider 支持
  - 示例：`v0.1.0` -> `v0.2.0`

- **PATCH**：向后兼容的 bug 修复
  - 修复错误行为
  - 性能优化
  - 示例：`v0.1.0` -> `v0.1.1`

### 预发布版本

- Alpha 版本：`v0.1.0-alpha.1`
- Beta 版本：`v0.1.0-beta.1`
- Release Candidate：`v0.1.0-rc.1`

### 版本号同步

以下文件的版本号必须保持同步：

- `package.json` 中的 `version`
- `src-tauri/Cargo.toml` 中的 `version`
- `src-tauri/tauri.conf.json` 中的 `version`

更新版本号时，必须同时更新这三个文件。

### CHANGELOG 维护

- 遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/) 格式
- 每个版本包含以下分类（按顺序）：
  - `新增`（Added）：新功能
  - `变更`（Changed）：现有功能的变更
  - `弃用`（Deprecated）：即将移除的功能
  - `移除`（Removed）：已移除的功能
  - `修复`（Fixed）：bug 修复
  - `安全`（Security）：安全性修复
- `[Unreleased]` 部分记录待发布的变更
- 每次发布时，将 `[Unreleased]` 内容移到新版本号下

### Tag 规范

- tag 名称格式：`v<version>`，如 `v0.1.0`
- tag 必须打在 `master` 分支上
- tag 必须带注释，说明版本主要内容
- tag 一旦创建，不允许删除或重新指向

示例：
```bash
git tag -a v0.1.0 -m "Release v0.1.0

新增:
- 支持 DeepSeek、OpenAI、Anthropic、OpenRouter provider
- 深色毛玻璃 UI
- API Key 安全存储

修复:
- 无（初始版本）
"
```

## 文档维护规范

### 文档类型

项目包含以下文档：

- **README.md**：项目介绍、安装说明、使用指南
- **CHANGELOG.md**：版本变更记录
- **AGENTS.md**：项目开发规范（本文件）
- **docs/**：详细设计文档、技术规范
- **代码注释**：代码内的文档说明

### README.md 维护

- 包含项目简介、特性列表、安装步骤、使用示例
- 使用中文编写，技术名词保留英文
- 包含截图或 GIF 展示 UI
- 包含贡献指南和许可证信息
- 及时更新，反映最新功能和安装步骤

### CHANGELOG.md 维护

- 每次发布前必须更新
- 记录所有用户可见的变更
- 内部重构若不影响用户行为，可不记录或简要记录
- 遵循 Keep a Changelog 格式，按新增、变更、修复等分类
- 使用中文描述，技术名词保留英文

### 代码注释规范

- 公共 API 必须有文档注释（Rust 使用 `///`，TypeScript 使用 JSDoc）
- 复杂算法和业务逻辑需要注释说明意图
- 避免无意义的注释（如 `// 循环` 在 for 循环前）
- 注释使用中文或英文均可，但同一文件保持一致
- TODO 注释必须关联 issue 或说明负责人和预计完成时间

示例（Rust）：
```rust
/// 查询指定供应商的账户余额
///
/// # Arguments
/// * `provider_id` - 供应商唯一标识
///
/// # Returns
/// 返回余额信息或错误
///
/// # Errors
/// 当网络请求失败或 API Key 无效时返回错误
pub async fn get_balance(provider_id: &str) -> Result<Balance, AppError> {
    // ...
}
```

示例（TypeScript）：
```typescript
/**
 * 格式化余额显示
 * @param amount - 余额金额
 * @param currency - 货币类型
 * @returns 格式化后的字符串，如 "$100.50"
 */
export function formatBalance(amount: number, currency: string): string {
  // ...
}
```

### 文档更新检查清单

每次代码变更后，检查是否需要更新：

- [ ] README.md（新功能、安装步骤变更）
- [ ] CHANGELOG.md（所有用户可见变更）
- [ ] AGENTS.md（开发规范变更）
- [ ] 代码注释（API 变更、复杂逻辑）
- [ ] 设计文档（架构变更）
- [ ] 迁移指南（破坏性变更）

## 依赖管理规范

### 依赖引入原则

引入新依赖前必须评估：

1. **必要性**：是否真的需要，能否用现有依赖或标准库实现
2. **维护状态**：是否活跃维护，最近更新时间，issue 响应速度
3. **包大小**：对最终产物大小的影响
4. **安全性**：是否有已知漏洞，安全历史记录
5. **许可证**：是否与项目许可证兼容（MIT）
6. **社区共识**：是否是社区推荐方案

### Rust 依赖管理

**添加依赖**
```bash
cd src-tauri
cargo add <crate-name>
```

**更新依赖**
```bash
cargo update  # 更新 Cargo.lock
cargo update -p <crate-name>  # 更新特定依赖
```

**检查过时依赖**
```bash
cargo outdated  # 需要安装 cargo-outdated
```

**安全审计**
```bash
cargo audit  # 检查已知漏洞
```

**依赖分类**
- `[dependencies]`：运行时必需
- `[dev-dependencies]`：测试和开发
- `[build-dependencies]`：构建脚本

**关键依赖锁定**
- Tauri、React、TypeScript 等核心依赖升级需要充分测试
- 大版本升级应在独立分支进行
- 记录升级原因和影响

### TypeScript/前端依赖管理

**添加依赖**
```bash
npm install <package-name>  # 运行时依赖
npm install -D <package-name>  # 开发依赖
```

**更新依赖**
```bash
npm update  # 按 package.json 范围更新
npm outdated  # 查看过时依赖
```

**安全审计**
```bash
npm audit  # 检查已知漏洞
npm audit fix  # 自动修复
```

**依赖分类**
- `dependencies`：运行时必需
- `devDependencies`：构建、测试、开发工具

**锁文件管理**
- `package-lock.json` 必须提交到版本控制
- 使用 `npm ci` 而非 `npm install` 确保可重复构建
- 解决冲突时优先保留 lock 文件的版本

### 依赖版本策略

**Rust（Cargo.toml）**
```toml
# 推荐：兼容版本范围
reqwest = "0.12"  # 等同于 ^0.12，允许 0.12.x 更新

# 精确版本（关键依赖）
tauri = "=2.0.0"

# Git 依赖（临时使用）
some-crate = { git = "https://github.com/user/repo", branch = "main" }
```

**TypeScript（package.json）**
```json
{
  "dependencies": {
    "react": "^18.3.1",  // 允许 18.x.x 更新
    "zustand": "~4.5.2"  // 允许 4.5.x 更新
  }
}
```

**版本符号说明**
- `^1.2.3`：允许 minor 和 patch 更新（1.x.x）
- `~1.2.3`：只允许 patch 更新（1.2.x）
- `1.2.3`：精确版本（不推荐，除非必要）

### 依赖更新流程

1. **评估影响**：查看 changelog，评估破坏性变更
2. **本地测试**：在开发分支更新并测试
3. **CI 验证**：确保 CI 通过
4. **分批更新**：不要一次更新所有依赖
5. **记录变更**：在 commit message 中说明更新原因

示例 commit：
```
chore(deps): update reqwest to 0.12.5

修复已知的 HTTP/2 连接问题，参见：
https://github.com/seanmonstar/reqwest/releases/tag/v0.12.5
```

### 依赖安全审计

**定期审计**
- 每月运行一次 `cargo audit` 和 `npm audit`
- 订阅依赖安全公告（GitHub Dependabot、RustSec）
- 高危漏洞必须在 7 天内修复或缓解

**审计工具配置**
```yaml
# .github/workflows/security.yml
name: Security Audit
on:
  schedule:
    - cron: '0 0 * * 1'  # 每周一
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Rust audit
        run: |
          cargo install cargo-audit
          cargo audit
      - name: npm audit
        run: npm audit --production
```

### 依赖清理

**识别未使用依赖**
```bash
# Rust
cargo install cargo-udeps
cargo udeps

# TypeScript
npm install -g depcheck
depcheck
```

**清理流程**
1. 运行检测工具
2. 确认依赖确实未使用
3. 从配置文件移除
4. 运行测试确保无影响
5. 提交：`chore(deps): remove unused dependency <name>`

## Git 与 GitHub 变更管理规范

- 本项目按 GitHub 项目处理；涉及 issue、PR、release、workflow、tag、artifact、Projects、milestone 时遵循 GitHub 工作流。
- 执行 git 或 GitHub 操作前先检查当前仓库状态、分支和远程信息。
- 只有用户明确要求时才 commit、push、tag、创建 PR、发布 release 或修改远程仓库。
- 不自动推送远程或发布 release。
- GitHub Actions 配置优先放在 `.github/workflows/`，release artifact 路径需与 Tauri 实际产物一致。
- 修改已有文件前先读取文件；若文件内容与预期不符，先报告再处理。
- 大规模重构必须先说明原因并获得确认。

## 中文 GitHub 规范

- GitHub 上面向项目协作者和用户的内容默认使用中文，包括 issue、PR、release notes、README、CHANGELOG、讨论说明和模板文案。
- 保留必要英文技术名词、包名、命令、API 名称、错误码、文件路径和 GitHub Actions 关键字，不强行翻译专有名词。
- Issue 标题建议使用中文短句，可带类型前缀，例如 `Bug: 托盘窗口无法定位到右下角`、`Feature: 添加 OpenRouter 余额查询`。
- PR 标题建议使用 Conventional Commits 格式：`feat: 添加供应商设置面板`、`fix: 修复 Tauri v2 capabilities 权限`、`docs: 更新中文 README`。
- PR 描述默认包含中文小节：`变更内容`、`验证结果`、`影响范围`、`截图/录屏`（如涉及 UI）、`关联 Issue`。
- Issue 模板默认包含中文字段：`问题描述`、`复现步骤`、`期望行为`、`实际行为`、`环境信息`、`补充信息`。
- Feature request 模板默认包含中文字段：`需求背景`、`目标用户`、`期望方案`、`替代方案`、`验收标准`。
- Release notes 默认中文，按 `新增`、`修复`、`优化`、`文档`、`破坏性变更` 分组；版本号遵循 SemVer，例如 `v0.1.0`。
- GitHub Actions workflow 的 `name`、job id、step id 可使用英文；面向用户的 artifact 名称和 release 说明优先中文或中英结合。
- Label 名称可使用中文或中英结合，但同一类标签保持一致，例如 `Bug`、`Feature`、`待确认`、`优先级：高`；新增 label 优先沿用仓库既有语言风格、分类粒度和颜色语义。
- 对外回复 issue/PR 评论、review comment、discussion 时默认中文；如果对方使用英文提问，可使用英文或中英双语回复，技术争议优先先给结论再列证据。
- 安全相关 issue 不公开粘贴密钥、token、完整请求头或敏感日志；必要时引导使用私密渠道或脱敏信息。
- 若使用 GitHub Projects 或 milestones，标题、说明和字段默认中文，状态值保持短句一致，例如 `待处理`、`进行中`、`已完成`；与既有项目字段冲突时先沿用既有规范。
- 创建或修改 `.github/ISSUE_TEMPLATE/`、`.github/PULL_REQUEST_TEMPLATE.md`、`.github/workflows/`、`.github/labels.yml`、release 配置前，先确认是否已有上游约定，避免覆盖用户已有 GitHub 配置。

## 禁止事项

- 禁止使用 Tauri v1 API 替代 v2 API。
- 禁止把 API Key 明文写入 config、README、测试数据或日志。
- 禁止把 React Query 管理的数据复制进 Zustand 形成双源状态。
- 禁止在未验证时宣称构建/测试通过。
- 禁止为了快速通过 TypeScript 而引入无意义 `any`、关闭 strict 或大范围忽略错误。
- 禁止无依据删除用户文件或设计预览。
- 禁止偏离 reviewed plan 大幅改架构，除非先说明理由并获得用户确认。

## 完成标准

一个任务只有同时满足以下条件，才可以称为完成：

1. 代码/文档符合本文件规范。
2. 与 reviewed plan 或用户要求一致。
3. 涉及用户可见文案时已使用中文并检查语义。
4. 涉及安全数据时未泄露真实密钥或敏感信息。
5. 已运行适用验证，或明确说明为什么无法运行。
6. 回复中列出变更摘要、验证结果和后续建议。
