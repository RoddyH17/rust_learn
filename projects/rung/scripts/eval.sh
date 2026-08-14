#!/bin/bash
# 性能评测。**测试不过就拒绝评测。**
#
# 用法:
#   ./scripts/eval.sh                    # 全部模式 × 全部规模
#   ./scripts/eval.sh insert-only        # 只跑一种模式
#
# 为什么先跑测试:
#   评估一个不正确的程序的速度是没有意义的。如果实现是错的,
#   测出来的「快」只说明它错得很快。这条规矩是硬的,不是建议。
#
# 为什么必须 --release:
#   debug build 关掉了所有优化、开着溢出检查和边界检查,
#   测出来的数字和真实性能没有任何关系。用 debug build 做评估
#   产生的结论是无意义的。
set -e
# pipefail 是必须的:没有它,`cargo test | tail` 返回的是 tail 的退出码(永远是 0),
# 于是下面那道闸门形同虚设 —— 测试全红也照样往下跑。
set -o pipefail
cd "$(dirname "$0")/.."

echo "── 先验证正确性 ───────────────────────────────────"
if ! cargo test --workspace --quiet 2>&1 | tail -20; then
  echo
  echo "❌ 测试未通过,跳过评测。"
  echo "   评估一个不正确的程序的速度是没有意义的。"
  echo "   先把 ./scripts/test.sh 跑绿。"
  exit 1
fi

echo
echo "── release build ──────────────────────────────────"
cargo build --workspace --release

echo
echo "── 评测 ───────────────────────────────────────────"
if [ -n "${1:-}" ]; then
  ./target/release/rung-eval "$1"
else
  ./target/release/rung-eval
fi
