#!/bin/bash
# 每日同步:提交全部更改并推送到 GitHub
# 用法: ./sync.sh ["commit message"]
set -e
cd "$(dirname "$0")"

msg="${1:-learn: daily update $(date +%Y-%m-%d)}"

git add -A
if git diff --cached --quiet; then
  echo "Nothing to commit."
  exit 0
fi
git commit -m "$msg"
git push origin main

echo ""
echo "✅ 代码已推送: https://github.com/RoddyH17/rust_learn"
echo "💡 别忘了写博客: ~/blog/content/ 新建 mdx 后跑 cd ~/blog && ./sync.sh"
