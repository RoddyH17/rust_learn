# Rung:一个以 Rust 语言能力增长为约束的交易系统内核

**设计与实施报告 · Design and Implementation Report**

作者:Roddy Huang(zh89@cornell.edu)
日期:2026-08-13
版本:v1.0
仓库:[`RoddyH17/rust_learn`](https://github.com/RoddyH17/rust_learn) · `projects/rung/`

---

## 摘要 Abstract

本文提出 **Rung**,一个用 Rust 从零构建的交易系统内核,其设计同时受两组约束支配:
**(a) 系统约束** —— 它必须是一个真实可用的限价订单簿、行情管道与微结构特征引擎;
**(b) 教学约束** —— 它的每一个阶段必须恰好逼出作者当前尚未掌握的某一项 Rust 语言能力,
且不得提前引入。

第二组约束是本设计的核心贡献。常规的「学习项目」把语言学习与系统构建视为两个前后相继的
阶段(先学完语法,再做项目);本文主张二者应当**互为约束条件**:系统的架构决策点应当被
刻意安排在语言能力的缺口处,使得「为什么必须用泛型」这类问题不是由教材断言,而是由
编译器在真实代码中拒绝编译来证明。

我们首先对作者八天的 Rust 学习成果进行量化审计(§2),识别出 10 项零覆盖的语言能力;
随后从 10 个候选系统中,依据五维评分模型选出主线(§3);进而推导出一个四 crate 的
workspace 架构,并论证其边界划分为何必须落在 crate 层而非 module 层(§4);最后给出
分三段共 10 个阶段的实施路径,每阶段配可执行的验收断言与不变量(§6),以及一套不依赖
主观判断的评估指标(§7)。

**关键词**:限价订单簿 · 撮合引擎 · 市场微结构 · Rust 所有权系统 · 基于性质的测试 ·
渐进式课程设计

---

## 目录

1. [引言](#1-引言)
2. [背景与基线审计](#2-背景与基线审计)
3. [选题:候选系统与评分模型](#3-选题候选系统与评分模型)
4. [系统架构](#4-系统架构)
5. [方法论:教学约束如何进入架构](#5-方法论教学约束如何进入架构)
6. [实施路径](#6-实施路径)
7. [评估方法](#7-评估方法)
8. [风险与效度威胁](#8-风险与效度威胁)
9. [相关工作](#9-相关工作)
10. [结论](#10-结论)
- [附录 A:Rust 能力矩阵](#附录-arust-能力矩阵)
- [附录 B:命名说明](#附录-b命名说明)
- [附录 C:术语表](#附录-c术语表)
- [附录 D:参考文献](#附录-d参考文献)

---

## 1. 引言

### 1.1 动机

作者的长期目标是进入金融科技的系统层工作:区块链基础设施、去中心化交易所、算法交易引擎、
做市。这些领域在 2026 年已高度 Rust 化 —— Solana、Polkadot、Near 的链本身用 Rust 写成;
新一代加密交易台、自营商与零售算法交易者正在用 Rust 构建全栈 [11];而 Rust 量化开发岗位
的招聘要求已明确列出「所有权与借用机制、并发原语、无锁结构、性能剖析」与「对市场微结构、
订单簿、订单状态机的深入理解」[10]。

因此语言学习不是目的。目的是**具备构建这类系统的能力**,而 Rust 是当前该领域的载体。

### 1.2 问题陈述

作者已完成八天 Rust 基础学习,覆盖所有权与借用、枚举与模式匹配、结构体与方法、trait 与
动态分发、`Vec`/`HashMap`、`Option`/`Result` 与错误传播、模块系统。但基线审计(§2)显示,
**十项对构建真实系统必需的能力覆盖率为零**,其中包括:自己书写泛型 `impl<T>`、显式生命周期
标注、智能指针、闭包、单元测试、多 crate workspace。

同时,现有的项目脚手架 `projects/orderbook/` 存在结构性缺陷(§2.3):它是单 crate 单文件、
零依赖、零测试的设计,预计工时 3.5 小时。这是**练习的体量与形态**,无法训练作者所要求的
「数据结构—算法—架构」与「库开发—协作」两项能力。

于是本文要回答的问题是:

> 如何设计一个系统,使其**既是**一个有工程价值的真实交易系统内核,**又是**一条能把上述
> 十项语言能力按依赖顺序逐一逼出的路径,且二者不互相妥协?

### 1.3 贡献

1. **一个量化的语言能力基线审计方法**(§2.1),以代码中的实际出现次数而非「学过没有」
   为判据,给出可复核的能力矩阵。
2. **一个五维项目选型模型**(§3),对 10 个候选系统给出评分,并揭示三个反直觉结论 ——
   其中最重要的一条是:**总分最高的项目不能作为起点**,因为评分衡量项目价值,不衡量
   依赖顺序。
3. **Rung 的架构**(§4):一个四 crate workspace,其边界划分依据「边界要不要硬」这一
   单一判据,并论证为何必须拆 crate 而非拆 module。
4. **教学约束的形式化**(§5):把「每个阶段恰好逼出一项能力」写成阶段的准入/准出条件,
   使课程设计成为架构约束的一部分而非附加文档。
5. **一套不依赖主观判断的评估指标**(§7):以不变量、测试覆盖、基准数字与延迟预算取代
   「感觉学会了」。

---

## 2. 背景与基线审计

### 2.1 审计方法

不采用「学过哪些章节」作为判据 —— 读过和写得出之间的差距正是本项目要消除的东西。
改用**代码计数法**:对 `rust_learn/` 全仓库的 `.rs` 文件做静态统计,以某项能力在
作者**自己书写**的代码中出现的次数为判据。抄写自教材示例的不计入。

### 2.2 审计结果

**已具备**(截至 2026-08-13,Day 1–8 完成):

| 能力 | 载体 | 状态 |
|---|---|---|
| 变量、类型、表达式与流程控制 | day2(274 + 236 行) | 扎实 |
| 所有权、move/clone/Copy、借用两规则、NLL | day3(262 + 199 行) | 扎实 |
| 数组、切片、胖指针 | day4(17 个编号节) | 扎实 |
| enum、`Option`、`match` | day5 | 扎实 |
| 模式全谱:`@` 绑定、范围、守卫、`ref mut`、`if let`/`while let` | day5.5(690 行迷你订单引擎) | 扎实 |
| struct 三形态、`impl`、关联函数、`Display` 手写 | day6 | 扎实 |
| trait:静态分发 `&impl Trait`、动态分发 `&dyn Trait` | day6 + day5.5 Stage 7 | 扎实 |
| `Vec` / `HashMap` / `entry` API / capacity | day7(12 个编号节) | 扎实 |
| `Result`、自定义错误 enum、`?`、`map_err`、`ok_or` | day7.5(11 个编号节) | **刚完成** |
| 模块系统:`mod`/`pub`/`pub(crate)`/`pub(in path)`/`use`/`as`/`self`/`super` | day8(11 个编号节) | **刚完成** |
| 外部 crate:serde + serde_json | day5.5 | 单点 |

**零覆盖**(本项目要逐一消除的缺口):

| # | 能力 | 实测 | 在真实系统中的位置 |
|---|---|---|---|
| G1 | 自己书写泛型 `impl<T>` / `where` | **0 处**(仅 1 处抄写的 `fn f<T: Hash>`) | 任何可复用库的地基 |
| G2 | 显式生命周期标注 | **0 处**(仅 1 处被注释掉的 `struct User<'a>`) | 零拷贝解析、返回借用 |
| G3 | `Box` / `Rc` / `RefCell` / 智能指针 | **0 处** | 图与树、trait object 存储 |
| G4 | 闭包 `Fn`/`FnMut`/`FnOnce` | **0 处** | 策略注入、回调 |
| G5 | 迭代器链 | `.map()` 3 次,`.filter()` **0 次** | 无处不在 |
| G6 | `#[test]` 单元测试 / 集成测试 | **全仓库 0 个** | 库开发的入场券 |
| G7 | 多文件 crate / `lib.rs` / workspace | **0 个**(仅注释中提及) | 「库开发—协作」的全部含义 |
| G8 | async / `tokio` / 线程 / `Arc<Mutex>` | **0 处** | 行情接入、网关 |
| G9 | 基准测试 / criterion / 性能剖析 | **0 个** | 「高频」二字的唯一证据 |
| G10 | `String` 深入(`push_str` vs `+`、为何不能下标) | 欠账两天 | 文本协议解析 |

一个值得记录的观察:G1–G7 全部属于「写库需要、写脚本不需要」的能力。这解释了为什么八天
的学习能产出可运行的代码,却产不出可复用的代码 —— 二者要求的是不同的能力集合。

### 2.3 现有项目脚手架的审计

`projects/orderbook/`(commit `85ec7c5`,此后未修改):

| 项目 | 实测 |
|---|---|
| 源文件 | 1 个(`src/main.rs`,523 行) |
| 其中实现代码 | **0 行**(全部为文档注释与提示词) |
| 「在这里写阶段 N」空槽 | 5 个,**全空** |
| ✍️ reflection 空白 | 35 处,**填写 0 处** |
| `fn main` 中的验收断言 | 5 组,**全部注释掉** |
| 实际执行的语句 | 2 行 `println!` |
| 依赖 | 0 |
| 测试 | 0 |
| edition | 2021(其余 day crate 为 2024,不一致) |

**对照作者目标的差距分析**:

| 目标 | orderbook 现状 | 判定 |
|---|---|---|
| 数据结构 | `BTreeMap<Price, VecDeque<Order>>` 选型推导完整,思考阶梯质量高 | ✅ |
| 算法 | 仅覆盖单次跨档位撮合;无队列位置、无延迟模型 | ⚠️ 浅 |
| 架构 | 单 crate 单文件,无 `lib.rs`,无公共 API 边界 | ❌ |
| 库开发—协作 | 无 workspace、无 `pub` 边界、无 doctest、无 semver 概念 | ❌ |
| 工程学训练 | 零测试、零基准、零 CI | ❌ |
| 金融科技背景 | 选题正确 | ✅ |

其自述的「不做」清单包含五项:队列位置建模、多线程与锁、快照与增量推送、性能基准测试、
撤改单时间优先级重排。**这五项恰好是工程价值最高的部分** —— 剔除它们之后,剩下的是一道
设计精良的练习题,而非项目。

**结论**:`orderbook` 不应被丢弃。它的**教学格式**(五段式:管道位置 / 要解决什么 · 不负责
什么 / 思考阶梯 / 候选方案与代价 / reflection 空白)经过验证是有效的,应当被 Rung 完整继承
并扩展到全部 10 个阶段;它的**体量与形态**则应被替换。具体地,orderbook 的阶段 0–5 成为
Rung 阶段 1–5 的教学素材来源。

---

## 3. 选题:候选系统与评分模型

### 3.1 评分维度

| 维度 | 定义 | 方向 |
|---|---|---|
| **A. 难度 / 工时** | 1–10,以 §2.2 的基线为参照;附独立工时估算 | 高 ≠ 好,是成本项 |
| **B. Rust 内功** | 能消除多少 G1–G10 缺口,并深化多少既有能力 | 越高越好 |
| **C. 技术栈拓宽** | 引入多少 Rust 之外的领域(链上、ML、列式存储、FFI) | 越高越好 |
| **D. HFT / 做市简历价值** | 能否作为高频、量化、做市方向的能力证据 | 越高越好 |
| **E. AI 相关度** | 与 ML / LLM 的结合深度 | 越高越好 |

### 3.2 候选系统

| # | 系统 | 内容摘要 |
|---|---|---|
| **P1** | 限价订单簿 + 撮合引擎 + 确定性回放 | `BookSide<K: Ord>` 泛型两边复用、arena 存储、IOC/FOK/Post-Only、proptest 不变量、criterion 基准 |
| **P2** | 交易所后端全栈 | 网关 → 前置风控 → 单线程撮合 → 清算账本 → 行情发布(快照 + 增量 + 断点重连) |
| **P3** | 实时行情管道 + L2 重建 + 列式 tick 存储 | 多交易所 WebSocket、`trait Exchange` 归一化、序列号缺口检测与 resync、Arrow/Parquet |
| **P4** | 做市策略引擎 + 微结构模拟器 | Avellaneda–Stoikov 报价、库存管理、逆向选择、VPIN、队列位置建模、双向延迟模型 |
| **P5** | 微结构特征引擎 + ONNX 在线推理 | 增量 OFI / microprice / 队列失衡 / 已实现波动率、环形缓冲、`ort` 热路径推理、延迟预算 |
| **P6** | 期权做市:IV 求解 + SVI 曲面 + 希腊字母风险引擎 | Black-Scholes/Black-76、Newton + Brent、二三阶希腊字母、无套利约束、PyO3 导出 |
| **P7** | 集中流动性 AMM 核心库 | Uniswap V3 tick 数学、`sqrtPriceX96` 定点数、跨 tick 步进、差分测试、`no_std` |
| **P8** | CEX↔DEX 跨场套利引擎 | `alloy` 读链上状态、`revm` 本地模拟、对数价格图上的负环检测、最优交易量 |
| **P9** | Rust MCP 量化研究智能体 | `rmcp` 暴露订单簿/回测/特征查询为 LLM 工具 |
| **P10** | Solana 链上 CLOB DEX | Anchor、账户模型、PDA、zero-copy 账户、计算单元预算 |

### 3.3 评分结果

| # | 系统 | A 难度 | 工时 | B 内功 | C 技术栈 | D 简历 | E AI | B+C+D+E |
|---|---|---|---|---|---|---|---|---|
| P1 | 订单簿 + 撮合引擎 | 5 | 40–60 | **9** | 4 | **10** | 2 | 25 |
| P2 | 交易所后端全栈 | 8 | 120–160 | **10** | 7 | 9 | 2 | 28 |
| P3 | 行情管道 + tick 存储 | 6 | 60–90 | 8 | **9** | 8 | 4 | 29 |
| P4 | 做市策略 + 微结构模拟 | 7 | 70–100 | 7 | 6 | **10** | 5 | 28 |
| P5 | 特征引擎 + ONNX 推理 | 7 | 60–90 | 7 | **9** | 9 | **9** | **34** |
| P6 | 期权 IV / 曲面 / 风险引擎 | 6 | 60–90 | 6 | 8 | 8 | 3 | 25 |
| P7 | 集中流动性 AMM 库 | 6 | 50–70 | 7 | 7 | 5 | 1 | 20 |
| P8 | CEX↔DEX 套利引擎 | 8 | 100–140 | 7 | **10** | 7 | 2 | 26 |
| P9 | MCP 量化智能体 | 4 | 30–40 | 4 | 7 | 4 | **10** | 25 |
| P10 | Solana 链上 CLOB | 6 | 60–90 | **3** | 9 | 6 | 1 | 19 |

### 3.4 三个反直觉结论

**结论 1:总分最高的 P5(34 分)不能作为起点。**
P5 的高分来源于它站在两个前置系统之上 —— 特征引擎需要订单簿事件流(P1)作为输入,需要
历史 tick(P3)作为训练与回放数据。评分模型衡量的是**项目价值**,不衡量**依赖顺序**。
把它排在第一位会在第一周就撞上「我的订单簿数据从哪来」这个死结。

**结论 2:P10 是陷阱选项。**
它的技术栈得分 9 极具吸引力,但内功得分仅 3。Anchor 是一个**受限的 Rust 子集**:无 std、
无线程、堆分配受限、大部分 crate 不可用、计算单元预算强约束。在其中作者会学到 Solana 的
账户模型,但学不到 Rust 的类型系统与所有权工程学。**用 Anchor 学 Rust,如同用模板引擎学
编程。** 区块链方向应当在 Rust 能力扎实之后进入,而非用它来获得 Rust 能力。

**结论 3:P9 不是项目,是外挂层。**
30–40 小时、内功 4 分,且脱离前置系统即为空壳(MCP 工具背后必须有真实能力可暴露)。
它的正确定位是任何主线项目的**最后一段**,而非独立选项。

### 3.5 选定主线

**P1 → P3 → P5**,即 **Rung**。

选择理由是**依赖闭合性**:P1 依赖为零且能消除 G1–G7 中的 7 项;P3 消费 P1 的事件类型并
产出 P5 所需的历史数据;P5 消费前两者。三段中每一段都可独立交付,且后一段严格复用前一段
的公共 API —— 这个「复用」本身就是对 crate 边界设计质量的检验。

P2 与 P4 保留为可选第四段(服务化 / 策略);P9 保留为可随时挂载的 AI 层。

---

## 4. 系统架构

### 4.1 命名

**Rung** —— 梯级。

在交易实务中,深度行情显示(DOM, Depth of Market)被交易员直接称为 **the ladder**(梯子),
其上每一个价格档位就是一个 **rung**(梯级)。这个名字因此在两个层面成立:

1. **系统层**:订单簿的基本组成单位就是价格档位,`rung` 即 `price level`;
2. **路径层**:项目本身分三段十阶,每一阶恰好登一级 —— 消除一项语言能力缺口。

crate 命名沿用该隐喻:`rung-core` / `rung-match` / `rung-feed` / `rung-alpha` / `rung-cli`。

### 4.2 分层与 crate 分解

```
                        ┌──────────────────────────────────────┐
                        │             rung-cli                 │  S1.10 / S2 / S3
                        │  演示、录制、回放、TUI 深度显示      │  唯一允许 println! 的地方
                        └───────────────┬──────────────────────┘
                                        │
              ┌─────────────────────────┼──────────────────────────┐
              │                         │                          │
   ┌──────────▼──────────┐  ┌───────────▼──────────┐  ┌────────────▼─────────┐
   │     rung-feed       │  │     rung-alpha       │  │     rung-replay      │
   │  (S2)               │  │  (S3)                │  │  (S1.9)              │
   │  交易所接入          │  │  增量微结构特征       │  │  事件日志 + 确定性   │
   │  L2 增量重建         │  │  环形缓冲 / Welford   │  │  重放                │
   │  序列号缺口自愈      │  │  ONNX 热路径推理      │  │                      │
   │  Arrow/Parquet 落盘  │  │  延迟预算测量         │  │                      │
   └──────────┬──────────┘  └───────────┬──────────┘  └────────────┬─────────┘
              │                         │                          │
              └─────────────────────────┼──────────────────────────┘
                                        │
                        ┌───────────────▼──────────────────────┐
                        │            rung-match                │  S1.5 – S1.8
                        │  submit / cancel / amend             │
                        │  价格-时间优先、跨档位撮合            │
                        │  IOC / FOK / Post-Only               │
                        └───────────────┬──────────────────────┘
                                        │
                        ┌───────────────▼──────────────────────┐
                        │            rung-core                 │  S1.1 – S1.4
                        │  Price / Qty newtype · Side · Order  │
                        │  PriceLevel(FIFO)                   │
                        │  BookSide<K: Ord>(泛型服务两边)     │
                        │  OrderBook · 索引                    │
                        │  零 I/O · 零依赖 · no_std 可行        │
                        └──────────────────────────────────────┘
```

**依赖方向严格单向向下。** 上层可以依赖下层,下层永远不知道上层存在。

### 4.3 crate 边界的判据

Day 8 的学习得出了一条判据,它直接决定了本架构的形状:

> 模块之间的边界靠自觉(同一个 crate 内,加一个 `pub(crate)` 就能穿过去);
> crate 之间的边界是硬的 —— 没写进 `Cargo.toml` 的 `dependencies`,就是编译不过。
> 所以真正想让架构分层不被破坏,**必须拆 crate,不能只拆 mod**。

据此,每一条边界都要回答「这条边界要不要硬」:

| 边界 | 要不要硬 | 理由 | 结论 |
|---|---|---|---|
| 类型与容器 ↔ 撮合规则 | **要** | 撮合规则会反复改(加 TIF、加自成交防范),类型不该跟着动;且 `rung-core` 要保持 `no_std` 可行 | 拆 crate |
| 撮合 ↔ I/O | **要** | 撮合必须可在无网络、无文件系统的环境下完整测试;一旦 `rung-match` 能 `println!`,确定性回放就不可信 | 拆 crate |
| 行情接入 ↔ 特征计算 | **要** | 特征引擎必须能吃「回放的历史」和「实时的现在」两种输入而不改代码 | 拆 crate |
| `PriceLevel` ↔ `BookSide` | 不要 | 同属容器层,一起演化 | 同 crate 内拆 mod |

**`rung-core` 的一条硬性约束**:零依赖、零 I/O、`#![no_std]` 兼容(即使当前不启用)。
这不是洁癖 —— 它是一个**可执行的架构断言**:一旦有人试图在 core 里 `println!` 调试,
编译就会失败。架构规则由编译器执行,而非由文档规定。

### 4.4 数据模型的关键决策

以下是 §6 各阶段将要正面撞上的设计点。此处只陈述问题与候选,**不给结论** —— 结论由作者
在阶段中推导并填入 reflection。

**D1. 价格的表示。** `f64` / `i64` / `Price(i64)` newtype。
关键约束:`f64` 不实现 `Ord`(因 NaN 的存在,浮点数只有偏序而无全序),因此**不能作为
`BTreeMap` 的 key**。这个约束会在阶段 3 以编译错误的形式出现,而非在阶段 1 由文档告知。

**D2. 档位容器。** `Vec<(Price, Level)>` / `HashMap<Price, Level>` / `BTreeMap<Price, Level>`。
判据是「最频繁的操作是什么」—— 每笔订单都要问「当前最优价」,`HashMap` 无序因而出局。

**D3. 档位内的排队结构。** `Vec<Order>` / `VecDeque<Order>` / 侵入式链表。
价格-时间优先要求 FIFO:队头出、队尾进。`Vec` 的头部删除是 O(n)。

**D4. 买卖两边的复用。** 两份独立 struct / 存负价格 / `BookSide<K: Ord>` + `Reverse<Price>`。
这是 **G1(泛型)的触发点**:两份代码逻辑完全一致、仅 key 类型不同,是泛型存在理由的
教科书式实例。且方法论要求**先写重复代码,再消除它**(§5.2)。

**D5. 订单的存储与撤单定位。** `Rc<RefCell<Order>>` / `HashMap<OrderId, Order>` + 档位存 ID /
**slotmap 世代索引**。
这是本项目最重要的一个 Rust 特有决策点。撤单需要 O(1) 定位到订单在档位中的位置,朴素做法
是让档位持有订单的引用 —— 但 Rust 的所有权系统不允许两处同时拥有。三条出路各有代价:
`Rc<RefCell>` 引入运行时借用检查与引用计数开销;索引间接层增加一次哈希查找;
生成式 arena(如 `slotmap`、`thunderdome`)以每次访问一次额外比较为代价捕获 use-after-free,
被认为是生产环境的正确权衡 [4]。**这个决策必须由基准数字支撑,而非偏好**(阶段 8)。

**D6. 事件的确定性。** 撮合引擎不得读取系统时钟、不得使用 `HashMap` 的迭代顺序、不得依赖
任何非确定性来源。同一份事件日志重放两次,输出必须逐字节相同 —— 这是回测可信度的前提,
也是阶段 9 的验收条件。

---

## 5. 方法论:教学约束如何进入架构

### 5.1 阶段的准入与准出条件

本项目的每个阶段被形式化为一个四元组:

```
Stage_i = ⟨ 系统目标 S_i , 语言缺口 G_i , 验收断言 A_i , 不变量 I_i ⟩
```

约束条件:

- **C1(必要性)**:`S_i` 在不使用 `G_i` 的前提下无法优雅实现。若能绕过,则该阶段的
  教学价值为零 —— 它退化为「顺便学了个语法」。
- **C2(充分性)**:`G_i` 在 `S_i` 中被用到的深度,足以让作者在下一个阶段独立复用它。
- **C3(不提前)**:`G_j (j > i)` 不得在 `Stage_i` 中出现。提前引入会破坏「先痛,再给药」
  的结构。
- **C4(可验证)**:`A_i` 是可执行的断言,`I_i` 是可被随机测试反复冲击的性质。
  「我觉得懂了」不构成通过。

### 5.2 先痛,再给药

这是从 day5.5 与 orderbook 脚手架中继承的设计原则,本项目将其提升为硬约束。

典型实例(阶段 3):
1. 作者先按最笨的办法写出 `struct Bids` 与 `struct Asks`,各自实现 `insert` / `best` /
   `cancel`;
2. 写完后被要求逐行对照两份代码,回答:「除了 key 的类型,有任何区别吗?」
3. 答案是没有。此时泛型不再是一个语法特性,而是**对一个已经亲手制造的问题的解答**。

对照实验:如果先讲泛型再写订单簿,作者会写出正确的泛型代码,但无法回答「为什么这里要用
泛型而不是两份 struct」—— 因为他从未见过那两份 struct 长什么样。

### 5.3 五段式阶段结构

继承自 `projects/orderbook/`,已验证有效:

| 段 | 内容 | 约束 |
|---|---|---|
| ① 管道位置 | 上游给你什么,下游要什么 | 强制建立系统视角 |
| ② 要解决什么 · **不负责什么** | 划定本阶段边界 | 「不负责」比「负责」更重要 —— 它防止阶段膨胀 |
| ③ **思考阶梯** | 由粗到细的一串问题,每问标注需要哪天的笔记 | **不给答案**;答完自然推出结论 |
| ④ 候选方案与代价 | 列出选项及其代价 | **仍不给答案**,只给判据 |
| ⑤ ✍️ reflection | 作者填空 | 项目结束时应有一份「为什么这么选」的完整记录 |

其中 ③ 是核心。orderbook 脚手架中的原话:

> **③ 才是重点。** 不要跳过思考阶梯直接看 ④ —— 阶梯是按顺序问的,每一问都比上一问更具体,
> 答完你自己就会推出该用什么。④ 只给候选和代价,不给答案。

### 5.4 与既有教学管线的衔接

现有管线为:`src/main.rs` 现场记录(编号小节)→ `examples/practice.rs` 练习 →
`NOTES.md`(Goals 评分表 / 概念 / Q&A / 编译错误账本 / 次日计划)→ Notion 渲染 → 博客。

**已识别的冲突**:`notion_sync.py` 的 `SECTION_RE`(`^\s*//\s*-{4,}\s*(.*?)\s*-{4,}\s*$`)
针对单个 `main.rs` 切分,而 Rung 是多 crate workspace,不存在单一的 `main.rs`。

**解决方案**:阶段记录与项目代码分离。每个阶段在 `projects/rung/stages/stage-NN.md` 留下
完整的五段式记录与 reflection;若某阶段引入了值得单独成篇的语言能力(如 G1 泛型、G6 测试),
则同时开一个 `dayN/` 单文件记录走既有管线。项目代码本身不进 Notion 渲染 —— 它进的是
`cargo doc`。这一分工与「NOTES 是技术细节唯一来源、博客只写叙事」的既有原则一致。

---

## 6. 实施路径

### 6.1 总览

| 段 | 系统 | 周期 | 工时 | 消除的缺口 |
|---|---|---|---|---|
| **S1** | `rung-core` + `rung-match` + `rung-replay` + `rung-cli` | 3–4 周 | 40–60 h | G1 G2 G3 G4 G5 G6 G7 G9 |
| **S2** | `rung-feed` | 3–4 周 | 60–90 h | G8、G10、trait 抽象、feature flags |
| **S3** | `rung-alpha` | 3–4 周 | 60–90 h | 零分配热路径、FFI、性能剖析 |
| **S4**(可选) | `rung-gateway`(P2)或 `rung-mm`(P4) | 4–6 周 | 100–150 h | 事件驱动架构、崩溃恢复 |

### 6.2 S1 阶段拆解

仓库结构:

```
projects/rung/
├── Cargo.toml                 # [workspace] members
├── DESIGN.md                  # 本文件
├── stages/                    # 每阶段的五段式记录 + reflection
│   └── stage-01.md ... stage-10.md
├── crates/
│   ├── rung-core/
│   │   ├── src/lib.rs
│   │   ├── src/types.rs       # Price / Qty / Side / OrderId / TimeInForce
│   │   ├── src/order.rs       # Order / OrderStatus
│   │   ├── src/level.rs       # PriceLevel:档位内 FIFO
│   │   ├── src/side.rs        # BookSide<K: Ord>
│   │   ├── src/book.rs        # OrderBook
│   │   └── tests/             # 集成测试:只能触碰公共 API
│   ├── rung-match/
│   │   ├── src/lib.rs
│   │   ├── src/engine.rs      # submit / cancel / amend
│   │   ├── src/trade.rs       # Trade / ExecReport
│   │   └── tests/
│   ├── rung-replay/
│   └── rung-cli/
└── benches/                   # criterion
```

**阶段表**:

| 阶段 | 系统目标 S | 语言缺口 G | 验收 A | 不变量 I |
|---|---|---|---|---|
| **1** | workspace 建立;`Price(i64)` / `Qty(u64)` newtype;`Side`;`Order`;手写 `Display` | **G7** workspace + `lib.rs`;**G6** 第一个 `#[test]` | `cargo test -p rung-core` 绿;`cargo doc` 有内容;`Price::from_yuan(3.1)` 显示 `3.10` | 分↔元转换往返无损 |
| **2** | `PriceLevel`:档位内 FIFO,插入 / 弹出 / 按 id 撤单 | **G6** 单元测试 vs 集成测试的区别;`?` 实战 | 同价两笔,先挂的先出 | 档位总量 = 各单数量之和 |
| **3** | `BookSide<K: Ord>` —— **一份代码服务买卖两边** | **G1 泛型 `impl<T>` / `where` / trait bound**;`Reverse<T>`;`Ord` vs `PartialOrd` | 同一类型参数化出 bid/ask,`best_price()` 两边都对 | 最优买价 = 最高买价;最优卖价 = 最低卖价 |
| **4** | 订单索引 `HashMap<OrderId, Order>` + 档位存 ID;撤单 O(1) 定位;空档位回收 | **G2 生命周期**(返回借用时);借用冲突用 Copy 化解 | 撤单后档位与索引双向一致 | 索引中的每个 id 恰好出现在一个档位中;不存在空档位 |
| **5** | `rung-match`:`submit(taker) -> Vec<Trade>`,跨档位撮合,部分成交 | **G4 闭包**;**G5 迭代器链**(`.filter()` / `.take_while()`);`let-else` | 跨两档成交,成交价按 maker | **买一 < 卖一恒成立**;成交量守恒 |
| **6** | TimeInForce:IOC / FOK / Post-Only;`amend` | 枚举驱动的状态机 | 每种 TIF 一组表驱动测试 | FOK 要么全成要么零成;Post-Only 永不吃单 |
| **7** | **proptest 不变量测试** | 基于性质的测试;shrinking;不变量设计 | 10k 条随机订单流不破任何不变量 | I₁–I₆ 全部 |
| **8** | **criterion 基准 + 对比实验**:`BTreeMap` vs 数组化档位;`Rc<RefCell>` vs slotmap 世代索引 | **G3 智能指针的真实取舍**;**G9 基准与剖析** | 一份**带数字**的报告:p50 / p99 延迟、吞吐 | 优化前后行为等价(同一 proptest 套件双跑) |
| **9** | `rung-replay`:事件日志序列化 + 确定性重放 | serde 深化;时间源 trait 抽象 | 同一日志重放两次,输出逐字节相同 | **确定性**(D6) |
| **10** | `rung-cli`:实时刷新的深度显示 + 成交流水 | 终端渲染;把库拼成应用 | 可录屏演示 | — |

**S1 的硬门槛**:每阶段结束时 `cargo test --workspace` 必须全绿,且该阶段的不变量测试
必须存在。**不允许「先写完再补测试」** —— G6 是本项目要训练的能力本身,不是附属品。

### 6.3 S2 与 S3 概要

**S2(`rung-feed`)**:多交易所 WebSocket 接入 → `trait Exchange` 归一化为统一事件类型 →
L2 增量重建 + **序列号缺口检测与自动 resync** → Arrow/Parquet 列式落盘 → 回放查询。

技术要点:序列号断层是真实世界最容易写错的地方,也是「数据级别设计逻辑」的核心训练;
`trait Exchange` 抽象各家不同的消息格式,是 trait + 关联类型的天然战场;热路径上的 serde
反序列化开销需要测量。

**S3(`rung-alpha`)**:流式增量特征(OFI、microprice、队列失衡、已实现波动率、成交量时钟),
全部增量计算,不许重扫窗口;环形缓冲 + Welford 增量统计 + 缓存友好布局;训练在 Python、
导出 ONNX、Rust 侧用 `ort` [7] 或 `candle` 在热路径推理;**端到端延迟预算测量**
(特征 μs + 推理 μs)。

### 6.4 前置清理(开工前必做)

1. ~~`day7.5/option/src/main.rs` 编译不过~~ —— 已修复(2026-08-13),11 个编号节,`cargo run` 通过
2. ~~`day8/module/src/main.rs` 编译不过(E0603)~~ —— 已修复,11 个编号节,`cargo run` 通过
3. ~~`day8/NOTES.md` 为空模板~~ —— 已填写
4. **G10(`String` 深入)** —— 仍欠账,建议在 S1 阶段 1 前用半天补上,因为 `Display` 实现
   与 CLI 渲染都要用

---

## 7. 评估方法

拒绝「感觉学会了」。以下指标全部可执行、可复核。

### 7.1 语言能力指标

对每一项缺口 G1–G10,定义一个可 grep 的完成判据:

| 缺口 | 完成判据 | 目标 |
|---|---|---|
| G1 泛型 | `impl<` 在自写代码中的出现次数 | ≥ 3 |
| G2 生命周期 | 显式 `<'a>` 标注处数 | ≥ 2 |
| G3 智能指针 | `Box`/`Rc`/`RefCell`/arena 的**基准对比实验**份数 | ≥ 1(带数字) |
| G4 闭包 | `FnMut` / `impl Fn` 作为参数的函数数 | ≥ 2 |
| G5 迭代器 | `.filter()` 出现次数 | ≥ 5 |
| G6 测试 | `cargo test --workspace` 的测试总数 | ≥ 40 |
| G7 workspace | workspace members 数 | ≥ 4 |
| G8 async | `.await` 出现次数(S2) | ≥ 20 |
| G9 基准 | criterion benchmark 组数 | ≥ 3 |
| G10 String | `push_str` / `format!` / 字节 vs 字符的专门记录 | 1 篇 |

### 7.2 系统正确性指标

**不变量清单**(由 proptest 在随机订单流下持续冲击):

- **I₁** 撮合完成后,`best_bid < best_ask`(若两侧均非空)
- **I₂** 成交量守恒:Σ trade.qty = 被消耗的 maker 量 = taker 的成交量
- **I₃** 档位总量 = 该档位内各订单数量之和
- **I₄** 订单索引中的每个 id,恰好出现在一个档位中(不多不少)
- **I₅** 不存在空档位驻留在 `BTreeMap` 中
- **I₆** 价格-时间优先:同价档位内,先进入的先成交
- **I₇** FOK 订单要么全部成交,要么零成交且不挂单
- **I₈** Post-Only 订单永不产生成交
- **I₉**(S1.9)同一事件日志重放两次,输出逐字节相同

### 7.3 性能指标

| 指标 | 测量方式 | 说明 |
|---|---|---|
| 单笔 `submit` 延迟 p50 / p99 / p999 | criterion + 直方图 | 「高频」二字的唯一证据 |
| 撤单延迟 p50 / p99 | criterion | 检验 D5 的索引设计 |
| 吞吐(订单 / 秒) | criterion 批量 | — |
| 三种订单存储方案的对比 | 同一套 proptest + 同一套 bench | **必须给出数字,不接受偏好** |
| (S3)端到端特征 + 推理延迟预算 | 自建计时 + 火焰图 | 分解到每一步 |

### 7.4 库质量指标

```bash
cargo test --workspace          # 单元 + 集成测试
cargo test --doc                # doctest:公共 API 的示例必须能跑
cargo clippy --workspace -- -D warnings
cargo doc --workspace --no-deps # 公共 API 文档完整性
cargo bench                     # 阶段 8 起
```

额外要求:`rung-core` 在 S1 结束时应当是一个**别人可以 `cargo add` 的库** ——
即使不真的发布到 crates.io。判据是:一个不了解内部实现的人,仅凭 `cargo doc` 生成的文档
就能正确使用它。

---

## 8. 风险与效度威胁

| # | 风险 | 表现 | 缓解 |
|---|---|---|---|
| R1 | **阶段膨胀** | 某阶段做着做着变成三个阶段,进度崩溃 | 五段式的「② 不负责什么」是硬边界;超出的写进 backlog,不当场做 |
| R2 | **测试被推迟** | 「先写完功能再补测试」—— 这正是 G6 从未被消除的原因 | 阶段准出条件包含不变量测试;无测试不进下一阶段 |
| R3 | **过早优化** | 在阶段 3 就开始想 SIMD 和缓存行 | 性能工作全部集中在阶段 8,且必须先有基准数字 |
| R4 | **AI 辅助掩盖能力缺口** | 代码能跑,但作者说不出为什么这么写 | 每阶段的 ⑤ reflection 必须由作者手写;§7.1 的 grep 判据统计的是**自写代码** |
| R5 | **S2 的异步复杂度** | 在闭包还不熟的情况下同时撞上 async + 并发 | S1 的阶段 5 强制练闭包;S2 开工前单独补 `tokio` 基础 |
| R6 | **依赖真实交易所** | 交易所改协议 / 限流 / 需要 API key,阻塞 S2 | S2 先做**离线回放**再做实时接入;公共行情不需要 key |
| R7 | **范围与野心不匹配** | 三段共 160–240 小时,可能中途放弃 | 每段独立可交付;S1 单独完成即已是一份完整的库项目 |

**效度威胁**:§7.1 的 grep 判据可以被形式上满足而实质上落空(例如为了凑 `impl<` 次数而写
无意义的泛型)。这一点无法由工具排除,只能由 ⑤ reflection 与 §7.2 的不变量共同约束 ——
无意义的泛型不会让任何不变量更容易成立。

---

## 9. 相关工作

**限价订单簿实现。**
[OrderBook-rs](https://github.com/joaquinbejar/OrderBook-rs) [1] 是一个线程安全、面向低延迟
交易系统的 Rust 订单簿,聚焦并发访问模式与无锁数据结构。它是 S1 完成后的**逐行对照对象** ——
其线程安全设计会与 Rung 的单线程内核有本质差异,这个差异本身就是 S4 的教材。
RustQuant 的 LOB 文章 [2] 与 "a sub-microsecond orderbook in rust" [3] 提供了档位存储与
CPU 友好布局的两种不同路线。关于订单存储,[4] 指出生成式 arena(`slotmap`/`thunderdome`)
以每次访问一次额外比较为代价捕获 use-after-free,「对任何要上生产的东西大概是正确的权衡」。

**回测与交易框架。**
[hftbacktest](https://github.com/nkaz001/hftbacktest) [5] 的核心卖点正是 Rung 的 S4/P4 目标:
**限价单的队列位置建模**与**双向延迟**(feed latency 与 order latency 分离),基于全量
tick 数据做市场重放。[barter-rs](https://github.com/barter-rs/barter-rs) [6] 展示了另一种
crate 分解方式(Barter / Barter-Instrument / Barter-Data / Barter-Execution),其边界划分
与 §4.2 高度一致 —— 数据、执行、工具分离,可作为架构的旁证。
nautilus_trader 的 Rust crates 提供了事件驱动回测的完整参考。

**ML 推理。**
[`ort`](https://ort.pyke.io/) [7] 是 ONNX Runtime 的 Rust 绑定,2026 年的基准显示相对朴素
方案可达 9x 加速;`candle` 是 HuggingFace 的纯 Rust 框架,无 C/C++ 运行时依赖。二者的取舍
是 S3 的一个决策点。

**期权与 DeFi(未选中但记录在案)。**
[OpenGreeks](https://github.com/marketcalls/opengreeks) [8] 以零依赖 Rust 重实现
Black-76 / BSM 并经 PyO3 暴露给 Python,一次 200 期权链的全希腊字母刷新从 vollib 的 ~9 ms
降到 ~0.3 ms —— 这是「Rust 核心 + Python 接口」这一量化工作实际形态的样板。
[OptionStratLib](https://github.com/joaquinbejar/OptionStratLib) [9] 是更完整的期权策略库。

**与本工作的差异。**
上述系统全部以**系统能力**为唯一目标。本工作的差异在于引入了第二组约束(§5.1 的 C1–C4):
架构决策点被刻意对齐到语言能力缺口上。因此 Rung 在若干处会**故意选择次优的初始实现**
(如阶段 3 要求先写两份重复的 `Bids`/`Asks`),这在纯工程语境下是不可接受的,在本语境下
则是设计的一部分。

---

## 10. 结论

本文给出了 Rung 的完整设计:一个四(最终五)crate 的 Rust workspace,自限价订单簿内核起,
经行情管道,达微结构特征与在线推理。

其核心主张是:**语言学习与系统构建不应分先后,而应互为约束**。八天的基础学习产出了可运行
的代码,却产不出可复用的代码,原因不在于学得不够多,而在于「写脚本」与「写库」要求的是
两个不同的能力集合(§2.2 的观察)。后者只有在真实的架构压力下才会被逼出来 —— 泛型的必要性
不能由教材断言,只能由两份一模一样的 `Bids`/`Asks` 证明;crate 边界的必要性不能靠自觉,
只能靠 `Cargo.toml` 里没写依赖就编译不过。

因此本设计的可证伪之处在于 §7:如果 S1 结束时 `cargo test --workspace` 的测试数低于 40、
自写代码中 `impl<` 少于 3 处、或拿不出一份带 p99 数字的基准报告,那么这套方法就没有奏效,
应当回到「先补课再开工」的常规路线。

下一步是 S1 阶段 1:建立 workspace,定义 `Price` 与 `Qty`,并写下这个仓库的第一个 `#[test]`。

---

## 附录 A:Rust 能力矩阵

> **2026-08-14 修订**:下表按原「语言缺口」框架编制,现已被 `ROADMAP.md` 的
> 五部 14 阶段结构取代。四大主题(进阶数据结构 / 指针 / trait / 并发内存通信)的
> 落点见下方新增的一节,以及 `ROADMAP.md` §三。

### 四大主题的覆盖(2026-08-14 新增)

| 主题 | 阶段 | 自己实现什么 | 与什么对测 |
|---|---|---|---|
| **进阶数据结构** | 2 | 单向链表 + 索引式双向链表 | `VecDeque` |
| | 3 | 数组化 price ladder | `BTreeMap` |
| | 4 | 开放寻址哈希表 | `HashMap` |
| | 5 | 世代索引 arena | `Vec` + `Rc<RefCell>` |
| **指针** | 2 | `Box` / `Option::take` / 索引链接 / 递归 `Drop` | — |
| | 5 | `Rc` / `RefCell` 运行时 panic / `Weak` 破循环 / 世代索引 | — |
| | 7 | 自己实现 `Drop` 与 `Deref` | — |
| | 8 | `Arc` 与 `Send`/`Sync` | — |
| **trait** | 2 | `LevelQueue` + **关联类型 `Handle`**(O(n) vs O(1) 编码在类型里) | — |
| | 6 | `OrderBook<L: LevelQueue, I: OrderIndex>` 泛型组装 | — |
| | 7 | `dyn` 虚表 vs 单态化的**实测**代价;运算符 trait / `From` | — |
| **并发内存通信** | 8 | `Arc<Mutex>` 多线程抢锁 | 单线程基线 |
| | 9 | `mpsc::channel` + 单撮合线程 | 阶段 8 |
| | 10 | **自写 SPSC 环形缓冲**:原子索引 / `Acquire`-`Release` / **伪共享** | 阶段 9 |

### 原语言缺口矩阵(历史记录)

| 能力 | Day 1–8 | S1 目标 | S2 目标 | S3 目标 |
|---|---|---|---|---|
| 所有权 / 借用 / NLL | ✅ 扎实 | 深化(借用冲突的三种化解) | — | — |
| enum / match / 模式全谱 | ✅ 扎实 | 深化(TIF 状态机) | — | — |
| struct / impl / trait | ✅ 扎实 | 深化(trait 作为约束) | trait + 关联类型 | — |
| `Option` / `Result` / `?` | ✅ 刚完成 | 贯穿使用 | 错误分层 | — |
| 模块 / 可见性 | ✅ 刚完成 | → workspace | feature flags | — |
| **泛型 `impl<T>`** | ❌ 0 | **阶段 3** | 泛型 + async | — |
| **生命周期** | ❌ 0 | **阶段 4** | 零拷贝反序列化 | — |
| **智能指针** | ❌ 0 | **阶段 8**(带基准) | `Arc` | — |
| **闭包** | ❌ 0 | **阶段 5** | 回调 / 背压 | 策略注入 |
| **迭代器链** | ❌ `.filter()` 0 次 | **阶段 5** | 流处理 | 增量特征 |
| **测试** | ❌ 0 | **阶段 1 起贯穿** + proptest | 集成测试 | 差分测试 |
| **workspace** | ❌ 0 | **阶段 1** | 新增 crate | 新增 crate |
| **基准 / 剖析** | ❌ 0 | **阶段 8** | 吞吐 | 延迟预算 |
| **async / tokio** | ❌ 0 | — | **贯穿** | — |
| **FFI / ONNX** | ❌ 0 | — | — | **贯穿** |

## 附录 B:命名说明

**Rung** /rʌŋ/ n. 梯级,梯子的横档。

选择依据见 §4.1。备选方案及淘汰理由:

| 候选 | 淘汰理由 |
|---|---|
| `Stratum` | 与比特币矿池的 Stratum 协议撞名,同处加密语境易混淆 |
| `Echelon` | 语义正确(梯队),但与订单簿的行话距离较远 |
| `Ladder` | 过于直白,且作为 crate 名太通用 |
| `Tessera` | 与交易无语义关联 |

crate 前缀:`rung-`。二进制:`rung`。

## 附录 C:术语表

| 术语 | 英文 | 释义 |
|---|---|---|
| 限价订单簿 | Limit Order Book (LOB) | 所有未成交挂单按价格分档、档内按时间排队的数据结构 |
| 价格档位 | Price Level / Rung | 同一价格上的所有挂单构成的队列 |
| 价格-时间优先 | Price-Time Priority | 撮合规则:先比价格,同价比时间 |
| taker / maker | — | 主动吃单方 / 被动挂单方。成交价按 maker 的报价 |
| 买卖价差 | Spread | 最优卖价 − 最优买价 |
| TIF | Time In Force | 订单有效方式:IOC(立即成交否则撤销)/ FOK(全成否则全撤)/ Post-Only(只挂不吃) |
| L2 行情 | Level 2 | 按档位聚合的深度行情(相对 L3 的逐单行情) |
| 序列号缺口 | Sequence Gap | 增量行情丢包,必须触发快照重同步 |
| 队列位置 | Queue Position | 一笔挂单在档位队列中的排位,决定成交概率 |
| 逆向选择 | Adverse Selection | 做市商被信息更充分的对手方选择性成交而受损 |
| 订单流失衡 | Order Flow Imbalance (OFI) | 买卖订单流强度差的微结构特征 |
| 微观价格 | Microprice | 按买卖挂单量加权的中间价 |
| 不变量 | Invariant | 系统在任何合法操作序列后都必须成立的性质 |
| 基于性质的测试 | Property-Based Testing | 随机生成输入并断言不变量,失败时自动收缩到最小反例 |
| 生成式索引 | Generational Index | arena 中带世代号的索引,可检测悬垂引用 |
| 单态化 | Monomorphization | 编译期为每个具体类型各生成一份泛型代码 |

## 附录 D:参考文献

[1] joaquinbejar, *OrderBook-rs: A high-performance, thread-safe limit order book implementation written in Rust.* https://github.com/joaquinbejar/OrderBook-rs

[2] RustQuant, *Building a limit order book in Rust.* https://rustquant.dev/blog/limit-order-book/

[3] A. Challani, *a sub-microsecond orderbook in rust.* https://www.rymnc.com/posts/orderbook-for-modern-cpus/

[4] Rust 社区关于生成式 arena 与 use-after-free 检测的讨论,见 [3] 与 `slotmap` 文档。

[5] nkaz001, *hftbacktest: high-frequency trading and market-making backtesting tool accounting for limit orders, queue positions, and latencies.* https://github.com/nkaz001/hftbacktest

[6] barter-rs, *Barter: open-source Rust framework for building event-driven live-trading & backtesting systems.* https://github.com/barter-rs/barter-rs

[7] pyke, *ort — ONNX Runtime for Rust.* https://ort.pyke.io/

[8] marketcalls, *OpenGreeks: Fast options pricing, Greeks, and implied volatility. Rust core, Python API.* https://github.com/marketcalls/opengreeks

[9] joaquinbejar, *OptionStratLib: a comprehensive Rust library for options trading and strategy development.* https://github.com/joaquinbejar/OptionStratLib

[10] Selby Jennings, *Rust Quant Developer | Crypto HFT* 职位要求(2026)。https://www.selbyjennings.com/en-us/job/rust-quant-developer-crypto-hft-pr590207_1778738943

[11] Rustify, *Rust for Blockchain 2026: Why Solana, Polkadot & Near Chose Rust.* https://rustify.rs/articles/learn-blockchain-with-rust

[12] 本仓库既有材料:`projects/orderbook/src/main.rs`(五段式教学格式的原型)、
`day5.5/NOTES.md`(项目式学习与练习式学习的分野)、`day8/NOTES.md`(crate 边界判据)。

---

*本文档随项目推进更新。修订记录见 git log。*
