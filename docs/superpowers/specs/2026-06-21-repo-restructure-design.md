# Pulse 仓库重组设计

**日期**: 2026-06-21
**状态**: Approved
**目标**: 把 `src/` 与 `src-tauri/` 平级堆在根目录的局面，整理成 `apps/` 单仓布局

## 背景

`Pulse` 是 Tauri v2 + Vite/React 项目。当前根目录同时挂着：

- 前端源码：`src/`
- Rust 后端：`src-tauri/`
- 9 个配置/编辑器文件：`eslint.config.js` / `vite.config.ts` / `tailwind.config.js` / `postcss.config.js` / `tsconfig.json` / `tsconfig.node.json` / `.editorconfig` / `.prettierrc.json` / `.prettierignore`
- 3 个项目元数据：`README.md` / `CHANGELOG.md` / `LICENSE`
- `package.json` / `package-lock.json` / `index.html` / `.gitignore` / `.github/`

维护者觉得"挤"，但其中大部分是工具链的硬约定（Vite/TS/ESLint/Tauri/GitHub/git 各按根目录约定工作），真正能挪动的很少。

## 目标结构

```
.
├── apps/
│   ├── web/                  # 前端（Vite + React + TS）
│   │   ├── src/              # 原 src/ 内容，原样
│   │   ├── index.html
│   │   ├── vite.config.ts
│   │   ├── tsconfig.json
│   │   ├── tsconfig.node.json
│   │   ├── tailwind.config.js
│   │   ├── postcss.config.js
│   │   └── eslint.config.js
│   └── tauri/                # 后端（Tauri + Rust）
│       ├── src/              # 原 src-tauri/src/ 内容，原样
│       ├── commands/
│       ├── providers/
│       ├── capabilities/
│       ├── icons/
│       ├── Cargo.toml
│       ├── Cargo.lock
│       ├── build.rs
│       └── tauri.conf.json
├── docs/                     # 项目文档
│   ├── CHANGELOG.md
│   └── LICENSE
├── .github/                  # 保持根（GitHub 硬约定）
├── .gitignore
├── .editorconfig
├── .prettierrc.json
├── .prettierignore
├── README.md                 # 保持根（GitHub 直读）
└── package.json              # 单根包，无 workspace 复杂度
```

## 不在范围内

- 不拆分 `package.json` 为多 workspace（避免 hoist / lockfile 复杂度）
- 不动 `src/` 或 `src-tauri/src/` 内部结构
- 不重写 README 内容，只同步路径

## 详细改动

### 移动（保留 git history）

| 原路径 | 新路径 |
|--------|--------|
| `src/` | `apps/web/src/` |
| `index.html` | `apps/web/index.html` |
| `vite.config.ts` | `apps/web/vite.config.ts` |
| `tsconfig.json` | `apps/web/tsconfig.json` |
| `tsconfig.node.json` | `apps/web/tsconfig.node.json` |
| `tailwind.config.js` | `apps/web/tailwind.config.js` |
| `postcss.config.js` | `apps/web/postcss.config.js` |
| `eslint.config.js` | `apps/web/eslint.config.js` |
| `src-tauri/` | `apps/tauri/` |
| `CHANGELOG.md` | `docs/CHANGELOG.md` |
| `LICENSE` | `docs/LICENSE` |

### 修改

**`package.json`** — scripts 改用显式 `--config` / `-p`：

```json
{
  "scripts": {
    "dev": "vite --config apps/web/vite.config.ts",
    "build": "tsc -p apps/web && vite build --config apps/web/vite.config.ts",
    "preview": "vite preview --config apps/web/vite.config.ts",
    "lint": "eslint --config apps/web/eslint.config.js apps/web/src",
    "typecheck": "tsc --noEmit -p apps/web",
    "format": "prettier --write \"apps/web/src/**/*.{ts,tsx,css,html}\"",
    "tauri": "tauri --config apps/tauri/tauri.conf.json",
    "tauri:dev": "tauri dev --config apps/tauri/tauri.conf.json",
    "tauri:build": "tauri build --config apps/tauri/tauri.conf.json"
  }
}
```

**`apps/tauri/tauri.conf.json`** — `build.frontendDist` 从 `"../dist"` 改为 `"../web/dist"`。`beforeDevCommand`/`beforeBuildCommand` 保持 `npm run dev` / `npm run build`（这些命令在根 npm scripts 中，CLI 从 CWD 根调用）。

**`apps/web/vite.config.ts`** — `server.watch.ignored: ["**/src-tauri/**"]` → `["../tauri/**"]`（chokidar glob，从 vite 根 `apps/web/` 出发）。

**`apps/web/eslint.config.js`** — 加 `root: true` 阻止向上搜索；`ignores` 中 `'src-tauri/'` → `'../tauri/'`。

**`apps/web/tsconfig.json`** — `include: ["src"]` / `paths: {"@/*": ["src/*"]}` / `baseUrl: "."` 全部保持相对，从 `apps/web/` 出发解析，无需改。

**`apps/web/index.html`** — `<script src="/src/main.tsx">` 不动；Vite 自动从 vite 根（`apps/web/`）解析 `/src/main.tsx` → `apps/web/src/main.tsx`。

**`apps/web/tailwind.config.js`** — `content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"]` 相对 config 文件位置，无需改。

**`.gitignore`** — `src-tauri/target/` → `apps/tauri/target/`；`src-tauri/gen/` → `apps/tauri/gen/`。

**`.prettierignore`** — `src-tauri/` → `apps/tauri/`。

**`.github/workflows/ci.yml`**：
- `workspaces: 'src-tauri'`（两处，rust-cache）→ `'apps/tauri'`
- `working-directory: src-tauri`（两处，test + build）→ `apps/tauri`
- 占位 dist 创建：`mkdir -p dist` → `mkdir -p apps/web/dist`；`> dist/index.html` → `> apps/web/dist/index.html`

**`.github/workflows/release.yml`**：
- `workspaces: 'src-tauri'` → `'apps/tauri'`
- `releaseBody` 中 `CHANGELOG.md` 链接 → `docs/CHANGELOG.md`

**`.github/CODEOWNERS`**：
- `/src/**` → `/apps/web/src/**`
- `/src-tauri/**` → `/apps/tauri/**`
- `/src-tauri/Cargo.toml` → `/apps/tauri/Cargo.toml`
- `/*.config.js` → `/apps/web/*.config.js`

**`.github/dependabot.yml`**：
- cargo `directory: "/src-tauri"` → `"/apps/tauri"`

**`README.md`**：
- 项目结构树整体更新
- `[CHANGELOG.md](CHANGELOG.md)` → `[CHANGELOG.md](docs/CHANGELOG.md)`
- "构建产物位于 `src-tauri/target/release/`" → "构建产物位于 `apps/tauri/target/release/`"

### 不动的部分

- 所有 `.ts/.tsx/.rs` 源码（内部引用都是相对的）
- `apps/web/src/` / `apps/tauri/src/` 子目录结构原样保留
- `.editorconfig` / `.prettierrc.json` 留在根（编辑器/Prettier 上行查找约定）

## 风险与缓解

| 风险 | 缓解 |
|------|------|
| Tauri CLI 默认找 `src-tauri/`，移动后定位失败 | scripts 全部显式 `--config apps/tauri/tauri.conf.json`；CLI 从 config 父目录推断 app dir |
| Vite `watch.ignored` 路径写错，dev 时重复触发 Tauri 编译 | 用 `../tauri/**`（chokidar glob 从 vite 根出发） |
| ESLint 在 apps/web/ 外查找配置 | 加 `root: true` 阻止向上搜索 |
| `git mv` 丢失历史 | 全部用 `git mv` 移动；如已用 `mv` 移动则需 `git add -u` 配合 |
| CI rust-cache 缓存键失效 | 自动重建，无害 |
| Dependabot cargo 路径变更后 PR 错位 | dependabot 自适应新路径，提交者需审 PR title/scope |

## 验证计划（必须全过）

1. `git mv` 全部文件路径（保留 history）
2. `npm install` 通过
3. `npm run typecheck` 通过（`tsc --noEmit -p apps/web`）
4. `npm run lint` 通过（`eslint --config apps/web/eslint.config.js apps/web/src`，0 warning）
5. `npm run build` 通过（产出 `apps/web/dist/index.html`）
6. `cd apps/tauri && cargo check` 通过（Rust 编译通过，模块路径不变）
7. README 结构树更新，链接指向 `docs/CHANGELOG.md`

**不在本环境验证**：GUI 相关的 `npm run tauri:dev` / `npm run tauri:build` / `cargo test`（依赖 Linux GUI 库，CI 兜底）。