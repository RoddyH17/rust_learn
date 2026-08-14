# 阶段 1 · workspace + 类型基座 + 第一个测试

> **预计 3–4 小时。** 骨架已经给你了,你的工作是把 18 个 `todo!()` 填掉。
>
> **开工前必读**:[`../PRIMER.md`](../PRIMER.md)(交易所和流动性是什么)、
> [`../METHOD.md`](../METHOD.md)(系统设计怎么想)。
> 没读这两篇就做,你会写出能跑的代码但答不出为什么。
>
> **还没做 [`stage-00.md`](stage-00.md) 的话先去做** —— 尤其是 T0.5 的三个第一反应。
> 这个文件里已经有答案了,先看会毁掉那份记录。

---

## 现在的状态

```bash
cd projects/rung
cargo test --workspace
```

```
lib   :  1 passed; 17 failed        ← 全部 panic 在 todo!()
tests :  2 passed;  4 failed
doc   :  0 passed;  3 failed
                    ─────────
                    24 红,3 绿
```

**先跑一遍,看它红。** 这不是仪式 —— 你要熟悉失败长什么样,因为接下来九个阶段你会看很多次。

你的任务:把 24 变成 0。

---

## ① 管道位置

```
[ 你在这里 ] ──▶ 阶段 2 档位 ──▶ 阶段 3 泛型两边 ──▶ 阶段 4 索引 ──▶ 阶段 5 撮合
```

**上游给你**:阶段 0 的五张纸。
**下游要的**:一组能被塞进容器、能比大小、能打印给人看的类型。

具体到下游的三个硬需求:

| 下游 | 需要什么 | 不满足会怎样 |
|---|---|---|
| 阶段 3 的 `BTreeMap<Price, _>` | `Price: Ord` | 编译不过 —— `f64` 就是死在这一条上 |
| 阶段 5 的借用冲突化解 | `Price: Copy` | 借用检查器拦你,绕不过去 |
| 阶段 10 的 CLI | `Price: Display` | 屏幕上打不出 `10.05` |

---

## ② 这一步要解决什么 · 不负责什么

**要解决**:给「价格」「数量」「买卖方向」「订单」四个概念定出 Rust 类型,
并让这个 crate 成为一个**别人能用的库**(有公共 API 边界、有测试、有文档)。

**不负责**:
- 不负责任何容器 —— `BTreeMap` / `VecDeque` 一个都不出现
- 不负责撮合、不负责挂单撤单
- 不负责性能 —— 阶段 8 才谈这个
- 不负责「订单状态」 —— 现在只有一个剩余量字段,够用

---

## ③ 思考阶梯

**按顺序答。每一问都比上一问更具体。**

### Q1. 钱能用小数表示吗

先做个实验,随便找个地方跑:

```rust
println!("{}", 0.1 + 0.2 == 0.3);
println!("{:.20}", 0.1 + 0.2);
println!("{:.20}", 10.05);
```

看到结果之后回答:一笔 10.05 元的订单,如果存成 `f64`,五分钟后你把它读出来和另一笔
10.05 元的订单比较,会相等吗?**你敢赌吗?**

【需要用到】不需要新知识,但这决定后面所有事。

✍️ 你的回答:


### Q2. 那用什么

交易所的实际做法在 [`PRIMER.md`](../PRIMER.md) §2 讲过了:**价格是离散的,最小单位叫 tick**。
所以存「多少个 tick」,一个整数。

现在的问题是:**为什么是 `i64` 而不是 `u64`?** 价格明明不可能是负的。

提示:去看 `types.rs` 里 `Price::diff_ticks` 的文档注释。
然后想:`best_bid - best_ask` 这个中间结果,在什么情况下是负的?

✍️ 你的回答:


### Q3. tick size 是全局常量,这个假设什么时候会崩 ★

`types.rs` 里写着:

```rust
pub const TICKS_PER_UNIT: i64 = 100;
```

这意味着**整个库假设所有品种的 tick size 都是 0.01**。

但真实交易所不是这样:低价股的 tick 可能是 0.001,指数期货可能是 0.25,
加密货币的 tick 随价格区间变化。

**这一问不要求你现在解决。** 要求你回答三件事:

1. 这个假设让什么变简单了?(想想如果 tick size 是运行时变量,`Price` 要多带什么)
2. 它什么时候会崩?给一个**具体的场景**,不要说「以后可能」
3. 崩的时候要改多少地方?—— 这决定了它是可逆还是不可逆的决定(`METHOD.md` 方法五)

【需要用到】`METHOD.md` 方法五、方法六。这是你的第一份 ADR 的核心内容。

✍️ 你的回答:
1.
2.
3.


### Q4. newtype 的代价

`Price` 可以直接用 `i64`。包成 `struct Price(i64)` 之后:

- `price_a < price_b` 还能写吗?
- `price + 100` 还能写吗?
- `println!("{}", price)` 还能写吗?

先猜,再去看代码验证(提示:看 `derive` 那一行,以及有没有 `impl Add`)。

然后回答:**你为这个 newtype 付了什么代价,换来了什么?**

【需要用到】day6「Debug 可以 derive,Display 不行」那一节 ——
这是你第一次有**真实理由**手写 `Display`:内部存的是 1005,给人看的是 10.05,
中间那道转换只有你知道,编译器不可能替你猜。

✍️ 你的回答:


### Q5. 那 OrderId 为什么不是 newtype ★

```rust
pub type OrderId = u64;   // 类型别名,不是 newtype
pub struct Price(i64);     // newtype
```

**这是一个故意留下的不一致。** `type` 别名在编译器眼里和 `u64` 完全是同一个类型 ——
`fn cancel(id: OrderId)` 你传个 `seq` 进去,编译器不会拦。

问自己:

1. 这个不一致是**偷懒**,还是**有理由**?
2. 什么时候 newtype 值得付那个代价,什么时候不值得?**给一条你自己的判断标准。**
3. `OrderId` 和 `Seq` 都是 `u64` 别名。这两个混用会出什么事?严重吗?

【提示】判断标准可以从「混用了会不会被发现」入手 —— 见 `METHOD.md` 方法六的
「编译期错误 → 运行时 panic → 静默错误」那张表。

✍️ 你的回答:
1.
2. 我的标准:
3.


### Q6. 模块为什么不 pub ★

`lib.rs` 里是这样写的:

```rust
mod types;                      // 注意:没有 pub
pub use types::{Price, Qty, ...};
```

而不是:

```rust
pub mod types;                  // 使用者写 rung_core::types::Price
```

两种写法使用者看到的路径不同。但真正的差别不在路径上。

问:**半年后你想把 `types.rs` 拆成 `price.rs` / `qty.rs` / `side.rs` 三个文件。
两种写法各要付什么代价?**

【需要用到】day8 的可见性规则。这是它第一次派上真实用场。

✍️ 你的回答:


### Q7. derive 实验:亲手拿掉,看它痛在哪 ★

这一问**必须动手做**,不能只想。

`Price` 现在的 derive 是:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
```

依次做四个实验,每次**只去掉一个**,`cargo test` 看报什么错,记下来,再加回去:

| 实验 | 去掉 | 预期哪里会炸 | 实际报错 |
|---|---|---|---|
| a | `Debug` | | ✍️ |
| b | `PartialEq` | | ✍️ |
| c | `Ord`(和 `PartialOrd`) | | ✍️ |
| d | `Copy` | | ✍️ |

实验 d 最值得做 —— 去掉 `Copy` 之后,报错会是 `use of moved value` 之类的**所有权错误**,
和前三个完全不是一类。想清楚为什么。

然后回答:**这些能力里,哪些是给「使用者」的,哪些是给「实现者」的?**

✍️ 你的回答:


---

## ④ 候选方案与代价

阶段 1 的候选已经在骨架里替你选好了(这是脚手架阶段的约定)。
但你要能说出**每一个选择的代价** —— 说不出来就是没想清楚(`METHOD.md` 方法六)。

| 决定 | 选了 | 代价是什么 |
|---|---|---|
| 价格的表示 | `Price(i64)`,tick 为单位 | ✍️ |
| tick size | 编译期常量 `TICKS_PER_UNIT` | ✍️ |
| 数量的表示 | `Qty(u64)`,减法走 `checked_sub` | ✍️ |
| `OrderId` | 类型别名而非 newtype | ✍️ |
| 模块可见性 | `mod` 私有 + `pub use` 重导出 | ✍️ |
| `Order` 的字段 | 全 `pub` | ✍️ |

最后一行是**已知的隐患**,不是疏漏 —— 见 `order.rs` 的文档注释。
现在不要修它,阶段 2 做出档位缓存、阶段 7 的 proptest 撞出 bug 之后再回来。

---

## 📖 String 补课(欠了两天了)

day7 和 day7.5 各欠了一次 `String`。现在还:因为**你马上要手写 `Display`**,
而那正是 `String` 的主场。

### 1. `format!` 和 `write!` 的区别 —— 这是性能问题不是风格问题

```rust
// ❌ 多分配一次
impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}.{:02}", self.0 / 100, self.0 % 100))
    }
}

// ✅ 直接往 f 里写,零分配
impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:02}", self.0 / 100, self.0 % 100)
    }
}
```

`format!` **在堆上分配一个新 `String`**,然后你把它整个拷进 `f`,再把它丢掉。
`write!` 直接往 `f` 这个缓冲区里写。

在 CLI 里一秒打几十行,这点开销无所谓。但撮合日志一秒可能几十万行 ——
**一次堆分配大约几十到上百纳秒,乘以几十万就是几十毫秒。** 这就是为什么它是性能问题。

### 2. `push_str` vs `+` —— 所有权的差别

```rust
let mut s = String::from("10");
s.push_str(".05");          // s 还是 s,只是变长了。没有移动
println!("{s}");            // ✅ 能用

let a = String::from("10");
let b = String::from(".05");
let c = a + &b;             // a 被**移动**进去了!
// println!("{a}");         // ❌ E0382: borrow of moved value
println!("{b}");            // ✅ b 只是被借用,还活着
```

`+` 的签名是 `fn add(self, other: &str) -> String` —— 注意左边是 `self` 不是 `&self`。
它**拿走**左边的所有权,在原有的缓冲区上追加,然后把它还给你。
这样设计是为了避免再分配一次,代价是原来那个绑定不能用了。

规律:**`+` 左边必须是 `String`(被移动),右边必须是 `&str`(被借用)。**

### 3. 为什么 `String` 不能用下标

```rust
let s = String::from("你好");
// let c = s[0];            // ❌ E0277: 根本没有实现 Index<usize>
println!("{}", s.len());    // 6,不是 2 —— 「你」和「好」各占 3 个字节
```

`String` 是 **UTF-8 编码的字节序列**。一个字符可能占 1 到 4 个字节:
ASCII 占 1 个,中文占 3 个,emoji 占 4 个。

所以 `s[0]` 这个写法有歧义:你要第 0 个**字节**,还是第 0 个**字符**?

- 要字节:`s.as_bytes()[0]` —— 得到 `228`,一个没有意义的数
- 要字符:`s.chars().nth(0)` —— 得到 `'你'`,但这是 **O(n)**,因为必须从头数

**Rust 的选择是干脆不提供下标**,逼你明确说要哪个。
`&s[0..3]` 这种切片语法倒是有,但它按**字节**切,切到字符中间会**运行时 panic**:

```rust
let s = String::from("你好");
let ok = &s[0..3];          // "你"  ✅ 正好切在字符边界
// let bad = &s[0..1];      // 💥 panic: byte index 1 is not a char boundary
```

这一条直接接上 day4 的切片:`&str` 是「一段 UTF-8 字节的视图」,
而**视图的边界必须落在字符边界上**。

### 4. 这三条和你现在要写的代码的关系

| 你要写的 | 用到哪条 |
|---|---|
| `Price::fmt` | 第 1 条:用 `write!` 不用 `format!` |
| `Side::fmt` 打 `"BUY"` | 第 1 条:`write!(f, "BUY")` 就够了,不需要 `String` |
| 补零 `3.10` | `{:02}` 格式说明符,不是字符串拼接 |
| 负数 `-0.50` | 符号要单独处理 —— `-50 / 100 == 0`,直接套公式会打出 `"0.-50"` |

✍️ **String 补课的 reflection**:上面三条里,哪一条你之前理解错了?


---

## 要写的东西(任务清单)

| # | 任务 | 文件 |
|---|---|---|
| T1.1 | ~~workspace 根 `Cargo.toml`~~ | ✅ 已给 |
| T1.2 | ~~`rung-core` 建起来,依赖区为空~~ | ✅ 已给 |
| T1.3 | `Price::from_units` / `to_units` / `diff_ticks` / `Display` | `src/types.rs` |
| T1.4 | `Qty::is_zero` / `checked_sub` / `checked_add` / `min` / `Display` | `src/types.rs` |
| T1.5 | `Side::opposite` / `higher_is_better` / `Display` | `src/types.rs` |
| T1.5b | `Order::new` / `is_filled` | `src/order.rs` |
| T1.6 | **derive 实验**(③ 的 Q7),结论写进 ADR-002 | — |
| T1.7 | ~~`lib.rs` 的公共 API 边界~~ | ✅ 已给,但要答 Q6 |
| T1.8 | ~~单元测试~~ | ✅ 已给 18 个 |
| T1.9 | ~~集成测试~~ | ✅ 已给 6 个 |
| T1.10 | ~~doctest~~ | ✅ 已给 3 个 |

实际要写的就是 **18 个 `todo!()`**,加上两份 ADR。

**填完之后**:把 `lib.rs` 顶上的 `#![allow(unused_variables)]` 删掉。
删了还有警告,说明你漏了什么。

---

## 验收

```bash
cd projects/rung
cargo test --workspace          # 27 个测试全绿(18 + 6 + 3)
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps --open
```

三条全过 + 两份 ADR 写完 = 阶段 1 结束。

**验收的一个额外要求**:打开 `cargo doc` 生成的网页,假装你是第一次看到这个库的人。
你能不能只靠这个页面正确用起来?不能的话,缺的是哪一段文档?

---

## 要产出的 ADR

### `decisions/ADR-001-价格用定点整数.md`

核心是 ③ 的 **Q1 + Q2 + Q3**。

「候选」至少三个:`f64` / 裸 `i64` / `Price(i64)` newtype / 十进制定点库(如 `rust_decimal`)。
最后一个别漏 —— 它是真实项目里常见的选择,你要能说出为什么这个项目不用它
(提示:回看 `Cargo.toml` 里那条架构断言)。

**「何时推翻」段必须写具体**,Q3 的第 2 问就是答案。

### `decisions/ADR-002-newtype与derive.md`

核心是 ③ 的 **Q4 + Q5 + Q7**。

重点回答:**newtype 的判断标准是什么?** 不要写「重要的类型就包」——
写一条你能拿去判断下一个类型的标准。

Q7 的四个实验结果做成表格附在里面 —— 那是**实测数据**,比任何论证都有说服力。

---

## ⑤ ✍️ Reflection

**开工前写(「我打算怎么做」)**:

✍️ 我预计最难的是 ______,因为 ______


**收工后写(「实际怎么样」)**:

✍️ 实际最难的是 ______,和我预计的 ______(一样 / 不一样)

✍️ 我卡最久的一个编译错误是:
  错误:
  原因:
  怎么修的:

✍️ 现在回看 [`stage-00.md`](stage-00.md) 的 T0.5 第 1 问(价格用什么类型),
  我当时写的是 ______,现在我 ______(改了 / 没改)。
  **让我改的是** ______

✍️ 这个阶段学到的、和 Rust 语法无关的一件事:


---

## 完成判据

- [ ] `cargo test --workspace` 27 绿
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 零警告
- [ ] `lib.rs` 的 `#![allow(unused_variables)]` 已删除
- [ ] ③ 思考阶梯 Q1–Q7 全部填写(Q7 必须真的动手做实验)
- [ ] ④ 的六条代价全部填写
- [ ] String 补课的 reflection 填写
- [ ] `ADR-001` 与 `ADR-002` 写完,每份都有「代价」和「何时推翻」
- [ ] ⑤ reflection 前后两半都填
- [ ] `git commit -m "rung stage 1: 类型基座"`

做完告诉我,我出 `stage-02.md`(档位内的 FIFO)。
从阶段 3 起我不再给方法签名 —— **签名就是 API 设计,那是你要练的东西。**
