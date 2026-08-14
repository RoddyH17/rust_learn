#!/bin/bash
# 自动格式化。既然有自动化手段,就没有不遵守格式约定的借口。
#
# 建议用法:先 commit 你的工作,再跑这个,然后 git diff 看它改了什么 ——
# 这样你能学到 rustfmt 的习惯,而不是让它悄悄改完你毫无察觉。
#
#   git add -A && git commit -m "wip"
#   ./scripts/fmt.sh
#   git diff
set -e
cd "$(dirname "$0")/.."
cargo fmt --all
echo "✅ 已格式化。跑 git diff 看看它改了什么。"
