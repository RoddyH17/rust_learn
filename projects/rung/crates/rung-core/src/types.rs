//! 基础类型:价格、数量、买卖方向、标识符。
//!
//! 这个模块里没有任何逻辑,只有「概念到类型」的映射。
//! 但它决定了后面九个阶段能不能写得动 —— 类型选错,撮合就写不出来。

use core::fmt;

// ---------------------------------------------------------------------------
// 标识符
// ---------------------------------------------------------------------------

/// 订单的唯一标识。
///
/// 这里用的是**类型别名**,不是 newtype —— 也就是说 `OrderId` 和 `u64`
/// 在编译器眼里完全是同一个类型,传错了不会报错。
///
/// 这是一个**故意留下的不一致**:`Price` 和 `Qty` 都包了 newtype,凭什么 `OrderId` 不包?
/// 想清楚这个问题(stage-01.md 的思考阶梯 Q5),它是「什么时候值得付 newtype 的代价」
/// 这个判断的第一次练习。
pub type OrderId = u64;

/// 到达序号:单调递增,用来实现「时间优先」。
///
/// 为什么不用时间戳?见 `PRIMER.md` §6 —— 两台机器的时钟不可能完全同步,
/// 而且同一微秒内可能来多笔订单。序号由系统统一分配,严格递增,天然唯一,
/// 而且**可重放**(阶段 9 的确定性靠它)。
pub type Seq = u64;

// ---------------------------------------------------------------------------
// 价格
// ---------------------------------------------------------------------------

/// 一个价格单位里有多少个 tick。
///
/// 100 = 两位小数,即 tick size 为 0.01。
///
/// ⚠️ 这是一个**全局常量**,意味着这个库假设所有品种的 tick size 都一样。
/// 真实交易所不是这样的 —— 见 stage-01.md 的思考阶梯 Q3。
pub const TICKS_PER_UNIT: i64 = 100;

/// 价格,以 tick 为单位的整数。
///
/// 内部存的是「多少个最小单位」,不是「多少元」。`Price` 里的 `1005` 表示 10.05。
///
/// 字段是**私有**的 —— crate 外面没法写 `Price(1005)`,只能走 [`Price::from_ticks`]。
/// 这不是啰嗦:它意味着「1005 这个数是 tick 不是元」这条约定,由编译器替你守着。
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
    /// 从 tick 数构造。这是 crate 外部唯一的构造入口。
    pub const fn from_ticks(ticks: i64) -> Self {
        Price(ticks)
    }

    /// 取出内部的 tick 数。
    pub const fn ticks(self) -> i64 {
        self.0
    }

    /// 从「元」构造,四舍五入到最近的 tick。
    ///
    /// ⚠️ 这个函数是整个库里**唯一**碰 `f64` 的地方,而且只用于人机边界
    /// (读配置、解析用户输入)。撮合路径上一个浮点数都不许出现。
    ///
    /// 提示:`0.1 + 0.2 == 0.3` 在浮点数里是 `false`。先写个 `println!` 验证一下,
    /// 再想想为什么这个函数必须 `.round()` 而不能直接 `as i64`。
    pub fn from_units(units: f64) -> Self {
        todo!("T1.3:units 元 → tick。想清楚为什么必须 round()")
    }

    /// 转回「元」。同样只用于展示,不用于计算。
    pub fn to_units(self) -> f64 {
        todo!("T1.3")
    }

    /// 两个价格之间差多少个 tick。
    ///
    /// 返回 `i64` 而不是 `Price` —— 因为「差值」和「价格」是两个概念,
    /// 一个价差不应该能被拿去当 `BTreeMap` 的 key。
    ///
    /// **这就是 i64 而不是 u64 的理由**:`bid - ask` 的中间结果可以是负数。
    pub fn diff_ticks(self, other: Price) -> i64 {
        todo!("T1.3")
    }
}

impl fmt::Display for Price {
    /// `1005 → "10.05"`,`310 → "3.10"`,`5 → "0.05"`,`-50 → "-0.50"`。
    ///
    /// 注意补零:`310` 要显示成 `"3.10"` 而不是 `"3.1"`。
    ///
    /// 提示一:`{:02}` 是「至少两位,不够补零」。
    /// 提示二:负数要单独处理符号 —— `-50 / 100` 是 `0`,`-50 % 100` 是 `-50`,
    ///         直接套公式会打出 `"0.-50"`。
    /// 提示三:**不要写 `write!(f, "{}", format!(...))`。**
    ///         `format!` 会在堆上分配一个新 `String`,而 `write!` 直接往 f 里写,
    ///         一次分配都不用。这一条在 stage-01.md 的「String 补课」一节里展开。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("T1.3:手写 Display。Debug 能 derive,Display 不行 —— 因为「给人看的样子」只有你知道")
    }
}

// ---------------------------------------------------------------------------
// 数量
// ---------------------------------------------------------------------------

/// 数量(手数 / 股数 / 张数)。
///
/// 用 `u64` 而不是 `i64`:**数量不可能是负的**。
/// 这不是省内存,是让「负数量」这种状态在类型上就不存在 —— 编译器替你挡掉一整类 bug。
///
/// 代价是减法会下溢,所以减法必须走 [`Qty::checked_sub`],它返回 `Option`。
///
/// ```
/// use rung_core::Qty;
///
/// let a = Qty::new(5);
/// let b = Qty::new(3);
/// assert_eq!(a.checked_sub(b), Some(Qty::new(2)));
/// assert_eq!(b.checked_sub(a), None);   // 不够减,不是 panic,是 None
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

    /// 是不是零。撮合里判断「这笔单吃完了没有」全靠它。
    pub fn is_zero(self) -> bool {
        todo!("T1.4")
    }

    /// 减法。不够减返回 `None`,**不 panic**。
    ///
    /// 想清楚:为什么不设计成「不够减就返回 0」(saturating)?
    /// 提示:撮合里如果出现「taker 剩余量不够减」,那是**逻辑错误**,不是正常情况。
    /// 返回 `None` 会逼调用方处理它;返回 0 会把 bug 藏起来。
    /// —— 这是 `METHOD.md` 方法六「把错误往上一档推」的一次实践。
    pub fn checked_sub(self, other: Qty) -> Option<Qty> {
        todo!("T1.4")
    }

    /// 加法。溢出返回 `None`。
    pub fn checked_add(self, other: Qty) -> Option<Qty> {
        todo!("T1.4")
    }

    /// 取两者中较小的那个。撮合里「这次成交多少」就是 `min(taker剩余, maker剩余)`。
    pub fn min(self, other: Qty) -> Qty {
        todo!("T1.4:标准库的 Ord::min 也能用,但显式写一个让意图更清楚")
    }
}

impl fmt::Display for Qty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("T1.4:数量没有小数,直接打内部的数就行")
    }
}

// ---------------------------------------------------------------------------
// 买卖方向
// ---------------------------------------------------------------------------

/// 买还是卖。
///
/// 只有两个变体,看起来用 `bool` 也行 —— 但 `is_buy: bool` 在函数签名里读不出来,
/// 而且 `submit(id, true, false)` 这种调用没人看得懂。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    /// 对手方。撮合时「买单要去卖盘找对手」就是这个。
    pub fn opposite(self) -> Side {
        todo!("T1.5")
    }

    /// 这一侧「更优」的价格是更高还是更低?
    ///
    /// 买盘:出价**高**的更优(更想要,排前面)。
    /// 卖盘:要价**低**的更优。
    ///
    /// 返回 `true` 表示「价格越高越优」。
    /// 这个方法现在看着没用 —— 阶段 3 处理买卖两边方向相反时你会回来找它。
    pub fn higher_is_better(self) -> bool {
        todo!("T1.5")
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("T1.5:Buy → \"BUY\",Sell → \"SELL\"")
    }
}

// ===========================================================================
// 测试
// ===========================================================================
//
// 这些是**单元测试**:它们和被测代码住在同一个文件里,所以能看见私有字段
//(下面直接写了 `Price(1005)`,这在 crate 外面是写不出来的)。
//
// 对照 `tests/public_api.rs` —— 那是**集成测试**,住在另一个 crate 里,
// 只能碰 pub 的东西。两者的区别就是这个。

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Price ----

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
        // 价格本身不该是负的,但 Price 是个通用的整数包装,
        // 负数进来时不能打出 "0.-50" 这种垃圾。
        assert_eq!(Price(-50).to_string(), "-0.50");
        assert_eq!(Price(-1005).to_string(), "-10.05");
    }

    #[test]
    fn price_元与tick互转() {
        assert_eq!(Price::from_units(10.05), Price(1005));
        assert_eq!(Price::from_units(3.1), Price(310));
        assert_eq!(Price::from_units(0.0), Price(0));
        assert_eq!(Price(1005).to_units(), 10.05);
    }

    #[test]
    fn price_往返无损() {
        // 从 tick 出发转成元再转回来,必须一模一样。
        // 反方向(元 → tick → 元)不保证,因为 f64 本来就存不下所有小数。
        for ticks in [0, 1, 5, 99, 100, 310, 1005, 999_999] {
            let p = Price(ticks);
            assert_eq!(Price::from_units(p.to_units()), p, "ticks = {ticks}");
        }
    }

    #[test]
    fn price_from_units_四舍五入() {
        // 10.05 在 f64 里其实是 10.0499999999999989...
        // 直接 `as i64` 会截断成 1004。必须 round。
        assert_eq!(Price::from_units(10.05), Price(1005));
        assert_eq!(Price::from_units(10.054), Price(1005));
        assert_eq!(Price::from_units(10.056), Price(1006));
    }

    #[test]
    fn price_可比较() {
        // 这条测试在证明 derive(Ord) 起了作用 —— 阶段 3 的 BTreeMap 全靠它。
        assert!(Price(1000) < Price(1005));
        assert_eq!(Price(1000).max(Price(1005)), Price(1005));

        let mut v = vec![Price(1005), Price(999), Price(1000)];
        v.sort();
        assert_eq!(v, vec![Price(999), Price(1000), Price(1005)]);
    }

    #[test]
    fn price_差值可以是负的() {
        // 这就是内部用 i64 而不是 u64 的理由。
        assert_eq!(Price(1050).diff_ticks(Price(1040)), 10);
        assert_eq!(Price(1040).diff_ticks(Price(1050)), -10);
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
        // 关键的一条:不 panic,不返回 0,返回 None。
        assert_eq!(Qty::new(3).checked_sub(Qty::new(5)), None);
    }

    #[test]
    fn qty_加法溢出时返回空() {
        assert_eq!(Qty::new(2).checked_add(Qty::new(3)), Some(Qty::new(5)));
        assert_eq!(Qty::new(u64::MAX).checked_add(Qty::new(1)), None);
    }

    #[test]
    fn qty_取较小() {
        assert_eq!(Qty::new(5).min(Qty::new(3)), Qty::new(3));
        assert_eq!(Qty::new(3).min(Qty::new(5)), Qty::new(3));
        assert_eq!(Qty::new(4).min(Qty::new(4)), Qty::new(4));
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
        // 取两次回到自己 —— 这是一条不变量。
        assert_eq!(Side::Buy.opposite().opposite(), Side::Buy);
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
