# Rung 实施路线:从第 0 步到可交付的库

> `DESIGN.md` 回答**做什么 / 为什么**(基线审计、选型评分、架构推导、评估指标)。
> 本文回答**怎么做**:从纸上作业到 S1 可交付的库,全部代码任务逐条列出。

---

## 一、两条轴:学 与 想

这是整个路线的骨架。它们**不是同一件事**,必须分开走,又必须在同一个阶段里同时发生。

```
     纵轴 · 学(由浅入深)              横轴 · 想(系统级)
     一阶段恰好一项语言能力            一阶段恰好一个架构决策
     不许提前引入                      记进 ADR,不记进代码注释
     判据:能不能独立复用              判据:说不说得出代价和何时该推翻
            │                                    │
   阶段 1   │ workspace + 第一个 #[test]         │ ADR-001/002 定点整数 + newtype
   阶段 2   │ VecDeque + ? + Option              │ ADR-003 冗余缓存与不变量
   阶段 3   │ ★ 泛型 impl<T> / where             │ ADR-004 抽象泄漏:Reverse 归谁
   阶段 4   │ ★ 生命周期 / 借用冲突              │ ADR-005 单一真相源  ADR-007 逻辑时钟
   阶段 5   │ ★ 闭包 + 迭代器链                  │ ADR-006 crate 边界与 API 承诺
   阶段 6   │ 枚举状态机 / matches!              │ ADR-008 原子性:两遍扫描 vs 回滚
   阶段 7   │ ★ proptest 基于性质的测试          │ 测试策略:手写测试为什么漏了它
   阶段 8   │ ★ Box/Rc/RefCell + criterion       │ ADR-009 用数字做决定,不用偏好
   阶段 9   │ serde 深化 / trait 抽象时间源      │ ADR-010 纯核心 + 副作用边缘
   阶段 10  │ 把库拼成应用                       │ 回看:哪个决定当初选错会全盘推倒
```

**★ = `DESIGN.md` §2.2 里那 10 项零覆盖能力的消除点。**

硬约束(`DESIGN.md` §5.1 的 C1):某阶段的系统目标,在**不使用**该阶段的语言能力时必须无法优雅实现。若能绕过,该阶段的教学价值为零。

### 横轴的载体:ADR

每阶段产出一份 `decisions/ADR-NNN-*.md`,格式见 [`decisions/TEMPLATE.md`](decisions/TEMPLATE.md):
**背景 / 候选(≥3)/ 决定 / 代价 / 何时推翻**。

**ADR 与代码注释的分工**:注释解释「这段代码在干什么」,ADR 解释「为什么不是另一种」。
项目结束时这十份 ADR 比代码更有价值 —— 它是能拿去和人讨论的东西。

---

## 二、协作分工:脚手架逐阶段退场

| 阶段 | Claude 交付 | 你交付 | 你在练什么 |
|---|---|---|---|
| **1–2** | 完整类型定义 + 方法签名 + `todo!()` 空体 + 全部 `#[test]` | 填函数体,直到测试全绿 | 语法与工具链 |
| **3–6** | 只给 `#[test]` 文件 + 类型定义 | **自己设计方法签名**,再实现 | API 设计 |
| **7–10** | 只给「要保证什么」的自然语言描述 | 自己写测试、自己写实现 | 独立工程 |

阶段 1-2 的 `todo!()` 让 `cargo test` 能编译但会 panic —— 红 → 绿,验收机械、不需要判断。
从阶段 3 起不再给签名,因为**签名就是 API 设计**,替你写掉等于替你上完了这门课。

每阶段的提示词 `stages/stage-NN.md` 沿用五段式:
① 管道位置 ② 要解决什么 · **不负责什么** ③ **思考阶梯**(不给答案) ④ 候选与代价(仍不给答案) ⑤ ✍️ reflection。

---

## 三、单次 session 的节奏(2–3 小时)

```
1. 读 stage-NN.md 的 ①②③          20 min   不碰键盘,先把思考阶梯答完
2. 写 ⑤ reflection 的前半           10 min   「我打算怎么做,为什么」
3. cargo test —— 看它红              5 min   先看见失败长什么样
4. 实现,直到绿                   60-90 min
5. cargo clippy / cargo doc         10 min   把警告清零
6. 写 ADR-NNN                       20 min   ★ 最容易被跳过,也最不该跳
7. 补 ⑤ reflection 的后半           10 min   「实际怎么做的,和打算的差在哪」
8. git commit(一阶段一个 commit)
```

第 3 步「先看它红」不是仪式 —— 阶段 7 的 proptest 会给你一个从没见过的失败形状,提前熟悉红色能省很多时间。

---

## 四、阶段 0 · 纸上作业(0 行代码,~60 min)

见 [`stages/stage-00.md`](stages/stage-00.md)。

| 任务 | 内容 |
|---|---|
| T0.1 | 用自己的话重画管道图 |
| T0.2 | 订单状态机 + **哪些转移不允许** |
| T0.3 | `OrderBook` 的问题清单 + 调用频率(≥8 条)—— **最重要的一题**,容器选型全靠它 |
| T0.4 | crate 依赖图 + 每条边界的「硬度」判定 |
| T0.5 | 三个第一反应,写完锁死 |

---

## 五、S1 代码任务全清单

### 目标结构(S1 结束时)

```
projects/rung/
├── Cargo.toml                    # [workspace] members
├── DESIGN.md · ROADMAP.md · README.md
├── stages/stage-00.md … stage-10.md
├── decisions/TEMPLATE.md · ADR-001.md … ADR-010.md
├── crates/
│   ├── rung-core/                # 阶段 1-4 · 零依赖 · 零 I/O
│   │   ├── src/{lib,types,order,level,side,book}.rs
│   │   └── tests/public_api.rs   # 集成测试:只能碰 pub
│   ├── rung-match/               # 阶段 5-6 · src/{lib,engine,exec}.rs
│   ├── rung-replay/              # 阶段 9
│   └── rung-cli/                 # 阶段 10
└── benches/orderbook.rs          # 阶段 8 · criterion
```

### 阶段 1 · workspace + 类型基座 + 第一个测试
**语言缺口:G7(workspace / `lib.rs` / `pub` 边界)、G6(第一个 `#[test]`)**
> `String` 欠账(day7 + day7.5 各欠一次)折进本阶段:写 `Display` 时必然碰 `format!`,
> 在 `stage-01.md` 里用一节讲透 `push_str` vs `+` 的所有权差别、为什么 `String` 不能下标索引。

| 任务 | 内容 |
|---|---|
| T1.1 | 根 `Cargo.toml`:`[workspace] members`、`[workspace.package]` 统一 edition 2024 |
| T1.2 | `cargo new --lib crates/rung-core`,依赖区**保持为空**(架构断言:core 零依赖) |
| T1.3 | `types.rs`:`Price` 最小单位存储 + `from_yuan` / `to_yuan` / `Display`(补零) |
| T1.4 | `Qty`:`is_zero()`、`checked_sub()` 返回 `Option`(数量不能为负,类型说了算) |
| T1.5 | `Side` / `OrderId` / `Seq`;`Side::opposite()` |
| T1.6 | **derive 决策**:要能当 `BTreeMap` key?要 `Copy`?要 `{:?}`?→ ADR-002 |
| T1.7 | `lib.rs`:`mod` 声明 + `pub use` 重导出,**决定哪些进公共 API** |
| T1.8 | 第一个 `#[cfg(test)] mod tests`:往返无损、`Display` 补零、下溢返回 `None` |
| T1.9 | `tests/public_api.rs`:**只 `use rung_core::...`**,碰不到私有项 —— 边界的第一次实证 |
| T1.10 | 一个 doctest,`cargo test --doc` 要绿 |

**系统追问 → ADR-001 / 002**:这个库的用户会怎么**误用**价格类型?能不能让误用直接编译不过?

### 阶段 2 · PriceLevel:档位内 FIFO
**语言缺口:`VecDeque`、`?` 实战、单元测试 vs 集成测试的区别**

| 任务 | 内容 |
|---|---|
| T2.1 | `level.rs`:队列 + 冗余的总量字段 |
| T2.2 | `push_back` / `front` / `pop_front` / `remove(id)` |
| T2.3 | 总量的维护:每个改动队列的方法都必须同步它 |
| T2.4 | `is_empty` / `len` / `total_qty` |
| T2.5 | 测试:FIFO 顺序、删中间元素后总量仍对、空档位判定 |
| T2.6 | **不变量测试**:总量 == Σ 队列中各单数量(此时手写,阶段 7 交给 proptest) |

**系统追问 → ADR-003**:冗余总量是缓存,缓存必然有失效问题 —— 你怎么保证它**永不**失效?
另一问:`remove(id)` 是 O(n)。**什么规模下这会成为问题?量化它**,别只说「可能会慢」。

### 阶段 3 · BookSide<K: Ord> —— 泛型的诞生 ★
**语言缺口:G1 泛型 `impl<T>` / `where` / trait bound / 单态化**
**顺序是硬规定,不许调换:**

| 任务 | 内容 |
|---|---|
| T3.0 | **先写两份重复代码**:`Bids` 与 `Asks` 各自实现 insert / best / remove。**必须真的写出来** |
| T3.1 | 逐行对照,记录:重复了多少行?差异**只在**哪里? |
| T3.2 | 试 `BTreeMap<f64, _>` → 收下 `f64: Ord is not satisfied`。回看阶段 1 的选择救了你什么 |
| T3.3 | 合并为泛型 `BookSide<K>` + trait bound |
| T3.4 | 买盘用 `Reverse<Price>` 作 key,卖盘用 `Price` |
| T3.5 | `best_key` / `level_mut` / `entry_or_default` / `prune_empty` |
| T3.6 | `iter_from_best()`:按优先级遍历档位(阶段 5 撮合要用) |
| T3.7 | 删掉 T3.0 的两份 struct,测试仍全绿 |

**系统追问 → ADR-004**:`Reverse` 是 `BookSide` 的实现细节还是 `OrderBook` 的?
若 `best_bid()` 返回 `Reverse<Price>`,调用方每次都要解包 —— 这叫**抽象泄漏**。这层包装该在哪里剥掉?
另一问:`BookSide<Price>` 和 `BookSide<Reverse<Price>>` 编译后是一份机器码还是两份?对体积和编译时间意味着什么?

### 阶段 4 · OrderBook:索引、所有权与生命周期 ★
**语言缺口:G2 显式生命周期、借用冲突的化解**

| 任务 | 内容 |
|---|---|
| T4.1 | `book.rs`:两个 `BookSide` + 订单索引 + 序号计数器 |
| T4.2 | **单一真相源**:索引持有订单本体,档位队列只存 id。想清楚为什么不能反过来 |
| T4.3 | `insert` 返回 `Result`:重复 id 要拒绝 |
| T4.4 | 序号自增写进订单 —— **时间优先用单调序号,不读系统时钟**(ADR-007;阶段 9 的确定性靠它) |
| T4.5 | `cancel` 的四种情况(id 不存在 / 档位不存在 / 档位内找不到 / 删完档位空了) |
| T4.6 | 空档位回收 —— 不清理会在阶段 5 咬你(最优价会指向一个空档位) |
| T4.7 | `best_bid` / `best_ask` / `spread`:在这里剥掉 `Reverse` |
| T4.8 | **生命周期实战**:返回借用的迭代器,编译器会要求你标注 |
| T4.9 | 测试 + 不变量:索引与档位双向一致;不存在空档位 |

**系统追问 → ADR-005**:100 万笔挂单时索引会怎样?内存布局?缓存命中率?
**不要求现在解决** —— 要求写下「我知道它会是问题,当前规模下我接受,信号 X 出现时回来改」。这就是 ADR「何时推翻」段存在的意义。

### 阶段 5 · rung-match:撮合 ★
**语言缺口:G4 闭包、G5 迭代器链(`.filter()` / `.take_while()`)、`let-else`**

| 任务 | 内容 |
|---|---|
| T5.0 | `cargo new --lib crates/rung-match`,依赖 `rung-core = { path = "../rung-core" }` |
| T5.1 | 试写 `impl OrderBook { ... }` → **编译不过**(不能给外部 crate 的类型加固有方法)。收下这个错 |
| T5.2 | 改用**扩展 trait**:`trait Matching` + `impl Matching for OrderBook` |
| T5.3 | `exec.rs`:成交、拒绝原因枚举、执行回报 |
| T5.4 | 撮合主循环:crossing 判断 |
| T5.5 | **成交价按 maker** —— 想清楚为什么 |
| T5.6 | 三种数量关系,**分支顺序会影响结果** |
| T5.7 | 跨档位:吃完一档继续下一档 |
| T5.8 | 剩余量挂回簿子 |
| T5.9 | 借用冲突:先把 key `Copy` 出来再动手(阶段 1 让价格是 `Copy` 的理由在此兑现) |
| T5.10 | **迭代器链**:用 `take_while` 表达「一直吃到不 crossing 为止」 |
| T5.11 | 自成交防范 |
| T5.12 | 测试:跨两档、成交价、部分成交、价格不够时原样挂上、不自成交 |

**系统追问 → ADR-006**:写到一半会发现 core 的 `pub` 不够用,你要回去改 core。
**这次改动会不会破坏 core 的其他使用者?** 新加的 `pub` 是一个**承诺**,一旦公开,改它就要升大版本。
所以真正的问题是:core 该暴露「档位的内部结构」,还是「一个能按优先级消费档位的方法」?两种 API 的未来自由度差在哪?

### 阶段 6 · TimeInForce 状态机
**语言缺口:枚举驱动的状态机、`matches!`、表驱动测试**

| 任务 | 内容 |
|---|---|
| T6.1–6.5 | GTC / IOC / FOK / PostOnly 四种语义 |
| T6.6 | `amend`:改价 = 撤 + 重挂(**丢失时间优先级**);仅减量 = 原地改(**保留优先级**) |
| T6.7 | 表驱动测试:每种 TIF × 每种簿状态 |

**系统追问 → ADR-008**:FOK 要「先知道能不能全成」,两条路 ——
(a) 预演:只读扫一遍算可成交总量;(b) 执行 + 回滚。
**(b) 在 Rust 里为什么特别难?**(`&mut self` 已经改了状态,回滚要你自己存快照 —— 要么克隆整个簿,要么记 undo 日志)
延伸:「预演」能力别处还需要吗(风控预检、模拟撮合)?**什么时候该提取抽象?出现第二个调用者时,不是第一个。**

### 阶段 7 · proptest:不变量 ★
**语言缺口:G6 深化 —— 基于性质的测试、shrinking**

| 任务 | 内容 |
|---|---|
| T7.1 | `[dev-dependencies] proptest`(dev,不进 core 运行时依赖) |
| T7.2 | 随机订单流生成器(价格限定窄区间以制造 crossing) |
| T7.3 | 把 `DESIGN.md` §7.2 的 I₁–I₈ 逐条编码成断言 |
| T7.4 | 跑 10k 条随机序列 |
| T7.5 | **预期会真找到 bug** —— 记录 shrink 的最小反例,修复,固化成回归测试 |

**系统追问**:proptest 找到的那个 bug,**手写测试为什么没发现?**
你手写测试时脑子里有个「典型场景」的模型,bug 恰好在模型之外。**写下那个模型的边界在哪 —— 这比修 bug 本身值钱。**

### 阶段 8 · criterion + 三方案对比 ★
**语言缺口:G3 `Box`/`Rc`/`RefCell`/arena、G9 基准与剖析**

| 任务 | 内容 |
|---|---|
| T8.1 | `benches/` + criterion;合成订单流(**固定随机种子**,保证可复现) |
| T8.2 | 方案 A(当前):索引 + 档位存 id |
| T8.3 | 方案 B:`Rc<RefCell<Order>>`,档位直接持有共享指针 |
| T8.4 | 方案 C:`slotmap` 世代索引 arena |
| T8.5 | 三个实现跑**同一套** proptest,证明行为等价(否则性能对比无意义) |
| T8.6 | 测:submit / cancel 的 p50/p99/p999、吞吐、峰值内存 |
| T8.7 | 火焰图定位热点 |
| T8.8 | 报告写进 ADR-009,**必须有数字** |

**系统追问 → ADR-009**:若方案 C 的 p99 好 3 倍但复杂度翻倍,**选哪个?**
没有标准答案,但有标准**答法**:说出判断依据(A 的 p99 是否已够用?复杂度成本由谁承担?什么信号出现时切换?)。
方案 B 还会让你亲身撞上**运行时借用 panic** —— 编译期检查换成运行时检查的代价,在这里是可测量的。

### 阶段 9 · rung-replay:确定性
**语言缺口:serde 深化、用 trait 抽象时间源**

| 任务 | 内容 |
|---|---|
| T9.1 | `Event` 枚举 + serde |
| T9.2 | 审查 `rung-match`:有没有读系统时钟 / 用 `HashMap` 迭代顺序 / 依赖指针地址?**全部消除** |
| T9.3 | 事件日志写入(JSON Lines 起步,够用就行) |
| T9.4 | `replay(log)` |
| T9.5 | 测试:同一日志重放两次,输出**逐字节相同**(I₉) |
| T9.6 | 测试:重放结果与实时撮合一致 |

**系统追问 → ADR-010**:确定性要求内核不许读时钟,那真实系统的时间戳从哪来?
答案是从**边缘**注入 —— 网关打时间戳,内核只接受已打好的事件。这个模式叫
**functional core, imperative shell**:纯的、可测的、确定的核心,被薄薄一层做 I/O 的壳包住。
写下:你的 crate 划分里哪些是 core,哪些是 shell?(直接决定 S2 `rung-feed` 的形状)

### 阶段 10 · rung-cli:把库拼成应用

| 任务 | 内容 |
|---|---|
| T10.1 | `cargo new crates/rung-cli`(binary) |
| T10.2 | ladder 渲染:两侧深度、最优价高亮、spread |
| T10.3 | 回放驱动:按事件推进,可暂停 / 单步 |
| T10.4 | 成交流水滚动显示 |
| T10.5 | `README.md`:这个库是什么、怎么 `cargo add`、最小可运行例子 |
| T10.6 | 录一段演示 |

**系统追问(收尾)**:回看阶段 1–9,**哪一个决定如果当初选错,会导致后面全部推倒重来?**
标记出来 —— 那才是这个系统的架构决策,其余的都只是实现选择。
**不可逆 vs 可逆,这两者的区别是架构师和程序员的分界线。**

---

## 六、S1 交付物与判据

| 类别 | 产出 | 判据 |
|---|---|---|
| 代码 | 5 个 crate 的 workspace | `cargo build --workspace` 绿 |
| 测试 | 单元 + 集成 + doctest + proptest | ≥ 40 个测试 |
| 性能 | criterion 报告 + 火焰图 | 有 p50/p99/p999 数字 |
| 文档 | `cargo doc` + README | 陌生人只看文档能正确使用 `rung-core` |
| **决策** | **ADR-001 … ADR-010** | 每份都有「代价」和「何时推翻」两段 |
| 记录 | `stages/stage-00.md … stage-10.md` | reflection 全部填写 |

**语言缺口消除判据**(`DESIGN.md` §7.1):
`impl<` ≥ 3 · 显式 `<'a>` ≥ 2 · 智能指针基准对比 ≥ 1 份带数字 · `impl Fn` 参数 ≥ 2 · `.filter()` ≥ 5 · 测试 ≥ 40 · workspace members ≥ 4 · criterion 组 ≥ 3

---

## 七、给 S2 预留的接口

阶段 9 的 `Event` 枚举就是 S2 `rung-feed` 的**汇合点**:实时行情归一化后产出的也是 `Event`,喂给同一个撮合内核。
所以定 `Event` 时要问一句:「交易所的增量行情能不能翻译成这几个变体?」

**不要为此提前设计** —— 只在 ADR-010 里写下这个约束,S2 开工时回来看。

---

## 八、验证

```bash
cd projects/rung

cargo test --workspace          # 单元 + 集成
cargo test --doc                # 公共 API 的文档示例必须能跑
cargo clippy --workspace -- -D warnings
cargo doc --workspace --no-deps --open
cargo bench                     # 阶段 8 起
cargo run -p rung-cli           # 阶段 10

# 语言缺口的机械判据
grep -rn "impl<" crates/ --include=*.rs | wc -l
grep -rn "\.filter(" crates/ --include=*.rs | wc -l
cargo test --workspace 2>&1 | grep "test result"
```

**每阶段的硬门槛**:`cargo test --workspace` 全绿 + 该阶段的不变量测试存在 + ADR 已写。

**不允许「先写完再补测试」,也不允许「先写完再补 ADR」** ——
事后补写的 ADR 会变成对既成事实的辩护,而不是决策记录。

---

## 九、当前进度

| 文档 | 状态 |
|---|---|
| 设计报告 `DESIGN.md`(做什么 / 为什么) | ✅ 完成 |
| 实施路线 `ROADMAP.md`(怎么做,本文) | ✅ 完成 |
| **领域科普 [`PRIMER.md`](PRIMER.md)**(交易所 / 流动性 / CEX vs DEX) | ✅ 完成 |
| **架构方法 [`METHOD.md`](METHOD.md)**(系统设计怎么想,七条方法) | ✅ 完成 |
| ADR 模板 `decisions/TEMPLATE.md` | ✅ 完成 |

| 阶段 | 状态 |
|---|---|
| **阶段 0 · 纸上作业** | 📋 提示词就绪 → [`stages/stage-00.md`](stages/stage-00.md) |
| **阶段 1 · 类型基座** | 🔨 **骨架 + 27 个测试就绪,24 红** → [`stages/stage-01.md`](stages/stage-01.md) |
| 阶段 2–10 | ⏸ 未开始 |

### 阅读顺序

```
1. PRIMER.md    ← 先知道自己在给谁设计系统(交易所、流动性、做市商)
2. METHOD.md    ← 再知道怎么思考(七条方法 + 工作示例)
3. stage-00.md  ← 纸上作业,0 行代码。T0.5 写完就锁死
4. stage-01.md  ← 开始写代码,把 24 个红变成绿
```

**下一步**:

```bash
cd projects/rung && cargo test --workspace     # 先看它红
```

在 stage-00 的 T0.5 写下三个第一反应之前,不要读 `stage-01.md` ——
那里已经有答案了。那五行字是整个项目唯一一次能记录「你还不知道答案时是怎么想的」的机会,
过了阶段 3 就再也写不出来。
