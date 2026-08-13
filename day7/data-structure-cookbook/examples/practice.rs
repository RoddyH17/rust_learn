// Practice — Day 7:Vec 与 HashMap
// Run with: cargo run --example practice
//
// 覆盖范围(全部对应 src/main.rs 里的编号节):
//   1-6   Vec:创建、push、索引 vs get、遍历、用 enum 装多类型、capacity
//   7-12  HashMap:哈希、insert 的所有权、覆盖更新、get、遍历、合并
//
// 规则和以前一样:**这个文件必须始终能编译**。
// 你要写的代码写在标好的空槽里;每写完一题,就把 main 里对应那一段验收取消注释,
// 跑 `cargo run --example practice`。全部取消注释且跑通 = 今天的题做完了。
//
// 评级:🌟 照着提示就能写   🌟🌟 需要想一下   🌟🌟🌟 卡住了回去翻 src/main.rs
// 参考(卡住时再看,别提前翻):
//   https://practice-rust-zh.beatai.org/collections/vector.html
//   https://practice-rust-zh.beatai.org/collections/hashmap.html

// Exercise 2 和 Exercise 6 要用。答案槽还空着的时候它是「没用到」的,先按住这个警告 ——
// 等你把题写完,这一行就名副其实了,到时候可以把 allow 删掉。
#[allow(unused_imports)]
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Exercise 1 · 越界了也不要崩 (🌟)   —— 对应第 3 节
// ---------------------------------------------------------------------------
// 写一个函数 `describe(v: &Vec<i32>, idx: usize) -> String`:
//   下标存在  → 返回 format!("v[{}] = {}", idx, 值)
//   下标不存在 → 返回 String::from("越界")
//
// 铁律:**不许用 `v[idx]`**。越界时它会 panic,而这题要的就是不 panic。
//   提示:`v.get(idx)` 给你的是 `Option<&i32>`,match 它。
//   提示:两个分支都要返回 String,类型才对得上。

// 在这里写 Exercise 1:


// ---------------------------------------------------------------------------
// Exercise 2 · 合并,但先来的说了算 (🌟🌟)   —— 对应第 12 节
// ---------------------------------------------------------------------------
// 写一个函数 `merge_keep(base: &HashMap<String, i32>, extra: &HashMap<String, i32>)
//                        -> HashMap<String, i32>`
// 规则:两个 map 都有的 key,**保留 base 的值**;只有 extra 有的 key,补进来。
//
//   提示:先 clone 一份 base 当结果,再遍历 extra。
//   提示:整道题的关键就一行,在 src/main.rs 第 12 节里。
//   注意:`out.entry(k.clone());` 这样写编译能过、也不报警、然后什么都不会发生。
//         entry() 只是把「那个位置」交给你,你得接着说要拿它干什么。

// 在这里写 Exercise 2:


// ---------------------------------------------------------------------------
// Exercise 3 · insert 之后它去哪了 (🌟🌟)   —— 对应第 8 节
// ---------------------------------------------------------------------------
// 下面这段是坏的。先**别急着改**,回答两个问题(写在注释里就行):
//   a. 报的是哪个错误号?哪一行触发的?
//   b. `name` 到底发生了什么 —— 是被复制了,还是被搬走了?
// 然后用**两种**不同的方式让它跑通,并说清两种的代价差在哪。
//
//     let name = String::from("blue");
//     let mut m: HashMap<String, i32> = HashMap::new();
//     m.insert(name, 10);
//     println!("{} 的分数是 {:?}", name, m.get("blue"));
//
//   提示一:一种是让 map 拿一份自己的。
//   提示二:另一种是调整顺序 —— 想想 println! 那行到底需不需要 `name` 还活着。
//   提示三:如果 key 的类型换成 `&str` 而不是 `String`,这个错误还会出现吗?为什么?

// 在这里写 Exercise 3(改好的版本 + 你的回答):


// ---------------------------------------------------------------------------
// Exercise 4 · 一列里放三种东西 (🌟🌟)   —— 对应第 5 节
// ---------------------------------------------------------------------------
// 一张表格的某一列,格子里可能是整数、小数或文字。
//
// 4a. 定义 `enum Cell`,三个变体:`Int(i64)`、`Float(f64)`、`Text(String)`。
// 4b. 写 `fn sum_numeric(col: &Vec<Cell>) -> f64`:
//     把这一列里所有数字加起来,`Text` 直接跳过。`Int` 要转成 f64 再加。
//
//   提示:Vec 要求元素同类型 —— enum 就是用来把三种东西变成同一个类型的。
//   提示:遍历时 match 三个变体;跳过某个分支就写 `Cell::Text(_) => {}`。
//   提示:`Int(3)` 加进去应该是 3.0,`as f64` 放在哪一步?

// 在这里写 Exercise 4:


// ---------------------------------------------------------------------------
// Exercise 5 · 先预测,再运行 (🌟)   —— 对应第 6 节
// ---------------------------------------------------------------------------
// 把下面四个空填上你的**预测**,然后取消 main 里的验证代码,看你猜对没有。
//
//     let mut v: Vec<i32> = Vec::with_capacity(3);
//     // 此刻   len = ____   capacity = ____
//     v.push(1); v.push(2); v.push(3); v.push(4);
//     // 此刻   len = ____   capacity = ____
//
//   提示:capacity 是「要来的位置」,len 是「真的放了几个」。
//   提示:第四个 push 会发生什么?容量不够时 Vec 不是加一格,想想它为什么那样长。

// Exercise 5 不用写代码,预测写在上面的注释里。


// ---------------------------------------------------------------------------
// Exercise 6 · 数一数每个词出现几次 (🌟🌟🌟)   —— 第 12 节的实战版
// ---------------------------------------------------------------------------
// 写 `fn word_count(text: &str) -> HashMap<String, i32>`,
// 统计每个词出现的次数。`"a b a c a"` 应该得到 `{a: 3, b: 1, c: 1}`。
//
//   提示:`text.split_whitespace()` 给你一个个的词。
//   提示:entry() 后面除了 or_insert,还能干什么?—— `or_insert(0)` 的返回值是
//         `&mut i32`,也就是「map 里那个格子本身」。拿到它之后就能直接往上加。
//   提示:改一个 &mut 指向的值要用 `*`,和第 4 节 `*i += 10` 是同一件事。
//   提示:整个函数体三行就够。

// 在这里写 Exercise 6:


fn main() {
    println!("=== Day 7 practice ===");

    // ---- Exercise 1 ----
    // let v = vec![10, 20, 30];
    // assert_eq!(describe(&v, 1), "v[1] = 20");
    // assert_eq!(describe(&v, 99), "越界");
    // println!("Exercise 1 ✅  {} / {}", describe(&v, 1), describe(&v, 99));

    // ---- Exercise 2 ----
    // let mut base: HashMap<String, i32> = HashMap::new();
    // base.insert(String::from("a"), 1);
    // base.insert(String::from("b"), 2);
    // let mut extra: HashMap<String, i32> = HashMap::new();
    // extra.insert(String::from("b"), 99);
    // extra.insert(String::from("c"), 3);
    // let out = merge_keep(&base, &extra);
    // assert_eq!(out.get("a"), Some(&1));
    // assert_eq!(out.get("b"), Some(&2));   // 保留 base 的,不是 99
    // assert_eq!(out.get("c"), Some(&3));
    // assert_eq!(out.len(), 3);
    // println!("Exercise 2 ✅  {:?}", out);

    // ---- Exercise 3 ----
    // 跑通你改好的那两个版本,各打印一行即可。

    // ---- Exercise 4 ----
    // let col = vec![
    //     Cell::Int(3),
    //     Cell::Text(String::from("小计")),
    //     Cell::Float(1.5),
    //     Cell::Int(10),
    // ];
    // assert_eq!(sum_numeric(&col), 14.5);
    // println!("Exercise 4 ✅  合计 = {}", sum_numeric(&col));

    // ---- Exercise 5 ----
    // let mut v: Vec<i32> = Vec::with_capacity(3);
    // println!("Exercise 5 · 建好:  len = {}, capacity = {}", v.len(), v.capacity());
    // v.push(1); v.push(2); v.push(3); v.push(4);
    // println!("Exercise 5 · 四个后:len = {}, capacity = {}", v.len(), v.capacity());
    // println!("      预测对了吗?");

    // ---- Exercise 6 ----
    // let wc = word_count("a b a c a b a");
    // assert_eq!(wc.get("a"), Some(&4));
    // assert_eq!(wc.get("b"), Some(&2));
    // assert_eq!(wc.get("c"), Some(&1));
    // assert_eq!(wc.get("z"), None);
    // println!("Exercise 6 ✅  {:?}", wc);

    println!("--- done ---");
}
