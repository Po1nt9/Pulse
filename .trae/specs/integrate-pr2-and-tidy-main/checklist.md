# Checklist

## 阶段 0：准备
- [x] PR #2 特性分支 `trae/solo-agent-xH8xyu` 已 fetch 到本地并可 checkout
- [x] 本地 `master` 与 `origin/master` 一致

## 阶段 1：审核
- [x] 领域 A（Rust 后端）问题清单已产出，含 critical/major/minor 分级与修复建议
- [x] 领域 B（前端）问题清单已产出
- [x] 领域 C（CI/仓库配置）问题清单已产出
- [x] 领域 D（文档与临时文件）问题清单已产出，含"应从 PR 移除的文件"清单

## 阶段 2：修复
- [x] 领域 A 的 critical/major 问题已修复，本地 `cargo check` 通过
- [x] 领域 A 的 `cargo test` 全部通过（69 测试，0 失败）
- [x] 领域 B 的 critical/major 问题已修复，本地 `npm run lint` 通过（0 warning）
- [x] 领域 B 的 `npm run build` 构建成功
- [x] 领域 C 的 critical/major 问题已修复
- [x] 领域 D 识别的临时产物已从 PR 分支移除（`pr_body.md`、`sync_labels.sh`、`AGENTS_IMPROVEMENT_SUMMARY.md`、`.trae/specs/github-repo-cleanup-pr/`）
- [x] 修复已提交到 `trae/solo-agent-xH8xyu` 并推送到 origin，提交信息符合 conventional commit
- [x] 推送后未对失败 CI 盲目重试或强推

## 阶段 3：合并
- [x] PR #2 的 Lint check-run conclusion=success
- [x] PR #2 的 Test check-run conclusion=success
- [x] 已通过 API 提交 APPROVE review 与审核摘要评论（APPROVE 因自身 PR 限制改为评论摘要）
- [x] 已通过 API 合并 PR #2 到 `master`（squash）
- [x] PR #2 state=closed 且 merged_at 非空
- [x] 关联 Issue #1 已关闭

## 阶段 4：删分支
- [x] 已确认 `trae/solo-agent-xH8xyu` 已合并入 `master`
- [x] 远程分支 `trae/solo-agent-xH8xyu` 已删除
- [x] 本地 `git fetch --prune` 后该远程跟踪引用已消失
- [x] `master` 与 `dev` 分支未被触及
- [x] `dev` 分支去留已评估并向用户报告（ahead_by=0，落后 master 8 提交，保留）

## 阶段 5：整理主仓库
- [x] 已拉取最新 `master` 并审查完整目录树
- [x] `master` 中不存在 `pr_body.md`、`sync_labels.sh`、`AGENTS_IMPROVEMENT_SUMMARY.md`、`.trae/specs/github-repo-cleanup-pr/` 等临时产物
- [x] 无空目录、无遗留备份文件（如 `*.bak`、`*.orig`）
- [x] `src/`、`src-tauri/src/`、`.github/`、`docs/`、`.agents/skills/` 目录布局清晰合理
- [x] 整理改动已以一次提交合入 `master` 并推送（目录已整洁，无需额外提交）
- [x] 已输出最终目录树与变更说明
