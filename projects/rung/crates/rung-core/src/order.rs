//! 子步 1b · 订单。
//!
//! 一个只读的值类型:外面能读每个字段,但**改不了**。
//! 只有订单簿(同一个 crate 内)能改 `seq` 和 `qty`。

use crate::types::{OrderId, Price, Qty, Seq, Side};

/// 一笔挂在簿上的订单。
///
/// 字段**全部私有**,对外只给访问器。这不是啰嗦 ——
///
/// 阶段 2 的档位会缓存「这一档的总量」。如果 `qty` 是 `pub`,任何人都能绕过档位
/// 直接改某笔订单的数量,那个缓存就**静默地**错了(见 `METHOD.md` 方法六:
/// 编译期错误 → 运行时 panic → 静默错误,一档比一档糟)。
///
/// 把字段设成私有,再把改动收敛到 `pub(crate)` 的几个方法上,
/// 「缓存和真相一致」这条不变量就只有几个地方需要维护,而不是全世界。
///
/// **这条规则后面九个阶段都成立:优化只许改实现,不许改接口。
/// 为了性能把字段改成 `pub` 是不可接受的 —— 那会破坏封装,也就改变了接口。**
///
/// ```
/// use rung_core::{Order, Price, Qty, Side};
///
/// let o = Order::new(1, Side::Buy, Price::from_ticks(1005), Qty::new(100));
/// assert_eq!(o.id(), 1);
/// assert_eq!(o.price().to_string(), "10.05");
/// assert_eq!(o.seq(), 0);   // 序号由订单簿分配,new 出来是 0
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: OrderId,
    side: Side,
    price: Price,
    qty: Qty,
    seq: Seq,
}

impl Order {
    /// 造一笔订单。`seq` 固定填 `0`。
    ///
    /// 序号由订单簿在收下这笔单时分配 —— 如果让调用方自己填,
    /// 「时间优先」就变成了「谁填的数小谁优先」。
    pub fn new(id: OrderId, side: Side, price: Price, qty: Qty) -> Self {
        todo!("1b:seq 填 0")
    }

    pub fn id(&self) -> OrderId {
        todo!("1b")
    }

    pub fn side(&self) -> Side {
        todo!("1b")
    }

    pub fn price(&self) -> Price {
        todo!("1b")
    }

    /// **剩余**数量,不是原始数量。部分成交后会减少。
    ///
    /// 不存原始数量:现在没有任何调用方需要它。等真的有人需要了再加。
    pub fn qty(&self) -> Qty {
        todo!("1b")
    }

    pub fn seq(&self) -> Seq {
        todo!("1b")
    }

    /// 行为规格:`qty` 为零 → `true`。
    pub fn is_filled(&self) -> bool {
        todo!("1b")
    }

    // -- 以下是 pub(crate):只有订单簿能调,crate 外面看不见 --

    /// 订单簿收下这笔单时,给它盖上序号。
    pub(crate) fn assign_seq(&mut self, seq: Seq) {
        todo!("1b")
    }

    /// 成交掉 `by` 这么多。
    ///
    /// 行为规格:
    /// - 够减 → 扣掉,返回 `Ok(())`
    /// - 不够减 → **不改动任何东西**,返回 `Err(())`
    ///
    /// 「不够减」在撮合里是逻辑错误,不是正常情况 —— 所以要返回错误而不是饱和到 0。
    ///
    /// *提示*:先算出新值再赋值,不要先赋值再检查。
    pub(crate) fn fill(&mut self, by: Qty) -> Result<(), ()> {
        todo!("1b")
    }
}

// ===========================================================================
// 定向测试 · 子步 1b
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn 样例单() -> Order {
        Order::new(1, Side::Buy, Price::from_ticks(1005), Qty::new(100))
    }

    #[test]
    fn order_访问器读得出构造时的值() {
        let o = 样例单();
        assert_eq!(o.id(), 1);
        assert_eq!(o.side(), Side::Buy);
        assert_eq!(o.price(), Price::from_ticks(1005));
        assert_eq!(o.qty(), Qty::new(100));
    }

    #[test]
    fn order_新建时序号为零() {
        assert_eq!(样例单().seq(), 0, "序号由订单簿分配,不是调用方给的");
    }

    #[test]
    fn order_盖序号() {
        let mut o = 样例单();
        o.assign_seq(42);
        assert_eq!(o.seq(), 42);
    }

    #[test]
    fn order_剩余量为零即成交完毕() {
        let mut o = 样例单();
        assert!(!o.is_filled());

        assert_eq!(o.fill(Qty::new(100)), Ok(()));
        assert!(o.is_filled());
    }

    #[test]
    fn order_部分成交() {
        let mut o = 样例单();
        assert_eq!(o.fill(Qty::new(30)), Ok(()));
        assert_eq!(o.qty(), Qty::new(70));
        assert!(!o.is_filled());
    }

    #[test]
    fn order_成交量超过剩余量时不改动任何东西() {
        let mut o = 样例单();
        assert_eq!(o.fill(Qty::new(101)), Err(()));
        assert_eq!(o.qty(), Qty::new(100), "失败的 fill 不许留下痕迹");
    }

    #[test]
    fn order_可以比较相等() {
        assert_eq!(样例单(), 样例单());

        let mut other = 样例单();
        other.assign_seq(1);
        assert_ne!(样例单(), other, "序号不同就是不同的单");
    }
}
