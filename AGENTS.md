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
- `参考项目/Pulse-UI.html`

实现时以 reviewed plan 为主线；设计还原时参考 preview 与 Pulse-UI。若计划内容与本文件或用户最新指令冲突，以用户最新指令和本文件为准。

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
- 优先参考 `preview.html` 与 `参考项目/Pulse-UI.html` 的布局、色彩、动效和层级。
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

## Git 与 GitHub 变更管理规范

- 本项目按 GitHub 项目处理；涉及 issue、PR、release、workflow、tag、artifact、Projects、milestone 时遵循 GitHub 工作流。
- 执行 git 或 GitHub 操作前先检查当前仓库状态、分支和远程信息。
- 只有用户明确要求时才 commit、push、tag、创建 PR、发布 release 或修改远程仓库。
- 不自动推送远程或发布 release。
- commit message 可使用 Conventional Commits，建议英文 type + 中文说明，例如 `feat: 添加供应商配置界面`。
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
- Label 名称可使用中文或中英结合，但同一类标签保持一致，例如 `类型: Bug`、`类型: Feature`、`状态: 待确认`、`优先级: 高`；新增 label 优先沿用仓库既有语言风格、分类粒度和颜色语义。
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
- 禁止无依据删除用户文件、参考项目或设计预览。
- 禁止偏离 reviewed plan 大幅改架构，除非先说明理由并获得用户确认。

## 完成标准

一个任务只有同时满足以下条件，才可以称为完成：

1. 代码/文档符合本文件规范。
2. 与 reviewed plan 或用户要求一致。
3. 涉及用户可见文案时已使用中文并检查语义。
4. 涉及安全数据时未泄露真实密钥或敏感信息。
5. 已运行适用验证，或明确说明为什么无法运行。
6. 回复中列出变更摘要、验证结果和后续建议。
