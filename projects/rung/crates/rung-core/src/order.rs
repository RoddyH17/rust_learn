//! 订单本体。
//!
//! 这个模块只有一个 struct 和一个构造函数 —— 阶段 1 不需要更多。
//! 状态、有效期、账户归属这些都还没有,它们会在需要的那个阶段才加进来。
//!
//! **为什么不一次设计到位?** 见 `METHOD.md` 方法七:
//! 一个调用者时提取的抽象几乎必然是错的,因为你在猜第二个调用者长什么样。
//! 阶段 5 需要 `owner` 字段做自成交防范时,你会亲手往这个公开 struct 上加一个字段,
//! 并且亲身体会到「这是一次破坏性变更」意味着什么。

use crate::types::{OrderId, Price, Qty, Seq, Side};

/// 一笔挂在簿上的订单。
///
/// ⚠️ **字段全是 `pub`,这是一个还没想清楚的决定。**
///
/// 好处是简单:任何人可以读、可以改。
/// 风险是 `qty` 可以被外面随意改 —— 而阶段 2 的档位会缓存「这一档的总量」,
/// 如果有人绕过档位直接改了某笔订单的 `qty`,那个缓存就静默地错了。
///
/// 现在**不要**去解决它。先记在 stage-01.md 的 reflection 里,
/// 等阶段 2 真的做出那个缓存、阶段 7 的 proptest 真的撞出这个 bug 时再回来。
/// 那时你才有资格判断该怎么改。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    /// 全局唯一。同一个 id 不许在簿上出现两次。
    pub id: OrderId,

    /// 买还是卖。
    pub side: Side,

    /// 限价。这笔单愿意接受的最差价格。
    pub price: Price,

    /// **剩余**数量,不是原始数量。
    ///
    /// 部分成交后这个值会减少。想知道原始数量?现在不支持 ——
    /// 想清楚:谁会需要「原始数量」?如果没人需要,就别存。
    pub qty: Qty,

    /// 到达序号,决定同价格档位内的排队位置。由订单簿分配,不是调用方给的。
    pub seq: Seq,
}

impl Order {
    /// 造一笔订单。
    ///
    /// 注意 `seq` 不在参数里 —— 序号由订单簿在收下这笔单时分配。
    /// 如果让调用方自己填序号,「时间优先」就成了「谁填的数小谁优先」。
    ///
    /// 所以这里先填 0,等阶段 4 订单簿收下它时再改写。
    pub fn new(id: OrderId, side: Side, price: Price, qty: Qty) -> Self {
        todo!("T1.5b:seq 先填 0")
    }

    /// 这笔单还有没有剩余量。
    pub fn is_filled(&self) -> bool {
        todo!("T1.5b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 样例单() -> Order {
        Order::new(1, Side::Buy, Price::from_ticks(1005), Qty::new(100))
    }

    #[test]
    fn 新建订单的序号是零() {
        assert_eq!(样例单().seq, 0, "序号由订单簿分配,不是调用方给的");
    }

    #[test]
    fn 新建订单字段正确() {
        let o = 样例单();
        assert_eq!(o.id, 1);
        assert_eq!(o.side, Side::Buy);
        assert_eq!(o.price, Price::from_ticks(1005));
        assert_eq!(o.qty, Qty::new(100));
    }

    #[test]
    fn 剩余量为零即成交完毕() {
        let mut o = 样例单();
        assert!(!o.is_filled());

        o.qty = Qty::ZERO;
        assert!(o.is_filled());
    }
}
