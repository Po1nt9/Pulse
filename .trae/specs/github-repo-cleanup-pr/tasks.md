# Tasks

## 任务 1: 安装 GitHub CLI
- [x] 1.1 安装 gh CLI 工具
  - 使用包管理器安装 gh
  - 验证安装成功：`gh --version`
- [x] 1.2 验证 gh 认证状态
  - 运行 `gh auth status`
  - 确认已认证到正确的仓库

## 任务 2: 整理 GitHub 仓库
- [x] 2.1 同步 Labels 到 GitHub
  - 读取 `.github/labels.yml`
  - 使用 `gh label create` 命令创建所有标签
  - 验证标签创建成功
- [x] 2.2 检查并创建 Milestone（可选）
  - 检查是否需要创建 v0.1.0 milestone
  - 如需要，使用 `gh api` 创建 milestone

## 任务 3: Git 管理
- [x] 3.1 确保分支已推送到远程
  - 检查当前分支是否已推送
  - 如未推送，执行 `git push -u origin <branch-name>`
- [x] 3.2 验证 commit 历史
  - 检查 commit message 是否符合 Conventional Commits
  - 如有问题，提示用户修正

## 任务 4: 创建 Pull Request
- [x] 4.1 准备 PR 内容
  - 根据 PR 模板准备标题和描述
  - 标题格式：`feat: 项目重构与 GitHub 仓库整理`
  - 描述包含：变更内容、验证结果、影响范围
- [x] 4.2 创建 PR
  - 使用 `gh pr create` 创建 PR
  - 指定 base 为 master
  - 指定 head 为当前分支
- [x] 4.3 验证 PR 创建成功
  - 获取 PR URL
  - 验证 PR 状态为 open

## 任务依赖
- [任务 2] 依赖 [任务 1]
- [任务 3] 依赖 [任务 1]
- [任务 4] 依赖 [任务 2, 任务 3]
