#!/bin/bash
# 编译并跑全部测试(单元 + 集成 + doctest)。
#
# 用法:
#   ./scripts/test.sh              # 全部
#   ./scripts/test.sh price        # 只跑名字里含 price 的测试
#
# 放大调试:测试失败时,先用过滤器缩小范围,再看单条。
#   ./scripts/test.sh price_display
#   cargo test -p rung-core price_display -- --exact --nocapture
set -e
cd "$(dirname "$0")/.."

filter="${1:-}"

echo "── build ──────────────────────────────────────────"
cargo build --workspace

echo
echo "── test ───────────────────────────────────────────"
if [ -n "$filter" ]; then
  cargo test --workspace "$filter"
else
  cargo test --workspace
  echo
  echo "── doctest ────────────────────────────────────────"
  cargo test --workspace --doc
fi
