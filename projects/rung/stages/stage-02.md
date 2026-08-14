# 阶段 2 · 档位内队列:三份实现,一个接口

> **修订:2026-08-14。** 预计 10–14 小时,分 4 个子步。
>
> 前置:阶段 1 全绿(`NaiveBook` 跑通,拿到三种模式的基线数字)。

---

## 1. 引言

这是**数据结构与指针**这一部的第一阶段,也是整个项目里指针知识最密集的一段。

要解决的部件是:**同一个价格上的多笔订单怎么排队。**

时间优先意味着它是一个 FIFO 队列。但还有一条要求把事情变复杂了:

> **撤单必须能 O(1) 定位到任意一笔单并摘出来。**

做市商每秒改上千次报价(`PRIMER.md` §5),每次改价都是一次撤单。
撤单如果要 O(n) 先扫一遍找它在哪,这个订单簿就没法用。

你要写**三份**实现:

| 实现 | 底层 | `push_back` | `pop_front` | `remove` | 存在的理由 |
|---|---|---|---|---|---|
| `DequeLevel` | `VecDeque` | O(1) | O(1) | **O(n)** | 基线 |
| `BoxListLevel` | `Option<Box<Node>>` 单向链表 | **O(n)** | O(1) | **O(n)** | 撞墙用 |
| `ArenaLevel` | `Vec<Slot>` + 索引双向链接 | O(1) | O(1) | **O(1)** | 真实订单簿的做法 |

**`BoxListLevel` 在每一项上都不比 `DequeLevel` 好。这是故意的。**
它存在的唯一价值,是让你亲手撞到 Rust 所有权最经典的三堵墙 ——
撞完之后你才会明白 `ArenaLevel` 为什么要那么写。

### 现在的状态

```bash
cd projects/rung
./scripts/test.sh level
```

共享测试套件对三份实现各实例化一遍,加上专属测试,一共 **58 条**(deque 18 / boxlist 19 / arena 21),
外加 1 条 `#[ignore]` 的爆栈对照实验。全红。

---

## 2. 关联类型:整个阶段的支点

```rust
trait LevelQueue {
    type Handle: Copy;
    fn push_back(&mut self, id: OrderId, qty: Qty) -> Self::Handle;
    fn remove(&mut self, h: Self::Handle) -> Option<(OrderId, Qty)>;
    // ...
}
```

三份实现的 `Handle` 根本不是同一种东西:

```rust
impl LevelQueue for DequeLevel   { type Handle = OrderId; }   // 只能拿 id 现搜 → O(n)
impl LevelQueue for BoxListLevel { type Handle = OrderId; }   // 同上
impl LevelQueue for ArenaLevel   { type Handle = NodeRef; }   // 直接指到节点 → O(1)
```

**整个复杂度差异被编码在「Handle 能是什么」里。**

`VecDeque` 里没有稳定的位置标识 —— 元素下标会随 `pop_front` 整体漂移,
所以除了 id 之外没有任何东西能在下一次操作后还指得准。
`ArenaLevel` 的节点住在固定的槽位里,所以下标是稳定的。

这就是**关联类型存在的理由**:不同实现需要携带的信息类型不同,而不只是值不同。
如果 `Handle` 是一个泛型参数而不是关联类型,调用方就得自己指定它 —— 而那是实现细节。

---

## 3. 三堵墙(子步 2b 的全部意义)

### 墙一:单向链表没法有尾指针

你会很自然地想加个 `tail` 让 `push_back` 变 O(1):

```rust
struct BoxListLevel {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,   // ❌ 错的
}
```

`Box<T>` 是**独占所有权**。尾节点已经被倒数第二个节点的 `next` 拥有了,
你不能再造一个 `Box` 指向它 —— 那是两个所有者,会被 drop 两次。

所以 `push_back` 只能从头遍历到尾,**O(n)**。

### 墙二:想加 `prev` 就彻底卡住

双向链表里每个节点被两个地方指着:前一个的 `next`,后一个的 `prev`。
`Box` 只允许一个所有者。**这条路是死的。**

三条出路,代价不同:

| 出路 | 代价 | 本项目在哪用 |
|---|---|---|
| `Rc<RefCell<Node>>` + `Weak` | 引用计数开销 + 运行时借用检查会 **panic** | 阶段 5 对测 |
| `NonNull<Node>` + `unsafe` | 要自己保证别名规则,写错就是 UB | 阶段 5 延伸(可选) |
| **索引代替指针** | 多一次数组寻址,全程安全 | **本阶段的 `ArenaLevel`** |

### 墙三:递归 `Drop` 会爆栈

默认的 `Drop` 是递归的:drop 头节点 → drop 它的 `next` → 一路下去。
链表有几万个节点,栈就有几万层。**直接 stack overflow。**

所以必须**手写 `Drop`**,用循环把链表拆开:

```rust
while let Some(mut node) = self.head.take() {
    self.head = node.next.take();
    // node 在这里被 drop,它的 next 已经是 None,不再往下递归
}
```

`Option::take()` 是关键 —— 它把值拿走、原地留 `None`。
这是 Rust 里挪动「被别人拥有的值」的标准手法。

> **必做的对照实验**(子步 2b 结束时):
>
> 1. 把你写好的 `impl Drop for BoxListLevel` **整个注释掉**
> 2. `cargo test -p rung-core boxlist_递归drop会爆栈 -- --ignored --nocapture`
> 3. 应该看到 `thread ... has overflowed its stack`,而且是**直接杀进程**,
>    不是普通的测试失败
> 4. 恢复 `Drop`,再跑一次 → 通过
>
> **把两次的输出抄进报告。** 这是这个子步最值钱的一次观察。

---

## 4. 子步与测试闸门

**顺序是硬规定。每个子步跑绿才进下一步。**

| 子步 | 文件 | 内容 | 闸门 |
|---|---|---|---|
| **2a** | `src/level/deque.rs` | `DequeLevel` —— 基线 | `./scripts/test.sh level::tests::deque` |
| **2b** | `src/level/boxlist.rs` | `BoxListLevel` + 手写 `Drop` | `./scripts/test.sh boxlist` |
| **2c** | `src/level/arena.rs` | `ArenaLevel` + 空闲链表 | `./scripts/test.sh arena` |
| **2d** | — | 全绿 + 清警告 + 跑基准 + 写报告 | `./scripts/eval.sh level` |

### 2c 的三个专属测试盯的是什么

除了共享套件之外,`arena.rs` 里有三条只针对它的测试:

| 测试 | 抓什么错 |
|---|---|
| `arena_槽位会被复用` | 没写空闲链表 → `Vec` 无限增长。正确实现只需 1 个槽,错的会涨到 10000 |
| `arena_摘队尾后尾指针正确回退` | `tail` 没跟着回退 → 下一次 `push_back` 接错地方 |
| `arena_链接不成环` | 链接接错 → `to_vec()` 死循环或漏元素 |

双向链表的 `unlink` 有**四种情况**,四种都要对:中间 / 队头 / 队尾 / 唯一一个。
共享套件里各有一条测试盯着。

### 2d 的收尾清单

1. `./scripts/test.sh` —— 全绿
2. 删掉 `src/lib.rs` 顶上的 `#![allow(unused_variables)]` 和 `#![allow(dead_code)]`
   (如果阶段 1 还没删)
3. `./scripts/lint.sh` —— 零警告
4. `./scripts/eval.sh level` —— 拿到三份实现的对比数字
5. 做墙三的对照实验
6. 写 `reports/stage-02.md` 和 `decisions/ADR-003-档位队列选型.md`

---

## 5. 评测

```bash
./scripts/eval.sh level          # 全部规模
./scripts/eval.sh level 5000     # 指定规模
```

负载形状:**挂 n 笔 → 随机摘掉一半 → 剩下的全部出队**。
照真实档位设计 —— `push_back` 每笔都做,`remove` 是撤单(占比高),`pop_front` 是成交。

三份实现跑**完全相同**的操作序列(同一个种子,打乱在计时之外完成),数字可以直接比。

### 预期与陷阱

- `BoxListLevel` 应该在每一项上都更慢
- `ArenaLevel` 的优势应该**随 n 增大而扩大**(`remove` 是 O(1) 而非 O(n))
- 小 n 时 `ArenaLevel` 可能反而**输给** `DequeLevel` —— 因为 `VecDeque` 内存连续、缓存友好,
  而 arena 每次跳转都是一次随机访问

**最后这一条如果真的出现了,那是报告里最值得写的一段。**
它说明「渐进复杂度更好」和「实际更快」是两件事,分界点在哪要靠测。

---

## 6. 报告

`reports/stage-02.md`,**≤4 页**,模板见 [`../reports/TEMPLATE.md`](../reports/TEMPLATE.md)。

这一阶段的报告有三个必须回答的问题:

1. **三份实现的 `remove` 复杂度为什么不同?** 追到「Handle 携带了什么信息」这一层
2. **`ArenaLevel` 从哪个 n 开始赢过 `DequeLevel`?** 给一个具体的数
3. **墙三的对照实验**:注释掉 `Drop` 前后的输出各贴一段

§4 复杂度表这次要填满 —— 三列(三份实现)× 五行(五个操作)。
**说清 N 指什么**:这里是「档位内的挂单笔数」,不是全簿的。

---

## 7. ADR

### `decisions/ADR-003-档位队列选型.md`

**这份 ADR 在三份实现都测完之后写。** 那时「代价」是测出来的数字。

候选就是这三份 + 一个你没实现的:`Rc<RefCell<Node>>` 双向链表。
第四个别漏 —— 你要能说出为什么本阶段不走那条路(提示:运行时借用检查的代价,
以及它把一个编译期错误变成了运行时 panic)。

「何时推翻」段必须写到 `ArenaLevel` 的**已知缺陷**:

> `NodeRef(3)` 指的是「下标 3」。如果下标 3 的节点被摘掉、槽位被复用给了另一笔单,
> 那个旧 handle 会指到**新的那笔单**上,而且不报错。这叫 **ABA 问题**。

本阶段先接受它(测试不构造这种情况)。**阶段 5 的世代索引会修掉** ——
给每个槽加版本号,handle 带上版本,对不上就返回 `None`。

---

## 8. 完成判据

- [ ] 2a–2d 四个子步依次跑绿(**不许跳步**)
- [ ] `./scripts/test.sh` 全绿
- [ ] `./scripts/lint.sh` 零警告
- [ ] `./scripts/eval.sh level` 拿到三份实现的对比数字
- [ ] 墙三对照实验做了,两次输出都记下来了
- [ ] `reports/stage-02.md` 写完,≤4 页,含三个必答问题
- [ ] `ADR-003` 写完,含四个候选和 ABA 缺陷
- [ ] `git commit -m "rung stage 2: 档位队列三份实现"`

做完告诉我,我出 `stage-03.md`(价格档位索引:`BTreeMap` vs 数组化 ladder)。

---

## 附:这一阶段覆盖了什么

对照老师点名的四项:

| 主题 | 本阶段的落点 |
|---|---|
| **进阶数据结构** | 单向链表、双向链表、空闲链表(一个最小的内存分配器) |
| **指针** | `Box` / `Option<Box>` / `Option::take()` / 索引代替指针 / **手写 `Drop`** |
| **trait** | `LevelQueue` + **关联类型 `Handle`**;三份实现跑同一套测试 |
| **并发** | 不在本阶段。见阶段 8–10 |
