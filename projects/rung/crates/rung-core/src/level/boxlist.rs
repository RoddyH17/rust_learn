//! 子步 2b · `BoxListLevel` —— `Box` 单向链表。
//!
//! # 这份实现在每一项上都不如 `DequeLevel`。这是故意的。
//!
//! 它存在的唯一价值,是让你亲手撞到 Rust 所有权最经典的两堵墙。
//! 撞完之后你才会明白 `ArenaLevel` 为什么要那么写。
//!
//! ## 墙一:单向链表没法有尾指针
//!
//! 你会很自然地想加一个 `tail` 字段让 `push_back` 变成 O(1):
//!
//! ```text
//! struct BoxListLevel {
//!     head: Option<Box<Node>>,
//!     tail: Option<Box<Node>>,   // ❌ 这行是错的
//! }
//! ```
//!
//! 错在哪:`Box<T>` 是**独占所有权**。尾节点已经被倒数第二个节点的 `next` 拥有了。
//! 你不能再造一个 `Box` 指向同一个节点 —— 那就是两个所有者,
//! 而两个所有者意味着会被 drop 两次。
//!
//! 所以 `push_back` 只能**从头遍历到尾**,O(n)。
//!
//! ## 墙二:想加 `prev` 就彻底卡住
//!
//! 双向链表里每个节点被两个地方指着:前一个的 `next` 和后一个的 `prev`。
//! `Box` 只允许一个所有者。**这条路是死的。**
//!
//! 出路有三条,代价各不相同:
//!
//! | 出路 | 代价 |
//! |---|---|
//! | `Rc<RefCell<Node>>` + `Weak` | 引用计数开销 + 运行时借用检查会 **panic** |
//! | `NonNull<Node>` + `unsafe` | 要自己保证别名规则,写错就是 UB |
//! | **用索引代替指针** | 多一次数组寻址,但全程安全 |
//!
//! [`super::ArenaLevel`] 走的是第三条。阶段 5 会把第一条也测一遍。
//!
//! ## 墙三:递归 `Drop` 会爆栈
//!
//! 默认的 `Drop` 是递归的:drop 头节点 → drop 它的 `next` → drop 下一个……
//! 链表有十万个节点,栈就有十万层。**直接 stack overflow。**
//!
//! 所以你必须**手写 `Drop`**,用循环把链表拆开。这是 `Drop` trait 的第一个真实用途。

use super::LevelQueue;
use crate::types::{OrderId, Qty};

/// 链表节点。
///
/// `next` 是 `Option<Box<Node>>`:
/// - `Option` 表达「可能没有下一个」
/// - `Box` 表达「这个节点在堆上,而且我独占它」
struct Node {
    id: OrderId,
    qty: Qty,
    next: Option<Box<Node>>,
}

/// `Box` 单向链表实现的档位队列。
///
/// **注意没有 `tail` 字段** —— 见模块文档的「墙一」。
pub struct BoxListLevel {
    head: Option<Box<Node>>,
    len: usize,
    total: Qty,
}

impl Default for BoxListLevel {
    fn default() -> Self {
        Self {
            head: None,
            len: 0,
            total: Qty::ZERO,
        }
    }
}

/// 手写 `Drop`,把递归拆成循环。
///
/// # 为什么必须手写
///
/// 编译器生成的 `Drop` 是递归的。十万个节点 = 十万层栈 = stack overflow。
///
/// # 怎么写
///
/// 循环地把头节点**取出来**(`take()`),再把它的 `next` 接上去当新的头。
/// 每一轮结束时,取出来的那个节点离开作用域被 drop —— 但它的 `next` 已经是
/// `None` 了,所以不会再往下递归。
///
/// ```text
/// while let Some(mut node) = self.head.take() {
///     self.head = node.next.take();
///     // node 在这里被 drop,它的 next 已经是 None,不递归
/// }
/// ```
///
/// *提示*:`Option::take()` 把值拿走,原地留下 `None`。
/// 这是 Rust 里挪动「被别人拥有的值」的标准手法 —— 因为你不能直接把它移走
/// (那会让原来的位置处于未初始化状态)。
impl Drop for BoxListLevel {
    fn drop(&mut self) {
        todo!("2b:把递归 drop 改成循环。写完跑 boxlist_不会因为长链表爆栈 这条测试")
    }
}

impl LevelQueue for BoxListLevel {
    /// 和 `DequeLevel` 一样只能拿 id 现搜 —— 单向链表也没有稳定的位置标识。
    type Handle = OrderId;

    fn new() -> Self {
        todo!("2b")
    }

    fn len(&self) -> usize {
        todo!("2b")
    }

    fn is_empty(&self) -> bool {
        todo!("2b")
    }

    fn total_qty(&self) -> Qty {
        todo!("2b")
    }

    /// **O(n)** —— 必须从头走到尾。见模块文档的「墙一」。
    ///
    /// *提示*:你需要一个 `&mut Option<Box<Node>>` 的游标,一路往下走到 `None` 那一格,
    /// 然后把新节点放进去:
    ///
    /// ```text
    /// let mut cur = &mut self.head;
    /// while cur.is_some() {
    ///     cur = &mut cur.as_mut().unwrap().next;
    /// }
    /// *cur = Some(Box::new(Node { .. }));
    /// ```
    ///
    /// 上面这段能编译,但借用检查器对它的容忍是有条件的。
    /// 如果你写成别的形状撞上了 E0499(同时存在两个可变借用),
    /// **把报错抄进报告** —— 那是这个子步最值钱的一次编译错误。
    fn push_back(&mut self, id: OrderId, qty: Qty) -> Self::Handle {
        todo!("2b")
    }

    /// *提示*:`self.head.as_ref().map(|n| (n.id, n.qty))`
    fn front(&self) -> Option<(OrderId, Qty)> {
        todo!("2b")
    }

    /// *提示*:`take()` 拿走头节点,把它的 `next` 装回 `self.head`,返回它的内容。
    fn pop_front(&mut self) -> Option<(OrderId, Qty)> {
        todo!("2b")
    }

    /// **O(n)** —— 遍历找到前驱,然后把它的 `next` 接到被删节点的 `next` 上。
    ///
    /// *提示*:单向链表的删除永远要**先找到前驱**,因为只有前驱能改指向。
    /// 头节点是特例(没有前驱),单独处理。
    ///
    /// *坑*:接线的顺序会影响正确性。先把被删节点的 `next` `take()` 出来,
    /// 再接给前驱 —— 反过来会把后半截链表丢掉。
    fn remove(&mut self, h: Self::Handle) -> Option<(OrderId, Qty)> {
        todo!("2b")
    }

    fn reduce_front(&mut self, by: Qty) -> Result<(), ()> {
        todo!("2b")
    }

    /// *提示*:用 `as_ref()` 拿一个不可变游标一路走下去,别用 `take()`
    /// —— 这是只读方法,不许改动链表。
    fn to_vec(&self) -> Vec<(OrderId, Qty)> {
        todo!("2b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 手写 `Drop` 之后,长链表能正常析构。
    #[test]
    fn boxlist_长链表能正常析构() {
        let mut l = BoxListLevel::new();
        for i in 0..2_000u64 {
            l.push_back(i, Qty::new(1));
        }
        assert_eq!(l.len(), 2_000);
        drop(l);
    }

    /// **亲眼看一次递归 `Drop` 爆栈。** 默认不跑,要手动触发:
    ///
    /// ```text
    /// cargo test -p rung-core boxlist_递归drop会爆栈 -- --ignored --nocapture
    /// ```
    ///
    /// 做法:
    /// 1. 先把你写好的 `impl Drop for BoxListLevel` **整个注释掉**
    /// 2. 跑上面那条命令 → 应该看到 `thread ... has overflowed its stack`,
    ///    而且是 **SIGSEGV 直接杀进程**,不是普通的测试失败
    /// 3. 把 `Drop` 恢复,再跑一次 → 通过
    ///
    /// 这个对照实验是子步 2b 最值钱的一次观察。**把两次的输出抄进报告。**
    ///
    /// (设成 `#[ignore]` 是因为它崩起来会连带杀掉整个测试进程 ——
    ///  不该让它干扰日常的 `./scripts/test.sh`。)
    #[test]
    #[ignore = "手动触发:先注释掉 impl Drop,再跑 --ignored 看它爆栈"]
    fn boxlist_递归drop会爆栈() {
        // 这个规模下递归 drop 需要几万层栈帧。
        // push_back 是 O(n),所以构造本身也要跑一会儿 —— 那个等待时间同样是数据点。
        let mut l = BoxListLevel::new();
        for i in 0..30_000u64 {
            l.push_back(i, Qty::new(1));
        }
        assert_eq!(l.len(), 30_000);
        drop(l); // 没有手写 Drop 的话,崩在这一行
    }
}
