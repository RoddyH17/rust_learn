//! Day 8 — module
//!
//! 2026-08-13 · 视频进度:包、单元包、模块与路径
//!
//! Package: 包含多个 binary crates (单元), 或者一个库的单元。每一个 package 至少包含一个
//! crate。比如我们使用 `cargo new` 创建出新文件来 —— `cargo new --lib xxx` 创建一个 lib
//! package(主要文件)。
//!
//! Modules (模块): `pub`, `mod`, `use`, `as` ....
//!
//! 今天这一天的主线是**可见性**:模块把代码切开,`pub` 决定切口开多大。
//! 默认全部私有 —— 这是 Rust 的立场:不主动公开的东西,就是实现细节。

// 模块 a 定义在 crate 根上,不放进 fn main —— 因为第 4 节要演示 `crate::` 绝对路径,
// 而函数内部定义的模块是够不着 `crate::` 的。
mod a {
    // 默认私有:模块外面看不见它。
    const NUM: usize = 1;

    // 加入 pub, 能够将私有的 mod 里面公开访问函数, rust 在 mod 里面写函数默认私有制。
    pub fn echo() {
        println!("   a::echo(),顺便读到私有常量 NUM = {}", NUM);
    }

    pub fn log() {
        echo();
        self::echo(); // 同属于 a 模块所以可以直接调用
    }

    // 模块是可以嵌套的
    pub mod b {
        use super::echo; // 使用 super 来访问父级模块

        pub fn log() {
            println!("   a::b::log()");
        }

        // 函数继承 —— 其实不是继承,是把父模块的名字借进来用
        pub fn echo_b() {
            print!("   a::b::echo_b() 转手调用父模块:");
            echo();
        }

        pub mod c {
            pub fn log_c() {
                println!("   a::b::c::log_c(),三层深");
            }
        }
    }

    // pub(in path):只在指定路径内公开。这个模块 a 自己能用,crate 根用不了。
    pub(in crate::a) mod secret {
        pub fn hidden() {
            println!("   a::secret::hidden(),只有 a 自己叫得动");
        }
    }

    // 证明 a 自己确实叫得动 secret
    pub fn call_secret() {
        secret::hidden();
    }
}

fn main() {
    // ---------- 1. Package / Crate / Module:三个层级 ----------
    // Package 是 cargo 管的单位(一个 Cargo.toml 就是一个 package)。
    // Crate 是编译的单位:一个 binary crate 或一个 library crate。
    // Module 是**代码组织**的单位,不产生新文件也不产生新编译单元,只是划分命名空间。
    // 所以模块是纯粹的逻辑切分 —— 写在一个文件里和拆成多个文件,对编译器是一回事。
    println!("1. package(Cargo.toml)⊃ crate(编译单位)⊃ module(命名空间)");

    // ---------- 2. 可见性:默认全部私有 ----------
    // 模块可见性:默认所有私有, 所以我们需要声明出来。
    mod visibility {
        pub fn open() {
            println!("2. open() 被外面调到了");
        }

        #[allow(dead_code)]
        fn closed() {} // 没有 pub,模块外面看不见
    }

    visibility::open();
    // visibility::closed();   // 取消注释:error[E0603]: function `closed` is private
    println!("2. 不写 pub 的东西不是「忘了写」,是「明确表示它是实现细节」");

    // ---------- 3. pub 的三种强度 ----------
    // 1. 前面加一个 pub —— 谁都能看
    // 2. pub(crate) —— 本 crate 内公开,别的 crate 看不到
    // 3. 可以申明一个具体的访问路径 pub(in path) —— 只在指定路径内公开
    mod strength {
        pub fn everyone() {}
        pub(crate) fn crate_only() {}
        // pub(in crate::a) fn only_in_a() {}   // 写在这里没意义,path 必须是自己的祖先
    }
    strength::everyone();
    strength::crate_only();

    a::call_secret();
    // a::secret::hidden();   // 取消注释:error[E0603]: module `secret` is private
    //                        // ↑ 这就是 pub(in crate::a) 的效果:a 自己能用,crate 根不能。
    //                        //   报错里 rustc 会指着 `pub(in crate::a) mod secret` 那一行。
    println!("3. pub / pub(crate) / pub(in path) —— 开口从大到小");

    // ---------- 4. 绝对路径:从 crate 根出发 ----------
    // 调用模块化方法:
    // a.echo() —— 这是错的, 这是调用函数, 而非路径访问, 点只是 struct 的 oop 这样的路径。
    // 正确写法是双冒号,并且如果我们要能公开调用, 我们需要在我们的模块函数里加 pub。
    print!("4. crate::a::echo() → ");
    crate::a::echo(); // 绝对路径:从 crate 根开始数

    print!("4. a::echo()        → ");
    a::echo(); // 相对路径:从当前位置开始数,这里当前位置就是 crate 根,所以两者等价

    // ---------- 5. self 与 super ----------
    // self:: 是「当前模块」,super:: 是「父模块」。
    // 模块 a 的 log() 里就同时用了裸调用和 self::echo(),两者完全等价 ——
    // 写 self:: 只是为了让「这是本模块的东西」更醒目。
    // 模块 b 的 `use super::echo` 则是往上一层取名字。
    print!("5. ");
    a::log();
    a::b::echo_b();

    // ---------- 6. use:把路径拉进作用域 ----------
    // 如果我们现在想直接访问 b 模块里的方法, 我们要怎么做?
    // b::log() 这样是不行的, 我们要先通过 a 来访问 b, 也就是要写成 a::b::log()。
    // rust 很聪明, 它可以抽象 namespace —— use 把长路径的最后一截拉到当前作用域里。
    use a::b::log;
    print!("6. use 之后直接写 log() → ");
    log();

    // use 拉进来的是**名字**,不是代码。没有任何运行时开销,只是省了打字。

    // ---------- 7. 花括号合并与 as 重命名 ----------
    // 两个都叫 log,同一个作用域装不下 —— 用 as 给其中一个改名。
    use a::{b::log as log2, log as log_a};
    print!("7. log_a() → ");
    log_a();
    print!("7. log2()  → ");
    log2();
    // 花括号只是省写:a::{b::log, log} 等价于分开写两行 use。

    // ---------- 8. 嵌套模块与深路径 ----------
    // 模块可以一层套一层,路径就跟着一节一节加。
    print!("8. ");
    a::b::c::log_c();
    println!("8. 路径有多深,取决于你把代码切了多细 —— 切得细好找,切得粗好写");

    // ---------- 9. as 的另一个身份:类型转换 ----------
    // as 在 use 里是重命名,在表达式里是类型转换。同一个关键字,两件事。
    let x: u8 = 1;
    println!("9. add(x as usize, 1) = {}", add(x as usize, 1));

    // ---------- 10. 函数里也能定义函数 ----------
    // 上面那个 add 就定义在 main 内部 —— 它的作用域只有 main,外面看不见。
    // 这是「最小可见性」的极端形态:连模块都不用建,直接把辅助函数关在函数里。
    fn add(a: usize, b: usize) -> usize {
        a + b
    }
    println!("10. add 只在 main 里存在,这是比 private 还小的开口");

    // ---------- 11. workspace:多个 package 怎么组织 ----------
    // 如果项目需要多个 lib 单元, 则我们需要通过 workspace 来处理问题。
    // 一个 workspace 能同时包含多个 package。
    // 在 .toml 文件下通过 [dependencies] 增加 crates 的路径, 然后在开头通过 [workspace] 统一。
    //
    // 根 Cargo.toml:
    //     [workspace]
    //     members = ["crates/core", "crates/engine"]
    //
    // crates/engine/Cargo.toml:
    //     [dependencies]
    //     core = { path = "../core" }
    //
    // 模块管一个 crate 内部的切分,workspace 管多个 crate 之间的切分 ——
    // 前者靠 pub 控制开口,后者靠 Cargo.toml 的 dependencies 控制谁能依赖谁。
    println!("11. mod 切 crate 内部,workspace 切 crate 之间");
}
