# GitHub 仓库整理与 PR 创建 Spec

## Why
项目已完成全面重构，需要将变更提交到 GitHub 并创建 Pull Request。同时需要确保 GitHub 仓库配置完整，包括标签同步、里程碑设置等。

## What Changes
- 安装 GitHub CLI (`gh`)
- 同步 labels.yml 中的标签到 GitHub 仓库
- 创建 GitHub Milestone（如需要）
- 确保当前分支已推送到远程
- 创建 Pull Request 到 master 分支
- PR 描述应包含完整的变更说明

## Impact
- Affected specs: 无
- Affected code: GitHub 仓库配置、PR 管理

## ADDED Requirements

### Requirement: GitHub CLI 安装
系统 SHALL 安装 GitHub CLI 工具，用于管理 GitHub 仓库。

#### Scenario: 安装 gh CLI
- **WHEN** 需要操作 GitHub 仓库
- **THEN** 通过包管理器安装 `gh` 工具
- **AND** 验证 `gh auth status` 返回成功

### Requirement: Labels 同步
系统 SHALL 将 labels.yml 中定义的标签同步到 GitHub 仓库。

#### Scenario: 同步标签到 GitHub
- **WHEN** labels.yml 文件存在
- **THEN** 使用 `gh label` 命令创建/更新标签
- **AND** 标签名称、颜色、描述与 labels.yml 一致

### Requirement: PR 创建
系统 SHALL 创建 Pull Request 将当前分支合并到 master。

#### Scenario: 创建 PR
- **WHEN** 当前分支有未合并的变更
- **THEN** 创建 PR 到 master 分支
- **AND** PR 标题符合 Conventional Commits 规范
- **AND** PR 描述包含变更内容、影响范围、验证结果
- **AND** PR 描述使用项目 PR 模板格式

### Requirement: Git 管理规范
系统 SHALL 确保所有 commit 符合项目规范。

#### Scenario: Commit Message 规范
- **WHEN** 创建 commit
- **THEN** commit message 符合 Conventional Commits 格式
- **AND** 包含 type、scope、description
- **AND** 如有破坏性更改，包含 BREAKING CHANGE 说明

## MODIFIED Requirements

无

## REMOVED Requirements

无
