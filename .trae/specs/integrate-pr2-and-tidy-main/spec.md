# 审核、修复、合并 PR #2 并整理主仓库 Spec

## Why

仓库当前有一个开放的 PR #2（`trae/solo-agent-xH8xyu` → `master`，标题"feat: 项目重构与 GitHub 仓库整理"），包含 38 个文件的改动：Rust 后端重构、前端修复、CI/CD 与仓库配置、文档扩展。该 PR 尚未被任何 reviewer 审核过，CI 的 Test 作业在抓取时仍处于 in_progress。需要先严格审核、修复发现的问题，再合并到主分支（`master`，仓库默认分支，不存在 `main` 分支，下文"主分支/main"均指 `master`），安全删除已合并的特性分支，最后整理主仓库的目录结构与内容物，保证主干干净可用。

## What Changes

- 审核 PR #2 的全部 38 个文件改动，按"Rust 后端 / 前端 / CI 与仓库配置 / 文档与临时文件"四个领域并行审核。
- 修复审核中发现的所有问题（代码缺陷、配置错误、不应进入主干的临时文件等），修复提交到 PR 特性分支 `trae/solo-agent-xH8xyu` 并推送，使 PR 重新变绿。
- 待 CI（Lint + Test）全部通过后，将 PR #2 合并到 `master`（采用 squash 或 merge commit，视审核结论而定）。
- 合并后安全删除远程特性分支 `trae/solo-agent-xH8xyu`（及本地引用）；评估 `dev` 分支是否需要保留。
- 整理 `master` 主仓库目录结构与内容物：移除通过 PR 混入的临时/工作产物（如 `pr_body.md`、`AGENTS_IMPROVEMENT_SUMMARY.md`、`sync_labels.sh`、`.trae/specs/github-repo-cleanup-pr/` 等），核对目录布局，确保主干只保留应长期存在的文件。
- **BREAKING**（仅对仓库历史而言）：合并会改变 `master` HEAD；删除特性分支后，基于该分支的本地工作需 rebase 到 `master`。

## Impact

- Affected specs: 无既有 spec 被修改（PR 内自带的 `.trae/specs/github-repo-cleanup-pr/` 属于待清理的工作产物）。
- Affected code:
  - Rust 后端：`src-tauri/src/{commands/balance.rs, commands/provider_key.rs, config.rs, error.rs, http.rs, tray.rs, window.rs}`、`Cargo.toml`、`Cargo.lock`
  - 前端：`src/components/NotificationToast.tsx`、`src/hooks/useSettings.ts`、`src/main.tsx`、`src/types/index.ts`、`package.json`
  - CI/仓库配置：`.github/workflows/{ci.yml,release.yml}`、`.github/{CODEOWNERS,dependabot.yml,labels.yml,ISSUE_TEMPLATE/*,PULL_REQUEST_TEMPLATE.md}`、`eslint.config.js`、`.prettierrc.json`、`.prettierignore`、`.editorconfig`、`.dockerignore`、`.gitignore`、`LICENSE`
  - 文档：`AGENTS.md`、`AGENTS_IMPROVEMENT_SUMMARY.md`、`README.md`、`CHANGELOG.md`
  - 临时/待清理：`pr_body.md`、`sync_labels.sh`、`.trae/specs/github-repo-cleanup-pr/`
- 工具链：环境未安装 `gh` CLI，所有 GitHub 操作（审核评论、合并、删分支、查 CI）通过 GitHub REST API + `curl` 完成；本地 git 操作通过 `git` 完成。

## ADDED Requirements

### Requirement: PR 审核覆盖全部改动领域

系统（执行流程）SHALL 对 PR #2 的全部 38 个文件按四个领域并行审核：Rust 后端、前端、CI/仓库配置、文档与临时文件。每个领域产出明确的问题清单（文件、行、问题、严重级别、建议修复）。

#### Scenario: 审核产出可执行
- **WHEN** 审核完成
- **THEN** 每个领域有一份问题清单，标注 critical / major / minor
- **AND** 每条问题给出具体修复建议

### Requirement: 修复全部审核问题并使 PR 变绿

系统 SHALL 在 PR 特性分支 `trae/solo-agent-xH8xyu` 上修复审核发现的所有 critical 与 major 问题（minor 问题尽量一并修复），推送后等待 CI 的 Lint 与 Test 作业全部 conclusion=success。

#### Scenario: 修复后 CI 通过
- **WHEN** 修复推送完成且 CI 运行结束
- **THEN** Lint = success 且 Test = success
- **AND** 修复提交信息遵循项目 conventional commit 风格

#### Scenario: CI 失败时
- **WHEN** 修复后 CI 仍失败
- **THEN** 读取失败日志，定位根因，继续修复，不盲目重试或强推

### Requirement: 合并 PR 到主分支

系统 SHALL 在 CI 全绿且无未解决 critical/major 问题后，将 PR #2 合并到 `master`。合并方式优先 squash（保持主干历史整洁），并在 PR 上留下审核通过记录。

#### Scenario: 合并成功
- **WHEN** 合并请求返回 200/201 且 PR state=closed、merged_at 非空
- **THEN** `master` HEAD 前进到合并后的 commit
- **AND** 关联 Issue #1 被 PR 的 "Closes #1" 自动关闭

### Requirement: 安全删除已合并特性分支

系统 SHALL 在确认 PR 已合并后，删除远程分支 `trae/solo-agent-xH8xyu`，并清理本地远程跟踪引用。删除前确认分支已合并到 `master`，未合并则不删除并报告。

#### Scenario: 安全删除
- **WHEN** PR 已 merged
- **THEN** 远程分支 `trae/solo-agent-xH8xyu` 被删除
- **AND** 本地 `git fetch --prune` 后该远程跟踪引用消失
- **AND** 删除操作不触及 `master` 与 `dev`

### Requirement: 整理主仓库目录结构与内容物

系统 SHALL 在合并后审查 `master` 的目录结构，移除不应长期存在的临时/工作产物，核对目录布局符合项目规范，并在需要时调整。整理结果以最终目录树与变更说明反馈给用户。

#### Scenario: 主干干净
- **WHEN** 整理完成
- **THEN** `master` 中不存在 `pr_body.md`、`sync_labels.sh`、`AGENTS_IMPROVEMENT_SUMMARY.md`、`.trae/specs/github-repo-cleanup-pr/` 等临时产物
- **AND** `src/`、`src-tauri/src/`、`.github/`、`docs/` 等目录布局清晰、无空目录、无遗留备份文件
- **AND** 整理改动以一次提交合入 `master` 并推送

## MODIFIED Requirements

### Requirement: 主分支命名约定

仓库默认分支为 `master`，不存在 `main` 分支。用户指令中的"main / 主分支"在本任务中统一映射为 `master`。不创建 `main` 分支，不重命名 `master`。

## REMOVED Requirements

（无）
