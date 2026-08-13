---
day: 8
date: 2026-08-13
topic: module
mood: [zen]
---

# Day 8 — module (2026-08-13)

> 📝 Live notes: [`module/src/main.rs`](module/src/main.rs) — written while watching, and the source everything else is generated from
>
> ✍️ Blog post: [Day 8](https://roddyh17.github.io/posts/rust/day-8-module/) (the blog has the story; this file has the technical details)
>
> 📓 Notion: **Day8: Module**

前七天写的所有代码都堆在一个 `fn main` 里。今天学的是**怎么把它切开**。

切开这件事有两个层次:一个 crate 内部靠 `mod` 切,多个 crate 之间靠 workspace 切。
而无论哪一层,决定「切口开多大」的都是同一件事 —— **可见性**。Rust 在这里的立场很硬:
**默认全部私有**。不主动写 `pub` 的东西不是「忘了写」,是「明确表示它是实现细节」。

这一天的内容看起来最像语法课,实际上是**项目课** —— 阶段实战要做的 workspace 多 crate
架构,地基就是今天这一节。

## Goals — what to master

| #  | Topic | You should be able to... | Self-check |
|----|-------|--------------------------|------------|
| 1  | 三个层级 | 说清 package / crate / module 各是什么单位 | 模块会产生新的编译单元吗? |
| 2  | 默认私有 | 说出不写 `pub` 会发生什么 | 为什么默认是私有而不是公开? |
| 3  | pub 的强度 | 区分 `pub` / `pub(crate)` / `pub(in path)` | `pub(crate)` 和 `pub` 在单 crate 项目里看得出区别吗? |
| 4  | 绝对路径 | 用 `crate::a::echo()` 调到模块函数 | 为什么是 `::` 不是 `.`? |
| 5  | self / super | 在嵌套模块里往上一层取名字 | 裸调用和 `self::` 有区别吗? |
| 6  | use | 用 `use` 把长路径的末段拉进作用域 | `use` 有运行时开销吗? |
| 7  | as | 用 `as` 解决 use 进来的重名 | `as` 在 use 里和在表达式里是同一件事吗? |
| 8  | 嵌套 | 写出三层嵌套并从外面调到最里面 | `pub mod` 会让里面的东西也自动公开吗? |
| 9  | 类型转换 | 用 `as` 做数值类型转换 | `u8 as usize` 和 `usize as u8` 哪个可能丢数据? |
| 10 | 函数内定义 | 在 `fn` 里定义 `fn` / `mod` | 这样定义的东西作用域有多大? |
| 11 | workspace | 说出 workspace 和 module 各解决什么问题 | 什么时候该拆 crate,什么时候只需拆 mod? |

## Concepts and examples

代码在 `module/src/main.rs`,11 个编号节,`cargo run` 一次跑完。

### 三个层级各是什么单位

| 层级 | 单位 | 由什么定义 | 会不会产生新编译单元 |
|------|------|-----------|-------------------|
| **Package** | cargo 管理的单位 | 一个 `Cargo.toml` | — |
| **Crate** | **编译**的单位 | 一个 `src/main.rs`(binary)或 `src/lib.rs`(library) | 是 |
| **Module** | **命名空间**的单位 | `mod` 关键字 | **否** |

Package 包含多个 binary crates,或者一个库的单元;每一个 package 至少包含一个 crate。
`cargo new --lib xxx` 建的就是一个 lib package。

**模块不产生编译单元**,这一条最值得记:它意味着模块是纯粹的逻辑切分,
写在一个文件里和拆成多个文件对编译器完全是一回事。所以「要不要拆文件」是给人看的决定,
不是给编译器的。

### 默认私有

```rust
mod visibility {
    pub fn open() {}
    fn closed() {}     // 模块外面看不见
}

visibility::open();
// visibility::closed();   // error[E0603]: function `closed` is private
```

模块可见性:默认所有私有,所以我们需要声明出来。Rust 在 `mod` 里写函数默认私有制。

### pub 的三种强度

| 写法 | 开口 | 什么时候用 |
|------|------|-----------|
| `pub` | 谁都能看(包括依赖你的 crate) | 真正的公共 API |
| `pub(crate)` | 本 crate 内公开,别的 crate 看不到 | 跨模块共享的内部工具 |
| `pub(in path)` | 只在指定路径的子树内公开 | 精确控制,给一小片代码开后门 |

```rust
mod a {
    pub(in crate::a) mod secret {
        pub fn hidden() {}
    }
    pub fn call_secret() { secret::hidden(); }   // ✅ a 自己够得着
}

a::call_secret();
// a::secret::hidden();   // ❌ error[E0603]: module `secret` is private
```

`pub(in crate::a)` 的意思是「只在 `crate::a` 这棵子树里公开」。crate 根不在这棵子树里,
所以根上够不着。注意 **path 必须是自己的祖先** —— 不能给不相干的模块开后门。

### 路径:`::` 不是 `.`

> `a.echo()` —— 这是错的,这是调用函数,而非路径访问,点只是 struct 的 oop 这样的路径。

这是今天最容易混的一条。**`.` 是「在一个值上面找方法」,`::` 是「在命名空间里找名字」**。
模块不是值,所以只能用 `::`。

```rust
crate::a::echo();   // 绝对路径:从 crate 根开始数
a::echo();          // 相对路径:从当前位置开始数
```

在 crate 根上写,这两者等价。选哪个的标准是:**代码搬家时哪个更不容易断**。
把一整个模块挪到别处,内部的相对路径能跟着走,绝对路径就得改。

### self 与 super

```rust
mod a {
    pub fn echo() {}
    pub fn log() {
        echo();          // 裸调用
        self::echo();    // 完全等价,只是更醒目
    }
    pub mod b {
        use super::echo;   // 往上一层取名字
        pub fn echo_b() { echo(); }
    }
}
```

`self::` 和裸调用编译出来一模一样,写它纯粹是为了让「这是本模块的东西」看得更清楚。
`super::` 才是真有必要的 —— 子模块**不会自动继承**父模块的名字,得显式往上取。

### use:拉的是名字,不是代码

> 如果我们现在想直接访问 b 模块里的方法,要写成 `a::b::log()`。
> rust 很聪明,它可以抽象 namespace。

```rust
use a::b::log;
log();                                    // 省掉了 a::b::
use a::{b::log as log2, log as log_a};    // 花括号合并 + as 解决重名
```

`use` 拉进来的是**名字**,没有任何运行时开销 —— 它只是省打字,不改变生成的代码。
花括号也只是省写:`a::{b::log, log}` 等价于分开写两行 `use`。

两个都叫 `log` 时同一个作用域装不下,`as` 给其中一个改名。

### `as` 的两个身份

| 位置 | 干什么 |
|------|--------|
| `use x as y` | **重命名**,编译期的名字替换 |
| `expr as T` | **类型转换**,可能真的改变数据 |

```rust
let x: u8 = 1;
add(x as usize, 1);
```

同一个关键字两件事。数值转换要当心方向:`u8 as usize` 是变宽,安全;
`usize as u8` 是变窄,**高位直接被截掉,不报错也不 panic** —— 这是 Rust 里少见的
「悄悄丢数据」的地方。

### 函数里也能定义函数和模块

```rust
fn main() {
    fn add(a: usize, b: usize) -> usize { a + b }
    mod visibility { pub fn open() {} }
}
```

作用域只有这个函数,外面完全看不见。这是**比 private 还小的开口** ——
连模块都不用建,直接把辅助函数关在函数里。

但注意:函数内定义的模块**够不着 `crate::`**,因为它根本不在 crate 的模块树上。
所以今天的 `mod a` 必须定义在 crate 根,第 4 节的绝对路径演示才成立。

### workspace:切 crate 之间

> 如果项目需要多个 lib 单元,则我们需要通过 workspace 来处理问题。
> 一个 workspace 能同时包含多个 package。

```toml
# 根 Cargo.toml
[workspace]
members = ["crates/core", "crates/engine"]

# crates/engine/Cargo.toml
[dependencies]
core = { path = "../core" }
```

| | 切什么 | 靠什么控制 |
|---|-------|-----------|
| `mod` | 一个 crate **内部** | `pub` 的强度 |
| workspace | 多个 crate **之间** | `Cargo.toml` 的 `dependencies` |

差别在于**强度**:模块之间的边界靠自觉(同一个 crate 里,加个 `pub(crate)` 就穿过去了);
crate 之间的边界是硬的 —— 没在 `dependencies` 里写,就是编译不过。
所以真正想让架构分层不被破坏,得拆 crate,不能只拆 mod。这一条直接决定了阶段实战项目的形状。

## Practice

```bash
cd day8/module && cargo run --example practice
```

1. **让它公开**(🌟)— 只加 `pub`,让 `shop::price()` 能被调到。`mod` 和 `fn` 是不是都要加?
2. **同一个调用,三条路径**(🌟🌟)— 用 `crate::` / `super::` / `use` 三种写法各写一个函数,调同一个 `helper()`
3. **两个同名函数,一个作用域**(🌟🌟)— `left::name()` 和 `right::name()` 都 `use` 进来,靠 `as` 化解
4. **开口开多大**(🌟🌟)— 四行调用,先预测哪几行能编译,再逐行验证
5. **pub 不会自动往里传**(🌟)— 回答:`pub mod` 公开的到底是什么?
6. **把 main 里的东西搬进模块**(🌟🌟🌟)— 把 Day 7 的 `word_count` 重构成 `text` 模块,`normalize` 必须私有

第 6 题是今天最重要的一道 —— 它是**公共 API 边界**的第一次练习。
「哪些该 pub,哪些该藏」这个问题,在阶段实战的 workspace 里会反复出现。

## Questions I asked

- **Q:** `a.echo()` 为什么不行?
  **A:** `.` 是在**一个值**上面找方法,`::` 是在**命名空间**里找名字。模块不是值,
  它连运行时的存在都没有 —— 编译完就没有「模块」这个东西了,只剩下被重整过的符号名。
  所以只能用 `::`。

- **Q:** 既然模块不产生编译单元,那拆文件到底有什么意义?
  **A:** 对编译器没意义,对人有意义。拆文件是为了让「找代码」和「读 diff」变快,
  以及让 `pub` 的边界在目录结构上看得见。真正对编译器有意义的拆分是拆 **crate** ——
  那才会产生独立的编译单元(也才能并行编译、独立测试、独立发布)。

- **Q:** `pub(crate)` 和 `pub` 在我现在这种单 crate 项目里有区别吗?
  **A:** 现在没有,一模一样。区别只在别的 crate 依赖你的时候才显现:`pub` 的能被依赖方看到,
  `pub(crate)` 的看不到。所以它的价值是**为将来准备的** —— 写库时默认写 `pub(crate)`,
  只把真正想承诺的东西写成 `pub`,因为 `pub` 一旦发布就是 semver 承诺,改它要升大版本。

- **Q:** `pub mod b` 之后,b 里面的私有函数是不是也跟着公开了?
  **A:** 不会。模块的 `pub` 和模块里每一项的 `pub` 是**两件独立的事**。
  `pub mod` 只是把门打开,门里每样东西还有自己的门。这是 Exercise 5 要答的。

- **Q:** 什么时候该拆 crate,什么时候只拆 mod?
  **A:** 判断标准是**边界要不要硬**。如果只是想让代码好找,拆 mod 够了;
  如果想保证「这一层永远不许依赖那一层」,必须拆 crate —— 因为 mod 之间的边界一个
  `pub(crate)` 就能穿过去,而 crate 之间没写进 `Cargo.toml` 就是编译不过。

## Errors I hit

- **`error[E0603]: module 'b' is private` —— `pub(in path)` 把自己关在外面了。**
  ```rust
  use a::{b::log as log2, log};   // ❌ 在 crate 根上写

  mod a {
      pub(in crate::a) mod b { pub fn log() {} }
  }
  ```
  → **cause:** `pub(in crate::a)` 的意思是「只在 `crate::a` 这棵子树内公开」。
  `use` 那一行写在 crate 根上,**根不在 `a` 里面**,所以够不着。
  报错里 rustc 会同时指出两处:引用的那一行,和 `pub(in crate::a) mod b` 的定义行。
  → **fix:** 想从根上用就改成 `pub mod b`。
  这个错反过来把 `pub(in path)` 讲清楚了 —— 它不是「更强的 pub」,是**更窄的 pub**。
  现在 `main.rs` 里保留了一个真正只给 `a` 用的 `secret` 模块,和一行注释掉的越界调用,
  取消注释就能复现这个错。

- **`error: expected item, found ...` —— 光写了一个 `pub`。**
  文件顶上孤零零一行 `pub`,后面什么都没有。
  → **cause:** `pub` 是修饰符,不是 item,它必须跟着一个 `fn` / `mod` / `struct` 等等。
  当时是想记「pub 这个关键字」,顺手写在了代码里。
  → **fix:** 删掉,记进注释。**笔记要写成注释,不要写成半截代码** —— 这次的教训。

- **`warning: constant 'NUM' is never used` / `function is never used`。**
  → **cause:** 模块里定义了但没调用。骨架期的固有状态。
  → **fix:** 要么真的用起来(`main.rs` 里选了这条:让 `echo()` 去读 `NUM`,顺便演示
  「私有项在模块内部是可见的」),要么挂 `#[allow(dead_code)]`(`practice.rs` 里选了这条,
  因为答案槽还空着)。

## Plan for Day 9

- **`String` —— 欠了两天了**(day7 欠一次,day7.5 又欠一次)。重点在 `push_str` 与 `+`
  的所有权差别、`format!`、以及为什么 `String` 不能用下标索引(字节 ≠ 字符,接上 Day 4 的切片)
- **基础教学到此结束**,接下来进入阶段实战项目。今天学的 workspace 就是那个项目的地基 ——
  但仅仅是「知道有这个东西」,真正的多 crate 划分要在项目里练
- 项目开工前的欠账清单(见 [`projects/rung/DESIGN.md`](../projects/rung/DESIGN.md)):自己写泛型 `impl<T>`(0 处)、
  生命周期标注(0 处)、`Box`/`Rc`/`RefCell`(0 处)、闭包(0 处)、`#[test]`(**全仓库 0 个**)
