//! 子步 2c · `ArenaLevel` —— 索引式侵入双向链表。
//!
//! # 核心思路:用索引代替指针
//!
//! 双向链表的困难在于每个节点被两个地方指着,而 `Box` 只允许一个所有者。
//!
//! 出路是:**让所有节点住在一个 `Vec` 里,链接用下标而不是指针。**
//!
//! ```text
//!   nodes: [ Occ{id:1,prev:None,next:Some(1)} , Occ{id:2,prev:Some(0),next:None} , Free ]
//!            └─ 下标 0 ─────────────────┘        └─ 下标 1 ─────────────┘         └ 2 ┘
//!   head = Some(0)
//!   tail = Some(1)
//!   free = Some(2)
//! ```
//!
//! 这样一来:
//!
//! - **所有权问题消失了** —— `Vec` 是唯一的所有者,下标只是数字,可以随便复制
//! - **`remove` 变成 O(1)** —— 拿到下标,改前驱的 `next` 和后继的 `prev`,收工
//! - **全程安全 Rust** —— 一行 `unsafe` 都不需要
//!
//! 代价是多一次数组寻址,以及要自己管理**空闲槽位**。
//!
//! # 空闲链表(free list)
//!
//! 摘掉一个节点后,它占的槽位不能就这么废掉 —— 否则反复挂撤会让 `Vec` 无限增长。
//! 标准做法是把空槽串成另一条链表:
//!
//! ```text
//! enum Slot {
//!     Occupied(Node),
//!     Free { next_free: Option<u32> },
//! }
//! ```
//!
//! `push_back` 时优先从空闲链表头取一个槽复用,没有了才 `Vec::push` 新的。
//! **这就是一个最小的内存分配器。**
//!
//! # 已知缺陷:handle 会「悬垂」
//!
//! `NodeRef(3)` 指的是「下标 3」。如果下标 3 的节点被摘掉、槽位被复用给了另一笔单,
//! 那个旧 handle 就会指到**新的那笔单**上 —— 而且不会报错。
//!
//! 这叫 **ABA 问题**。本阶段先接受它(测试里不会构造这种情况),
//! 阶段 5 的**世代索引**会修掉:给每个槽加一个版本号,handle 带上版本,
//! 版本对不上就返回 `None`。
//!
//! 这条缺陷要写进 ADR 的「何时推翻」段。

use super::LevelQueue;
use crate::types::{OrderId, Qty};

/// 指向 arena 中某个槽位的 handle。
///
/// 就是一个下标。`Copy` 的,可以随便传。
///
/// ⚠️ 见模块文档:本阶段的 `NodeRef` 没有版本号,槽位复用后旧 handle 会指错。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRef(pub(crate) u32);

/// 链表节点。`prev` / `next` 是**下标**,不是指针。
#[derive(Debug, Clone)]
struct Node {
    id: OrderId,
    qty: Qty,
    prev: Option<u32>,
    next: Option<u32>,
}

/// 一个槽位:要么装着节点,要么是空闲的(并指向下一个空闲槽)。
#[derive(Debug, Clone)]
enum Slot {
    Occupied(Node),
    Free { next_free: Option<u32> },
}

/// 索引式侵入双向链表实现的档位队列。
///
/// 三份实现里唯一 `remove` 是 **O(1)** 的 —— 这正是真实订单簿的做法。
#[derive(Debug, Clone, Default)]
pub struct ArenaLevel {
    /// 所有槽位。**唯一的所有者。**
    nodes: Vec<Slot>,
    /// 队头的下标。
    head: Option<u32>,
    /// 队尾的下标。有了它 `push_back` 才是 O(1)。
    tail: Option<u32>,
    /// 空闲链表的头。
    free: Option<u32>,
    /// 在用的节点数(不含空闲槽)。
    len: usize,
    /// 冗余缓存。
    total: Qty,
}

impl ArenaLevel {
    /// 取一个空槽放进 `node`,返回它的下标。
    ///
    /// 行为规格:
    /// - 空闲链表非空 → 取它的头,把 `free` 指向下一个空闲槽,把槽改成 `Occupied`
    /// - 空闲链表为空 → `nodes.push(Occupied(node))`,下标就是 `len - 1`
    ///
    /// *提示*:这是这个文件里最像「内存分配器」的一段。想清楚两条路径都要
    /// 正确维护 `free`。
    fn alloc(&mut self, node: Node) -> u32 {
        todo!("2c")
    }

    /// 把下标 `i` 的槽位还给空闲链表,返回原来装着的节点。
    ///
    /// 行为规格:
    /// - 槽位是 `Occupied` → 取出节点,槽位改成 `Free { next_free: 原来的 self.free }`,
    ///   `self.free = Some(i)`,返回 `Some(节点)`
    /// - 槽位已经是 `Free` → `None`,什么都不改
    ///
    /// *提示*:`std::mem::replace` 能一步把槽位换掉并拿到旧值。
    fn dealloc(&mut self, i: u32) -> Option<Node> {
        todo!("2c")
    }

    /// 读一个槽位里的节点,空闲槽返回 `None`。
    fn node(&self, i: u32) -> Option<&Node> {
        todo!("2c")
    }

    /// 可变地读一个槽位里的节点。
    fn node_mut(&mut self, i: u32) -> Option<&mut Node> {
        todo!("2c")
    }

    /// 把下标 `i` 的节点从链表里摘出来(只改链接,不动槽位)。
    ///
    /// 行为规格:
    /// - 有前驱 → 前驱的 `next` 指向本节点的 `next`;没有 → `self.head` 指向本节点的 `next`
    /// - 有后继 → 后继的 `prev` 指向本节点的 `prev`;没有 → `self.tail` 指向本节点的 `prev`
    ///
    /// **四种情况都要对**:中间 / 队头 / 队尾 / 唯一一个。
    /// 这四种情况就是双向链表最容易写错的地方,共享测试套件里各有一条。
    fn unlink(&mut self, i: u32) {
        todo!("2c")
    }
}

impl LevelQueue for ArenaLevel {
    /// **能直接指到节点** —— 这就是 `remove` 变成 O(1) 的全部原因。
    type Handle = NodeRef;

    fn new() -> Self {
        todo!("2c")
    }

    fn len(&self) -> usize {
        todo!("2c")
    }

    fn is_empty(&self) -> bool {
        todo!("2c")
    }

    fn total_qty(&self) -> Qty {
        todo!("2c")
    }

    /// **O(1)** —— 有 `tail` 下标,直接接上去。
    ///
    /// 步骤:`alloc` 一个槽 → 把它的 `prev` 设成旧的 `tail` →
    /// 旧 `tail` 的 `next` 指向它 → `self.tail` 更新 → 空队列时 `head` 也要设。
    fn push_back(&mut self, id: OrderId, qty: Qty) -> Self::Handle {
        todo!("2c")
    }

    fn front(&self) -> Option<(OrderId, Qty)> {
        todo!("2c")
    }

    /// *提示*:就是 `remove(NodeRef(self.head?))`。别重复写一遍摘链逻辑。
    fn pop_front(&mut self) -> Option<(OrderId, Qty)> {
        todo!("2c")
    }

    /// **O(1)** —— 这是这份实现存在的理由。
    ///
    /// 步骤:确认槽位是 `Occupied`(否则返回 `None`)→ `unlink` → `dealloc` →
    /// 更新 `len` 和 `total`。
    fn remove(&mut self, h: Self::Handle) -> Option<(OrderId, Qty)> {
        todo!("2c")
    }

    fn reduce_front(&mut self, by: Qty) -> Result<(), ()> {
        todo!("2c")
    }

    /// 从 `head` 沿着 `next` 走一遍。
    ///
    /// *坑*:如果链接维护错了,这里可能**死循环**。
    /// 保险做法是走的步数不超过 `len`。
    fn to_vec(&self) -> Vec<(OrderId, Qty)> {
        todo!("2c")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 专属测试:摘掉的槽位必须被复用,不能让 `Vec` 无限增长。
    ///
    /// 这条测的是空闲链表。没写空闲链表的话,`nodes` 会涨到 10000,
    /// 而正确实现只需要 1 个槽。
    #[test]
    fn arena_槽位会被复用() {
        let mut l = ArenaLevel::new();
        for i in 0..10_000u64 {
            let h = l.push_back(i, Qty::new(1));
            let _ = l.remove(h);
        }
        assert!(l.is_empty());
        assert!(
            l.nodes.len() <= 4,
            "空闲槽位没有被复用:nodes 涨到了 {},正确实现应该只用 1 个槽",
            l.nodes.len()
        );
    }

    /// 专属测试:摘队尾之后 `tail` 要跟着回退,否则下一次 `push_back` 会接错地方。
    #[test]
    fn arena_摘队尾后尾指针正确回退() {
        let mut l = ArenaLevel::new();
        l.push_back(1, Qty::new(10));
        let h2 = l.push_back(2, Qty::new(20));
        let h3 = l.push_back(3, Qty::new(30));

        let _ = l.remove(h3);
        let _ = l.remove(h2);
        l.push_back(4, Qty::new(40));

        assert_eq!(
            l.to_vec(),
            vec![(1, Qty::new(10)), (4, Qty::new(40))],
            "tail 没有正确回退"
        );
    }

    /// 专属测试:链接不能形成环,否则 `to_vec` 会死循环。
    #[test]
    fn arena_链接不成环() {
        let mut l = ArenaLevel::new();
        let mut hs = Vec::new();
        for i in 0..50u64 {
            hs.push(l.push_back(i, Qty::new(1)));
        }
        // 隔一个摘一个 —— 最容易把链接接错的模式
        for (k, h) in hs.iter().enumerate() {
            if k % 2 == 0 {
                let _ = l.remove(*h);
            }
        }
        assert_eq!(l.len(), 25);
        assert_eq!(l.to_vec().len(), 25, "遍历结果和 len 对不上,链接错了");
    }
}
