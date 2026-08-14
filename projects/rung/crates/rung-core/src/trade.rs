//! 子步 1c · 成交与错误类型。
//!
//! 这两个是**值类型**,不是规则 —— 所以它们住在 core,不住在 rung-match。
//! 判断依据:`Trade` 描述「发生了什么」,撮合规则决定「什么时候发生」。
//! 前者是数据,后者是逻辑。

use crate::types::{OrderId, Price, Qty, Seq};
use core::fmt;

/// 一笔成交。
///
/// 成交价**永远是 maker 的价格**,不是 taker 的。理由见 `PRIMER.md` §3:
/// maker 先把报价挂出来做了承诺,taker 是接受这个承诺的人。
/// 如果按 taker 报的价成交,taker 报个离谱的价就能把 maker 的单吃走 —— 没人敢挂单了。
///
/// ```
/// use rung_core::{Trade, Price, Qty};
///
/// let t = Trade { taker_id: 10, maker_id: 4, price: Price::from_ticks(1040), qty: Qty::new(2), seq: 7 };
/// assert_eq!(t.to_string(), "TRADE #7 taker=10 maker=4 @10.40 x2");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trade {
    /// 主动来吃单的那一方。
    pub taker_id: OrderId,
    /// 被吃掉的挂单方。
    pub maker_id: OrderId,
    /// 成交价 = maker 的挂单价。
    pub price: Price,
    /// 这一笔成交了多少。
    pub qty: Qty,
    /// 这笔成交的序号,用于确定性排序。
    pub seq: Seq,
}

impl fmt::Display for Trade {
    /// 行为规格,格式固定:
    ///
    /// ```text
    /// TRADE #7 taker=10 maker=4 @10.40 x2
    /// ```
    ///
    /// 各字段之间一个空格,价格前面一个 `@`,数量前面一个 `x`。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("1c")
    }
}

/// 订单簿拒绝一笔操作的原因。
///
/// 用枚举而不是 `String`:调用方需要能 `match` 出「是哪一种失败」,
/// 才写得出「只处理重复 id、别的原样往上抛」这种逻辑。字符串没法被穷尽匹配。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookError {
    /// 这个 id 已经在簿上了。
    DuplicateId(OrderId),
    /// 数量为零的订单没有意义。
    ZeroQty,
}

impl fmt::Display for BookError {
    /// 行为规格:
    /// - `DuplicateId(7)` → `"order id 7 already in book"`
    /// - `ZeroQty` → `"order quantity must be non-zero"`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("1c")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trade_display() {
        let t = Trade {
            taker_id: 10,
            maker_id: 4,
            price: Price::from_ticks(1040),
            qty: Qty::new(2),
            seq: 7,
        };
        assert_eq!(t.to_string(), "TRADE #7 taker=10 maker=4 @10.40 x2");
    }

    #[test]
    fn book_error_display() {
        assert_eq!(
            BookError::DuplicateId(7).to_string(),
            "order id 7 already in book"
        );
        assert_eq!(
            BookError::ZeroQty.to_string(),
            "order quantity must be non-zero"
        );
    }

    #[test]
    fn book_error_可以被穷尽匹配() {
        // 这条测试在证明 enum 比 String 强在哪:调用方能精确认出失败种类。
        let e = BookError::DuplicateId(3);
        let 描述 = match e {
            BookError::DuplicateId(id) => format!("重复:{id}"),
            BookError::ZeroQty => "零量".to_string(),
        };
        assert_eq!(描述, "重复:3");
    }
}
