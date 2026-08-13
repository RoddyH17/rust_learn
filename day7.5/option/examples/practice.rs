// Practice — Day 7.5:Option 的泛型逻辑与错误处理
// Run with: cargo run --example practice
//
// 覆盖范围(全部对应 src/main.rs 里的编号节):
//   1-3    Option:引入规则 Some/None、消去规则 match、unwrap 的代价
//   4-8    错误:可恢复 vs 不可恢复、越界即 panic、match Result、map、自定义错误 enum
//   9-11   ? 传播、Option ↔ Result 互转、泛型四层职责
//
// 规则和以前一样:**这个文件必须始终能编译**。
// 你要写的代码写在标好的空槽里;每写完一题,就把 main 里对应那一段验收取消注释,
// 跑 `cargo run --example practice`。全部取消注释且跑通 = 今天的题做完了。
//
// 评级:🌟 照着提示就能写   🌟🌟 需要想一下   🌟🌟🌟 卡住了回去翻 src/main.rs
// 参考(卡住时再看,别提前翻):
//   https://practice-rust-zh.beatai.org/compound-types/enum.html
//   https://practice-rust-zh.beatai.org/result-error/result.html

// Exercise 4 要用。答案槽还空着的时候它是「没用到」的,先按住这个警告。
#[allow(unused_imports)]
use std::num::ParseIntError;

// ---------------------------------------------------------------------------
// Exercise 1 · 不许 unwrap (🌟)   —— 对应第 2、3 节
// ---------------------------------------------------------------------------
// 写一个函数 `describe(x: Option<i32>) -> String`:
//   Some(n) → 返回 format!("有值 {}", n)
//   None    → 返回 String::from("空")
//
// 铁律:**函数体里不许出现 unwrap 或 expect**。这题要的就是把两种形态都摊开写。
//   提示:`match x { Some(n) => ..., None => ... }`,两个分支都要返回 String。
//   自问:为什么 `Some(n)` 里的 n 不用写类型?

// 在这里写 Exercise 1:


// ---------------------------------------------------------------------------
// Exercise 2 · 把「没有」升级成「为什么没有」 (🌟🌟)   —— 对应第 10 节
// ---------------------------------------------------------------------------
// 写一个函数 `nth(arr: &[i32], i: usize) -> Result<i32, String>`:
//   下标存在  → Ok(那个值)
//   下标不存在 → Err(format!("下标 {} 越界,长度只有 {}", i, arr.len()))
//
//   提示:`arr.get(i)` 给的是 `Option<&i32>`,而你要返回的是 `Result<i32, _>` ——
//         中间差了两步:一是 Option → Result,二是 &i32 → i32。
//   提示:ok_or 收的是「错误值」,ok_or_else 收的是「造错误值的闭包」。
//         错误信息要 format! 出来时,用哪个更省?
//   自问:第 10 节的 first() 返回的是 `Result<&i32, _>`,这题要 `Result<i32, _>`。
//         差的那个 & 该在哪一步去掉?

// 在这里写 Exercise 2:


// ---------------------------------------------------------------------------
// Exercise 3 · 错误要能被 match (🌟🌟)   —— 对应第 8 节
// ---------------------------------------------------------------------------
// 3a. 定义 `enum ParseError`,两个变体:`Empty`、`NotANumber(String)`。
//     记得 `#[derive(Debug)]`,不然 println!("{:?}") 用不了。
// 3b. 写 `fn to_int(s: &str) -> Result<i32, ParseError>`:
//     空字符串(去掉首尾空格后为空)→ Err(ParseError::Empty)
//     解析失败                      → Err(ParseError::NotANumber(s.to_string()))
//     成功                          → Ok(数字)
//
//   提示:`s.trim().is_empty()` 判空;`s.trim().parse::<i32>()` 返回 Result。
//   提示:parse 的错误类型是 ParseIntError,不是你的 ParseError —— 需要换一层。
//         `.map_err(|_| ParseError::NotANumber(...))` 只改错误那一侧,成功侧不动。
//   自问:为什么不干脆用 `Result<i32, String>`?
//         —— 试着写出「只处理 Empty、其他原样往上抛」的代码,你就知道差别了。

// 在这里写 Exercise 3:


// ---------------------------------------------------------------------------
// Exercise 4 · 把 match 链压成 ? (🌟🌟)   —— 对应第 9 节
// ---------------------------------------------------------------------------
// 下面这段能跑,但很啰嗦。用 `?` 把它改写成三行以内,行为完全一样。
//
//     fn sum_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
//         let x = match a.trim().parse::<i32>() {
//             Ok(v) => v,
//             Err(e) => return Err(e),
//         };
//         let y = match b.trim().parse::<i32>() {
//             Ok(v) => v,
//             Err(e) => return Err(e),
//         };
//         Ok(x + y)
//     }
//
//   提示:`?` 干的就是上面那个 match —— 成功取值,失败提前 return。
//   提示:`?` 只能写在返回 Result(或 Option)的函数里。想想为什么 —— 它要 return,
//         就得知道该 return 成什么形状。
//   自问:把返回类型改成 `i32`(不是 Result),`?` 会报什么错?先猜再试。

// 在这里写 Exercise 4:


// ---------------------------------------------------------------------------
// Exercise 5 · 先预测,再运行 (🌟)   —— 对应第 7 节
// ---------------------------------------------------------------------------
// 把下面三个空填上你的**预测**,然后取消 main 里的验证代码,看你猜对没有。
//
//     let a: Result<i32, String> = Ok(3);
//     let b: Result<i32, String> = Err(String::from("boom"));
//
//     a.map(|v| v * 10)   →  ____________
//     b.map(|v| v * 10)   →  ____________
//     b.map_err(|e| e.len())  →  ____________
//
//   提示:map 只作用在成功那一侧。那 Err 里的值经过 map 之后变了吗?
//   自问:如果 map 对 Err 也生效,`Result` 还能表达「失败」这件事吗?

// Exercise 5 不用写代码,预测写在上面的注释里。


// ---------------------------------------------------------------------------
// Exercise 6 · 一个坏的就整批作废 (🌟🌟🌟)   —— 第 7 + 9 节的实战版
// ---------------------------------------------------------------------------
// 写 `fn parse_all(line: &str) -> Result<Vec<i32>, String>`:
//   `"1 2 3"`   → Ok(vec![1, 2, 3])
//   `"1 x 3"`   → Err(String::from("x 不是数字"))     ← 整体失败,不是跳过
//
// 注意和第 7 节的区别:第 7 节是「坏的跳过,好的继续」,这题是「有一个坏的就全盘失败」。
// 这两种语义在真实系统里天天要选 —— 行情解析通常跳过,订单解析必须整体失败。
//
//   提示一(笨办法,先写通这个):for 循环 + 一个 Vec,遇到 Err 就 return Err。
//   提示二(漂亮办法):`Result` 实现了一件很反直觉的事 ——
//         `Vec<Result<T, E>>` 可以 collect 成 `Result<Vec<T>, E>`。
//         也就是说 `.map(...).collect::<Result<Vec<i32>, String>>()` 直接就是答案。
//         第一个 Err 会短路掉整个 collect。
//   提示三:`s.split_whitespace()` 给你一个个的词。
//   自问:漂亮办法里,如果有两个坏词,返回的是哪一个的错误?为什么?

// 在这里写 Exercise 6:


fn main() {
    println!("=== Day 7.5 practice ===");

    // ---- Exercise 1 ----
    // assert_eq!(describe(Some(7)), "有值 7");
    // assert_eq!(describe(None), "空");
    // println!("Exercise 1 ✅  {} / {}", describe(Some(7)), describe(None));

    // ---- Exercise 2 ----
    // let arr = [10, 20, 30];
    // assert_eq!(nth(&arr, 1), Ok(20));
    // assert!(nth(&arr, 99).is_err());
    // println!("Exercise 2 ✅  {:?} / {:?}", nth(&arr, 1), nth(&arr, 99));

    // ---- Exercise 3 ----
    // println!("Exercise 3 ·  {:?}", to_int("42"));
    // println!("Exercise 3 ·  {:?}", to_int("   "));
    // println!("Exercise 3 ·  {:?}", to_int("abc"));
    // assert_eq!(to_int(" 42 ").unwrap(), 42);
    // assert!(matches!(to_int(""), Err(ParseError::Empty)));
    // assert!(matches!(to_int("abc"), Err(ParseError::NotANumber(_))));
    // println!("Exercise 3 ✅");

    // ---- Exercise 4 ----
    // assert_eq!(sum_two("3", " 4 "), Ok(7));
    // assert!(sum_two("3", "x").is_err());
    // println!("Exercise 4 ✅  {:?}", sum_two("3", " 4 "));

    // ---- Exercise 5 ----
    // let a: Result<i32, String> = Ok(3);
    // let b: Result<i32, String> = Err(String::from("boom"));
    // println!("Exercise 5 · a.map      = {:?}", a.clone().map(|v| v * 10));
    // println!("Exercise 5 · b.map      = {:?}", b.clone().map(|v| v * 10));
    // println!("Exercise 5 · b.map_err  = {:?}", b.clone().map_err(|e| e.len()));
    // println!("      预测对了吗?");

    // ---- Exercise 6 ----
    // assert_eq!(parse_all("1 2 3"), Ok(vec![1, 2, 3]));
    // assert!(parse_all("1 x 3").is_err());
    // println!("Exercise 6 ✅  {:?} / {:?}", parse_all("1 2 3"), parse_all("1 x 3"));

    println!("--- done ---");
}
