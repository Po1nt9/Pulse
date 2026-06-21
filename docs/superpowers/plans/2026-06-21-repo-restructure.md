# Pulse 仓库重组实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把根目录的 `src/`、`src-tauri/`、9 个前端配置文件和项目元数据，整理成 `apps/web/` + `apps/tauri/` + `docs/` 的单仓布局，保留 git history，所有现有命令能继续工作。

**Architecture:** 单根 `package.json`（不拆 workspace，避免 hoist / lockfile 复杂度）；`apps/web/` 自带前端工具链配置（vite/tsconfig/eslint/tailwind/postcss）；`apps/tauri/` 保持 Tauri 目录布局，配置文件用相对路径。所有 npm scripts 用显式 `--config` / `-p` 从根调用，避免 `cd` 跨平台问题。

**Tech Stack:** Tauri v2, Vite, React, TypeScript, ESLint 9 flat config, Prettier, Tailwind, Rust, GitHub Actions

---

## 文件结构总览

**创建：**
- `apps/web/` — 前端（Vite 根目录）
- `apps/tauri/` — Tauri/Rust 后端
- `docs/` — 项目文档

**移动（`git mv` 保留 history）：**
- `src/` → `apps/web/src/`
- `src-tauri/` → `apps/tauri/`
- `index.html`、`vite.config.ts`、`tsconfig.json`、`tsconfig.node.json`、`tailwind.config.js`、`postcss.config.js`、`eslint.config.js` → `apps/web/`
- `CHANGELOG.md`、`LICENSE` → `docs/`

**修改（路径/配置调整）：**
- `package.json` — scripts
- `apps/tauri/tauri.conf.json` — `frontendDist`
- `apps/web/vite.config.ts` — `watch.ignored`
- `apps/web/eslint.config.js` — 加 `root: true`、`ignores`
- `.gitignore` — `src-tauri/{target,gen}` → `apps/tauri/{target,gen}`
- `.prettierignore` — `src-tauri/` → `apps/tauri/`
- `.github/CODEOWNERS` — 路径前缀
- `.github/dependabot.yml` — cargo directory
- `.github/workflows/ci.yml` — working-directory、workspaces、占位 dist
- `.github/workflows/release.yml` — workspaces、releaseBody 链接
- `README.md` — 结构树、CHANGELOG 链接、构建产物路径

**保持原位（无需改）：**
- 所有 `.ts/.tsx/.rs` 源码（路径都是相对的）
- `apps/web/src/` / `apps/tauri/src/` 内部子目录
- `.github/` 根目录
- `.editorconfig`、`.prettierrc.json` 根目录
- `apps/web/tsconfig.json`、`tsconfig.node.json`、`tailwind.config.js`、`postcss.config.js`、`index.html` 内部相对路径（config 位置变了，路径自动从新位置解析）

---

## Task 1: 移动文件（保留 git history）

**Files:**
- Move: `src/`, `src-tauri/`, `index.html`, `vite.config.ts`, `tsconfig.json`, `tsconfig.node.json`, `tailwind.config.js`, `postcss.config.js`, `eslint.config.js`, `CHANGELOG.md`, `LICENSE`

- [ ] **Step 1.1: 创建目标目录**

```bash
cd F:/Coding/Pulse
mkdir -p apps/web apps/tauri docs
```

Expected: 无输出。`ls apps/` 显示 `web/ tauri/`，`ls docs/` 显示空。

- [ ] **Step 1.2: 移动前端源码与配置到 apps/web/**

```bash
cd F:/Coding/Pulse
git mv src apps/web/src
git mv index.html apps/web/index.html
git mv vite.config.ts apps/web/vite.config.ts
git mv tsconfig.json apps/web/tsconfig.json
git mv tsconfig.node.json apps/web/tsconfig.node.json
git mv tailwind.config.js apps/web/tailwind.config.js
git mv postcss.config.js apps/web/postcss.config.js
git mv eslint.config.js apps/web/eslint.config.js
```

Expected: 每条 `git mv` 输出 `rename ...` 信息。最终 `ls apps/web/` 包含 `src/`、`index.html`、所有 `.config.*` / `tsconfig.*` 文件。

- [ ] **Step 1.3: 移动 Tauri 后端到 apps/tauri/**

```bash
cd F:/Coding/Pulse
git mv src-tauri apps/tauri
```

Expected: `rename src-tauri -> apps/tauri`。`ls apps/tauri/` 包含 `src/`、`commands/`、`providers/`、`capabilities/`、`icons/`、`Cargo.toml`、`Cargo.lock`、`build.rs`、`tauri.conf.json`。

- [ ] **Step 1.4: 移动项目文档到 docs/**

```bash
cd F:/Coding/Pulse
git mv CHANGELOG.md docs/CHANGELOG.md
git mv LICENSE docs/LICENSE
```

Expected: 两条 rename 信息。`ls docs/` 包含 `CHANGELOG.md`、`LICENSE`、`superpowers/`（之前 commit 进去的）。

- [ ] **Step 1.5: 暂存并提交移动操作**

```bash
cd F:/Coding/Pulse
git status
git commit -m "chore: 将源码与配置移入 apps/ 和 docs/ 目录"
```

Expected: `git status` 干净，无未提交修改。`git commit` 输出 `[master <hash>] chore: ...` 并列出多个 `rename` 行。

---

## Task 2: 更新前端配置（vite / eslint）

**Files:**
- Modify: `apps/web/vite.config.ts`, `apps/web/eslint.config.js`

**说明：** `tsconfig.json`、`tsconfig.node.json`、`tailwind.config.js`、`postcss.config.js`、`index.html` 内部路径都是相对 config 文件位置，**无需修改**——移动后它们自动从 `apps/web/` 出发解析。

- [ ] **Step 2.1: 更新 vite.config.ts 的 watch.ignored**

打开 `apps/web/vite.config.ts`，把 `server.watch.ignored` 从：

```ts
ignored: ["**/src-tauri/**"],
```

改为：

```ts
ignored: ["../tauri/**"],
```

其他内容（plugins、clearScreen、port、build.outDir、build.sourcemap）保持不变。

- [ ] **Step 2.2: 更新 eslint.config.js：加 root: true 并改 ignores**

打开 `apps/web/eslint.config.js`，在 `tseslint.config(...)` 的**第一个对象**里：

- 加 `"root": true`（阻止 ESLint 向父目录搜索 config）
- 把 `'src-tauri/'` 改为 `'../tauri/'`

修改后的第一个对象应为：

```js
{
  root: true,
  ignores: ['node_modules/', 'dist/', '../tauri/', '*.config.js'],
},
```

后面所有对象（js.configs.recommended、tseslint.configs.recommended、files plugins、languageOptions、rules、settings）保持不变。

- [ ] **Step 2.3: 提交前端配置更新**

```bash
cd F:/Coding/Pulse
git add apps/web/vite.config.ts apps/web/eslint.config.js
git commit -m "chore(apps/web): 调整 vite watch 与 eslint 忽略路径"
```

---

## Task 3: 更新 Tauri 配置

**Files:**
- Modify: `apps/tauri/tauri.conf.json`

- [ ] **Step 3.1: 更新 frontendDist 路径**

打开 `apps/tauri/tauri.conf.json`，找到 `build` 对象：

```json
"build": {
  "frontendDist": "../dist",
  "devUrl": "http://localhost:1420",
  "beforeDevCommand": "npm run dev",
  "beforeBuildCommand": "npm run build"
}
```

改为：

```json
"build": {
  "frontendDist": "../web/dist",
  "devUrl": "http://localhost:1420",
  "beforeDevCommand": "npm run dev",
  "beforeBuildCommand": "npm run build"
}
```

`beforeDevCommand` / `beforeBuildCommand` 保持不变——这些命令在调用 `tauri` 的 CWD（即仓库根）执行，根 `package.json` 的 `dev` / `build` scripts 已用 `--config` 指向 `apps/web/`。

- [ ] **Step 3.2: 提交 Tauri 配置更新**

```bash
cd F:/Coding/Pulse
git add apps/tauri/tauri.conf.json
git commit -m "chore(apps/tauri): frontendDist 改为 ../web/dist"
```

---

## Task 4: 更新根 package.json scripts

**Files:**
- Modify: `package.json`

- [ ] **Step 4.1: 替换 scripts 段**

打开 `package.json`，把整个 `scripts` 对象替换为：

```json
"scripts": {
  "dev": "vite --config apps/web/vite.config.ts",
  "build": "tsc -p apps/web && vite build --config apps/web/vite.config.ts",
  "preview": "vite preview --config apps/web/vite.config.ts",
  "lint": "eslint --config apps/web/eslint.config.js apps/web/src",
  "typecheck": "tsc --noEmit -p apps/web",
  "format": "prettier --write \"apps/web/src/**/*.{ts,tsx,css,html}\"",
  "test": "npm run lint",
  "tauri": "tauri --config apps/tauri/tauri.conf.json",
  "tauri:dev": "tauri dev --config apps/tauri/tauri.conf.json",
  "tauri:build": "tauri build --config apps/tauri/tauri.conf.json"
}
```

`name` / `version` / `private` / `type` / `dependencies` / `devDependencies` 段**完全不动**。

`test` 保留为 `npm run lint` 别名（与原行为一致）。

- [ ] **Step 4.2: 提交 package.json 更新**

```bash
cd F:/Coding/Pulse
git add package.json
git commit -m "chore: 更新 npm scripts 指向 apps/ 下配置"
```

---

## Task 5: 更新根级 ignore 文件

**Files:**
- Modify: `.gitignore`, `.prettierignore`

- [ ] **Step 5.1: 更新 .gitignore 的 src-tauri 路径**

打开 `.gitignore`，找到：

```
src-tauri/target/
src-tauri/gen/
```

替换为：

```
apps/tauri/target/
apps/tauri/gen/
```

其他行（`node_modules/`、`dist/`、`.env`、`*.log`、`.vscode/` 等）保持不变。

- [ ] **Step 5.2: 更新 .prettierignore 的 src-tauri 路径**

打开 `.prettierignore`，把：

```
src-tauri/
```

改为：

```
apps/tauri/
```

`node_modules/`、`dist/`、`package-lock.json`、`*.lock` 保持不变。

- [ ] **Step 5.3: 提交 ignore 文件更新**

```bash
cd F:/Coding/Pulse
git add .gitignore .prettierignore
git commit -m "chore: ignore 路径更新 src-tauri → apps/tauri"
```

---

## Task 6: 更新 .github 配置

**Files:**
- Modify: `.github/CODEOWNERS`, `.github/dependabot.yml`, `.github/workflows/ci.yml`, `.github/workflows/release.yml`

- [ ] **Step 6.1: 更新 CODEOWNERS 路径前缀**

打开 `.github/CODEOWNERS`，逐行替换（**仅改路径，`@Po1nt9` 不动**）：

| 原行 | 新行 |
|------|------|
| `/src/**/*.tsx @Po1nt9` | `/apps/web/src/**/*.tsx @Po1nt9` |
| `/src/**/*.ts @Po1nt9` | `/apps/web/src/**/*.ts @Po1nt9` |
| `/src-tauri/**/*.rs @Po1nt9` | `/apps/tauri/**/*.rs @Po1nt9` |
| `/src-tauri/Cargo.toml @Po1nt9` | `/apps/tauri/Cargo.toml @Po1nt9` |
| `/*.config.js @Po1nt9` | `/apps/web/*.config.js @Po1nt9` |

`/docs/**/*.md` 和 `/*.md` 仍匹配新位置（`docs/` 在根、`README.md` 在根），无需改。

- [ ] **Step 6.2: 更新 dependabot.yml 的 cargo directory**

打开 `.github/dependabot.yml`，把：

```yaml
directory: "/src-tauri"
```

改为：

```yaml
directory: "/apps/tauri"
```

其他字段（schedule、reviewers、labels、commit-message）保持不变。

- [ ] **Step 6.3: 更新 ci.yml（4 处 + 占位 dist）**

打开 `.github/workflows/ci.yml`，做 6 处替换：

1. test job 的 rust-cache 配置：`workspaces: 'src-tauri'` → `workspaces: 'apps/tauri'`
2. test job：`working-directory: src-tauri` → `working-directory: apps/tauri`
3. build job 的 rust-cache 配置：`workspaces: 'src-tauri'` → `workspaces: 'apps/tauri'`
4. build job：`working-directory: src-tauri` → `working-directory: apps/tauri`
5. "Create placeholder frontend dist" 的 `mkdir -p dist` → `mkdir -p apps/web/dist`
6. 同一步骤的 `> dist/index.html` → `> apps/web/dist/index.html`

修改后的 "Create placeholder frontend dist" 步骤应为：

```yaml
      - name: Create placeholder frontend dist
        run: |
          mkdir -p apps/web/dist
          echo '<!DOCTYPE html><html><head><title>placeholder</title></head><body></body></html>' > apps/web/dist/index.html
```

- [ ] **Step 6.4: 更新 release.yml（workspaces + releaseBody）**

打开 `.github/workflows/release.yml`，做 2 处替换：

1. rust-cache 配置：`workspaces: 'src-tauri'` → `workspaces: 'apps/tauri'`
2. releaseBody 链接（保留 `v0.1.0` 那种版本号，仅改路径）：

```yaml
releaseBody: 'See [CHANGELOG.md](https://github.com/Po1nt9/Pulse/blob/master/CHANGELOG.md) for changes.'
```

改为：

```yaml
releaseBody: 'See [CHANGELOG.md](https://github.com/Po1nt9/Pulse/blob/master/docs/CHANGELOG.md) for changes.'
```

- [ ] **Step 6.5: 提交 .github 更新**

```bash
cd F:/Coding/Pulse
git add .github/
git commit -m "ci: 更新 github 配置路径"
```

---

## Task 7: 更新 README

**Files:**
- Modify: `README.md`

- [ ] **Step 7.1: 更新项目结构树**

打开 `README.md`，找到 `## 项目结构` 段落（约第 59-86 行），整段 `\`\`\` ... \`\`\`` 代码块替换为：

````markdown
```
.
├── apps/
│   ├── web/                        # 前端（Vite + React + TS）
│   │   ├── src/
│   │   │   ├── components/         # React 组件
│   │   │   ├── hooks/              # React Query Hooks
│   │   │   ├── store/              # Zustand Store
│   │   │   ├── types/              # TypeScript 类型定义
│   │   │   ├── utils/              # 工具函数
│   │   │   ├── App.tsx             # 应用根组件
│   │   │   └── main.tsx            # 入口文件
│   │   ├── index.html
│   │   ├── vite.config.ts
│   │   ├── tsconfig.json
│   │   ├── tailwind.config.js
│   │   └── eslint.config.js
│   └── tauri/                      # Tauri / Rust 后端
│       ├── src/
│       │   ├── commands/           # Tauri Commands
│       │   ├── providers/          # Provider Adapters
│       │   ├── error.rs            # 错误处理
│       │   ├── config.rs           # 配置管理
│       │   ├── http.rs             # HTTP Client
│       │   ├── keychain.rs         # Keychain 存储
│       │   ├── tray.rs             # 托盘图标管理
│       │   ├── window.rs           # 窗口管理
│       │   └── lib.rs              # 库入口
│       ├── icons/                  # 应用图标
│       └── Cargo.toml              # Rust 依赖
├── docs/                           # 项目文档
└── package.json                    # 前端配置（单根包）
```
````

- [ ] **Step 7.2: 更新 CHANGELOG 链接**

打开 `README.md`，找到文末：

```markdown
详见 [CHANGELOG.md](CHANGELOG.md)
```

改为：

```markdown
详见 [CHANGELOG.md](docs/CHANGELOG.md)
```

- [ ] **Step 7.3: 更新构建产物路径**

打开 `README.md`，找到：

```
构建产物位于 `src-tauri/target/release/` 目录。
```

改为：

```
构建产物位于 `apps/tauri/target/release/` 目录。
```

- [ ] **Step 7.4: 提交 README 更新**

```bash
cd F:/Coding/Pulse
git add README.md
git commit -m "docs: README 路径与结构树更新"
```

---

## Task 8: 验证

**Files:** 无（纯验证步骤）

- [ ] **Step 8.1: 重新安装依赖**

```bash
cd F:/Coding/Pulse
npm install
```

Expected: 安装成功。如果有 `ENOENT` 或版本冲突报错，按错误信息排查（很可能是某个依赖包名写错——但本次未改 dependencies，所以不应有此类错误）。

- [ ] **Step 8.2: typecheck 通过**

```bash
cd F:/Coding/Pulse
npm run typecheck
```

Expected: 无错误输出（exit code 0）。如有 TS 报错，多半是某个 import 路径在移动后解析失败——按错误信息回退到对应源文件修。

- [ ] **Step 8.3: lint 通过**

```bash
cd F:/Coding/Pulse
npm run lint
```

Expected: 无错误输出。配置设了 `--max-warnings 0`，任何 warning 都会失败。如果出现 `Definition for rule ... was not found` 之类的报错，说明 eslint.config.js 的语法或加载路径有问题，回退到 Task 2。

- [ ] **Step 8.4: build 通过**

```bash
cd F:/Coding/Pulse
npm run build
```

Expected: 成功，生成 `apps/web/dist/index.html`。验证产物存在：

```bash
ls F:/Coding/Pulse/apps/web/dist/index.html
```

应输出该文件路径。

- [ ] **Step 8.5: Rust 编译通过**

```bash
cd F:/Coding/Pulse/apps/tauri
cargo check
```

Expected: `Finished` 状态（可能是 `dev` profile），无 error。Rust 模块路径都是相对的，理论上无需调整；但 `tauri::generate_context!()` 宏会读 `tauri.conf.json`——已移到 `apps/tauri/`，与 `Cargo.toml` 同目录，所以仍能解析。

- [ ] **Step 8.6: 验证最终 git 状态**

```bash
cd F:/Coding/Pulse
git status
git log --oneline -10
```

Expected: `git status` 干净（只可能有未追踪的 `node_modules/`、`apps/web/dist/`、`apps/tauri/target/`，这些都在 .gitignore 中）。`git log` 应展示本次重组的 7 个 commit（Task 1、2、3、4、5、6、7 各一个），加上 Task 8 不需要 commit（仅验证）。

---

## 自审检查

- **Spec 覆盖**：每个 spec 中的"修改"项都映射到了 Task 1-7；每个 spec 中的"不在范围"项在"保持原位"和 Task 内部说明中重申。
- **占位符扫描**：全文无 TBD/TODO；所有命令完整；所有代码块是可直接复制粘贴的。
- **类型/路径一致性**：所有路径 `apps/web/`、`apps/tauri/`、`docs/` 在每个 Task 中一致；`apps/web/eslint.config.js` 中的 `root: true` 与 `--config apps/web/eslint.config.js` 配套；`tauri --config apps/tauri/tauri.conf.json` 与 `apps/tauri/tauri.conf.json` 中 `frontendDist: "../web/dist"` 配套。