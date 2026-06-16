# Pulse 重构设计 Spec

> 日期：2026-06-16
> 范围：Pulse V0.1.0 现状对齐 + 严重 Bug 修复 + 代码质量 + 补全 plan 缺失项
> 执行方式：分 3 阶段提交，每阶段独立验证
> 验证手段：`cargo check` + `cargo test`（阶段 2、3）+ `npm run build`

## 背景

`docs/superpowers/plans/2026-06-16-pulse-v0.1.0-reviewed.md` 已审阅完成并由 5 个子代理审查，代码基本按 plan 实现。但实际代码与 plan 之间存在 3 处功能不一致，会导致用户可见的功能失效；另有 7 处代码质量问题和 4 处 plan 遗漏项需要补全。

本次重构**不修改架构**，只做契约对齐、健壮性提升、plan 落地收尾。

## 范围之外（明确不做）

- 不调整 Tauri v2 capability 权限列表
- 不修改 provider 业务逻辑与新增供应商
- 不动 CI 多平台构建矩阵
- 不新增端到端测试
- 不动 Cargo.lock / package-lock.json
- 不动 icon 资源
- 不动 GitHub workflow 的 release 任务

## 阶段 1：修严重 Bug（前后端契约）

### 1.1 修复 `ProviderType` 序列化

- 文件：`src-tauri/src/config.rs`
- 现状：`#[serde(rename_all = "snake_case")]` 把 `OpenAi` 序列化为 `"open_ai"`
- 改成：`#[serde(rename_all = "lowercase")]`，使 `OpenAi → "openai"`、`DeepSeek → "deepseek"`、`OpenRouter → "openrouter"`
- 影响文件：`src-tauri/src/providers/mod.rs:63-81` 的 match 不需改（match 的是 enum variant，不是字符串）
- 验证：现有 `provider_type_snake_case_serialization` 单测需要更新为 `provider_type_lowercase_serialization`，断言 `"openai"`

### 1.2 同步 TypeScript `AppSettings` 类型

- 文件：`src/types/index.ts`
- 现状：`AppSettings` 有 `provider_overrides: Record<string, { enabled: boolean }>` 字段，但 Rust 端 `AppSettings` 没有该字段
- 决策：移除 TS 端的 `provider_overrides` 字段。**原因**：Rust 端 `ProviderConfig.enabled` 已经在 `toggle_provider` command 中支持，直接复用即可。`provider_overrides` 是原 plan 的设计但未被实现且无业务需求，引入会变成死代码。AGENTS.md 允许"偏离 reviewed plan 大幅改架构"前提是"先说明理由并获得用户确认"——本项已在澄清环节向用户说明。
- 改成：从 `AppSettings` interface 移除 `provider_overrides: Record<string, { enabled: boolean }>`

### 1.3 新增 `useToggleProvider` hook

- 文件：`src/hooks/useProviders.ts`
- 后端 `toggle_provider` command 已存在于 `src-tauri/src/commands/providers.rs:52-66`，本步骤只在前端补 hook
- 实现：

```typescript
export function useToggleProvider() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, enabled }: { id: string; enabled: boolean }) =>
      tauriInvoke<void>('toggle_provider', { provider_id: id, enabled }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [PROVIDERS_KEY] });
      queryClient.invalidateQueries({ queryKey: ['balance'] });
    },
  });
}
```

### 1.4 修正 `SettingsPanel`

- 文件：`src/components/SettingsPanel.tsx`
- 移除 `handleToggleProvider` 内对 `settings.provider_overrides` 的引用
- 改用 `useToggleProvider`
- 新代码（伪代码）：

```typescript
const toggleProvider = useToggleProvider();
const handleToggleProvider = (providerId: string, enabled: boolean) => {
  toggleProvider.mutate({ id: providerId, enabled });
};
```

### 阶段 1 验证

```bash
cd src-tauri && cargo check
npm run build
```

## 阶段 2：代码质量

### 2.1 区分 keychain 错误与未配置

- 文件：`src-tauri/src/commands/balance.rs`、`src-tauri/src/commands/usage.rs`
- 现状：`keychain::retrieve(&provider_id).await.ok()` 把所有错误吞成 `None`
- 改成：使用 `keychain::has(&provider_id)` 先判定；不存在返回 "API key not configured" 错误；存在则 retrieve，任何 retrieve 错误透传 `AppError::Keychain`
- 重构为公共辅助函数，balance 和 usage 共用

### 2.2 tray 图标兜底

- 文件：`src-tauri/src/tray.rs:15`
- 现状：`app.default_window_icon().unwrap().clone()` 在无图标配置时 panic
- 改成：使用 `if let Some(icon) = app.default_window_icon()`，否则用 `app-icon` 内置默认（fallback 通过 tauri 资源），最终失败记 log 返回错误而非 panic

### 2.3 toastStore 生命周期

- 文件：`src/store/toastStore.ts`
- 现状：`addToast` 内 `setTimeout` 在 4s 后清理；组件卸载后 setTimeout 仍触发
- 改法：把 timer 管理移到 `NotificationToast` 组件内的 `useEffect`，store 只负责"增/删"，生命周期归 React
- 简化：移除 store 内的 setTimeout；由 `NotificationToast` 渲染时记录 `addedAt`，并用一个统一 effect 周期性扫描超时 toast 调用 `removeToast`

### 2.4 useTray listen 错误处理

- 文件：`src/hooks/useTray.ts:18-26`
- 现状：`unlisten.then(f => f())` 静默吞掉 reject
- 改成：把 `listen` 返回的 promise `await` 化（在 effect 中使用），错误用 `console.error` 记录；或者 `.catch(e => console.error('useTray listen failed:', e))`

### 2.5 window_position 持久化

- 文件：`src-tauri/src/window.rs`
- 现状：`position_near_tray` 每次都按 work_area 重算，未读取 `settings.window_position`
- 改成：托盘单击时优先使用 `settings.window_position`（如果存在），fallback 才是 `position_near_tray`
- 涉及：在 `window.rs` 接受可选的 `Option<(i32, i32)>` 位置参数；`toggle_window` 读取 `AppState.config`
- 注意：本次只读取，不写回。写回留作未来增强。

### 2.6 AppState::new 错误处理

- 文件：`src-tauri/src/lib.rs:31`
- 现状：`AppState::new().expect("Failed to initialize app state")` panic
- 改成：把 `AppState::new()` 移入 `setup()` 闭包，返回 `Box<dyn Error>`，Tauri 会记录并退出；这样比直接 panic 信息更友好
- 注意：`setup` 闭包返回 `Result<(), Box<dyn Error>>`，所以 `?` 已经可用

### 阶段 2 验证

```bash
cd src-tauri && cargo check && cargo test
npm run build
```

## 阶段 3：补全 plan 缺失项

### 3.1 ProviderSettings 实际挂载

- 文件：`src/components/DetailPanel.tsx`
- 在 `DetailPanel` 的"余额" GlassPanel 旁边加一个"编辑设置" GlassPanel
- 引入 `ProviderSettings` 组件，传入 `provider` 和 `onUpdate`（用 `useUpdateProvider`）
- 触发条件：用户点击顶部"编辑"图标按钮（位于"设置"图标旁）

### 3.2 ProviderSettings 输入防抖

- 文件：`src/components/ProviderSettings.tsx`
- 现状：每个 onChange 立即调用 onUpdate
- 改成：本地 state 持有输入，onChange 只更新本地；onBlur 时才提交到 `onUpdate`
- 优点：避免输入"12"被解析为 12 次 mutation

### 3.3 CI lint 步骤

- 文件：`.github/workflows/ci.yml`
- 在 `test` job 中加 `npm run lint` 步骤（在 `npm run build` 之前）
- 当前 package.json 已有 `lint` 脚本

### 3.4 CSP 区分 dev / prod

- 文件：`src-tauri/tauri.conf.json`
- 现状：CSP 在 dev 与 prod 共用
- 改成：移除 root 下的 `csp`，新增 `devCsp` 字段。dev 包含 `'unsafe-eval'` 和 `http://localhost:1420`，prod 仅 `default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'`
- 供应商 API 域名仍保留在 `connect-src`

### 3.5 OverviewPanel 减少重复 query

- 文件：`src/components/OverviewPanel.tsx`
- 现状：每个 `ProviderRow` 内部调用 `useBalance(providerId)`，会触发 N 次 `get_balance`
- 改成：`ProviderRow` 改为读 React Query 缓存（`queryClient.getQueryData(['balance', providerId])`）；初次进入 OverviewPanel 时 `refresh_all_balances` 已写入每个 provider 的缓存
- 详细：`useBalance` 仍保留供 DetailPanel 用；OverviewPanel 的 `ProviderRow` 用 `useQueryClient` + `useEffect` 订阅缓存

### 3.6 custom provider URL 处理单测

- 文件：`src-tauri/src/providers/custom.rs`
- 抽出 `fn normalize_base_url(url: &str) -> String` 统一处理末尾斜杠
- 加单测覆盖：`"https://x.com"`、`"https://x.com/"`、`"https://x.com//"`

### 阶段 3 验证

```bash
cd src-tauri && cargo check && cargo test
npm run build
```

## 风险与回滚

每个阶段独立 commit，便于回滚：

- 阶段 1 改动小、影响面集中（只动 provider_type / settings / 两个 hook），可独立回滚
- 阶段 2 涉及错误处理和窗口逻辑，回滚成本中等
- 阶段 3 涉及 UI 行为变化，回滚成本中等

每阶段之间不保留过渡状态。

## 验收

每个阶段满足：

1. 阶段对应验证命令全部通过
2. 没有引入 `any`、没有关闭 TypeScript strict
3. 改动文件与本 spec 一一对应
4. AGENTS.md 中关于"完成标准"的 6 条全部满足

## 不在范围

- 写新的供应商适配器
- 修改现有 provider 的业务逻辑
- 改 Tauri 权限/插件
- 写 e2e 测试
- UI 大改（颜色、布局、动效）
- 添加新依赖
- 国际化（i18n）
