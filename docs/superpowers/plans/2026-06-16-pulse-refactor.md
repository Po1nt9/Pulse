# Pulse 重构实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 按 spec `docs/superpowers/specs/2026-06-16-pulse-refactor-design.md` 执行 3 阶段重构：修严重 Bug、修代码质量、补 plan 缺失项。

**Architecture:** 不改架构。分阶段提交，每阶段独立验证 `cargo check` + `npm run build`（阶段 2、3 加 `cargo test`）。

**Tech Stack:** Rust (Tauri v2)、React 18、TypeScript、Zustand、TanStack Query。

**前置依赖：** 已阅读 spec 全文；本地有 `cargo` 和 `npm`；无网络依赖（无需下载新依赖）。

---

## Task 1: 修复 `ProviderType` 序列化

**Files:**
- Modify: `src-tauri/src/config.rs:18-25`
- Modify: `src-tauri/src/config.rs:218-223`（测试断言更新）

- [ ] **Step 1: 修改 serde 重命名规则**

将 `src-tauri/src/config.rs:18` 的 `#[serde(rename_all = "snake_case")]` 改为 `#[serde(rename_all = "lowercase")]`：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    DeepSeek,
    OpenAi,
    Anthropic,
    OpenRouter,
    Custom,
}
```

- [ ] **Step 2: 更新单测断言**

将 `src-tauri/src/config.rs:218-223` 的 `provider_type_snake_case_serialization` 改名为 `provider_type_lowercase_serialization` 并更新断言：

```rust
#[test]
fn provider_type_lowercase_serialization() {
    let pt = ProviderType::OpenAi;
    let json = serde_json::to_string(&pt).unwrap();
    assert_eq!(json, "\"openai\"");
}
```

- [ ] **Step 3: 运行 cargo check 验证**

```bash
cd src-tauri && cargo check
```

Expected: 编译通过，无 error。

- [ ] **Step 4: 运行测试验证**

```bash
cd src-tauri && cargo test --lib config
```

Expected: 全部测试通过，包含新断言 `"openai"`。

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/config.rs
git commit -m "fix: ProviderType 序列化为 lowercase，与 TS 端一致"
```

---

## Task 2: 移除 TS 端 `AppSettings.provider_overrides`

**Files:**
- Modify: `src/types/index.ts:58`

- [ ] **Step 1: 移除字段**

将 `src/types/index.ts:52-59`：

```typescript
export interface AppSettings {
  theme: string;
  auto_start: boolean;
  global_refresh_interval: number;
  show_notifications: boolean;
  window_position: [number, number] | null;
  provider_overrides: Record<string, { enabled: boolean }>;
}
```

改为：

```typescript
export interface AppSettings {
  theme: string;
  auto_start: boolean;
  global_refresh_interval: number;
  show_notifications: boolean;
  window_position: [number, number] | null;
}
```

- [ ] **Step 2: 运行构建验证**

```bash
npm run build
```

Expected: TypeScript 编译通过（如果 SettingsPanel 还在引用 `provider_overrides` 会报错，先做 Task 3-4 后再构建也可）。

- [ ] **Step 3: Commit**

```bash
git add src/types/index.ts
git commit -m "fix: 移除 AppSettings.provider_overrides，Rust 端未实现该字段"
```

---

## Task 3: 新增 `useToggleProvider` hook

**Files:**
- Modify: `src/hooks/useProviders.ts`

- [ ] **Step 1: 在 `useProviders.ts` 末尾添加新 hook**

在 `src/hooks/useProviders.ts` 文件末尾追加：

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

- [ ] **Step 2: 验证 TypeScript 编译**

```bash
npm run build
```

Expected: 通过。

- [ ] **Step 3: Commit**

```bash
git add src/hooks/useProviders.ts
git commit -m "feat: 新增 useToggleProvider hook，封装 toggle_provider command"
```

---

## Task 4: 修正 `SettingsPanel.handleToggleProvider`

**Files:**
- Modify: `src/components/SettingsPanel.tsx:1-32`

- [ ] **Step 1: 替换 import 和 handler**

将 `src/components/SettingsPanel.tsx:1-5` 改为：

```typescript
import { useUIStore } from '../store/uiStore';
import { useSettings, useUpdateSettings } from '../hooks/useSettings';
import { useProviders, useToggleProvider } from '../hooks/useProviders';
import { GlassPanel } from './GlassPanel';
import { ArrowLeft } from 'lucide-react';
```

将 `src/components/SettingsPanel.tsx:11-32`（`useUpdateSettings()` 后到 `return` 前）替换为：

```typescript
export function SettingsPanel() {
  const navigateToOverview = useUIStore((state) => state.navigateToOverview);
  const { data: settings } = useSettings();
  const { data: providers } = useProviders();
  const updateSettings = useUpdateSettings();
  const toggleProvider = useToggleProvider();

  const handleToggleNotifications = () => {
    if (settings) {
      updateSettings.mutate({ ...settings, show_notifications: !settings.show_notifications });
    }
  };

  const handleToggleProvider = (providerId: string, enabled: boolean) => {
    toggleProvider.mutate({ id: providerId, enabled });
  };
```

注意：`useToggleProvider` 已添加（Task 3）；`useUpdateSettings` 保留供 `handleToggleNotifications` 使用。

- [ ] **Step 2: 验证 TypeScript 编译**

```bash
npm run build
```

Expected: 通过，TS 报告 `useToggleProvider` 已使用、`settings.provider_overrides` 引用已消除。

- [ ] **Step 3: 验证 Rust 端**

```bash
cd src-tauri && cargo check
```

Expected: 通过。

- [ ] **Step 4: 阶段 1 验证（双端）**

```bash
cd src-tauri && cargo check
npm run build
```

Expected: 两者都通过。

- [ ] **Step 5: Commit**

```bash
git add src/components/SettingsPanel.tsx
git commit -m "fix: SettingsPanel 通过 useToggleProvider 修改 provider enabled"
```

阶段 1 完成。可以选择此时 commit 阶段 1 整体（squash 4 个 commit），或保留 4 个原子 commit 自行决定。

---

## Task 5: 区分 keychain 错误与未配置

**Files:**
- Modify: `src-tauri/src/commands/balance.rs:18-58`
- Modify: `src-tauri/src/commands/usage.rs:17-56`
- Create: `src-tauri/src/commands/provider_key.rs`（新文件，提供公共辅助）

- [ ] **Step 1: 创建公共辅助函数文件**

创建 `src-tauri/src/commands/provider_key.rs`：

```rust
use crate::commands::keychain;
use crate::error::{AppError, Result};

/// Resolve the API key for a provider. Returns `Err(AppError::Keychain)` if the
/// keychain itself fails, and `Ok(None)` only if the key is genuinely missing.
pub async fn resolve_api_key(provider_id: &str) -> Result<Option<String>> {
    match keychain::has(provider_id).await {
        Ok(true) => keychain::retrieve(provider_id).await.map(Some),
        Ok(false) => Ok(None),
        Err(e) => Err(e),
    }
}
```

- [ ] **Step 2: 在 commands/mod.rs 声明新模块**

修改 `src-tauri/src/commands/mod.rs:1-5`：

```rust
pub mod balance;
pub mod keychain;
pub mod providers;
pub mod provider_key;
pub mod settings;
pub mod usage;
```

- [ ] **Step 3: 重构 balance.rs 使用辅助函数**

将 `src-tauri/src/commands/balance.rs:1-7` 改为：

```rust
use tauri::State;
use serde::Serialize;

use crate::AppState;
use crate::providers::{create_balance_provider, BalanceInfo};
use crate::commands::provider_key;
use crate::error::{AppError, Result};
```

将 `src-tauri/src/commands/balance.rs:18-58` 的 `get_balance` 函数体中，替换：

```rust
let api_key = provider_key::resolve_api_key(&provider_id).await?;
```

并把后续 `if let Some(key) = api_key { ... } else { ... }` 调整为：

```rust
match provider_key::resolve_api_key(&provider_id).await? {
    Some(key) => {
        let adapter = create_balance_provider(&provider.provider_type, &provider.api_base_url);
        match adapter.get_balance(&key, &state.http_client).await {
            Ok(balance) => Ok(ProviderBalance {
                provider_id: provider.id.clone(),
                provider_name: provider.name.clone(),
                balance: Some(balance),
                error: None,
                last_updated: Some(chrono::Local::now().to_rfc3339()),
            }),
            Err(e) => Ok(ProviderBalance {
                provider_id: provider.id.clone(),
                provider_name: provider.name.clone(),
                balance: None,
                error: Some(e.to_string()),
                last_updated: None,
            }),
        }
    }
    None => Ok(ProviderBalance {
        provider_id: provider.id.clone(),
        provider_name: provider.name.clone(),
        balance: None,
        error: Some("API key not configured".to_string()),
        last_updated: None,
    }),
}
```

（`refresh_all_balances` 同样处理，把 `keychain::retrieve(&provider.id).await.ok()` 改为 `provider_key::resolve_api_key(&provider.id).await?` 然后 match。）

- [ ] **Step 4: 重构 usage.rs 同理**

将 `src-tauri/src/commands/usage.rs:1-7` 改为：

```rust
use tauri::State;
use serde::Serialize;

use crate::AppState;
use crate::providers::{create_usage_provider, UsageData};
use crate::commands::provider_key;
use crate::error::Result;
```

把 `get_usage` 函数体中 `keychain::retrieve(&provider_id).await.ok()` 改为 `match provider_key::resolve_api_key(&provider_id).await? { ... }`，处理逻辑同 balance.rs。

- [ ] **Step 5: 验证**

```bash
cd src-tauri && cargo check
```

Expected: 编译通过。

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/commands/
git commit -m "refactor: 区分 keychain 错误与未配置，公共辅助 provider_key"
```

---

## Task 6: tray 图标兜底

**Files:**
- Modify: `src-tauri/src/tray.rs:14-15`

- [ ] **Step 1: 用 if let 替换 unwrap**

将 `src-tauri/src/tray.rs:14-15`：

```rust
TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
```

改为：

```rust
let icon = match app.default_window_icon() {
    Some(icon) => icon.clone(),
    None => return Err(tauri::Error::AssetNotFound("default window icon".into())),
};

TrayIconBuilder::new()
    .icon(icon)
```

- [ ] **Step 2: 验证**

```bash
cd src-tauri && cargo check
```

Expected: 编译通过。

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/tray.rs
git commit -m "fix: tray 图标缺失时返回错误而非 panic"
```

---

## Task 7: toastStore 生命周期交给 React

**Files:**
- Modify: `src/store/toastStore.ts`
- Modify: `src/components/NotificationToast.tsx`

- [ ] **Step 1: 简化 toastStore**

将 `src/store/toastStore.ts` 整个文件替换为：

```typescript
import { create } from 'zustand';
import { Toast } from '../types';

interface ToastState {
  toasts: Toast[];
  addToast: (message: string, type: Toast['type']) => string;
  removeToast: (id: string) => void;
}

export const useToastStore = create<ToastState>((set) => ({
  toasts: [],
  addToast: (message, type) => {
    const id = `toast-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    set((state) => ({ toasts: [...state.toasts, { id, message, type }] }));
    return id;
  },
  removeToast: (id) => set((state) => ({
    toasts: state.toasts.filter((t) => t.id !== id),
  })),
}));
```

（移除 setTimeout，store 不再负责生命周期，仅返回新 toast 的 id 供调用方决策。）

- [ ] **Step 2: 在 NotificationToast 接管自动清理**

将 `src/components/NotificationToast.tsx` 整个文件替换为：

```typescript
import { useEffect } from 'react';
import { useToastStore } from '../store/toastStore';
import { X, CheckCircle, AlertCircle, Info } from 'lucide-react';

const TOAST_LIFETIME_MS = 4000;
const SWEEP_INTERVAL_MS = 500;
const toastAddedAt = new Map<string, number>();

export function NotificationToast() {
  const toasts = useToastStore((state) => state.toasts);
  const removeToast = useToastStore((state) => state.removeToast);

  useEffect(() => {
    const now = Date.now();
    toasts.forEach((t) => {
      if (!toastAddedAt.has(t.id)) toastAddedAt.set(t.id, now);
    });
    const expired = toasts
      .filter((t) => now - (toastAddedAt.get(t.id) ?? now) >= TOAST_LIFETIME_MS)
      .map((t) => t.id);
    expired.forEach((id) => {
      toastAddedAt.delete(id);
      removeToast(id);
    });
  }, [toasts, removeToast]);

  useEffect(() => {
    const timer = setInterval(() => {
      const now = Date.now();
      const current = useToastStore.getState().toasts;
      current.forEach((t) => {
        if (!toastAddedAt.has(t.id)) toastAddedAt.set(t.id, now);
      });
      const expired = current
        .filter((t) => now - (toastAddedAt.get(t.id) ?? now) >= TOAST_LIFETIME_MS)
        .map((t) => t.id);
      expired.forEach((id) => {
        toastAddedAt.delete(id);
        useToastStore.getState().removeToast(id);
      });
    }, SWEEP_INTERVAL_MS);
    return () => clearInterval(timer);
  }, []);

  if (toasts.length === 0) return null;

  return (
    <div className="fixed bottom-4 left-4 right-4 flex flex-col gap-2 z-50" aria-live="polite" aria-atomic="true">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`glass-panel px-4 py-3 flex items-center gap-3 animate-popup-in ${
            toast.type === 'error' ? 'border-status-danger/30' : ''
          }`}
        >
          {toast.type === 'success' && <CheckCircle className="w-4 h-4 text-status-ok" />}
          {toast.type === 'error' && <AlertCircle className="w-4 h-4 text-status-danger" />}
          {toast.type === 'info' && <Info className="w-4 h-4 text-accent" />}
          <span className="text-sm text-white/80 flex-1">{toast.message}</span>
          <button
            onClick={() => removeToast(toast.id)}
            className="text-white/40 hover:text-white/60"
            aria-label="关闭通知"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      ))}
    </div>
  );
}
```

- [ ] **Step 3: 验证**

```bash
npm run build
```

Expected: TypeScript 编译通过。

- [ ] **Step 4: Commit**

```bash
git add src/store/toastStore.ts src/components/NotificationToast.tsx
git commit -m "refactor: toast 自动清理交由 React useEffect 管理"
```

---

## Task 8: useTray listen 错误处理

**Files:**
- Modify: `src/hooks/useTray.ts:18-26`

- [ ] **Step 1: 改用 async + catch**

将 `src/hooks/useTray.ts:18-26`：

```typescript
useEffect(() => {
  const unlisten = listen('refresh-requested', () => {
    refreshMutationRef.current.mutate();
  });

  return () => {
    unlisten.then((f) => f());
  };
}, []);
```

改为：

```typescript
useEffect(() => {
  let cleanup: (() => void) | null = null;
  let cancelled = false;
  listen('refresh-requested', () => {
    refreshMutationRef.current.mutate();
  })
    .then((f) => {
      if (cancelled) f();
      else cleanup = f;
    })
    .catch((e) => console.error('useTray: failed to subscribe refresh-requested', e));

  return () => {
    cancelled = true;
    cleanup?.();
  };
}, []);
```

- [ ] **Step 2: 验证**

```bash
npm run build
```

Expected: 通过。

- [ ] **Step 3: Commit**

```bash
git add src/hooks/useTray.ts
git commit -m "fix: useTray listen promise 错误不再静默吞掉"
```

---

## Task 9: window_position 持久化（只读）

**Files:**
- Modify: `src-tauri/src/window.rs:1-52`

- [ ] **Step 1: 接受可选位置参数**

将 `src-tauri/src/window.rs` 整个文件替换为：

```rust
use tauri::{AppHandle, Manager, PhysicalPosition, WebviewWindow};
use tokio::sync::RwLockReadGuard;

use crate::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;

fn position_near_tray(window: &WebviewWindow) {
    if let Ok(Some(monitor)) = window.current_monitor() {
        let scale = monitor.scale_factor();
        let available = monitor.work_area();
        let window_size = match window.outer_size() {
            Ok(s) => s,
            Err(_) => return,
        };
        let margin = (16.0 * scale) as i32;
        let x = available.position.x + available.size.width as i32 - window_size.width as i32 - margin;
        let y = available.position.y + available.size.height as i32 - window_size.height as i32 - margin;
        let _ = window.set_position(tauri::Position::Physical(PhysicalPosition::new(x, y)));
    }
}

fn apply_known_position(window: &WebviewWindow, pos: (i32, i32)) {
    let _ = window.set_position(tauri::Position::Physical(PhysicalPosition::new(pos.0, pos.1)));
}

pub fn show_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        position_near_tray(&window);
    }
}

pub fn hide_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

pub fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => { let _ = window.hide(); }
            Ok(false) => {
                let _ = window.show();
                let _ = window.set_focus();

                if let Some(state) = app.try_state::<AppState>() {
                    let stored = {
                        let cfg = state.config.blocking_read();
                        cfg.settings.window_position
                    };
                    match stored {
                        Some(pos) => apply_known_position(&window, pos),
                        None => position_near_tray(&window),
                    }
                } else {
                    position_near_tray(&window);
                }
            }
            Err(_) => {}
        }
    }
}
```

注意：使用 `blocking_read` 因为 `toggle_window` 是同步函数。`AppState` 已经 `manage` 进了 app（`lib.rs`），`try_state` 一定能拿到。

- [ ] **Step 2: 验证**

```bash
cd src-tauri && cargo check
```

Expected: 通过。

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/window.rs
git commit -m "feat: window 打开时优先使用持久化位置"
```

---

## Task 10: AppState::new 错误处理

**Files:**
- Modify: `src-tauri/src/lib.rs:20-72`

- [ ] **Step 1: 改为在 setup 中初始化 AppState**

将 `src-tauri/src/lib.rs:20-72` 替换为：

```rust
use tauri::Manager;

pub mod commands;
pub mod config;
pub mod error;
pub mod http;
pub mod notification;
pub mod providers;
pub mod tray;
pub mod window;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub config: Arc<RwLock<config::AppConfig>>,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> crate::error::Result<Self> {
        Ok(Self {
            config: Arc::new(RwLock::new(config::read_config_sync()?)),
            http_client: http::create_client(),
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            commands::balance::get_balance,
            commands::balance::refresh_all_balances,
            commands::usage::get_usage,
            commands::providers::list_providers,
            commands::providers::add_provider,
            commands::providers::update_provider,
            commands::providers::delete_provider,
            commands::providers::toggle_provider,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::keychain::store_api_key,
            commands::keychain::retrieve_api_key,
            commands::keychain::delete_api_key,
            commands::keychain::has_api_key,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_state = AppState::new()
                .map_err(|e| Box::<dyn std::error::Error>::from(format!("init app state: {e}")))?;
            app.manage(app_state);

            tray::setup_tray(&app_handle)?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

要点：
- `AppState::new()` 移到 `setup` 闭包，错误转为 `Box<dyn Error>`（Tauri 接受此类型）
- `app.manage(...)` 在 `setup` 内调用
- single-instance 处理也改为 `if let Some` 避免 panic
- 不再在 builder 链上调用 `.manage(...)`

- [ ] **Step 2: 验证**

```bash
cd src-tauri && cargo check
```

Expected: 通过。

- [ ] **Step 3: 运行 cargo test**

```bash
cd src-tauri && cargo test --lib
```

Expected: 全部通过。

- [ ] **Step 4: 阶段 2 验证（双端）**

```bash
cd src-tauri && cargo check && cargo test --lib
npm run build
```

Expected: 全部通过。

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "refactor: AppState 初始化移入 setup，错误不再 panic"
```

---

## Task 11: 挂载 ProviderSettings 到 DetailPanel

**Files:**
- Modify: `src/components/DetailPanel.tsx:1-20`

- [ ] **Step 1: 在 header 引入编辑按钮**

将 `src/components/DetailPanel.tsx:1-10` 改为：

```typescript
import { useEffect, useState } from 'react';
import { useUIStore } from '../store/uiStore';
import { useBalance } from '../hooks/useBalance';
import { useUsage } from '../hooks/useUsage';
import { useProviders, useUpdateProvider } from '../hooks/useProviders';
import { BalanceDisplay } from './BalanceDisplay';
import { UsageChart } from './UsageChart';
import { MetricToggle } from './MetricToggle';
import { ProgressBar } from './ProgressBar';
import { GlassPanel } from './GlassPanel';
import { ProviderSettings } from './ProviderSettings';
import { ArrowLeft, Settings, Edit3 } from 'lucide-react';
```

- [ ] **Step 2: 在函数体内加入状态与数据**

在 `DetailPanel` 函数内 `const openSettings = useUIStore(...)` 之后添加：

```typescript
  const { data: providers } = useProviders();
  const updateProvider = useUpdateProvider();
  const [isEditing, setIsEditing] = useState(false);
  const selectedProvider = providers?.find((p) => p.id === selectedProviderId) ?? null;
```

- [ ] **Step 3: 替换 header JSX**

将 header 部分：

```typescript
        <button onClick={openSettings} className="glass-button p-2" aria-label="设置">
          <Settings className="w-4 h-4" />
        </button>
```

之前增加编辑按钮：

```typescript
        <button
          onClick={() => setIsEditing((v) => !v)}
          className="glass-button p-2"
          aria-label={isEditing ? '关闭编辑' : '编辑设置'}
        >
          <Edit3 className="w-4 h-4" />
        </button>
        <button onClick={openSettings} className="glass-button p-2" aria-label="设置">
          <Settings className="w-4 h-4" />
        </button>
```

- [ ] **Step 4: 在错误面板后插入编辑面板**

在 `</div>` 结束 content 之前（即错误 GlassPanel 之后）插入：

```typescript
        {isEditing && selectedProvider && (
          <GlassPanel>
            <h3 className="text-sm font-medium text-white/70 mb-4">编辑设置</h3>
            <ProviderSettings
              provider={selectedProvider}
              onUpdate={(p) => updateProvider.mutate({ id: p.id, provider: p })}
            />
          </GlassPanel>
        )}
```

- [ ] **Step 5: 验证**

```bash
npm run build
```

Expected: 通过。

- [ ] **Step 6: Commit**

```bash
git add src/components/DetailPanel.tsx
git commit -m "feat: DetailPanel 挂载 ProviderSettings"
```

---

## Task 12: ProviderSettings 输入 onBlur 提交

**Files:**
- Modify: `src/components/ProviderSettings.tsx`

- [ ] **Step 1: 重写为本地 state + onBlur**

将 `src/components/ProviderSettings.tsx` 整个文件替换为：

```typescript
import { useEffect, useState } from 'react';
import { ProviderConfig } from '../types';

interface ProviderSettingsProps {
  provider: ProviderConfig;
  onUpdate: (provider: ProviderConfig) => void;
}

interface FormState {
  display_name: string;
  api_base_url: string;
  warning_threshold_percent: number;
  refresh_interval_seconds: number;
}

function toForm(p: ProviderConfig): FormState {
  return {
    display_name: p.display_name,
    api_base_url: p.api_base_url,
    warning_threshold_percent: p.warning_threshold_percent,
    refresh_interval_seconds: p.refresh_interval_seconds,
  };
}

export function ProviderSettings({ provider, onUpdate }: ProviderSettingsProps) {
  const [form, setForm] = useState<FormState>(() => toForm(provider));

  useEffect(() => {
    setForm(toForm(provider));
  }, [provider]);

  const commit = (next: FormState) => {
    setForm(next);
    onUpdate({
      ...provider,
      display_name: next.display_name,
      api_base_url: next.api_base_url,
      warning_threshold_percent: next.warning_threshold_percent,
      refresh_interval_seconds: next.refresh_interval_seconds,
    });
  };

  return (
    <div className="space-y-4">
      <div>
        <label className="text-xs text-white/50 block mb-1">显示名称</label>
        <input
          type="text"
          className="glass-input"
          value={form.display_name}
          onChange={(e) => setForm((f) => ({ ...f, display_name: e.target.value }))}
          onBlur={() => commit(form)}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">API Base URL</label>
        <input
          type="text"
          className="glass-input"
          value={form.api_base_url}
          onChange={(e) => setForm((f) => ({ ...f, api_base_url: e.target.value }))}
          onBlur={() => commit(form)}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">警告阈值 (%)</label>
        <input
          type="number"
          className="glass-input"
          value={form.warning_threshold_percent}
          onChange={(e) => setForm((f) => ({ ...f, warning_threshold_percent: parseFloat(e.target.value) }))}
          onBlur={() => commit(form)}
          min={0}
          max={100}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">刷新间隔 (秒)</label>
        <input
          type="number"
          className="glass-input"
          value={form.refresh_interval_seconds}
          onChange={(e) => setForm((f) => ({ ...f, refresh_interval_seconds: parseInt(e.target.value) }))}
          onBlur={() => commit(form)}
          min={60}
        />
      </div>
    </div>
  );
}
```

- [ ] **Step 2: 验证**

```bash
npm run build
```

Expected: 通过。

- [ ] **Step 3: Commit**

```bash
git add src/components/ProviderSettings.tsx
git commit -m "fix: ProviderSettings 输入 onBlur 才提交，避免每个字符触发 mutation"
```

---

## Task 13: CI lint 步骤

**Files:**
- Modify: `.github/workflows/ci.yml:18-30`

- [ ] **Step 1: 在 test job 中加 lint 步骤**

将 `.github/workflows/ci.yml:18-30` 替换为：

```yaml
      - name: Install dependencies
        run: npm ci

      - name: Lint
        run: npm run lint

      - name: Build frontend
        run: npm run build
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: 在 test job 加入 npm run lint 步骤"
```

---

## Task 14: CSP 区分 dev / prod

**Files:**
- Modify: `src-tauri/tauri.conf.json:31-33`

- [ ] **Step 1: 拆分为 devCsp + 严格 prod csp**

将 `src-tauri/tauri.conf.json:31-33`：

```json
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; connect-src 'self' ipc: http://localhost:1420 https://api.deepseek.com https://api.openai.com https://api.anthropic.com https://openrouter.ai https://api.openrouter.ai; font-src 'self' https://fonts.gstatic.com data:; img-src 'self' data: blob:;"
    }
```

改为：

```json
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self' ipc: https://api.deepseek.com https://api.openai.com https://api.anthropic.com https://openrouter.ai https://api.openrouter.ai; font-src 'self' https://fonts.gstatic.com data:; img-src 'self' data: blob:;",
      "devCsp": "default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; connect-src 'self' ipc: http://localhost:1420 https://api.deepseek.com https://api.openai.com https://api.anthropic.com https://openrouter.ai https://api.openrouter.ai; font-src 'self' https://fonts.gstatic.com data:; img-src 'self' data: blob:;"
    }
```

要点：prod 去掉 `'unsafe-eval'` 和 `http://localhost:1420`；保留供应商 API。

- [ ] **Step 2: 验证（cargo check 不直接验证 CSP，但确保 schema 仍合法）**

```bash
cd src-tauri && cargo check
```

Expected: 通过（CSP 字段是字符串，schema 不强制枚举）。

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tauri.conf.json
git commit -m "fix: CSP 区分 dev 与 prod，prod 去掉 unsafe-eval 与 localhost"
```

---

## Task 15: OverviewPanel 减少重复 query

**Files:**
- Modify: `src/components/OverviewPanel.tsx:1-30`
- Modify: `src/hooks/useBalance.ts:16-27`

- [ ] **Step 1: 修改 useRefreshAllBalances 移除无效 invalidate**

将 `src/hooks/useBalance.ts:16-27`：

```typescript
export function useRefreshAllBalances() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => tauriInvoke<ProviderBalance[]>('refresh_all_balances'),
    onSuccess: (data) => {
      data.forEach((balance) => {
        queryClient.setQueryData([BALANCE_KEY, balance.provider_id], balance);
      });
      queryClient.invalidateQueries({ queryKey: [BALANCE_KEY] });
    },
  });
}
```

改为：

```typescript
export function useRefreshAllBalances() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => tauriInvoke<ProviderBalance[]>('refresh_all_balances'),
    onSuccess: (data) => {
      data.forEach((balance) => {
        queryClient.setQueryData([BALANCE_KEY, balance.provider_id], balance);
      });
    },
  });
}
```

要点：`setQueryData` 已经会通知所有订阅该 key 的组件重渲染；多余的 `invalidateQueries({ queryKey: [BALANCE_KEY] })` 会触发 N 次 `get_balance` 重拉，去掉。

- [ ] **Step 2: 重写 OverviewPanel 让 ProviderRow 订阅缓存但不主动 fetch**

将 `src/components/OverviewPanel.tsx:1-30` 替换为：

```typescript
import { useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useProviders } from '../hooks/useProviders';
import { useRefreshAllBalances } from '../hooks/useBalance';
import { useUIStore } from '../store/uiStore';
import { ProviderBalance, ProviderConfig } from '../types';
import { ProviderCard } from './ProviderCard';
import { GlassPanel } from './GlassPanel';
import { RefreshCw, Settings, Plus } from 'lucide-react';

function ProviderRow({
  provider,
  onClick,
}: {
  provider: ProviderConfig;
  onClick: () => void;
}) {
  // 订阅 balance 缓存但不主动 fetch；refresh_all_balances 的 setQueryData 会触发重渲染。
  const { data: balance } = useQuery<ProviderBalance | undefined>({
    queryKey: ['balance', provider.id],
    queryFn: () => undefined,
    enabled: false,
    staleTime: Infinity,
  });

  const providerBalance: ProviderBalance =
    balance ?? {
      provider_id: provider.id,
      provider_name: provider.display_name || provider.name,
      balance: null,
      error: null,
      last_updated: null,
    };
  return <ProviderCard provider={providerBalance} onClick={onClick} />;
}

export function OverviewPanel() {
  const { data: providers, isLoading } = useProviders();
  const refreshMutation = useRefreshAllBalances();
  const navigateToDetail = useUIStore((s) => s.navigateToDetail);
  const openSettings = useUIStore((s) => s.openSettings);
  const openAddProviderModal = useUIStore((s) => s.openAddProviderModal);

  useEffect(() => {
    if (providers && providers.length > 0) {
      refreshMutation.mutate();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [providers?.length]);

  const handleRefresh = () => {
    refreshMutation.mutate();
  };

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between p-4">
        <div>
          <h1 className="text-lg font-semibold text-white/90">Pulse</h1>
          <p className="text-[11px] uppercase tracking-[0.08em] text-white/40">所有供应商</p>
        </div>
        <div className="flex gap-1">
          <button onClick={handleRefresh} className="glass-button p-2" disabled={refreshMutation.isPending} aria-label="刷新">
            <RefreshCw className={`w-4 h-4 ${refreshMutation.isPending ? 'animate-spin' : ''}`} />
          </button>
          <button onClick={openAddProviderModal} className="glass-button p-2" aria-label="添加供应商">
            <Plus className="w-4 h-4" />
          </button>
          <button onClick={openSettings} className="glass-button p-2" aria-label="设置">
            <Settings className="w-4 h-4" />
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto px-4 pb-4">
        <GlassPanel padding="sm">
          {isLoading && (
            <div className="text-center py-8 text-white/40">加载中...</div>
          )}

          {providers?.map((provider) => (
            <ProviderRow
              key={provider.id}
              provider={provider}
              onClick={() => navigateToDetail(provider.id)}
            />
          ))}

          {providers?.length === 0 && (
            <div className="text-center py-8">
              <p className="text-white/40 mb-3">暂无供应商</p>
              <button onClick={openAddProviderModal} className="glass-button">
                <Plus className="w-4 h-4 inline mr-1" />
                添加供应商
              </button>
            </div>
          )}
        </GlassPanel>
      </div>
    </div>
  );
}
```

要点：使用 `useQuery({ enabled: false, queryFn: () => undefined })` 来**订阅**缓存但不**拉取**。`refresh_all_balances.onSuccess` 的 `setQueryData` 触发订阅者重渲染，无 N+1。

- [ ] **Step 3: 验证**

```bash
npm run build
```

Expected: 通过。

- [ ] **Step 4: Commit**

```bash
git add src/components/OverviewPanel.tsx src/hooks/useBalance.ts
git commit -m "perf: OverviewPanel 用 useQuery 订阅缓存，去掉 N+1 拉取"
```

---

## Task 16: custom provider URL 处理单测

**Files:**
- Modify: `src-tauri/src/providers/custom.rs`

- [ ] **Step 1: 抽出 normalize 函数**

将 `src-tauri/src/providers/custom.rs` 中 `get_balance` 函数体内：

```rust
        let url = format!("{}/user/balance", self.api_base_url.trim_end_matches('/'));
```

改为调用 `normalize_base_url`：

```rust
        let url = format!("{}/user/balance", normalize_base_url(&self.api_base_url));
```

在文件顶部（use 之后）添加函数：

```rust
/// 去掉 base URL 末尾的 `/`，最多 trim 多个连续斜杠。
pub fn normalize_base_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}
```

- [ ] **Step 2: 在 tests 模块添加单测**

将 `src-tauri/src/providers/custom.rs` 末尾的 `#[cfg(test)] mod tests` 替换为：

```rust
#[cfg(test)]
mod tests {
    use super::normalize_base_url;

    #[test]
    fn normalize_no_trailing_slash() {
        assert_eq!(normalize_base_url("https://x.com"), "https://x.com");
    }

    #[test]
    fn normalize_single_trailing_slash() {
        assert_eq!(normalize_base_url("https://x.com/"), "https://x.com");
    }

    #[test]
    fn normalize_multiple_trailing_slashes() {
        assert_eq!(normalize_base_url("https://x.com///"), "https://x.com");
    }

    #[test]
    fn normalize_empty() {
        assert_eq!(normalize_base_url(""), "");
    }

    #[test]
    fn normalize_internal_slash_preserved() {
        assert_eq!(
            normalize_base_url("https://api.example.com/v1/"),
            "https://api.example.com/v1"
        );
    }
}
```

- [ ] **Step 3: 验证**

```bash
cd src-tauri && cargo test --lib custom
```

Expected: 5 个测试全部通过。

- [ ] **Step 4: 阶段 3 验证（双端）**

```bash
cd src-tauri && cargo check && cargo test --lib
npm run build
```

Expected: 全部通过。

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/providers/custom.rs
git commit -m "refactor: custom provider URL 末尾斜杠处理抽出函数，加单测"
```

---

## 阶段 3 完成

执行完 Task 16 后，所有 3 阶段（16 个任务）结束。

## 全部验证（最终）

```bash
cd src-tauri && cargo check && cargo test --lib
npm run build
npm run lint
```

Expected: 全部通过。

## 不做

- commit / push / tag（除非用户明确要求）
- 修改 Cargo.lock / package-lock.json
- 写 e2e 测试
- 新增依赖
