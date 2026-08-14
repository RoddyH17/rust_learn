//! 子步 1d–1e · `NaiveBook` —— 黄金参考模型。
//!
//! # 这个东西存在的理由
//!
//! 它是一个**故意写得最笨**的订单簿:所有挂单堆在一个 `Vec` 里,每次要什么就全表扫一遍。
//! 没有索引、没有分档、没有任何优化。
//!
//! 它慢得离谱。这没关系 —— 它的职责不是快,是**显然正确**。
//!
//! 后面每一个阶段做出来的快版本,都要拿同一串随机订单流喂给它和 `NaiveBook`,
//! 断言两者**逐笔成交完全一致**。这叫**黄金参考模型**(golden reference model):
//!
//! > 拿一个你信得过、但不许用在正式实现里的东西,来给随机测试对答案。
//!
//! 不变量断言只能抓出「买一 < 卖一被破坏」这类**结构性**错误。
//! 参考模型能抓出「成交价算错了 1 个 tick」「排队顺序错了一位」这类
//! **语义性**错误 —— 而那才是撮合引擎真正会出的 bug。
//!
//! # 它同时也是基线
//!
//! 阶段 1 结束时你会用 `./scripts/eval.sh` 测出它的吞吐。那个数字是分母:
//! 后面每个阶段的报告都要回答「比 NaiveBook 快了几倍,且结果一致」。
//!
//! # 允许用什么
//!
//! **随便用。** 这个文件里可以用 `sort_by`、`clone`、`retain`、任何最直白的写法。
//! 慢不是问题,写错才是。追求简单到一眼能看出它是对的。

use crate::order::Order;
use crate::trade::{BookError, Trade};
use crate::types::{OrderId, Price, Qty, Seq, Side};

/// 最笨的订单簿:一个 `Vec`,全表扫描。
///
/// ```
/// use rung_core::{NaiveBook, Order, Price, Qty, Side};
///
/// let mut book = NaiveBook::new();
/// book.insert(Order::new(1, Side::Buy,  Price::from_ticks(1000), Qty::new(5))).unwrap();
/// book.insert(Order::new(2, Side::Sell, Price::from_ticks(1010), Qty::new(5))).unwrap();
///
/// assert_eq!(book.best_bid(), Some(Price::from_ticks(1000)));
/// assert_eq!(book.best_ask(), Some(Price::from_ticks(1010)));
/// assert_eq!(book.spread(), Some(10));
/// ```
#[derive(Debug, Clone, Default)]
pub struct NaiveBook {
    /// 全部挂单,**无序**。顺序不代表任何东西 —— 优先级每次现算。
    orders: Vec<Order>,
    /// 下一个要分配的序号。从 1 开始,0 表示「还没被簿子收下」。
    next_seq: Seq,
}

impl NaiveBook {
    /// 空簿子。`next_seq` 从 **1** 开始 —— 0 保留给「尚未分配」。
    pub fn new() -> Self {
        todo!("1d")
    }

    /// 簿上还有几笔单。
    pub fn len(&self) -> usize {
        todo!("1d")
    }

    /// 行为规格:`len() == 0` → `true`。
    pub fn is_empty(&self) -> bool {
        todo!("1d")
    }

    /// 这个 id 在不在簿上。
    pub fn contains(&self, id: OrderId) -> bool {
        todo!("1d")
    }

    // -----------------------------------------------------------------
    // 挂单与撤单
    // -----------------------------------------------------------------

    /// **无条件**挂单,不撮合。
    ///
    /// 行为规格:
    /// - 数量为零 → `Err(BookError::ZeroQty)`,簿子不变
    /// - id 已存在 → `Err(BookError::DuplicateId(id))`,簿子不变
    /// - 否则 → 分配当前 `next_seq` 给这笔单,存进簿子,`next_seq += 1`,
    ///   返回 `Ok(分配到的序号)`
    ///
    /// ⚠️ **重要**:因为不撮合,用 `insert` 可以造出一个「买一 ≥ 卖一」的**交叉簿**。
    /// 这不是 bug,是刻意的 —— 测试需要能直接摆出任意簿面状态。
    /// 「买一 < 卖一」这条不变量只在**只用 `submit`** 的前提下成立。
    ///
    /// *提示*:两个校验的顺序会影响「零量且重复 id」时报哪个错。
    /// 先查零量 —— 测试是按这个顺序写的。
    pub fn insert(&mut self, order: Order) -> Result<Seq, BookError> {
        todo!("1d")
    }

    /// 撤单。
    ///
    /// 行为规格:
    /// - 找得到 → 从簿上移除并返回 `Some(那笔单)`(带着它当时的剩余量和序号)
    /// - 找不到 → `None`,簿子不变
    ///
    /// 返回订单本体而不是 `bool`:调用方可能要拿它改价重挂。
    ///
    /// *提示*:`Vec` 有 `iter().position(|o| ...)` 找下标,`remove(i)` 按下标删除并返回元素。
    pub fn cancel(&mut self, id: OrderId) -> Option<Order> {
        todo!("1d")
    }

    // -----------------------------------------------------------------
    // 查询
    // -----------------------------------------------------------------

    /// 某一侧的最优价。
    ///
    /// 行为规格:
    /// - 这一侧一笔单都没有 → `None`
    /// - `Side::Buy` → 所有买单里的**最高**价
    /// - `Side::Sell` → 所有卖单里的**最低**价
    ///
    /// *提示*:`iter().filter(...)` 挑出这一侧,再 `map(|o| o.price())`,
    /// 然后按 `higher_is_better()` 决定用 `max()` 还是 `min()`。
    pub fn best(&self, side: Side) -> Option<Price> {
        todo!("1d")
    }

    /// `best(Side::Buy)` 的简写。
    pub fn best_bid(&self) -> Option<Price> {
        todo!("1d")
    }

    /// `best(Side::Sell)` 的简写。
    pub fn best_ask(&self) -> Option<Price> {
        todo!("1d")
    }

    /// 买卖价差,以 tick 计:`best_ask - best_bid`。
    ///
    /// 行为规格:
    /// - 任意一侧为空 → `None`
    /// - 否则 → `Some(ask.diff_ticks(bid))`
    ///
    /// ⚠️ 结果**可能是负数** —— 如果簿子被 `insert` 摆成了交叉状态。见 `insert` 的说明。
    pub fn spread(&self) -> Option<i64> {
        todo!("1d")
    }

    /// 某一侧的挂单总量。
    ///
    /// 行为规格:这一侧为空 → `Qty::ZERO`。溢出时 panic 是可以接受的
    /// (总量溢出 `u64` 意味着输入本身就荒谬)。
    pub fn total_qty(&self, side: Side) -> Qty {
        todo!("1d")
    }

    /// 某一侧的深度快照:按优先级从优到劣排列的 `(价格, 该档位总量)`。
    ///
    /// 行为规格:
    /// - 同价格的多笔单要**合并成一档**
    /// - 买盘按价格**从高到低**,卖盘按价格**从低到高**
    /// - 这一侧为空 → 空 `Vec`
    ///
    /// 例:买盘有 (10.00, 5)、(10.20, 3)、(10.20, 1),则
    /// `depth(Buy)` = `[(10.20, 4), (10.00, 5)]`
    ///
    /// **这个方法是后面所有阶段对答案的接口。** 快版本的 `depth` 必须和它逐项相同。
    ///
    /// *提示*:最直白的做法 —— 收集这一侧所有单,按价格排序,然后线性扫描合并相邻同价的。
    /// 不要为了优雅去用 `HashMap`:那样顺序就不确定了,而这里**顺序是语义的一部分**。
    pub fn depth(&self, side: Side) -> Vec<(Price, Qty)> {
        todo!("1e")
    }

    // -----------------------------------------------------------------
    // 撮合
    // -----------------------------------------------------------------

    /// 提交一笔订单,先撮合,吃不完的挂回簿上(GTC 语义)。
    ///
    /// # 完整行为规格
    ///
    /// **0. 前置校验**(和 `insert` 一致)
    ///    - 数量为零 → `Err(BookError::ZeroQty)`
    ///    - id 已在簿上 → `Err(BookError::DuplicateId(id))`
    ///
    /// **1. 找对手方**:`taker.side().opposite()` 那一侧的全部挂单。
    ///
    /// **2. 筛出能成交的**(价格穿越,crossing):
    ///    - taker 是 `Buy` → maker 的价格 **≤** taker 的价格
    ///    - taker 是 `Sell` → maker 的价格 **≥** taker 的价格
    ///
    /// **3. 排优先级**(价格优先,时间优先):
    ///    - taker 是 `Buy` → 对手是卖盘,按价格**升序**(便宜的先被吃)
    ///    - taker 是 `Sell` → 对手是买盘,按价格**降序**(贵的先被吃)
    ///    - 同价 → 按 `seq` **升序**(先挂的先成交)
    ///
    /// **4. 依次吃**,对每一个 maker:
    ///    - 成交量 = `min(taker 剩余, maker 剩余)`
    ///    - 成交价 = **maker 的价格**(不是 taker 的)
    ///    - 生成一笔 `Trade`,seq 取当前 `next_seq` 并自增
    ///    - maker 扣量;扣到零就从簿上移除
    ///    - taker 扣量;扣到零就停止
    ///
    /// **5. 剩余处理**:全部对手吃完后 taker 还有剩余
    ///    → 分配 `next_seq` 给它,挂回簿上。
    ///
    /// # 保证的不变量
    ///
    /// 只用 `submit` 操作的簿子,任何时刻都满足 `best_bid < best_ask`
    /// (两侧都非空时)。因为但凡相交就会立刻成交掉。
    ///
    /// # 提示
    ///
    /// *提示一*:最笨的写法是每次都 `clone` 一份对手方的 `(下标, 价格, seq)` 三元组、
    /// 排好序,然后按下标去改原 `Vec`。**这样很慢,但这里就是要慢。**
    ///
    /// *提示二*:边遍历边从 `Vec` 里删元素会让下标失效。
    /// 一个干净的做法:先把要成交的记下来,循环结束后再用 `retain` 一次性清掉空单。
    ///
    /// *提示三*:`Trade` 的 `seq` 和订单的 `seq` 共用同一个计数器 ——
    /// 它是一条全局事件序列,这样阶段 9 的重放才有唯一顺序。
    ///
    /// # 坑
    ///
    /// - taker 的数量在成交过程中会变,别拿它的**初始**数量去算后面几笔
    /// - maker 扣到零必须移除,否则 `best()` 会指向一个空单
    /// - 对手方一笔都没有 / 一笔都不 crossing → 返回空 `Vec`,taker 原样挂上
    pub fn submit(&mut self, taker: Order) -> Result<Vec<Trade>, BookError> {
        todo!("1e")
    }
}

// ===========================================================================
// 定向测试 · 子步 1d(挂单、撤单、查询)
// ===========================================================================

#[cfg(test)]
mod tests_1d {
    use super::*;

    fn p(t: i64) -> Price {
        Price::from_ticks(t)
    }
    fn q(n: u64) -> Qty {
        Qty::new(n)
    }
    fn buy(id: OrderId, price: i64, qty: u64) -> Order {
        Order::new(id, Side::Buy, p(price), q(qty))
    }
    fn sell(id: OrderId, price: i64, qty: u64) -> Order {
        Order::new(id, Side::Sell, p(price), q(qty))
    }

    // ---- 空簿子 ----

    #[test]
    fn 空簿子的各项查询() {
        let b = NaiveBook::new();
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
        assert_eq!(b.best_bid(), None);
        assert_eq!(b.best_ask(), None);
        assert_eq!(b.spread(), None);
        assert_eq!(b.total_qty(Side::Buy), Qty::ZERO);
        assert!(!b.contains(1));
    }

    // ---- insert ----

    #[test]
    fn 挂单后序号从1开始递增() {
        let mut b = NaiveBook::new();
        assert_eq!(b.insert(buy(1, 1000, 5)), Ok(1));
        assert_eq!(b.insert(buy(2, 1001, 5)), Ok(2));
        assert_eq!(b.insert(sell(3, 1010, 5)), Ok(3));
        assert_eq!(b.len(), 3);
    }

    #[test]
    fn 挂零量单被拒且簿子不变() {
        let mut b = NaiveBook::new();
        assert_eq!(b.insert(buy(1, 1000, 0)), Err(BookError::ZeroQty));
        assert!(b.is_empty());
    }

    #[test]
    fn 挂重复id被拒且簿子不变() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        assert_eq!(b.insert(buy(1, 1002, 9)), Err(BookError::DuplicateId(1)));
        assert_eq!(b.len(), 1);
        assert_eq!(b.best_bid(), Some(p(1000)), "被拒的单不许留下痕迹");
    }

    #[test]
    fn 零量且重复id时先报零量() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        assert_eq!(b.insert(buy(1, 1000, 0)), Err(BookError::ZeroQty));
    }

    // ---- 最优价 ----

    #[test]
    fn 最优买价是最高价() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        b.insert(buy(2, 1020, 3)).unwrap();
        b.insert(buy(3, 990, 7)).unwrap();
        assert_eq!(b.best_bid(), Some(p(1020)));
    }

    #[test]
    fn 最优卖价是最低价() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1050, 5)).unwrap();
        b.insert(sell(2, 1040, 3)).unwrap();
        b.insert(sell(3, 1060, 7)).unwrap();
        assert_eq!(b.best_ask(), Some(p(1040)));
    }

    #[test]
    fn 只有一侧时价差为空() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        assert_eq!(b.best_bid(), Some(p(1000)));
        assert_eq!(b.best_ask(), None);
        assert_eq!(b.spread(), None);
    }

    #[test]
    fn 价差以tick计() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1020, 5)).unwrap();
        b.insert(sell(2, 1040, 5)).unwrap();
        assert_eq!(b.spread(), Some(20));
    }

    #[test]
    fn insert可以摆出交叉簿且价差为负() {
        // insert 不撮合,所以能造出这种状态。这是刻意允许的 —— 测试要能直接摆簿面。
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1050, 5)).unwrap();
        b.insert(sell(2, 1040, 5)).unwrap();
        assert_eq!(b.spread(), Some(-10), "insert 不撮合,簿子可以交叉");
    }

    // ---- 总量 ----

    #[test]
    fn 总量按侧统计() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        b.insert(buy(2, 1020, 3)).unwrap();
        b.insert(sell(3, 1050, 7)).unwrap();
        assert_eq!(b.total_qty(Side::Buy), q(8));
        assert_eq!(b.total_qty(Side::Sell), q(7));
    }

    // ---- cancel ----

    #[test]
    fn 撤单返回订单本体() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        b.insert(buy(2, 1020, 3)).unwrap();

        let c = b.cancel(2).expect("应该撤得掉");
        assert_eq!(c.id(), 2);
        assert_eq!(c.price(), p(1020));
        assert_eq!(c.qty(), q(3));
        assert_eq!(c.seq(), 2, "撤下来的单带着它当时的序号");

        assert_eq!(b.len(), 1);
        assert_eq!(b.best_bid(), Some(p(1000)), "最优价要跟着变");
    }

    #[test]
    fn 撤一个不存在的单返回空而不是崩溃() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        assert_eq!(b.cancel(99), None);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn 撤完之后id可以重新使用() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        b.cancel(1).unwrap();
        assert!(!b.contains(1));
        assert!(b.insert(buy(1, 1020, 3)).is_ok(), "撤掉之后 id 就空出来了");
    }
}

// ===========================================================================
// 定向测试 · 子步 1e(深度快照、撮合)
// ===========================================================================

#[cfg(test)]
mod tests_1e {
    use super::*;

    fn p(t: i64) -> Price {
        Price::from_ticks(t)
    }
    fn q(n: u64) -> Qty {
        Qty::new(n)
    }
    fn buy(id: OrderId, price: i64, qty: u64) -> Order {
        Order::new(id, Side::Buy, p(price), q(qty))
    }
    fn sell(id: OrderId, price: i64, qty: u64) -> Order {
        Order::new(id, Side::Sell, p(price), q(qty))
    }

    // ---- depth ----

    #[test]
    fn 深度快照空簿子给空表() {
        let b = NaiveBook::new();
        assert_eq!(b.depth(Side::Buy), vec![]);
        assert_eq!(b.depth(Side::Sell), vec![]);
    }

    #[test]
    fn 深度快照合并同价档位() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        b.insert(buy(2, 1020, 3)).unwrap();
        b.insert(buy(3, 1020, 1)).unwrap();
        assert_eq!(b.depth(Side::Buy), vec![(p(1020), q(4)), (p(1000), q(5))]);
    }

    #[test]
    fn 深度快照买盘价高在前卖盘价低在前() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        b.insert(buy(2, 1020, 3)).unwrap();
        b.insert(sell(3, 1050, 2)).unwrap();
        b.insert(sell(4, 1040, 6)).unwrap();

        assert_eq!(b.depth(Side::Buy), vec![(p(1020), q(3)), (p(1000), q(5))]);
        assert_eq!(b.depth(Side::Sell), vec![(p(1040), q(6)), (p(1050), q(2))]);
    }

    // ---- 撮合:不成交的情况 ----

    #[test]
    fn 空簿子上提交直接挂上() {
        let mut b = NaiveBook::new();
        let t = b.submit(buy(1, 1000, 5)).unwrap();
        assert!(t.is_empty());
        assert_eq!(b.best_bid(), Some(p(1000)));
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn 价格不交叉时不成交原样挂上() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1050, 5)).unwrap();

        let t = b.submit(buy(2, 1000, 3)).unwrap();
        assert!(t.is_empty(), "买 10.00 吃不动卖 10.50");
        assert_eq!(b.best_bid(), Some(p(1000)));
        assert_eq!(b.best_ask(), Some(p(1050)));
        assert_eq!(b.len(), 2);
    }

    #[test]
    fn 同侧不会互相成交() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        let t = b.submit(buy(2, 1000, 5)).unwrap();
        assert!(t.is_empty(), "买单不会和买单成交");
        assert_eq!(b.len(), 2);
    }

    // ---- 撮合:成交价按 maker ----

    #[test]
    fn 成交价按maker的价格() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1040, 5)).unwrap();

        // taker 愿意出到 10.50,但 maker 只要 10.40 —— 按 10.40 成交。
        let t = b.submit(buy(2, 1050, 5)).unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].price, p(1040), "成交价是 maker 的,不是 taker 的");
        assert_eq!(t[0].taker_id, 2);
        assert_eq!(t[0].maker_id, 1);
        assert_eq!(t[0].qty, q(5));
        assert!(b.is_empty(), "双方都吃完了");
    }

    // ---- 撮合:三种数量关系 ----

    #[test]
    fn taker小于maker时maker留下剩余() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1040, 10)).unwrap();

        let t = b.submit(buy(2, 1040, 4)).unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].qty, q(4));
        assert_eq!(b.len(), 1, "maker 还在");
        assert_eq!(b.total_qty(Side::Sell), q(6));
        assert_eq!(b.best_bid(), None, "taker 吃完了,没有剩余挂回");
    }

    #[test]
    fn taker等于maker时两边都清空() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1040, 5)).unwrap();

        let t = b.submit(buy(2, 1040, 5)).unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].qty, q(5));
        assert!(b.is_empty());
    }

    #[test]
    fn taker大于maker时剩余挂回() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1040, 3)).unwrap();

        let t = b.submit(buy(2, 1040, 8)).unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].qty, q(3));
        assert_eq!(b.best_ask(), None, "maker 被吃光");
        assert_eq!(b.best_bid(), Some(p(1040)), "taker 剩的 5 手挂回买盘");
        assert_eq!(b.total_qty(Side::Buy), q(5));
    }

    // ---- 撮合:优先级 ----

    #[test]
    fn 价格优先_先吃便宜的卖单() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1050, 5)).unwrap();
        b.insert(sell(2, 1040, 5)).unwrap();

        let t = b.submit(buy(3, 1050, 5)).unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].maker_id, 2, "10.40 比 10.50 优先被吃");
        assert_eq!(t[0].price, p(1040));
    }

    #[test]
    fn 时间优先_同价先挂的先成交() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1040, 3)).unwrap(); // seq 1
        b.insert(sell(2, 1040, 3)).unwrap(); // seq 2

        let t = b.submit(buy(3, 1040, 3)).unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].maker_id, 1, "同价,先挂的先成交");
    }

    #[test]
    fn 跨档位成交按价格从优到劣() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1050, 3)).unwrap();
        b.insert(sell(2, 1040, 2)).unwrap();

        // 买 10.50 x 5:先吃 10.40 的 2 手,再吃 10.50 的 3 手
        let t = b.submit(buy(3, 1050, 5)).unwrap();
        assert_eq!(t.len(), 2, "应该跨两个档位");

        assert_eq!(t[0].maker_id, 2);
        assert_eq!(t[0].price, p(1040));
        assert_eq!(t[0].qty, q(2));

        assert_eq!(t[1].maker_id, 1);
        assert_eq!(t[1].price, p(1050));
        assert_eq!(t[1].qty, q(3));

        assert!(b.is_empty());
    }

    #[test]
    fn 卖单taker吃买盘时价高的先被吃() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 3)).unwrap();
        b.insert(buy(2, 1020, 3)).unwrap();

        let t = b.submit(sell(3, 1000, 3)).unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].maker_id, 2, "买盘里出价高的先被吃");
        assert_eq!(t[0].price, p(1020));
    }

    // ---- 撮合:不变量 ----

    #[test]
    fn 只用submit的簿子买一永远小于卖一() {
        let mut b = NaiveBook::new();
        b.submit(buy(1, 1000, 5)).unwrap();
        b.submit(buy(2, 1020, 5)).unwrap();
        b.submit(sell(3, 1010, 3)).unwrap(); // 会吃掉 10.20 的一部分
        b.submit(sell(4, 1030, 5)).unwrap();

        if let (Some(bid), Some(ask)) = (b.best_bid(), b.best_ask()) {
            assert!(bid < ask, "买一 {bid} 必须小于卖一 {ask}");
        }
    }

    #[test]
    fn 成交量守恒() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1040, 3)).unwrap();
        b.insert(sell(2, 1050, 4)).unwrap();

        let t = b.submit(buy(3, 1050, 10)).unwrap();
        let 成交总量: u64 = t.iter().map(|x| x.qty.get()).sum();

        assert_eq!(成交总量, 7, "卖盘原有 7 手,全被吃掉");
        assert_eq!(b.total_qty(Side::Buy), q(3), "taker 10 手吃掉 7 手,剩 3 手");
        assert_eq!(b.total_qty(Side::Sell), Qty::ZERO);
    }

    #[test]
    fn 成交序号严格递增() {
        let mut b = NaiveBook::new();
        b.insert(sell(1, 1040, 2)).unwrap();
        b.insert(sell(2, 1050, 2)).unwrap();

        let t = b.submit(buy(3, 1050, 4)).unwrap();
        assert_eq!(t.len(), 2);
        assert!(t[0].seq < t[1].seq, "成交序号必须严格递增");
    }

    // ---- 撮合:前置校验 ----

    #[test]
    fn 提交零量单被拒() {
        let mut b = NaiveBook::new();
        assert_eq!(b.submit(buy(1, 1000, 0)), Err(BookError::ZeroQty));
        assert!(b.is_empty());
    }

    #[test]
    fn 提交重复id被拒() {
        let mut b = NaiveBook::new();
        b.insert(buy(1, 1000, 5)).unwrap();
        assert_eq!(b.submit(buy(1, 1020, 3)), Err(BookError::DuplicateId(1)));
        assert_eq!(b.len(), 1);
    }

    // ---- 撮合:确定性 ----

    #[test]
    fn 同一串操作跑两次结果完全相同() {
        // 这是阶段 9 重放能成立的前提,现在就锁住。
        fn 跑一遍() -> (Vec<Trade>, Vec<(Price, Qty)>) {
            let mut b = NaiveBook::new();
            b.insert(sell(1, 1050, 3)).unwrap();
            b.insert(sell(2, 1040, 2)).unwrap();
            b.insert(buy(3, 1000, 4)).unwrap();
            let t = b.submit(buy(4, 1050, 6)).unwrap();
            (t, b.depth(Side::Buy))
        }
        assert_eq!(跑一遍(), 跑一遍());
    }
}
