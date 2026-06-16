# Tasks

## 阶段 0：准备
- [x] Task 0.1: 拉取 PR #2 特性分支到本地。`git fetch origin trae/solo-agent-xH8xyu` 并创建本地跟踪分支；确认本地 `master` 与 `origin/master` 一致。
- [x] Task 0.2: 确认 GitHub REST API 可用（环境无 `gh` CLI），记录 token 与 repo（`Po1nt9/Pulse`）供后续合并/删分支/查 CI 使用。

## 阶段 1：并行审核 PR #2（4 个领域并发）
- [x] Task 1.1: 审核领域 A — Rust 后端改动。覆盖 `src-tauri/src/commands/{balance.rs,provider_key.rs}`、`config.rs`、`error.rs`、`http.rs`、`tray.rs`、`window.rs`、`Cargo.toml`、`Cargo.lock`。重点：async trait 移除是否正确、`refresh_all_balances` 并发与锁跨 await、`error.rs` serde 简化是否丢信息、`window.rs` blocking_read 修复、新增 23 个测试的有效性。产出问题清单（critical/major/minor + 修复建议）。
- [x] Task 1.2: 审核领域 B — 前端改动。覆盖 `src/components/NotificationToast.tsx`、`src/hooks/useSettings.ts`、`src/main.tsx`、`src/types/index.ts`、`package.json`。重点：NotificationToast 双重清理修复、useSettings 冗余 invalidateQueries 移除、React Query 配置、依赖变更合理性。产出问题清单。
- [x] Task 1.3: 审核领域 C — CI/CD 与仓库配置改动。覆盖 `.github/workflows/{ci.yml,release.yml}`、`.github/{CODEOWNERS,dependabot.yml,labels.yml,ISSUE_TEMPLATE/*,PULL_REQUEST_TEMPLATE.md}`、`eslint.config.js`、`.prettierrc.json`、`.prettierignore`、`.editorconfig`、`.dockerignore`、`.gitignore`、`LICENSE`。重点：workflow 语法与安全性（权限最小化、pin 版本）、dependabot 配置、模板可用性。产出问题清单。
- [x] Task 1.4: 审核领域 D — 文档与临时文件改动。覆盖 `AGENTS.md`（+1000 行）、`AGENTS_IMPROVEMENT_SUMMARY.md`、`pr_body.md`、`sync_labels.sh`、`.trae/specs/github-repo-cleanup-pr/`、`README.md`、`CHANGELOG.md`。重点：AGENTS.md 内容质量与重复、识别不应进入主干的临时产物、CHANGELOG 是否更新。产出问题清单与"应从 PR 移除的文件"清单。

## 阶段 2：修复审核发现的问题（按领域并发，依赖阶段 1）
- [x] Task 2.1: 在 PR 特性分支上修复领域 A（Rust）问题。修复后本地运行 `cargo check` 与 `cargo test` 验证。
- [x] Task 2.2: 在 PR 特性分支上修复领域 B（前端）问题。修复后本地运行 `npm run lint` 与 `npm run build` 验证。
- [x] Task 2.3: 在 PR 特性分支上修复领域 C（CI/配置）问题。
- [x] Task 2.4: 在 PR 特性分支上移除/修复领域 D 识别出的临时产物（删除 `pr_body.md`、`sync_labels.sh`、`AGENTS_IMPROVEMENT_SUMMARY.md`、`.trae/specs/github-repo-cleanup-pr/` 等不应入主干的文件），核对 AGENTS.md。
- [x] Task 2.5: 汇总各领域修复，在 PR 特性分支上提交（conventional commit），推送到 `origin/trae/solo-agent-xH8xyu`。

## 阶段 3：合并 PR #2 到 master（依赖阶段 2）
- [x] Task 3.1: 轮询 PR #2 的 CI check-runs，直到 Lint 与 Test 均 conclusion=success（或失败则回到阶段 2）。
- [x] Task 3.2: 通过 REST API 提交 PR #2 review（APPROVE）并留下审核摘要评论。
- [x] Task 3.3: 通过 REST API 合并 PR #2（优先 squash）到 `master`；确认返回成功且 PR state=closed、merged_at 非空、Issue #1 已关闭。

## 阶段 4：安全删除特性分支（依赖阶段 3）
- [x] Task 4.1: 确认 `trae/solo-agent-xH8xyu` 已合并入 `master`（`git branch --merged` 或 API `merged=true`）。
- [x] Task 4.2: 通过 REST API 删除远程分支 `trae/solo-agent-xH8xyu`；本地 `git fetch --prune` 清理跟踪引用。
- [x] Task 4.3: 评估 `dev` 分支与 `master` 的差异，决定保留或删除，并向用户报告结论（默认保留，除非明显废弃）。

## 阶段 5：整理主仓库目录结构与内容物（依赖阶段 3，可与阶段 4 并发）
- [x] Task 5.1: 拉取最新 `master`，审查完整目录树，列出残留临时/工作产物与结构问题。
- [x] Task 5.2: 移除残留临时产物（若阶段 2 已在 PR 分支移除则核对确认；否则在此清理），清理空目录与遗留备份文件。
- [x] Task 5.3: 核对目录布局：`src/`、`src-tauri/src/`、`.github/`、`docs/`、`.agents/skills/` 等是否清晰合理；必要时小幅调整。
- [x] Task 5.4: 在 `master` 上以一次提交合入整理改动并推送；输出最终目录树与变更说明。

# Task Dependencies
- Task 1.1 / 1.2 / 1.3 / 1.4 互相独立，可并发。
- Task 2.1 / 2.2 / 2.3 / 2.4 分别依赖 1.1 / 1.2 / 1.3 / 1.4；四者可并发，但都修改同一分支，提交需串行化（由主流程汇总后统一提交，见 Task 2.5）。
- Task 2.5 依赖 2.1–2.4。
- Task 3.1 依赖 2.5；Task 3.2、3.3 依赖 3.1。
- Task 4.1、4.2 依赖 3.3；Task 4.3 依赖 4.1。
- Task 5.1 依赖 3.3；Task 5.2、5.3 依赖 5.1；Task 5.4 依赖 5.2、5.3。
- 阶段 4 与阶段 5 可并发推进（均依赖阶段 3 完成）。
