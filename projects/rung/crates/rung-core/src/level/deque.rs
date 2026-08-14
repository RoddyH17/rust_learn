//! 子步 2a · `DequeLevel` —— 基线实现。
//!
//! 用标准库的 `VecDeque`。这是**基线**,不是答案 ——
//! 后面两份实现要和它对测,拿出加速比。

use super::LevelQueue;
use crate::types::{OrderId, Qty};
use std::collections::VecDeque;

/// 用 `VecDeque` 存的档位队列。
///
/// `VecDeque` 是一个**环形缓冲**:底层一块连续内存,首尾各一个下标。
/// 所以两端进出都是 O(1),而且元素连续排列、缓存友好。
///
/// 但它有一个致命的地方:**中间删除是 O(n)。**
/// 更糟的是,连「找到那笔单在哪」都是 O(n) —— 因为 `VecDeque` 里没有稳定的位置标识,
/// 元素的下标会随着 `pop_front` 整体漂移。
///
/// 这就是为什么它的 `Handle` 只能是 `OrderId`:除了 id 之外没有任何东西
/// 能在下一次操作之后还指得准。
#[derive(Debug, Clone, Default)]
pub struct DequeLevel {
    /// 队头在前,队尾在后。
    items: VecDeque<(OrderId, Qty)>,
    /// 冗余缓存:`items` 中所有 `Qty` 之和。
    total: Qty,
}

impl LevelQueue for DequeLevel {
    /// `VecDeque` 里没有稳定的位置标识,所以只能拿 id 现搜。
    ///
    /// **这个选择直接决定了 `remove` 是 O(n)。**
    type Handle = OrderId;

    fn new() -> Self {
        todo!("2a")
    }

    fn len(&self) -> usize {
        todo!("2a")
    }

    fn is_empty(&self) -> bool {
        todo!("2a")
    }

    fn total_qty(&self) -> Qty {
        todo!("2a")
    }

    /// *提示*:`VecDeque::push_back`。别忘了同步 `total`。
    fn push_back(&mut self, id: OrderId, qty: Qty) -> Self::Handle {
        todo!("2a:返回值就是 id")
    }

    /// *提示*:`VecDeque::front` 返回 `Option<&T>`,而你要返回 `Option<T>`。
    /// `(OrderId, Qty)` 是 `Copy` 的,所以 `.copied()` 就行。
    fn front(&self) -> Option<(OrderId, Qty)> {
        todo!("2a")
    }

    fn pop_front(&mut self) -> Option<(OrderId, Qty)> {
        todo!("2a:别忘了同步 total")
    }

    /// **O(n)** —— 先线性搜索找到下标,再按下标删除。
    ///
    /// *提示*:`iter().position(|(i, _)| *i == h)` 找下标,`remove(idx)` 删除。
    /// `VecDeque::remove` 返回 `Option<T>`。
    ///
    /// 跑基准时特别留意这个方法 —— 它是 `DequeLevel` 输掉的地方。
    fn remove(&mut self, h: Self::Handle) -> Option<(OrderId, Qty)> {
        todo!("2a")
    }

    /// *提示*:`front_mut()` 拿到可变引用。先算出新值再写回,
    /// 不够减的时候一个字节都不许改。
    fn reduce_front(&mut self, by: Qty) -> Result<(), ()> {
        todo!("2a")
    }

    fn to_vec(&self) -> Vec<(OrderId, Qty)> {
        todo!("2a")
    }
}
