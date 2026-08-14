#!/bin/bash
# clippy + 格式检查。警告即失败 —— 不给「等会儿再清」留口子。
set -e
cd "$(dirname "$0")/.."

echo "── fmt --check ────────────────────────────────────"
cargo fmt --all -- --check || {
  echo
  echo "❌ 格式不对。跑 ./scripts/fmt.sh 自动修好,然后 git diff 看它改了什么。"
  exit 1
}

echo
echo "── clippy ─────────────────────────────────────────"
cargo clippy --workspace --all-targets -- -D warnings

echo
echo "✅ 干净"
