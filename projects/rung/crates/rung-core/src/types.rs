//! 子步 1a · 基础类型:价格、数量、买卖方向、标识符。
//!
//! 这个模块没有逻辑,只有「概念 → 类型」的映射。每个函数的行为都在文档注释里写死了,
//! 包括边界情况。**不需要你猜任何东西** —— 需要你做的是把它实现出来,并且实现对。

use core::fmt;

// ---------------------------------------------------------------------------
// 标识符
// ---------------------------------------------------------------------------

/// 订单的唯一标识,由调用方指定。
///
/// 用类型别名而不是 newtype:`OrderId` 和 `u64` 在编译器眼里是同一个类型。
/// 这是一个**有意识的取舍** —— newtype 能防止和 `Seq` 混用,代价是每处都要 `.0`。
/// 阶段 8 我们会用实测数据回头看这个取舍值不值。
pub type OrderId = u64;

/// 到达序号:单调递增,实现「时间优先」。
///
/// 不用系统时钟的两个理由(见 `PRIMER.md` §6):
/// 1. 两台机器的时钟不可能完全同步,同一微秒内也可能来多笔订单
/// 2. 用时钟的话,同一批订单跑两次结果会不同 —— 回测就不可信了
///
/// 序号由**订单簿**分配,不是调用方给的。调用方给的会被覆盖。
pub type Seq = u64;

// ---------------------------------------------------------------------------
// 价格
// ---------------------------------------------------------------------------

/// 一个价格单位里有多少个 tick。100 = 两位小数,tick size 为 0.01。
///
/// ⚠️ 这是编译期常量,意味着本库假设所有品种的 tick size 相同。
/// 真实交易所不是这样(低价股 0.001、指数期货 0.25、加密货币随价格区间变)。
/// 这个简化的代价与何时该推翻,写进 ADR-001。
pub const TICKS_PER_UNIT: i64 = 100;

/// 价格,以 tick 为单位的整数。
///
/// 内部存「多少个最小单位」,不是「多少元」:`Price` 里的 `1005` 表示 10.05。
///
/// 字段私有,crate 外只能走 [`Price::from_ticks`]。
/// 这样「1005 是 tick 不是元」这条约定由编译器守着,不靠人记。
///
/// ```
/// use rung_core::Price;
///
/// let p = Price::from_ticks(1005);
/// assert_eq!(p.to_string(), "10.05");
/// assert_eq!(p.ticks(), 1005);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Price(i64);

impl Price {
    /// 从 tick 数构造。crate 外部唯一的构造入口。
    ///
    /// 不做任何校验 —— 负数、极大值都接受。校验是订单簿的事,不是类型的事。
    pub const fn from_ticks(ticks: i64) -> Self {
        Price(ticks)
    }

    /// 取出内部的 tick 数。
    pub const fn ticks(self) -> i64 {
        self.0
    }

    /// 从「元」构造,**四舍五入**到最近的 tick。
    ///
    /// 行为规格:
    /// - `10.05` → `Price(1005)`
    /// - `10.054` → `Price(1005)`(向下舍)
    /// - `10.056` → `Price(1006)`(向上舍)
    /// - `0.0` → `Price(0)`
    /// - 负数照常处理:`-0.5` → `Price(-50)`
    ///
    /// ⚠️ **坑**:`10.05` 在 f64 里实际是 `10.049999999999998934...`。
    /// 直接写 `(units * 100.0) as i64` 会截断成 `1004`,而不是 `1005`。
    /// 必须先 `.round()` 再转换。
    ///
    /// 这是整个库里唯一碰 `f64` 的地方,只用于人机边界(读配置、解析输入)。
    /// 撮合路径上一个浮点数都不许出现。
    pub fn from_units(units: f64) -> Self {
        todo!("1a")
    }

    /// 转回「元」。只用于展示,不用于计算。
    ///
    /// 行为规格:`Price(1005)` → `10.05`。
    pub fn to_units(self) -> f64 {
        todo!("1a")
    }

    /// 两个价格差多少个 tick,即 `self - other`。
    ///
    /// 行为规格:
    /// - `Price(1050).diff_ticks(Price(1040))` → `10`
    /// - `Price(1040).diff_ticks(Price(1050))` → `-10`
    ///
    /// 返回 `i64` 而不是 `Price`:价差和价格是两个概念,价差不该能当 `BTreeMap` 的 key。
    /// **这就是内部用 `i64` 而不是 `u64` 的理由** —— 差值可以是负的。
    pub fn diff_ticks(self, other: Price) -> i64 {
        todo!("1a")
    }
}

impl fmt::Display for Price {
    /// 行为规格(注意补零):
    ///
    /// | 内部值 | 显示 |
    /// |---|---|
    /// | `1005` | `"10.05"` |
    /// | `310` | `"3.10"` ← 不是 `"3.1"` |
    /// | `5` | `"0.05"` |
    /// | `0` | `"0.00"` |
    /// | `-50` | `"-0.50"` |
    /// | `-1005` | `"-10.05"` |
    ///
    /// *提示一*:`{:02}` 是「至少两位,不够补零」。
    ///
    /// *提示二*:负数要单独处理符号。`-50 / 100` 是 `0`,`-50 % 100` 是 `-50`,
    /// 直接套公式会打出 `"0.-50"`。先取绝对值算整数和小数部分,符号单独写。
    ///
    /// *提示三*:**不要写 `write!(f, "{}", format!(...))`。**
    /// `format!` 会在堆上分配一个新 `String`,拷进 `f`,再丢掉。`write!` 直接往
    /// `f` 里写,零分配。详见 stage-01.md 的「String 补课」。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("1a:Debug 能 derive,Display 不行 —— 「给人看的样子」编译器猜不出来")
    }
}

// ---------------------------------------------------------------------------
// 数量
// ---------------------------------------------------------------------------

/// 数量(手数 / 股数 / 张数)。
///
/// 用 `u64` 而不是 `i64`:数量不可能是负的。这不是省内存,是让「负数量」这种状态
/// 在类型上就不存在。代价是减法会下溢,所以减法必须走 [`Qty::checked_sub`]。
///
/// ```
/// use rung_core::Qty;
///
/// let a = Qty::new(5);
/// let b = Qty::new(3);
/// assert_eq!(a.checked_sub(b), Some(Qty::new(2)));
/// assert_eq!(b.checked_sub(a), None);   // 不够减 → None,不是 panic
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Qty(u64);

impl Qty {
    pub const ZERO: Qty = Qty(0);

    pub const fn new(n: u64) -> Self {
        Qty(n)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    /// 行为规格:`Qty(0)` → `true`,其余 → `false`。
    ///
    /// 撮合里判断「这笔单吃完了没有」全靠它。
    pub fn is_zero(self) -> bool {
        todo!("1a")
    }

    /// 减法。行为规格:
    /// - `Qty(5).checked_sub(Qty(3))` → `Some(Qty(2))`
    /// - `Qty(5).checked_sub(Qty(5))` → `Some(Qty(0))`
    /// - `Qty(3).checked_sub(Qty(5))` → `None`
    ///
    /// **不 panic,不返回 0。** 撮合里如果出现「剩余量不够减」,那是逻辑错误。
    /// 返回 `None` 会逼调用方处理它;返回 `0`(saturating)会把 bug 藏起来 ——
    /// 静默的错误是最糟的一档。
    ///
    /// *提示*:`u64` 自带 `checked_sub`,包一层就行。
    pub fn checked_sub(self, other: Qty) -> Option<Qty> {
        todo!("1a")
    }

    /// 加法。溢出返回 `None`。
    ///
    /// 行为规格:`Qty(u64::MAX).checked_add(Qty(1))` → `None`。
    pub fn checked_add(self, other: Qty) -> Option<Qty> {
        todo!("1a")
    }

    /// 取两者中较小的。撮合里「这次成交多少」就是 `min(taker剩余, maker剩余)`。
    ///
    /// *注*:`Qty` 已经 derive 了 `Ord`,所以标准库的 `Ord::min` 本来就能用。
    /// 这里显式写一个同名的固有方法,是为了让调用处的意图更醒目 ——
    /// 固有方法优先于 trait 方法被解析。
    pub fn min(self, other: Qty) -> Qty {
        todo!("1a")
    }
}

impl fmt::Display for Qty {
    /// 行为规格:数量没有小数,直接打内部的数。`Qty(100)` → `"100"`。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("1a")
    }
}

// ---------------------------------------------------------------------------
// 买卖方向
// ---------------------------------------------------------------------------

/// 买还是卖。
///
/// 用 enum 而不是 `bool`:`submit(id, true, false)` 这种调用没人看得懂。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    /// 对手方。行为规格:`Buy → Sell`,`Sell → Buy`。
    ///
    /// 不变量:`s.opposite().opposite() == s`。
    pub fn opposite(self) -> Side {
        todo!("1a")
    }

    /// 这一侧「更优」的价格是不是更高的那个。
    ///
    /// 行为规格:
    /// - `Buy` → `true`(买盘出价**高**的更优:更想要,排前面)
    /// - `Sell` → `false`(卖盘要价**低**的更优)
    ///
    /// 见 `PRIMER.md` §4 的价格优先规则。阶段 3 处理买卖两边方向相反时会用到。
    pub fn higher_is_better(self) -> bool {
        todo!("1a")
    }
}

impl fmt::Display for Side {
    /// 行为规格:`Buy` → `"BUY"`,`Sell` → `"SELL"`。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("1a")
    }
}

// ===========================================================================
// 定向测试 · 子步 1a
// ===========================================================================
//
// 这些是**单元测试**:和被测代码同一个文件,所以能看见私有字段
//(下面直接写 `Price(1005)`,在 crate 外面写不出来)。
// 对照 `tests/public_api.rs` —— 那是集成测试,住在另一个 crate 里,只能碰 pub 的东西。
//
// 跑法:./scripts/test.sh types

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Price · 显示 ----

    #[test]
    fn price_display_补零() {
        assert_eq!(Price(1005).to_string(), "10.05");
        assert_eq!(Price(310).to_string(), "3.10"); // 不是 "3.1"
        assert_eq!(Price(5).to_string(), "0.05");
        assert_eq!(Price(0).to_string(), "0.00");
        assert_eq!(Price(100).to_string(), "1.00");
    }

    #[test]
    fn price_display_负数() {
        assert_eq!(Price(-50).to_string(), "-0.50");
        assert_eq!(Price(-1005).to_string(), "-10.05");
        assert_eq!(Price(-5).to_string(), "-0.05");
    }

    // ---- Price · 单位转换 ----

    #[test]
    fn price_元转tick() {
        assert_eq!(Price::from_units(10.05), Price(1005));
        assert_eq!(Price::from_units(3.1), Price(310));
        assert_eq!(Price::from_units(0.0), Price(0));
        assert_eq!(Price::from_units(-0.5), Price(-50));
    }

    #[test]
    fn price_tick转元() {
        assert_eq!(Price(1005).to_units(), 10.05);
        assert_eq!(Price(0).to_units(), 0.0);
    }

    #[test]
    fn price_四舍五入而非截断() {
        // 10.05 在 f64 里是 10.049999999999998934...
        // 直接 as i64 会得到 1004。
        assert_eq!(Price::from_units(10.05), Price(1005));
        assert_eq!(Price::from_units(10.054), Price(1005));
        assert_eq!(Price::from_units(10.056), Price(1006));
    }

    #[test]
    fn price_往返无损() {
        // tick → 元 → tick 必须一模一样。
        // 反方向(元 → tick → 元)不保证,f64 本来就存不下所有小数。
        for ticks in [0, 1, 5, 99, 100, 310, 1005, 999_999, -1005] {
            let p = Price(ticks);
            assert_eq!(Price::from_units(p.to_units()), p, "ticks = {ticks}");
        }
    }

    // ---- Price · 比较与差值 ----

    #[test]
    fn price_可排序_这是阶段3的前提() {
        assert!(Price(1000) < Price(1005));

        let mut v = vec![Price(1005), Price(999), Price(1000)];
        v.sort();
        assert_eq!(v, vec![Price(999), Price(1000), Price(1005)]);
    }

    #[test]
    fn price_差值可以为负() {
        assert_eq!(Price(1050).diff_ticks(Price(1040)), 10);
        assert_eq!(Price(1040).diff_ticks(Price(1050)), -10);
        assert_eq!(Price(1000).diff_ticks(Price(1000)), 0);
    }

    // ---- Qty ----

    #[test]
    fn qty_是否为零() {
        assert!(Qty::ZERO.is_zero());
        assert!(Qty::new(0).is_zero());
        assert!(!Qty::new(1).is_zero());
    }

    #[test]
    fn qty_减法不够减时返回空() {
        assert_eq!(Qty::new(5).checked_sub(Qty::new(3)), Some(Qty::new(2)));
        assert_eq!(Qty::new(5).checked_sub(Qty::new(5)), Some(Qty::ZERO));
        assert_eq!(Qty::new(3).checked_sub(Qty::new(5)), None);
        assert_eq!(Qty::ZERO.checked_sub(Qty::new(1)), None);
    }

    #[test]
    fn qty_加法溢出时返回空() {
        assert_eq!(Qty::new(2).checked_add(Qty::new(3)), Some(Qty::new(5)));
        assert_eq!(Qty::new(u64::MAX).checked_add(Qty::new(1)), None);
        assert_eq!(Qty::ZERO.checked_add(Qty::ZERO), Some(Qty::ZERO));
    }

    #[test]
    fn qty_取较小() {
        assert_eq!(Qty::new(5).min(Qty::new(3)), Qty::new(3));
        assert_eq!(Qty::new(3).min(Qty::new(5)), Qty::new(3));
        assert_eq!(Qty::new(4).min(Qty::new(4)), Qty::new(4));
        assert_eq!(Qty::ZERO.min(Qty::new(9)), Qty::ZERO);
    }

    #[test]
    fn qty_display() {
        assert_eq!(Qty::new(100).to_string(), "100");
        assert_eq!(Qty::ZERO.to_string(), "0");
    }

    // ---- Side ----

    #[test]
    fn side_对手方() {
        assert_eq!(Side::Buy.opposite(), Side::Sell);
        assert_eq!(Side::Sell.opposite(), Side::Buy);
    }

    #[test]
    fn side_取两次回到自己() {
        // 一条不变量。
        assert_eq!(Side::Buy.opposite().opposite(), Side::Buy);
        assert_eq!(Side::Sell.opposite().opposite(), Side::Sell);
    }

    #[test]
    fn side_哪边更优() {
        assert!(Side::Buy.higher_is_better(), "买盘:出价高的排前面");
        assert!(!Side::Sell.higher_is_better(), "卖盘:要价低的排前面");
    }

    #[test]
    fn side_display() {
        assert_eq!(Side::Buy.to_string(), "BUY");
        assert_eq!(Side::Sell.to_string(), "SELL");
    }
}
