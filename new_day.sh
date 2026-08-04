#!/bin/bash
# 新建一天的学习目录:dayN/ + cargo 项目 + practice 骨架 + NOTES.md(标准格式)
# 用法: ./new_day.sh <N> [topic]
# 例:  ./new_day.sh 4 enums
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

# practice 骨架:cargo run --example practice
mkdir -p "$dir/$topic/examples"
cat > "$dir/$topic/examples/practice.rs" <<'EOF'
// Practice — run with: cargo run --example practice
//
// This file must always compile as shipped. Exercises ask you to
// uncomment code, fix it, or write a few lines yourself.

fn main() {
    println!("--- Exercise 1: TODO ---");

    println!("--- done ---");
}
EOF

# NOTES.md:从标准模板生成,替换占位符
sed -e "s/__N__/${n}/g" \
    -e "s/__TOPIC__/${topic}/g" \
    -e "s/__DATE__/$(date +%Y-%m-%d)/g" \
    -e "s/__NEXT__/$((n + 1))/g" \
    NOTES_TEMPLATE.md > "$dir/NOTES.md"

echo "✅ 已创建:"
echo "   $dir/$topic/                       (cargo 项目, cargo run)"
echo "   $dir/$topic/examples/practice.rs   (练习, cargo run --example practice)"
echo "   $dir/NOTES.md                      (标准格式笔记)"
echo ""
echo "别忘了: .vscode/launch.json 里复制一段配置改成 day${n}"
echo "下一步: cd ~/blog && ./new_post.sh rust day-${n}-${topic} \"Day ${n} · ${topic}\""
