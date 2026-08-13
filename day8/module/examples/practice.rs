// Practice — Day 8:模块系统
// Run with: cargo run --example practice
//
// 覆盖范围(全部对应 src/main.rs 里的编号节):
//   1-3    package / crate / module 三个层级、默认私有、pub 的三种强度
//   4-8    绝对路径与相对路径、self 与 super、use、as 重命名、嵌套模块
//   9-11   as 的类型转换身份、函数内定义、workspace
//
// 规则和以前一样:**这个文件必须始终能编译**。
// 你要写的代码写在标好的空槽里;每写完一题,就把 main 里对应那一段验收取消注释,
// 跑 `cargo run --example practice`。全部取消注释且跑通 = 今天的题做完了。
//
// 评级:🌟 照着提示就能写   🌟🌟 需要想一下   🌟🌟🌟 卡住了回去翻 src/main.rs
// 参考(卡住时再看,别提前翻):
//   https://practice-rust-zh.beatai.org/crate-module/module.html
//   https://kaisery.github.io/trpl-zh-cn/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

// 答案槽还空着的时候,下面那些模块里的函数一个都没被调用,所以全是 dead_code 警告。
// 等你把 main 里的验收逐条取消注释,它们就名副其实了 —— 到时候可以把这一行删掉。
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Exercise 1 · 让它公开 (🌟)   —— 对应第 2 节
// ---------------------------------------------------------------------------
// 下面这个模块一行都调不动。只加 `pub`(不许改结构、不许改函数名),
// 让 main 里的 `shop::price()` 能跑起来。
//
//   提示:模块本身要 pub 吗?模块里的函数要 pub 吗?两个都要还是只要一个?
//   自问:如果只给 `mod` 加 pub、不给 `fn` 加,报的是哪个错?先猜再试。

mod shop {
    fn price() -> u32 {
        42
    }

    // 这个别动 —— Exercise 5 要用它。
    #[allow(dead_code)]
    fn cost() -> u32 {
        20
    }
}

// ---------------------------------------------------------------------------
// Exercise 2 · 同一个调用,三条路径 (🌟🌟)   —— 对应第 4、5 节
// ---------------------------------------------------------------------------
// 在下面的 `deep::inner` 里补三个函数,它们都要调到 `deep::helper()`,
// 但**分别用三种不同的路径写法**:
//   `via_crate()`  用绝对路径 `crate::...`
//   `via_super()`  用 `super::...`
//   `via_use()`    在函数外先 `use super::helper;`,函数里直接写 `helper()`
//
//   提示:这个文件是 example,它的 crate 根就是这个文件本身,所以绝对路径从
//         `crate::deep::helper` 开始数。
//   自问:三种写法编译出来的机器码有区别吗?—— 想清楚这个,你就知道该按什么标准选了。

pub mod deep {
    pub fn helper() -> &'static str {
        "helper 被调到了"
    }

    pub mod inner {
        // 在这里写 Exercise 2 的三个函数:
    }
}

// ---------------------------------------------------------------------------
// Exercise 3 · 两个同名函数,一个作用域 (🌟🌟)   —— 对应第 7 节
// ---------------------------------------------------------------------------
// `left::name()` 和 `right::name()` 名字一样。
// 写一个函数 `fn both() -> String`,返回 `"L+R"` —— 要求函数体里**两个都用 use 拉进来**,
// 调用时直接写短名字,不许写 `left::name()` 这种全路径。
//
//   提示:`use ... as ...`。
//   提示:`use` 是可以写在函数体里的,作用域就只有那个函数。
//   自问:如果不用 as,直接 `use left::name; use right::name;` 会报什么错?

mod left {
    pub fn name() -> &'static str {
        "L"
    }
}

mod right {
    pub fn name() -> &'static str {
        "R"
    }
}

// 在这里写 Exercise 3:

// ---------------------------------------------------------------------------
// Exercise 4 · 开口开多大 (🌟🌟)   —— 对应第 3 节
// ---------------------------------------------------------------------------
// 下面四行,**先预测哪几行能编译**,写在注释里,然后逐行取消注释验证。
//
//     outer::pub_fn();              // 预测:____
//     outer::crate_fn();            // 预测:____
//     outer::inner::deep_fn();      // 预测:____
//     outer::inner::restricted();   // 预测:____
//
//   提示:`pub(in crate::outer)` 的意思是「只在 crate::outer 这棵子树里公开」。
//   提示:main 所在的位置是 crate 根,不在 outer 里面。
//   自问:`pub(crate)` 和 `pub` 在这个 example 里表现一样吗?什么情况下才看得出区别?

pub mod outer {
    pub fn pub_fn() -> &'static str {
        "pub"
    }

    pub(crate) fn crate_fn() -> &'static str {
        "pub(crate)"
    }

    pub mod inner {
        pub fn deep_fn() -> &'static str {
            "deep"
        }

        pub(in crate::outer) fn restricted() -> &'static str {
            "restricted"
        }
    }

    // 证明 outer 自己够得着 restricted
    pub fn reach_restricted() -> &'static str {
        inner::restricted()
    }
}

// ---------------------------------------------------------------------------
// Exercise 5 · pub 不会自动往里传 (🌟)   —— 对应第 2、8 节
// ---------------------------------------------------------------------------
// 回答两个问题(写在注释里就行,不用写代码):
//   a. Exercise 1 里的 `shop::cost()` 就算你给 `mod shop` 加了 `pub`,
//      在 main 里也调不到。为什么?
//   b. 一句话总结:`pub mod` 公开的到底是什么?
//
//   提示:模块的 pub 和模块里每一项的 pub 是**两件独立的事**。
//         `pub mod` 只是把「门」打开,门里每样东西还有自己的门。

// Exercise 5 不用写代码,答案写在上面的注释里。

// ---------------------------------------------------------------------------
// Exercise 6 · 把 main 里的东西搬进模块 (🌟🌟🌟)   —— 今天的实战题
// ---------------------------------------------------------------------------
// 这是 Day 7 第 12 节 word_count 的代码,现在整坨堆在一起。
// 把它重构成一个模块 `text`,要求:
//   - 对外只公开一个 `pub fn word_count(&str) -> HashMap<String, usize>`
//   - `normalize`(把词转小写、去掉标点)必须是模块内**私有**的辅助函数
//   - main 里只写 `text::word_count(...)`,碰不到 normalize
//
//     fn word_count(t: &str) -> HashMap<String, usize> {
//         let mut m = HashMap::new();
//         for w in t.split_whitespace() {
//             let w = w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
//             if w.is_empty() { continue; }
//             *m.entry(w).or_insert(0) += 1;
//         }
//         m
//     }
//
//   提示:`use std::collections::HashMap;` 写在模块**里面**,不要写在文件顶上 ——
//         想想这两种写法的区别是什么。
//   提示:`"Hello, hello world"` 应该得到 `{hello: 2, world: 1}`。
//   自问:normalize 设成私有,对**调用方**有什么实际好处?
//         —— 这一条就是后面写库时「公共 API 边界」的雏形,值得认真答。

// 在这里写 Exercise 6:

fn main() {
    println!("=== Day 8 practice ===");

    // ---- Exercise 1 ----
    // assert_eq!(shop::price(), 42);
    // println!("Exercise 1 ✅  price = {}", shop::price());

    // ---- Exercise 2 ----
    // assert_eq!(deep::inner::via_crate(), "helper 被调到了");
    // assert_eq!(deep::inner::via_super(), "helper 被调到了");
    // assert_eq!(deep::inner::via_use(), "helper 被调到了");
    // println!("Exercise 2 ✅  三条路径同一个结果");

    // ---- Exercise 3 ----
    // assert_eq!(both(), "L+R");
    // println!("Exercise 3 ✅  {}", both());

    // ---- Exercise 4 ----
    // println!("Exercise 4 · {}", outer::pub_fn());
    // println!("Exercise 4 · {}", outer::crate_fn());
    // println!("Exercise 4 · {}", outer::inner::deep_fn());
    // println!("Exercise 4 · {}", outer::inner::restricted());   // 这行呢?
    // println!("Exercise 4 · 绕一下:{}", outer::reach_restricted());
    //       预测对了吗?

    // ---- Exercise 5 ----
    // 不用写代码,答案写在题目注释里。

    // ---- Exercise 6 ----
    // let wc = text::word_count("Hello, hello world");
    // assert_eq!(wc.get("hello"), Some(&2));
    // assert_eq!(wc.get("world"), Some(&1));
    // println!("Exercise 6 ✅  {:?}", wc);
    // text::normalize("x");   // 取消注释:应该报错 —— 这正是这题要的

    println!("--- done ---");
}
