#!/bin/bash
# 编译。默认 debug;加 --release 编优化版(评测必须用它)。
set -e
cd "$(dirname "$0")/.."
cargo build --workspace "$@"
