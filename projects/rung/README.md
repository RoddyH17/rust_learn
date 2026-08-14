# Rung

用 Rust 从零构建的交易系统内核:限价订单簿与撮合引擎。

**这个文件是唯一的入口。** 其余文档是查的,不是读的。

---

## 现在就跑这三条

```bash
cd projects/rung

./scripts/test.sh      # 看它红:131 个测试,2 绿
cat stages/stage-01.md # 你的任务书
```

然后打开 `crates/rung-core/src/types.rs`,把第一个 `todo!()` 填掉。

---

## 你现在在哪

```
阶段 1 ──▶ 阶段 2 ──▶ 阶段 3-5 ──▶ 阶段 6-7 ──▶ 阶段 8-10 ──▶ 阶段 11-14
  ▲         已就绪      数据结构      抽象/trait     并发         系统
  │
 你在这
```

| 阶段 | 内容 | 状态 |
|---|---|---|
| **1** | 类型 + `NaiveBook` 参考模型 + 基线 | 🔨 **做这个** |
| 2 | 档位队列:三份实现(`VecDeque` / `Box` 链表 / arena) | 📋 骨架已就绪 |
| 3–14 | 见 [`ROADMAP.md`](ROADMAP.md) | ⏸ 待写 |

---

## 文档地图

**开工前读一遍(共约 40 分钟):**

| 文档 | 讲什么 | 什么时候读 |
|---|---|---|
| [`PRIMER.md`](PRIMER.md) | 交易所、流动性、订单簿为什么长这样、CEX vs 链上 DEX | **先读**,零基础 |
| [`METHOD.md`](METHOD.md) | 系统设计的七条方法 | 再读 |

**动手时看:**

| 文档 | 讲什么 |
|---|---|
| [`stages/stage-NN.md`](stages/) | **当前阶段的任务书** —— 子步、闸门、规格、提示 |
| 源码里的文档注释 | 每个函数的行为规格(空值/越界/失败时状态) |

**收尾时填:**

| 文档 | 什么时候写 |
|---|---|
| [`reports/TEMPLATE.md`](reports/TEMPLATE.md) | 每阶段结束,≤4 页 |
| [`decisions/TEMPLATE.md`](decisions/TEMPLATE.md) | **两份实现都测完之后** —— 那时代价是测出来的数字 |

**查阅(不用通读):**

| 文档 | 什么时候查 |
|---|---|
| [`ROADMAP.md`](ROADMAP.md) | 想知道后面十几个阶段各是什么 |
| [`DESIGN.md`](DESIGN.md) | 想知道为什么选这个项目、评估指标是什么 |
| ~~[`stages/stage-00.md`](stages/stage-00.md)~~ | **已降级为可选**,可以直接跳过 |

---

## 每天的循环

```
1. 读 stage-NN.md 当前子步的规格          20 min
2. ./scripts/test.sh <filter>  看它红      5 min
3. 填 todo!(),直到这个子步绿           60-90 min
4. ./scripts/lint.sh                      10 min
5. git commit(一子步一个)
```

阶段结束时额外做:`./scripts/eval.sh` 拿数字 → 填报告 → 写 ADR。

---

## 命令速查

```bash
./scripts/test.sh              # 全部测试(单元 + 集成 + doctest)
./scripts/test.sh types        # 只跑名字含 types 的
./scripts/lint.sh              # fmt + clippy,警告即失败
./scripts/fmt.sh               # 自动格式化
./scripts/eval.sh              # 性能评测(测试不过会拒跑,强制 release)
./scripts/eval.sh level        # 阶段 2 的档位队列三份实现对比

# 看单条测试的完整输出
cargo test -p rung-core price_display_补零 -- --exact --nocapture

# 生成并打开 API 文档
cargo doc --workspace --no-deps --open
```

---

## 三条硬规矩

1. **测试不过就不许评测。** `eval.sh` 机械执行 —— 评估一个不正确的程序的速度是没有意义的
2. **评测必须 release build。** debug 数字和真实性能没有关系
3. **优化只许改实现,不许改接口。** 为了性能把私有字段改成 `pub` 是不可接受的

---

## 项目结构

```
projects/rung/
├── README.md          ← 你在这
├── PRIMER.md          领域科普
├── METHOD.md          架构方法
├── ROADMAP.md         五部 14 阶段
├── DESIGN.md          设计报告
├── scripts/           一任务一脚本
├── stages/            每阶段的任务书
├── reports/           每阶段的对比报告
├── decisions/         ADR 决策记录
└── crates/
    ├── rung-core/     类型 + NaiveBook + 档位队列(零依赖、零 I/O)
    └── rung-eval/     性能评测程序(不用改)
```
