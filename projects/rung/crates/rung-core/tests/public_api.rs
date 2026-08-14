//! 集成测试:站在**使用者**的位置上看这个库。
//!
//! 这个文件被编译成**一个独立的 crate**,通过 `use rung_core::...` 引用被测代码。
//! 也就是说,它和你未来的用户处在完全相同的位置 —— 私有的东西它一样看不见。
//!
//! 这就是集成测试和单元测试的根本区别:
//!
//! | | 位置 | 能看见什么 | 回答什么问题 |
//! |---|---|---|---|
//! | 单元测试 `#[cfg(test)] mod tests` | 和被测代码同一个文件 | **包括私有项** | 这段逻辑对不对 |
//! | 集成测试 `tests/*.rs` | 独立 crate | **只有 pub 的** | 这个库好不好用 |
//!
//! 所以每加一个 `pub`,都应该问一句:「集成测试需要它吗?」
//! 不需要就别公开 —— 公开了就是承诺,承诺是不可逆的(见 `METHOD.md` 方法五)。

use rung_core::{Order, Price, Qty, Side, TICKS_PER_UNIT};

#[test]
fn 只用公共api就能造出一笔订单() {
    let o = Order::new(1, Side::Buy, Price::from_ticks(1005), Qty::new(100));

    assert_eq!(o.id, 1);
    assert_eq!(o.price.to_string(), "10.05");
    assert_eq!(o.qty.to_string(), "100");
    assert_eq!(o.side.to_string(), "BUY");
}

#[test]
fn 外面构造价格只能走from_ticks() {
    // 下面这一行**故意是注释**。取消注释,应该看到:
    //     error[E0423]: cannot initialize a tuple struct which contains private fields
    //
    // let p = Price(1005);
    //
    // 这不是限制,是设计:`Price` 内部存的是 tick 不是元,
    // 而「1005 这个数是 tick」这条约定,如果靠人记就会有人记错。
    // 把字段设成私有,编译器就替你记着了。
    //
    // 对照 `src/types.rs` 里的单元测试 —— 那里直接写 `Price(1005)` 是合法的,
    // 因为它和字段住在同一个模块。同一行代码,在两个位置一个能编译一个不能,
    // 差别就是 day8 学的可见性。

    let p = Price::from_ticks(1005);
    assert_eq!(p.ticks(), 1005);
}

#[test]
fn tick常量是公开的() {
    // 使用者需要知道 tick size 才能正确构造价格,所以它必须 pub。
    // 这是一个「必须公开」的例子 —— 对照上面那个「必须私有」的例子。
    assert_eq!(TICKS_PER_UNIT, 100);
    assert_eq!(Price::from_ticks(TICKS_PER_UNIT).to_string(), "1.00");
}

#[test]
fn 价格可排序_这是阶段3的前提() {
    // 阶段 3 要把 Price 当 BTreeMap 的 key,前提是它实现了 Ord。
    // 这条测试现在看着无聊,但它锁住了一个后面必须成立的性质。
    let mut prices = vec![
        Price::from_ticks(1005),
        Price::from_ticks(999),
        Price::from_ticks(1050),
    ];
    prices.sort();

    assert_eq!(
        prices,
        vec![
            Price::from_ticks(999),
            Price::from_ticks(1005),
            Price::from_ticks(1050),
        ]
    );
}

#[test]
fn 数量减不动时返回none而不是崩溃() {
    // 库的错误处理风格:能返回 Option/Result 的地方绝不 panic。
    // 使用者可以自己决定是 unwrap 还是优雅处理,但选择权在他手上。
    let 剩余 = Qty::new(3);
    let 想吃 = Qty::new(5);

    assert_eq!(剩余.checked_sub(想吃), None);
    assert_eq!(想吃.checked_sub(剩余), Some(Qty::new(2)));
}

#[test]
fn 一个最小的使用场景() {
    // 假装你是这个库的用户,想表达:「以 10.50 买 5 手」。
    // 如果这段代码写起来别扭,说明 API 设计有问题 —— 这才是集成测试真正的价值。
    let 买单 = Order::new(
        1,
        Side::Buy,
        Price::from_units(10.50),
        Qty::new(5),
    );
    let 卖单 = Order::new(
        2,
        Side::Sell,
        Price::from_units(10.40),
        Qty::new(3),
    );

    // 买单出价 10.50,卖单要价 10.40 —— 买的比卖的贵,能成交。
    // (真正的判断逻辑在 rung-match,这里只是验证类型够用。)
    assert!(买单.price >= 卖单.price, "10.50 >= 10.40,可以成交");

    // 这次最多能成交多少?两边剩余量的较小者。
    assert_eq!(买单.qty.min(卖单.qty), Qty::new(3));

    // 成交价按 maker(先挂的那个)。见 PRIMER.md §3。
    assert_eq!(卖单.price.to_string(), "10.40");
}
