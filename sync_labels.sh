#!/bin/bash

# 从 labels.yml 同步标签到 GitHub 仓库

echo "开始同步标签到 GitHub 仓库..."

# 类型标签
gh label create "Bug" --color "d73a4a" --description "Something isn't working" --force
gh label create "Feature" --color "a2eeef" --description "New feature or request" --force
gh label create "Documentation" --color "0075ca" --description "Improvements or additions to documentation" --force
gh label create "Enhancement" --color "c2e0c6" --description "Improvement to existing feature" --force
gh label create "Question" --color "d876e3" --description "Further information is requested" --force

# 状态标签
gh label create "待确认" --color "fbca04" --description "Issue needs to be confirmed" --force
gh label create "进行中" --color "0e8a16" --description "Work in progress" --force
gh label create "已完成" --color "0e8a16" --description "Issue has been resolved" --force
gh label create "需要更多信息" --color "b60205" --description "Needs more information from reporter" --force

# 优先级标签
gh label create "优先级：高" --color "b60205" --description "High priority" --force
gh label create "优先级：中" --color "fbca04" --description "Medium priority" --force
gh label create "优先级：低" --color "0e8a16" --description "Low priority" --force

echo ""
echo "标签同步完成！正在验证..."
echo ""

# 验证标签创建成功
gh label list
