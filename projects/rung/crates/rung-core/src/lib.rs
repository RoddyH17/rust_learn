//! # rung-core —— 限价订单簿的类型与容器层
//!
//! Rung 的最底层。这一层提供「价格」「数量」「订单」「成交」这些概念,
//! 以及一个**故意写得最笨但显然正确**的订单簿 [`NaiveBook`]。
//!
//! ## 阶段 1 交付的是什么
//!
//! 一个**能跑的、慢的、正确的**订单簿。
//!
//! 这个顺序是刻意的:先做出一个笨的但正确的版本,后面九个阶段的工作是让它变快,
//! 并且每一步都要证明**结果和笨版本完全一致**。
//!
//! [`NaiveBook`] 因此有两个身份:
//!
//! 1. **黄金参考模型** —— 随机测试拿它对答案。不变量断言只能抓出「买一 < 卖一被破坏」
//!    这类结构性错误;参考模型能抓出「成交价错了 1 个 tick」这类语义性错误
//! 2. **性能基线** —— 阶段 1 结束时用 `./scripts/eval.sh` 测出的吞吐是分母,
//!    后面每份报告都要回答「快了几倍,且结果一致」
//!
//! ## 架构约束(硬规矩,不是风格)
//!
//! - **零依赖**:`Cargo.toml` 的 `[dependencies]` 必须永远是空的
//! - **零 I/O**:不碰网络、文件、标准输出
//! - **零非确定性**:不读系统时钟、不用随机数、不依赖 `HashMap` 的迭代顺序
//! - **优化只许改实现,不许改接口** —— 为了性能把私有字段改成 `pub` 是不可接受的
//!
//! 交易所的撮合内核必须公平,而公平要求确定性:同一批订单不管跑多少次、
//! 在哪台机器上跑,结果必须一样。详见 `PRIMER.md` §6。
//!
//! ## 进度
//!
//! | 阶段 | 内容 | 状态 |
//! |---|---|---|
//! | 1 | 类型 + `NaiveBook` 参考模型 + 基线 | 🔨 进行中 |
//! | 2 | `PriceLevel`:`Vec` vs `VecDeque` 对比 | ⏸ |
//! | 3 | `BookSide<K: Ord>`:两份重复代码 vs 泛型 | ⏸ |
//! | 4 | 索引:档位存 `Order` vs 存 `OrderId` | ⏸ |
//! | 5 | 撮合:每次重扫 vs 增量 | ⏸ |
//!
//! ## 用法
//!
//! ```
//! use rung_core::{NaiveBook, Order, Price, Qty, Side};
//!
//! let mut book = NaiveBook::new();
//!
//! // 挂两笔卖单
//! book.insert(Order::new(1, Side::Sell, Price::from_ticks(1050), Qty::new(3))).unwrap();
//! book.insert(Order::new(2, Side::Sell, Price::from_ticks(1040), Qty::new(2))).unwrap();
//!
//! // 买 10.50 x 5:先吃 10.40 的 2 手,再吃 10.50 的 3 手
//! let trades = book.submit(Order::new(3, Side::Buy, Price::from_ticks(1050), Qty::new(5))).unwrap();
//!
//! assert_eq!(trades.len(), 2);
//! assert_eq!(trades[0].price, Price::from_ticks(1040));  // 成交价按 maker
//! assert_eq!(trades[1].price, Price::from_ticks(1050));
//! assert!(book.is_empty());
//! ```

// 阶段 1 的函数体还是 todo!(),所以参数「没被用到」、私有字段「没被读过」、
// pub(crate) 的方法「没人调用」—— 全都是骨架期的必然状态。
//
// **把 18 个 todo!() 全部实现完之后,删掉下面这两行。**
// 删了还有警告,说明你真的漏了什么。
#![allow(unused_variables)]
#![allow(dead_code)]

mod naive;
mod order;
mod trade;
mod types;

// ---------------------------------------------------------------------------
// 公共 API —— 「承诺」的边界
// ---------------------------------------------------------------------------
//
// 只有在这里 `pub use` 的东西才是对外的承诺。承诺一旦给出去,改它就要升大版本,
// 所以默认动作是**不给**。
//
// 上面四个 `mod` 都没有 `pub` —— 模块本身私有,外面只看得到下面重导出的名字。
// 好处:以后把 types.rs 拆成三个文件、或者改名,对使用者完全无感。

pub use naive::NaiveBook;
pub use order::Order;
pub use trade::{BookError, Trade};
pub use types::{OrderId, Price, Qty, Seq, Side, TICKS_PER_UNIT};
