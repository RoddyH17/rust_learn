// Practice — Mini Project: 迷你订单引擎 (Day 5 + 5.5 + 6 合并)
// Run with: cargo run --example practice
//
// 这一份和前几天不一样:不是十道互不相干的小题,而是**一个项目**。
// 你要从零写出一个能跑的订单状态引擎,大约 100 行代码。
//
// 覆盖范围:
//   Day 5    enum 的三种变体形态、Option/Result 的思路
//   Day 5.5  match:结构体解构、守卫 guard、`@` 绑定、ref mut、if let / while let
//   Day 6    struct、impl(方法 vs 关联函数)、trait、静态分发 vs 动态分发
//   JSON     serde derive + json! 宏 + serde_json 解析(Stage 9-11)
//
// JSON 的规矩:**数据进出一律走 serde,不手写字符串解析器。**
//   - 依赖:serde(derive 特性)+ serde_json,已经在 Cargo.toml 里了
//   - 造测试数据用 `json!` 宏,不用手拼字符串
//   - 类型用 `#[derive(Serialize, Deserialize)]`,不手写 impl
//   - 解析失败走 `Result`,不 unwrap 到 panic
//
// 规则和以前一样:**这个文件必须始终能编译**。你要写的代码写在下面标好的位置,
// 每写完一个 Stage,就把 main 里对应那一段验收断言取消注释,`cargo run --example practice`
// 跑一遍。全部取消注释且断言全过 = 项目完成。
//
// 评级:🌟 照着提示就能写   🌟🌟 需要想一下   🌟🌟🌟 卡住了回去翻笔记
// 参考(卡住时再看,别提前翻):
//   https://practice-rust-zh.beatai.org/pattern-match/patterns.html
//   https://practice-rust-zh.beatai.org/pattern-match/match-iflet.html
//   https://practice-rust-zh.beatai.org/compound-types/struct.html
//   https://practice-rust-zh.beatai.org/compound-types/enum.html
//   https://practice-rust-zh.beatai.org/generics-traits/traits.html
// ============================================================================
// 需求:一个订单在生命周期里只可能处于三种状态之一 —— 挂着、成交了、被撤了。
//       成交要记下成交价和成交量;撤单要记下原因。引擎要能分类、能结算、能打印。
//
//       你在 5.5 的笔记里自己写过这句话,这个项目就是它的展开:
//         struct 表达「同时拥有这些东西」(AND)
//         enum   表达「只能是这些可能性之一」(OR)
// ============================================================================

// ---------------------------------------------------------------------------
// Stage 1 · 两个 enum (🌟)
// ---------------------------------------------------------------------------
// 1a. 写一个 `Side`,只有 `Buy` 和 `Sell` 两个单元变体。
//     提示:后面要用 `==` 比较它、要 `{:?}` 打印它、还要在结构体被 clone 时跟着走。
//           想想 Day 5 说的 derive —— 你需要哪几个?(答案不止一个)
//     提示:它没有任何载荷,所以按 Day 5 的内存布局那节,它是 1 个字节 —— 也就是说
//           让它 Copy 是免费的,不 Copy 反而会给你自己找麻烦。
//
// 1b. 写一个 `OrderStatus`,三个变体,**故意让三种形态都出现一次**:
//       Pending                                          单元变体
//       Filled { execution_price: f64, quantity: f64 }   结构体变体
//       Canceled { reason: String }                      结构体变体,带 String
//     提示:`Canceled` 里是 String 不是 &str —— 那么这个 enum 还能 Copy 吗?
//           为什么 `Side` 可以而它不行?

// 在这里写 Stage 1:

// ---------------------------------------------------------------------------
// Stage 2 · Order 结构体 (🌟)
// ---------------------------------------------------------------------------
// 写一个 `Order`,同时拥有:id: u32、side: Side、price: f64、quantity: f64、
// status: OrderStatus。
//   提示:这就是「AND」那一半。五个字段全都要有,没有默认值。
//   提示:后面 main 里会 clone 订单放进队列,所以 derive 别漏。

// 在这里写 Stage 2:

// ---------------------------------------------------------------------------
// Stage 3 · impl:关联函数 + 方法 (🌟🌟)
// ---------------------------------------------------------------------------
// 给 `Order` 写一个 impl 块,包含四个东西:
//
// 3a. `new(id, side, price, quantity) -> Order`
//     关联函数(没有 self),新订单的状态一律是 `Pending`。
//     提示:字段初始化简写能让函数体缩成一行。
//
// 3b. `notional(&self) -> f64`
//     返回 price * quantity。
//     提示:只读,所以是 `&self`。
//
// 3c. `fill(&mut self, execution_price: f64, quantity: f64)`
//     把 status 换成 `Filled { .. }`。
//     ⚠️ Day 6 的坑:如果你写成 `fn fill(mut self, ..)`,它能编译、什么也不会发生、
//        而且调用之后订单就没了。三种接收者的区别想清楚再动手。
//
// 3d. `cancel(&mut self, reason: &str)`
//     把 status 换成 `Canceled { .. }`。
//     提示:参数是 `&str` 而字段是 `String` —— 中间要过一道,想想 Day 4 说过
//           哪个方向要花一次堆分配。

// 在这里写 Stage 3:

// ---------------------------------------------------------------------------
// Stage 4 · ref mut:原地改一个 enum 里的 String (🌟🌟🌟)
// ---------------------------------------------------------------------------
// 再给 `Order` 加一个方法:
//   `annotate_cancel(&mut self, extra: &str)`
// 行为:如果当前状态是 `Canceled`,就把 extra **追加**到已有的 reason 后面;
//       其它状态什么也不做。
//
//   提示:你要改的是 enum 变体**里面**那个 String,不是替换整个 status。
//   提示:直接 `match self.status { OrderStatus::Canceled { reason } => ... }`
//         会把 String 移出去 —— 而 self 只是个 &mut,移不走,编译器会拦你。
//         5.5 笔记第 8 条讲的就是这个:想借用而不是转移时用什么?
//   提示:拿到可变借用之后,`String` 上追加内容的方法叫 push_str。
//   提示:另一种写法是 `match &mut self.status { .. }`,两种都对,建议两种都试一次,
//         看看绑定出来的 reason 类型有没有区别。

// 在这里写 Stage 4:

// ---------------------------------------------------------------------------
// Stage 5 · classify:match 的主场 (🌟🌟🌟)
// ---------------------------------------------------------------------------
// 写一个**自由函数**(不在 impl 里):
//   `classify(order: &Order) -> String`
// 按状态返回:
//   Pending                            → "等待成交"
//   Filled 且 quantity >= 100.0        → "大额成交 @ {execution_price}"
//   Filled 其它情况                     → "成交 @ {execution_price}"
//   Canceled                           → "已撤单:{reason}"
//
//   提示:四条规则,但只有三个变体 —— 多出来的那条要靠**守卫 guard**(`if 条件`)。
//   提示:守卫那一支必须写在普通 Filled 那一支**前面**。想想 Day 5 那个 `o => {}`
//         吞掉后续分支的教训:match 是自上而下第一个匹配上的赢。
//   提示:order 是 `&Order`,所以 `match &order.status`。这样绑定出来的
//         `quantity` 是 `&f64` —— 和 100.0 比较时需要解引用。
//   提示:第三条用 `..` 忽略掉不关心的字段,别把 quantity 也写出来。

// 在这里写 Stage 5:

// ---------------------------------------------------------------------------
// Stage 6 · size_bucket:`@` 绑定 + 范围模式 (🌟🌟)
// ---------------------------------------------------------------------------
// 写一个自由函数 `size_bucket(q: u32) -> String`:
//   1..=10    → "小单(q)"
//   11..=100  → "中单(q)"
//   其它       → "大单(q)"
// 例如 size_bucket(5) == "小单(5)"。
//
//   提示:5.5 笔记第 3 条 —— `n @ 1..=10` 既检查范围,又把值绑给 n。
//   提示:最后一支不能用 `_`,因为你还要把那个数字打印出来。用一个绑定名代替。

// 在这里写 Stage 6:

// ---------------------------------------------------------------------------
// Stage 7 · trait + 动态分发 (🌟🌟🌟)
// ---------------------------------------------------------------------------
// 7a. 定义 `trait Describe { fn describe(&self) -> String; }`
//
// 7b. 为 `Order` 实现:格式是 `"#1 Buy 5 @ 100"`
//     也就是 `#{id} {side:?} {quantity} @ {price}`。
//
// 7c. 再定义一个和订单毫无关系的 `struct Account { owner: String, balance: f64 }`,
//     也为它实现 `Describe`,格式 `"账户 Roddy 余额 1000"`。
//     这一步的意义:证明 trait 连接的是**行为**,不是共同的父类或数据。
//
// 7d. 写 `print_all(items: &[&dyn Describe])`,遍历打印每个 `describe()`,
//     每行前面缩进两个空格。
//     ⚠️ 这里必须是 `&dyn`,不能是 `&impl`。先自己想为什么,再往下看:
//        因为 main 里会传 `&[&o1, &o2, &acct]` —— 一个订单和一个账户混在同一个切片里。
//        `impl Trait` 是静态分发,编译期要为**某一个**具体类型单态化,
//        而切片要求所有元素同类型。异质集合只能走 vtable。
//        (可以试着改成 `&[&impl Describe]` 看编译器怎么说,再改回来。)

// 在这里写 Stage 7:

// ---------------------------------------------------------------------------
// Stage 8 · settle:while let + if let (🌟🌟)
// ---------------------------------------------------------------------------
// 写 `settle(queue: &mut Vec<Order>) -> f64`:
//   把队列里的订单一个个弹出来,**只有 Filled 的**才计入总额
//   (总额累加 execution_price * quantity),其余跳过。函数返回时队列必须是空的。
//
//   提示:`Vec::pop()` 返回 `Option<Order>` —— 配 `while let Some(x) = ...` 正好。
//   提示:只关心一种变体、其它都不管,用 `if let` 比 match 干净。
//   提示:order 是 pop 出来的所有权值(不是引用),所以这里可以直接解构,
//         不需要 ref。和 Stage 4 对照着体会一下区别。

// 在这里写 Stage 8:

// ---------------------------------------------------------------------------
// Stage 9 · serde 派生:让类型自己会读写 JSON (🌟🌟)
// ---------------------------------------------------------------------------
// 规矩:**数据进出一律走 serde,不手写字符串解析器。**
// 依赖已经写进 Cargo.toml 了(serde + serde_json),你只需要在文件顶部加:
//
//   use serde::{Deserialize, Serialize};
//   use serde_json::json;
//
// 然后给 `Side`、`OrderStatus`、`Order` 三个类型都加上
// `Serialize, Deserialize` 两个 derive。
//
//   提示:和 Day 6 的 `#[derive(Debug)]` 是同一个机制 —— 过程宏,编译期生成代码,
//         零运行时开销。区别只是这两个 trait 来自 serde 而不是 std。
//   提示:derive 可以并排写:`#[derive(Debug, Clone, Serialize, Deserialize)]`。
//   提示:`Side` 已经是 Copy 了,加这两个不影响。
//
// 先自己猜一下:一个带载荷的 enum 变体,serde 默认会序列化成什么形状?
// 是 `"Filled"` 这样的裸字符串,还是套一层?写完之后用下面这行验:
//   println!("{}", serde_json::to_string(&some_order).unwrap());

// 在这里写 Stage 9(在上面各个类型的 derive 里补,不用新增代码块):

// ---------------------------------------------------------------------------
// Stage 10 · json! 宏:造数据,然后解析回来 (🌟🌟)
// ---------------------------------------------------------------------------
// 这一步不用写新函数,直接看 main 里 Stage 9-10 那段断言 —— 你要让它跑通。
// 需要知道的是 **serde 默认的枚举表示法(externally tagged)**:
//
//   OrderStatus::Pending                      → "Pending"                  裸字符串
//   OrderStatus::Filled { .. }                → { "Filled": { .. } }       套一层
//   OrderStatus::Canceled { reason }          → { "Canceled": { "reason": .. } }
//   Side::Buy                                 → "Buy"
//
//   提示:单元变体没有载荷,所以只需要一个名字;带载荷的变体要用「变体名 → 载荷」
//         这一层来告诉解析器该按哪个变体读。这正是 Day 5 说的 tag —— 只不过
//         内存里那个 tag 是一个字节,JSON 里它是一个 key。
//   提示:`json!({ .. })` 里写的是 JSON 字面量,不是 Rust 结构体,所以 key 要加引号。
//   提示:`serde_json::from_value::<Order>(v)` 从 `Value` 转;
//         `serde_json::from_str::<Order>(s)` 从字符串转。Stage 10 用前者。

// 在这里写 Stage 10(如果 Stage 9 做对了,这一步可能一行都不用写):

// ---------------------------------------------------------------------------
// Stage 11 · load_feed:解析一整段订单流,并处理失败 (🌟🌟🌟)
// ---------------------------------------------------------------------------
// 写一个自由函数:
//   `load_feed(raw: &str) -> Result<Vec<Order>, serde_json::Error>`
// 把一段 JSON 数组解析成 `Vec<Order>`。解析失败要**返回 Err,不能 panic**。
//
//   提示:函数体可以只有一行。`serde_json::from_str` 本身就返回 `Result`,
//         而且它对 `Vec<T>` 是现成支持的 —— 你不需要自己遍历数组。
//   提示:返回类型已经和它对上了,所以直接 return 它的结果即可。想练 `?` 的话
//         写成 `Ok(serde_json::from_str(raw)?)` 也对,想想这两种写法差在哪。
//   提示:这就是 Day 5 说的 Result 的用处 —— 失败要带原因。对比一下:
//         如果这里返回 `Option<Vec<Order>>`,你会丢掉什么信息?
//
// 验收里有两条坏数据,报错分别长这样(自己跑一遍核对):
//   {"id":"不是数字"}   → invalid type: string "...", expected u32 at line .. column ..
//   "side":"Hold"       → unknown variant `Hold`, expected `Buy` or `Sell`
//
//   ⚠️ 注意第二条:**枚举的穷尽性一路延伸到了数据边界。** Rust 里 match 不许你漏掉
//      变体,serde 则不许外部数据编造一个不存在的变体。这是同一个想法的两端。

// 在这里写 Stage 11:

// ============================================================================
// 验收:每写完一个 Stage,取消注释对应的一段。全部放开且跑通 = 完成。
// ============================================================================

fn main() {
    println!("=== 迷你订单引擎 ===");
    println!("从 Stage 1 开始写,每完成一个 Stage 就取消注释下面对应的一段。\n");

    // ---- Stage 1 + 2 + 3 ----
    // let mut o1 = Order::new(1, Side::Buy, 100.0, 5.0);
    // assert_eq!(o1.notional(), 500.0);
    // println!("✅ Stage 1-3:建单与 notional");

    // ---- Stage 5(先写 classify,才能验 fill / cancel)----
    // assert_eq!(classify(&o1), "等待成交");
    // o1.fill(101.0, 5.0);
    // assert_eq!(classify(&o1), "成交 @ 101");
    //
    // let mut o2 = Order::new(2, Side::Sell, 50.0, 200.0);
    // o2.fill(49.5, 200.0);
    // assert_eq!(classify(&o2), "大额成交 @ 49.5");   // ← 守卫这一支
    //
    // let mut o3 = Order::new(3, Side::Buy, 10.0, 1.0);
    // o3.cancel("余额不足");
    // assert_eq!(classify(&o3), "已撤单:余额不足");
    // println!("✅ Stage 5:classify 四条规则");

    // ---- Stage 4 ----
    // o3.annotate_cancel("(已通知用户)");
    // assert_eq!(classify(&o3), "已撤单:余额不足(已通知用户)");
    // // 对非 Canceled 的订单调用它应该什么也不发生:
    // let before = classify(&o1);
    // o1.annotate_cancel("(不该出现)");
    // assert_eq!(classify(&o1), before);
    // println!("✅ Stage 4:ref mut 原地追加");

    // ---- Stage 6 ----
    // assert_eq!(size_bucket(5), "小单(5)");
    // assert_eq!(size_bucket(50), "中单(50)");
    // assert_eq!(size_bucket(500), "大单(500)");
    // assert_eq!(size_bucket(10), "小单(10)");    // ← 边界:1..=10 是闭区间
    // assert_eq!(size_bucket(11), "中单(11)");
    // println!("✅ Stage 6:@ 绑定与范围模式");

    // ---- Stage 7 ----
    // let acct = Account { owner: String::from("Roddy"), balance: 1000.0 };
    // assert_eq!(o1.describe(), "#1 Buy 5 @ 100");
    // assert_eq!(acct.describe(), "账户 Roddy 余额 1000");
    // println!("Describe 输出:");
    // print_all(&[&o1, &o2, &acct]);          // ← 异质切片,这就是要 dyn 的原因
    // println!("✅ Stage 7:trait 与动态分发");

    // ---- Stage 8 ----
    // let mut queue = vec![o1.clone(), o2.clone(), o3.clone()];
    // let total = settle(&mut queue);
    // assert_eq!(total, 101.0 * 5.0 + 49.5 * 200.0);   // o3 被撤单,不计入
    // assert!(queue.is_empty(), "settle 之后队列必须清空");
    // println!("结算总额 = {}", total);
    // println!("✅ Stage 8:while let + if let");

    // ---- Stage 9 + 10 ----
    // let raw = json!({
    //     "id": 7, "side": "Sell", "price": 20.0, "quantity": 3.0,
    //     "status": { "Filled": { "execution_price": 19.5, "quantity": 3.0 } }
    // });
    // let parsed: Order = serde_json::from_value(raw).expect("这条应该能解析");
    // assert_eq!(parsed.id, 7);
    // assert_eq!(parsed.side, Side::Sell);
    // assert_eq!(classify(&parsed), "成交 @ 19.5");
    // // 再转回去,确认往返一致
    // let back = serde_json::to_string(&parsed).unwrap();
    // assert!(back.contains("\"Filled\""), "结构体变体应该是 {{\"Filled\": {{..}}}}");
    // println!("✅ Stage 9-10:derive + json! + 往返");

    // ---- Stage 11 ----
    // let feed = r#"[
    //     {"id":1,"side":"Buy","price":100.0,"quantity":5.0,"status":"Pending"},
    //     {"id":2,"side":"Sell","price":50.0,"quantity":200.0,
    //      "status":{"Filled":{"execution_price":49.5,"quantity":200.0}}}
    // ]"#;
    // let mut orders = load_feed(feed).expect("这段 feed 是合法的");
    // assert_eq!(orders.len(), 2);
    // assert_eq!(classify(&orders[0]), "等待成交");
    // assert_eq!(classify(&orders[1]), "大额成交 @ 49.5");
    // assert_eq!(settle(&mut orders), 49.5 * 200.0);   // 只有第二单成交了
    //
    // // 失败路径:两种坏数据,都要走 Err 而不是 panic
    // assert!(load_feed(r#"[{"id":"不是数字"}]"#).is_err());
    // assert!(load_feed(r#"[{"id":1,"side":"Hold","price":1.0,"quantity":1.0,"status":"Pending"}]"#).is_err());
    // println!("坏数据的报错:{}", load_feed(r#"[{"id":"x"}]"#).unwrap_err());
    // println!("✅ Stage 11:解析 feed 与失败路径");

    // ---- 全部完成 ----
    // println!("\n🎉 项目跑通了。");
}

// ============================================================================
// 全部写完之后,回答这四个问题(答案写进 day5.5/NOTES.md 的「Questions I asked」):
//
// 1. Stage 1 里 `Side` 能 derive Copy 而 `OrderStatus` 不能。是哪一个字段决定的?
//    这和 Day 4 「数组是不是 Copy 由元素类型决定」是同一条规则吗?
//
// 2. Stage 4 如果不用 ref / &mut,编译器报的是哪个错?把错误码抄下来。
//    它和 Day 6 `..user1` 那个部分移动的错误是同一个吗?
//
// 3. Stage 5 里如果把守卫那一支挪到普通 Filled 那一支**后面**,程序会怎样?
//    是编译错误、警告,还是静默地跑出错误结果?先猜,再动手验。
//
// 4. Stage 7 里如果把 `&[&dyn Describe]` 改成 `&[&impl Describe]`,
//    编译器说什么?用这条报错解释一遍静态分发和动态分发的取舍。
//
// 5. Stage 9 里 `OrderStatus::Pending` 序列化成裸字符串 `"Pending"`,而
//    `Filled { .. }` 序列化成 `{"Filled": {..}}`。为什么两者形状不一样?
//    这和 Day 5 内存布局那节「单元变体只要一个 tag,带载荷的要 tag + 载荷」
//    是不是同一件事?
//
// 6. Stage 11 喂进一个 `"side":"Hold"`,serde 报 `unknown variant`。
//    把它和 Day 5 的「match 必须穷尽」放在一起想:
//    编译期的穷尽性检查和运行期的反序列化校验,守的是不是同一条边界?
// ============================================================================
