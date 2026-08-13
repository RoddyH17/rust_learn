# rust_learn — 每日 Rust 学习

一天一个文件夹:可运行的代码 + 学习笔记。把 Rust 当作第一门编程语言,从零学起。配套博客在 [RoddyH17.github.io](https://github.com/RoddyH17/RoddyH17.github.io)(本地 `~/blog`)。

**内容分工**:`NOTES.md` 是技术细节的唯一来源(命令、概念、踩坑),博客只写学习叙事与反思,两边互相链接、不重复内容。

- ✍️ **博客**: https://roddyh17.github.io/
- 📁 **本仓库**: `dayN/` — 每天的可运行 crate + `NOTES.md`

## 目录结构

```
rust_learn/
├── day1/
│   ├── hello_world/        # rustc 裸编译示例
│   ├── hello_cargo/        # cargo 标准项目
│   └── NOTES.md            # 当天学习笔记
├── day2/ ...
├── new_day.sh              # 新建一天的脚手架
└── sync.sh                 # 一键提交+推送
```

## 每日工作流(三条命令)

```bash
# 1. 开工:建 dayN 目录 + cargo 项目 + 笔记模板
cd ~/rust_learn && ./new_day.sh 2 variables
# ... 写代码 (cd day2/variables && cargo run)、写 day2/NOTES.md ...

# 2. 建博客草稿(写完删掉 draft: true 行)
cd ~/blog && ./new_post.sh rust day-2-variables "Day 2 · Variables"

# 3. 收工:两边各推一次
cd ~/rust_learn && ./sync.sh "day 2: variables"
cd ~/blog && ./sync.sh "post: day 2 variables"
```

## 学习进度

| Day | 日期 | 主题 | 笔记 | 博客 |
|-----|------|------|------|------|
| 1 | 2026-07-31 | 环境安装 · hello world · cargo | [NOTES](day1/NOTES.md) | [Day 1](https://roddyh17.github.io/posts/rust/day-1-hello-rust/) |
| 2 | 2026-08-04 | Variables · 类型 · 函数与表达式 · 流程控制 | [NOTES](day2/NOTES.md) | [Day 2](https://roddyh17.github.io/posts/rust/day-2-variables/) |
| 3 | 2026-08-06 | 所有权与内存 · move/clone/Copy · 借用 · NLL | [NOTES](day3/NOTES.md) | [Day 3](https://roddyh17.github.io/posts/rust/day-3-ownership/) |
| 4 | 2026-08-08 | 数组 · 切片 · 胖指针 · enum 初识 | [NOTES](day4/NOTES.md) | [Day 4](https://roddyh17.github.io/posts/rust/day-4-arrays-and-slices/) |
| 5 | 2026-08-09 | enum · struct · match | [NOTES](day5/NOTES.md) | [Day 5](https://roddyh17.github.io/posts/rust/day-5-enums-and-match/) |
| 5.5 | 2026-08-11 | 模式全谱 · 守卫 · trait 对象 · **迷你订单引擎** | [NOTES](day5.5/NOTES.md) | [Day 5.5](https://roddyh17.github.io/posts/rust/day-5-5-pattern-matching/) |
| 6 | 2026-08-10 | struct 三形态 · impl · trait · Display | [NOTES](day6/NOTES.md) | [Day 6](https://roddyh17.github.io/posts/rust/day-6-structs-and-traits/) |
| 7 | 2026-08-11 | Vec · HashMap · entry API · capacity | [NOTES](day7/NOTES.md) | [Day 7](https://roddyh17.github.io/posts/rust/day-7-data-structure-cookbook/) |
| 7.5 | 2026-08-12 | Option 的引入/消去规则 · Result · 自定义错误 enum · `?` | [NOTES](day7.5/NOTES.md) | — |
| 8 | 2026-08-13 | 模块系统 · 可见性 · 路径 · use/as · workspace | [NOTES](day8/NOTES.md) | — |

**基础教学到 Day 8 结束。** 接下来进入阶段实战项目。

## 阶段实战项目 · Rung

> 📐 设计报告:[`projects/rung/DESIGN.md`](projects/rung/DESIGN.md)

**Rung**(梯级)—— 一个用 Rust 从零构建的交易系统内核:限价订单簿与撮合引擎 → 实时行情
管道与 L2 重建 → 微结构特征与在线推理。

它的设计受两组约束支配:它必须是一个真实可用的系统,**并且**它的每个阶段必须恰好逼出一项
当前尚未掌握的 Rust 能力(泛型、生命周期、智能指针、闭包、测试、workspace、async、基准)。
详细的基线审计、选型评分、架构推导与实施路径见设计报告。

```
projects/
├── rung/           # 阶段实战项目(设计报告已完成,S1 待开工)
└── orderbook/      # 前身:五段式教学格式的原型,其阶段 0-5 已并入 Rung S1
```
