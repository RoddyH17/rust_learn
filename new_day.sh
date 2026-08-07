#!/bin/bash
# 新建一天的学习目录:dayN/ + cargo 项目 + 现场记录骨架 + practice 骨架 + NOTES.md
#
# 用法: ./new_day.sh <N> [topic] [slug] [date]
# 例:  ./new_day.sh 4 slice_char_enum
#      ./new_day.sh 4 slice_char_enum day-4-slices 2026-08-07   # 补历史某天
#
#   topic  cargo 包名,也是目录名(下划线);默认 practice
#   slug   博客文章的 slug,写进 NOTES.md 的链接;默认由 topic 推导(下划线→连字符)
#   date   学习日期;默认今天
#
# 产出的 src/main.rs 是**现场记录骨架**,不是 cargo 默认的 hello world ——
# 一边看视频一边往里写:注释即笔记,代码即还原老师讲的东西。它是这一天的唯一源头,
# practice / Notion / NOTES / 博客全部由它生成。
set -e
cd "$(dirname "$0")"

n="${1:?用法: ./new_day.sh <N> [topic] [slug] [date]}"
topic="${2:-practice}"
slug="${3:-day-${n}-$(echo "$topic" | tr '_' '-')}"
date="${4:-$(date +%Y-%m-%d)}"
dir="day${n}"

if [ -e "$dir" ]; then
  echo "❌ $dir 已存在"
  exit 1
fi

mkdir "$dir"
(cd "$dir" && cargo new --quiet "$topic")
rm -rf "$dir/$topic/.git" "$dir/$topic/.gitignore"

# --- 现场记录骨架 ---------------------------------------------------------
# //! 是 Rust 的模块级文档注释,内容按 Markdown 渲染 —— rust-analyzer 悬停能看到,
# cargo doc --open 会出网页。正文用普通 // 注释,VSCode 里会自动续行,写着不费劲。
cat > "$dir/$topic/src/main.rs" <<EOF
//! Day ${n} — ${topic}
//!
//! ${date} · 视频进度:
//!
//! 今天的主线(一句话,学完再回来补):
//!

fn main() {
    // ---------- 1.  ----------
    // 老师说:
    // 我的理解:

    // ---------- 2.  ----------

    // ---------- 3.  ----------
}
EOF

# --- practice 骨架 --------------------------------------------------------
# 由 Claude 读完 main.rs 之后生成真正的题目。铁律:这个文件必须原样可编译,
# 要改的坏代码放注释里。
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

# --- NOTES.md -------------------------------------------------------------
sed -e "s/__N__/${n}/g" \
    -e "s/__TOPIC__/${topic}/g" \
    -e "s/__SLUG__/${slug}/g" \
    -e "s/__DATE__/${date}/g" \
    -e "s/__NEXT__/$((n + 1))/g" \
    NOTES_TEMPLATE.md > "$dir/NOTES.md"

# --- .vscode/launch.json:自动加两条,不再要人手工复制 -----------------------
python3 - "$n" "$topic" <<'PY'
import re, sys

n, topic = sys.argv[1], sys.argv[2]
path = '.vscode/launch.json'
try:
    src = open(path).read()
except FileNotFoundError:
    print('⚠️  没有 .vscode/launch.json,跳过调试配置')
    raise SystemExit

if f'"Day {n} · {topic} ' in src:
    print(f'✓ launch.json 里已有 Day {n} 的配置,跳过')
    raise SystemExit

entries = f'''        {{
            "type": "lldb",
            "request": "launch",
            "name": "Day {n} · {topic} (main)",
            "cargo": {{
                "args": [
                    "build",
                    "--manifest-path",
                    "${{workspaceFolder}}/day{n}/{topic}/Cargo.toml"
                ],
                "filter": {{ "name": "{topic}", "kind": "bin" }}
            }},
            "cwd": "${{workspaceFolder}}/day{n}/{topic}"
        }},
        {{
            "type": "lldb",
            "request": "launch",
            "name": "Day {n} · {topic} (practice)",
            "cargo": {{
                "args": [
                    "build",
                    "--example",
                    "practice",
                    "--manifest-path",
                    "${{workspaceFolder}}/day{n}/{topic}/Cargo.toml"
                ],
                "filter": {{ "name": "practice", "kind": "example" }}
            }},
            "cwd": "${{workspaceFolder}}/day{n}/{topic}"
        }}
'''

# 文件是 JSONC(带注释),不能用 json 模块解析。在最后一个 } 与结尾的 ] 之间插入,
# 并给前一条补上逗号。
m = list(re.finditer(r'\n(\s*)\}\n(\s*)\]', src))
if not m:
    print('⚠️  launch.json 结构不认识,请手动添加 Day %s 配置' % n)
    raise SystemExit
last = m[-1]
src = src[:last.start()] + '\n' + last.group(1) + '},\n' + entries + last.group(2) + ']' + src[last.end():]
open(path, 'w').write(src)
print(f'✓ launch.json 已加入 Day {n} 的 main / practice 两条配置')
PY

echo "✅ 已创建:"
echo "   $dir/$topic/src/main.rs            ← 现场记录:一边写笔记一边编译(今天的源头)"
echo "   $dir/$topic/examples/practice.rs   (练习骨架,学完由 Claude 生成题目)"
echo "   $dir/NOTES.md                      (标准格式笔记)"
echo ""
echo "开工:  code $dir/$topic/src/main.rs  然后 cd $dir/$topic && cargo run"
echo "学完后交给 Claude:读 main.rs → 生成 practice → 推 Notion → 生成 NOTES 与博客"
echo "博客草稿(通常由 Claude 建): cd ~/blog && ./new_post.sh rust ${slug} \"Day ${n} · ...\""
