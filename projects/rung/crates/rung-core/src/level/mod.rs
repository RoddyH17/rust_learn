//! 阶段 2 · 档位内队列 —— 三份实现,一个接口。
//!
//! # 这个部件要解决什么
//!
//! 同一个价格上可能挂着很多笔单。它们之间按**时间优先**排队:先挂的先成交。
//! 所以档位内部是一个 FIFO 队列。
//!
//! 但它不只是队列 —— 还有一个要求把事情变复杂了:
//!
//! > **撤单必须能 O(1) 定位到任意一笔单并把它摘出来。**
//!
//! 做市商每秒改上千次报价(见 `PRIMER.md` §5),每次改价都是一次撤单。
//! 如果撤单要 O(n) 先把队列扫一遍找到它在哪,这个订单簿就没法用。
//!
//! # 三份实现
//!
//! | 实现 | 底层 | `push_back` | `pop_front` | `remove` | 教什么 |
//! |---|---|---|---|---|---|
//! | [`DequeLevel`] | `VecDeque` | O(1) | O(1) | **O(n)** | 基线 |
//! | [`BoxListLevel`] | `Option<Box<Node>>` 单向链表 | **O(n)** | O(1) | **O(n)** | `Box` / `take()` / 递归 `Drop` |
//! | [`ArenaLevel`] | `Vec<Slot>` + 索引双向链接 | O(1) | O(1) | **O(1)** | 索引代替指针 / 空闲链表 |
//!
//! [`BoxListLevel`] 在每一项上都不比 [`DequeLevel`] 好。**这是故意的** ——
//! 它存在的价值是让你亲手撞到两件事:
//!
//! 1. **单向链表没法有尾指针。** `Box` 是独占所有权,尾节点已经被前一个节点的
//!    `next` 拥有了,你不能再拿一个 `Box` 指向它。所以 `push_back` 只能从头遍历。
//! 2. **想加 `prev` 指针就彻底卡住了。** 双向链表里每个节点被两个地方指着
//!    (前一个的 `next` 和后一个的 `prev`),而 `Box` 只允许一个所有者。
//!
//! 撞到第 2 条的那一刻,你就理解了为什么 Rust 里写双向链表是出了名的难,
//! 以及为什么 [`ArenaLevel`] 要用**索引**代替指针。
//!
//! # 关联类型 `Handle` 不是装饰
//!
//! ```text
//! impl LevelQueue for DequeLevel { type Handle = OrderId; }   // 只能拿 id 去搜 → O(n)
//! impl LevelQueue for ArenaLevel { type Handle = NodeRef; }   // 直接指到节点 → O(1)
//! ```
//!
//! **整个复杂度差异被编码在「Handle 能是什么」里。**
//! 这就是关联类型存在的理由 —— 不同实现需要携带的「位置信息」根本不是同一种东西。

use crate::types::{OrderId, Qty};

mod arena;
mod boxlist;
mod deque;

pub use arena::{ArenaLevel, NodeRef};
pub use boxlist::BoxListLevel;
pub use deque::DequeLevel;

/// 一个档位内的 FIFO 队列。
///
/// 所有实现必须满足的**语义**(不管内部怎么做):
///
/// - `push_back` 进队尾,`pop_front` 出队头 —— 严格 FIFO
/// - `total_qty()` 恒等于队列中所有单的数量之和
/// - `remove(h)` 之后,该 handle **失效**,再用它是未定义行为
///   (阶段 5 的世代索引会修掉这一条)
pub trait LevelQueue: Default {
    /// 「这笔单在队列里的位置」。不同实现携带的信息不同 ——
    /// 这正是各实现 `remove` 复杂度不同的根源。
    type Handle: Copy + core::fmt::Debug;

    /// 空队列。
    fn new() -> Self;

    /// 队列里有几笔单。
    fn len(&self) -> usize;

    /// 行为规格:`len() == 0` → `true`。
    fn is_empty(&self) -> bool;

    /// 队列中所有单的数量之和。
    ///
    /// ⚠️ 这是一个**冗余缓存** —— 本来可以遍历求和算出来。
    /// 缓存必然有失效问题,所以每一个改动队列的方法都必须同步它。
    /// 把维护点收敛到最少的几个方法上,是这条不变量唯一可靠的守法。
    fn total_qty(&self) -> Qty;

    /// 挂到队尾,返回一个能在将来定位到它的 handle。
    ///
    /// 行为规格:
    /// - 数量为零的单**也接受**(校验是订单簿的职责,不是队列的)
    /// - `len` +1,`total_qty` 增加 `qty`
    fn push_back(&mut self, id: OrderId, qty: Qty) -> Self::Handle;

    /// 看一眼队头,不移除。空队列 → `None`。
    fn front(&self) -> Option<(OrderId, Qty)>;

    /// 摘掉队头。
    ///
    /// 行为规格:空队列 → `None`;否则 `len` -1,`total_qty` 减去它的数量。
    fn pop_front(&mut self) -> Option<(OrderId, Qty)>;

    /// 按 handle 摘掉任意位置的一笔单。
    ///
    /// 行为规格:
    /// - 找得到 → 返回 `Some((id, 剩余数量))`,`len` -1,`total_qty` 相应减少
    /// - 找不到(已经被摘过 / handle 不属于这个队列)→ `None`,队列不变
    ///
    /// **这是三份实现拉开差距的地方。**
    fn remove(&mut self, h: Self::Handle) -> Option<(OrderId, Qty)>;

    /// 队头部分成交:扣掉 `by` 这么多。
    ///
    /// 行为规格:
    /// - 空队列 → `Err(())`
    /// - 队头数量不够减 → `Err(())`,**什么都不改**
    /// - 够减 → 扣掉,`total_qty` 相应减少,返回 `Ok(())`。
    ///   注意扣到零时**不自动出队** —— 出不出队是撮合逻辑的决定,不是队列的
    ///
    /// # 关于返回类型
    ///
    /// clippy 的 `result_unit_err` 会对 `Result<(), ()>` 报警,建议用自定义错误类型。
    /// 这里**明确不采纳**,理由:这个操作只有一种失败方式(不够减),
    /// 造一个只有单个变体的错误枚举不增加任何信息,只增加噪音。
    ///
    /// 但注意这是一个**会随需求变化的判断**:一旦出现第二种失败原因
    /// (比如「队头被锁定」),就该立刻换成枚举。到那时 clippy 就是对的了。
    ///
    /// —— 能说清为什么不听 linter,和听 linter 一样重要。
    #[allow(clippy::result_unit_err)]
    fn reduce_front(&mut self, by: Qty) -> Result<(), ()>;

    /// 按队列顺序导出全部内容。
    ///
    /// 只用于测试与对比 —— 三份实现的 `to_vec()` 必须逐项相同。
    fn to_vec(&self) -> Vec<(OrderId, Qty)>;
}

// ===========================================================================
// 共享测试套件 —— 三份实现跑同一套
// ===========================================================================
//
// 这是 ECE 2400 PA5 里「generic tests templated on the type」的做法:
// 同一份测试代码,对每个实现各实例化一遍。
//
// 好处不只是省打字 —— 它**强制三份实现语义完全一致**。
// 只要有一份行为不同,它就会在同一条测试上红。

#[cfg(test)]
mod tests {
    // 这里不需要 `use super::*` —— 每个由宏生成的子模块自己写了 `use super::super::*`。

    macro_rules! level_suite {
        ($modname:ident, $ty:ty) => {
            mod $modname {
                use super::super::*;

                type L = $ty;

                fn q(n: u64) -> Qty {
                    Qty::new(n)
                }

                // ---- 空队列 ----

                #[test]
                fn 空队列的各项查询() {
                    let l = L::new();
                    assert_eq!(l.len(), 0);
                    assert!(l.is_empty());
                    assert_eq!(l.total_qty(), Qty::ZERO);
                    assert_eq!(l.front(), None);
                    assert_eq!(l.to_vec(), vec![]);
                }

                #[test]
                fn 空队列出队返回空() {
                    let mut l = L::new();
                    assert_eq!(l.pop_front(), None);
                    assert!(l.is_empty());
                }

                #[test]
                fn 空队列扣量报错() {
                    let mut l = L::new();
                    assert_eq!(l.reduce_front(q(1)), Err(()));
                }

                // ---- FIFO ----

                #[test]
                fn 先进先出() {
                    let mut l = L::new();
                    l.push_back(1, q(10));
                    l.push_back(2, q(20));
                    l.push_back(3, q(30));

                    assert_eq!(l.to_vec(), vec![(1, q(10)), (2, q(20)), (3, q(30))]);
                    assert_eq!(l.pop_front(), Some((1, q(10))));
                    assert_eq!(l.pop_front(), Some((2, q(20))));
                    assert_eq!(l.pop_front(), Some((3, q(30))));
                    assert_eq!(l.pop_front(), None);
                }

                #[test]
                fn 队头不移除() {
                    let mut l = L::new();
                    l.push_back(1, q(10));
                    l.push_back(2, q(20));

                    assert_eq!(l.front(), Some((1, q(10))));
                    assert_eq!(l.front(), Some((1, q(10))), "front 不改变队列");
                    assert_eq!(l.len(), 2);
                }

                // ---- 总量这条不变量 ----

                #[test]
                fn 总量等于各单之和() {
                    let mut l = L::new();
                    assert_eq!(l.total_qty(), Qty::ZERO);

                    l.push_back(1, q(10));
                    assert_eq!(l.total_qty(), q(10));

                    l.push_back(2, q(20));
                    assert_eq!(l.total_qty(), q(30));

                    l.pop_front();
                    assert_eq!(l.total_qty(), q(20));
                }

                #[test]
                fn 总量在任意操作序列后都对得上() {
                    let mut l = L::new();
                    let h1 = l.push_back(1, q(10));
                    let _h2 = l.push_back(2, q(20));
                    let h3 = l.push_back(3, q(30));

                    let _ = l.remove(h3);
                    let _ = l.pop_front();
                    l.push_back(4, q(40));
                    let _ = l.reduce_front(q(5));

                    let 手算: u64 = l.to_vec().iter().map(|(_, x)| x.get()).sum();
                    assert_eq!(l.total_qty().get(), 手算, "缓存的总量和实际内容不一致");
                    let _ = h1;
                }

                // ---- remove ----

                #[test]
                fn 摘掉中间一笔() {
                    let mut l = L::new();
                    l.push_back(1, q(10));
                    let h2 = l.push_back(2, q(20));
                    l.push_back(3, q(30));

                    assert_eq!(l.remove(h2), Some((2, q(20))));
                    assert_eq!(l.to_vec(), vec![(1, q(10)), (3, q(30))]);
                    assert_eq!(l.len(), 2);
                    assert_eq!(l.total_qty(), q(40));
                }

                #[test]
                fn 摘掉队头() {
                    let mut l = L::new();
                    let h1 = l.push_back(1, q(10));
                    l.push_back(2, q(20));

                    assert_eq!(l.remove(h1), Some((1, q(10))));
                    assert_eq!(l.front(), Some((2, q(20))), "队头要跟着变");
                    assert_eq!(l.to_vec(), vec![(2, q(20))]);
                }

                #[test]
                fn 摘掉队尾() {
                    let mut l = L::new();
                    l.push_back(1, q(10));
                    let h2 = l.push_back(2, q(20));

                    assert_eq!(l.remove(h2), Some((2, q(20))));
                    assert_eq!(l.to_vec(), vec![(1, q(10))]);

                    // 队尾摘掉之后还能继续挂 —— 尾指针要维护对
                    l.push_back(3, q(30));
                    assert_eq!(l.to_vec(), vec![(1, q(10)), (3, q(30))]);
                }

                #[test]
                fn 摘掉唯一一笔() {
                    let mut l = L::new();
                    let h = l.push_back(1, q(10));

                    assert_eq!(l.remove(h), Some((1, q(10))));
                    assert!(l.is_empty());
                    assert_eq!(l.front(), None);
                    assert_eq!(l.total_qty(), Qty::ZERO);

                    // 清空之后还能重新用
                    l.push_back(2, q(20));
                    assert_eq!(l.to_vec(), vec![(2, q(20))]);
                }

                #[test]
                fn 重复摘同一个handle返回空() {
                    let mut l = L::new();
                    let h = l.push_back(1, q(10));
                    l.push_back(2, q(20));

                    assert_eq!(l.remove(h), Some((1, q(10))));
                    assert_eq!(l.remove(h), None, "摘过的 handle 再摘要返回 None");
                    assert_eq!(l.len(), 1, "失败的 remove 不许改动队列");
                }

                // ---- reduce_front ----

                #[test]
                fn 队头部分成交() {
                    let mut l = L::new();
                    l.push_back(1, q(10));
                    l.push_back(2, q(20));

                    assert_eq!(l.reduce_front(q(3)), Ok(()));
                    assert_eq!(l.front(), Some((1, q(7))));
                    assert_eq!(l.total_qty(), q(27));
                    assert_eq!(l.len(), 2, "扣量不改变笔数");
                }

                #[test]
                fn 队头扣到零不自动出队() {
                    let mut l = L::new();
                    l.push_back(1, q(10));

                    assert_eq!(l.reduce_front(q(10)), Ok(()));
                    assert_eq!(l.front(), Some((1, Qty::ZERO)));
                    assert_eq!(l.len(), 1, "出不出队是撮合的决定,不是队列的");
                }

                #[test]
                fn 队头扣量超额时什么都不改() {
                    let mut l = L::new();
                    l.push_back(1, q(10));

                    assert_eq!(l.reduce_front(q(11)), Err(()));
                    assert_eq!(l.front(), Some((1, q(10))), "失败的扣量不许留下痕迹");
                    assert_eq!(l.total_qty(), q(10));
                }

                // ---- 规模 ----

                #[test]
                fn 大量挂单后顺序仍然正确() {
                    let mut l = L::new();
                    for i in 0..200u64 {
                        l.push_back(i, q(i + 1));
                    }
                    assert_eq!(l.len(), 200);

                    let v = l.to_vec();
                    for (i, (id, qty)) in v.iter().enumerate() {
                        assert_eq!(*id, i as u64);
                        assert_eq!(*qty, q(i as u64 + 1));
                    }
                }

                #[test]
                fn 挂满再全部出队() {
                    let mut l = L::new();
                    for i in 0..100u64 {
                        l.push_back(i, q(1));
                    }
                    for i in 0..100u64 {
                        assert_eq!(l.pop_front(), Some((i, q(1))));
                    }
                    assert!(l.is_empty());
                    assert_eq!(l.total_qty(), Qty::ZERO);
                }

                #[test]
                fn 反复挂满清空不出问题() {
                    // 这条专门压 ArenaLevel 的空闲链表:
                    // 摘掉的槽位要能被后来的 push_back 复用,而不是无限增长。
                    let mut l = L::new();
                    for round in 0..10u64 {
                        let mut hs = Vec::new();
                        for i in 0..20u64 {
                            hs.push(l.push_back(round * 100 + i, q(1)));
                        }
                        for h in hs {
                            assert!(l.remove(h).is_some());
                        }
                        assert!(l.is_empty(), "第 {round} 轮之后应该是空的");
                    }
                }
            }
        };
    }

    level_suite!(deque, DequeLevel);
    level_suite!(boxlist, BoxListLevel);
    level_suite!(arena, ArenaLevel);
}
