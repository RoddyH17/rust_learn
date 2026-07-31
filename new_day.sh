#!/bin/bash
# 新建一天的学习目录:dayN/ + cargo 项目 + NOTES.md 模板
# 用法: ./new_day.sh <N> [topic]
# 例:  ./new_day.sh 2 ownership
set -e
cd "$(dirname "$0")"

n="${1:?用法: ./new_day.sh <N> [topic]}"
topic="${2:-practice}"
dir="day${n}"

if [ -e "$dir" ]; then
  echo "❌ $dir 已存在"
  exit 1
fi

mkdir "$dir"
(cd "$dir" && cargo new "$topic")
rm -rf "$dir/$topic/.git" "$dir/$topic/.gitignore"

cat > "$dir/NOTES.md" <<EOF
# Day $n — $topic ($(date +%Y-%m-%d))

## 今天完成

-

## 关键概念

-

## 明日计划

-
EOF

echo "✅ 已创建:"
echo "   $dir/$topic/       (cargo 项目, cd 进去 cargo run)"
echo "   $dir/NOTES.md      (笔记模板)"
echo ""
echo "下一步: cd ~/blog && ./new_post.sh rust_learn day-${n}-${topic} \"Day $n · ${topic}\""
