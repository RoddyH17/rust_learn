//! 集成测试:站在**使用者**的位置上看这个库。
//!
//! 这个文件被编译成**一个独立的 crate**,通过 `use rung_core::...` 引用被测代码。
//! 它和你未来的用户处在完全相同的位置 —— 私有的东西它一样看不见。
//!
//! | | 位置 | 能看见什么 | 回答什么问题 |
//! |---|---|---|---|
//! | 单元测试 `#[cfg(test)] mod tests` | 和被测代码同一个文件 | **包括私有项** | 这段逻辑对不对 |
//! | 集成测试 `tests/*.rs` | 独立 crate | **只有 pub 的** | 这个库好不好用 |
//!
//! 每加一个 `pub` 都该问一句:「集成测试需要它吗?」不需要就别公开。

use rung_core::{BookError, NaiveBook, Order, Price, Qty, Side, TICKS_PER_UNIT};

fn p(t: i64) -> Price {
    Price::from_ticks(t)
}
fn q(n: u64) -> Qty {
    Qty::new(n)
}

#[test]
fn 外面构造价格只能走from_ticks() {
    // 下面这一行**故意是注释**。取消注释,应该看到:
    //     error[E0423]: cannot initialize a tuple struct which contains private fields
    //
    // let bad = Price(1005);
    //
    // 对照 src/types.rs 的单元测试 —— 那里直接写 `Price(1005)` 是合法的,
    // 因为它和字段住在同一个模块。同一行代码在两个位置一个能编译一个不能,
    // 差别就是 day8 学的可见性。

    let p = Price::from_ticks(1005);
    assert_eq!(p.ticks(), 1005);
    assert_eq!(p.to_string(), "10.05");
}

#[test]
fn 外面改不了订单的字段() {
    // 同样故意注释掉。取消注释应该看到 E0616:field is private
    //
    // let mut o = Order::new(1, Side::Buy, p(1000), q(5));
    // o.qty = q(0);
    //
    // 这不是为了刁难。阶段 2 的档位会缓存「本档总量」,如果外面能绕过档位
    // 直接改订单数量,那个缓存就静默地错了 —— 而静默的错误最难查。

    let o = Order::new(1, Side::Buy, p(1000), q(5));
    assert_eq!(o.qty(), q(5)); // 只读
}

#[test]
fn tick常量是公开的() {
    // 使用者需要知道 tick size 才能正确构造价格,所以它必须 pub。
    // 这是「必须公开」的例子,对照上面两个「必须私有」的例子。
    assert_eq!(TICKS_PER_UNIT, 100);
    assert_eq!(Price::from_ticks(TICKS_PER_UNIT).to_string(), "1.00");
}

#[test]
fn 一个最小的完整使用场景() {
    // 假装你是这个库的用户。如果这段代码写起来别扭,说明 API 设计有问题 ——
    // 这才是集成测试真正的价值。
    let mut book = NaiveBook::new();

    book.insert(Order::new(1, Side::Sell, p(1050), q(3)))
        .unwrap();
    book.insert(Order::new(2, Side::Sell, p(1040), q(2)))
        .unwrap();
    book.insert(Order::new(3, Side::Buy, p(1000), q(4)))
        .unwrap();

    assert_eq!(book.best_bid(), Some(p(1000)));
    assert_eq!(book.best_ask(), Some(p(1040)));
    assert_eq!(book.spread(), Some(40));

    // 买 10.50 x 5 —— 跨两档吃光卖盘
    let trades = book
        .submit(Order::new(4, Side::Buy, p(1050), q(5)))
        .unwrap();

    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].price, p(1040)); // 成交价按 maker
    assert_eq!(trades[1].price, p(1050));

    assert_eq!(book.best_ask(), None, "卖盘被吃光");
    assert_eq!(book.depth(Side::Buy), vec![(p(1000), q(4))]);
}

#[test]
fn 错误是可以被穷尽匹配的() {
    // 用 enum 而不是 String 的全部理由:调用方能写出「只处理某一种失败」的逻辑。
    let mut book = NaiveBook::new();
    book.insert(Order::new(1, Side::Buy, p(1000), q(5)))
        .unwrap();

    let 结果 = book.insert(Order::new(1, Side::Buy, p(1010), q(5)));

    match 结果 {
        Err(BookError::DuplicateId(id)) => assert_eq!(id, 1),
        Err(BookError::ZeroQty) => panic!("不该是零量"),
        Ok(_) => panic!("重复 id 应该被拒"),
    }
}

#[test]
fn 库不会因为坏输入而崩溃() {
    // 这个库的错误处理风格:能返回 Option/Result 的地方绝不 panic。
    // 选择权留给使用者。
    let mut book = NaiveBook::new();

    assert!(book.cancel(999).is_none());
    assert_eq!(book.best_bid(), None);
    assert_eq!(book.spread(), None);
    assert!(
        book.submit(Order::new(1, Side::Buy, p(1000), q(0)))
            .is_err()
    );
    assert!(book.is_empty());
}
