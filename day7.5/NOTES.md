---
day: 7.5
date: 2026-08-12
topic: option
mood: [zen, frage]
---

# Day 7.5 — option 泛型的逻辑 (2026-08-12)

> 📝 Live notes: [`option/src/main.rs`](option/src/main.rs) — written while watching, and the source everything else is generated from
>
> ✍️ Blog post: [Day 7.5](https://roddyh17.github.io/posts/rust/day-7-5-option-and-result/) (the blog has the story; this file has the technical details)
>
> 📓 Notion: **Day7.5: Option 泛型的逻辑**

今天的三条主线写在文件头上:复习 `Option` 在 Rust 里的地位、理解泛型编程与函数式编程的
演化历史、学 Rust 的报错机制。但真正把三条串起来的是**一句类型论的话**:

> 每个类型都由两组规则定义 —— **引入规则**(怎么造出它)和**消去规则**(怎么用掉它)。
> `Option<T>` 的引入规则是 `Some` 和 `None`;消去规则就是 `match`。

一旦接受这个说法,`Option` 就不再是「一个可空的容器」,而是**一对规则**。
`Result<T, E>` 是同一个模子刻的,只是把 `None` 换成了「带原因的 None」。
今天所有的 API —— `unwrap` / `map` / `ok_or` / `?` —— 都是这两条规则的糖。

## Goals — what to master

| #  | Topic | You should be able to... | Self-check |
|----|-------|--------------------------|------------|
| 1  | 引入与消去 | 说出 `Option<T>` 的引入规则和消去规则各是什么 | 除了 `match`,还有别的消去方式吗?它们凭什么合法? |
| 2  | Option 是 enum | 手写出 `Option<T>` 的定义 | `None` 为什么必须标类型,`Some(5)` 却不用? |
| 3  | unwrap 的代价 | 说清 `unwrap` 什么时候可以放心用 | `unwrap` 和 `expect` 差在哪?差的那部分值多少钱? |
| 4  | 两类错误 | 区分可恢复(`Result`)与不可恢复(`panic!`) | 凭什么说越界访问是「不可恢复」的? |
| 5  | 越界即 panic | 解释 Rust 为什么选择 panic 而不是返回垃圾值 | 这和 buffer overflow 这类攻击有什么关系? |
| 6  | match Result | 用 `match` 处理 `File::open` 的两种结果 | `Ok(file)` 里的 `file` 是借来的还是拿到的? |
| 7  | map | 说出 `map` 作用在哪一侧 | `Err` 经过 `map` 之后变了吗?为什么必须不变? |
| 8  | 错误做成类型 | 定义一个错误 enum 代替 `String` | 用 `String` 当错误类型,调用方会损失什么? |
| 9  | `?` | 用 `?` 把 match 链压掉 | `?` 为什么只能写在返回 `Result` 的函数里? |
| 10 | 互转 | 用 `ok_or` / `ok` / `err` 在 Option 与 Result 间来回 | `ok_or` 是加信息还是减信息?`ok()` 呢? |
| 11 | 泛型四层 | 说出泛型 T / enum / match / 函数各管什么 | 这四层里,哪几层发生在编译期,哪几层发生在运行期? |

## Concepts and examples

代码在 `option/src/main.rs`,11 个编号节,`cargo run` 一次跑完。

### Option:一对规则,不是一个容器

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**引入规则**造值(`Some` / `None`),**消去规则**用值(`match`)。这不是术语游戏 ——
它解释了一件平时觉得别扭的事:**为什么 Rust 不让你直接拿 `Option` 里的东西**。
因为「直接拿」不在消去规则里。你只能走 `match`,而 `match` 强制你把 `None` 那条路也写出来。

再往前推一步就更彻底了 —— **Böhm–Berarducci 编码**告诉你:任何代数数据类型都同构于一个
关于结果类型的多态函数类型。换句话说,`Option<T>` 和「一个接受两个处理函数、返回结果的
函数」是同一个东西。`match` 只是这个函数的语法形式。

`None` 必须标类型(第 1 节):

```rust
let some_number = Some(5);              // 推断出 Option<i32>
let no_number: Option<i32> = None;      // 光看 None 无从知道 T 是什么
```

### 消去规则的三种形态

| 写法 | 是什么 | `None` 时 | 什么时候用 |
|------|--------|-----------|-----------|
| `match x { Some(n) => .., None => .. }` | 完整的消去规则 | 你自己写的那条分支 | 两种情况都要认真处理 |
| `x.unwrap()` | 只写 `Some` 那条,`None` 交给 panic | **panic** | 逻辑上不可能是 None |
| `x.map(f)` | 只改 `Some` 那条,`None` 原样穿过 | `None` | 只想做值的变换,不做决策 |

`unwrap` 不是「取值」,是**把 `None` 这条分支的处理权交给 panic**。所以它不是省事,是把
决策外包了(第 3 节)。

### 两类错误:值 vs 终止

Rust 中的错误处理将可恢复和不可恢复的错误进行区分:

| | 类型 | 是什么 | 能不能 match |
|---|------|--------|-------------|
| 可恢复 | `Result<T, E>` | 一个**值** | 能 |
| 不可恢复 | `panic!` | 一次**终止** | 不能 |

关键区别不在「严重程度」,在于**它是不是一个值**。`Result` 是值,所以能被返回、被传递、
被 `match`;`panic!` 不是值,它直接掀桌子。

**越界为什么归入不可恢复**(第 5 节):Buffer overflow 是一种经典的安全系统攻击行为,
一些语言的设计就是为了避免这种行为。在 Rust 中,一旦访问越界数组,程序会直接 panic ——
因为读到数组外面的内存,读到的是**什么**是不确定的,程序已经不知道自己在干什么了,
继续跑下去比崩掉更危险。想不 panic 就走 `get` 那条路,把「可能没有」写回类型里。

主动触发用 `panic!()`,配 `RUST_BACKTRACE=1 cargo run` 看完整调用栈。

### map:只改一半

```rust
match num.parse::<i32>().map(|i| i * 2) {
    Ok(n) => println!("{}", n),
    Err(..) => {}
}
```

`map` 只作用在成功那一侧,`Err` 原样穿过去(第 7 节)。这不是设计上的偷懒 —— 如果 `map`
对 `Err` 也生效,`Result` 就不能表达「失败」了,因为失败会被后续变换悄悄改掉。
要改错误那一侧,用的是另一个方法:`map_err`。

### 把错误做成类型

```rust
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn div(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)   // 失败是一种常态,包装起来返回
    } else {
        Ok(a / b)
    }
}
```

用这种方式封装错误会显得非常整齐,但整齐只是副产品。真正的收益在调用方:

```rust
match div(1.0, 0.0) {
    Ok(v) => ...,
    Err(MathError::DivisionByZero) => ...,     // 能精确认出是哪一种失败
    Err(MathError::NegativeSquareRoot) => ...,
}
```

换成 `Result<f64, String>` 这段就写不出来 —— **字符串没法被穷尽匹配**。错误一旦变成
`String`,调用方就只剩下「打印出来给人看」这一条路。

### `?`:传播,而不是处理

```rust
fn div_then_sqrt(a: f64, b: f64) -> Result<f64, MathError> {
    let q = div(a, b)?;    // 失败 → 立刻 return Err(..);成功 → q 是 f64,不是 Result
    let r = sqrt(q)?;
    Ok(r)
}
```

两处可能失败,函数体里一个 `match` 都没有(第 9 节)。

`?` 只能写在返回 `Result`(或 `Option`)的函数里 —— **它要 `return`,就得知道该 return
成什么形状**。这一条解释了为什么 `?` 不能随便写在 `main` 里:除非把 `main` 的签名也改成
返回 `Result`。

### Option ↔ Result:加信息与减信息

| 方向 | 方法 | 干了什么 |
|------|------|---------|
| `Option<T>` → `Result<T, E>` | `ok_or(e)` / `ok_or_else(f)` | **加信息**:把「没有」升级成「因为 e 而没有」 |
| `Result<T, E>` → `Option<T>` | `.ok()` | **减信息**:丢掉错误原因,只留成功侧 |
| `Result<T, E>` → `Option<E>` | `.err()` | 反过来只留错误侧 |

```rust
fn first(arr: &[i32]) -> Result<&i32, String> {
    arr.get(0).ok_or("out of index".to_string())
}
```

### 泛型的逻辑:四层各管一件事

| 层 | 决定什么 | 发生在 |
|----|---------|--------|
| 泛型 `T` | 里面可以装什么类型 | **编译期** |
| `enum` | 值可能有哪些形态 | **编译期** |
| `match` | 当前值属于哪种形态 | 运行期 |
| 函数 | 输入如何转换为输出 | 运行期 |

`Option<T>` 之所以强,是因为它同时用满了前三层:`T` 管内容,`Some`/`None` 管形态,
`match` 管分派。这也是「泛型编程」这个词的实际含义 —— 不是「能装任何东西的盒子」,
而是**把「装什么」这个决定推迟到使用现场,并且推迟的部分全部在编译期结算**。

## Practice

```bash
cd day7.5/option && cargo run --example practice
```

1. **不许 unwrap**(🌟)— 写 `describe(Option<i32>) -> String`,函数体里不许出现 `unwrap`/`expect`
2. **把「没有」升级成「为什么没有」**(🌟🌟)— 写 `nth(&[i32], usize) -> Result<i32, String>`,注意 `&i32` → `i32` 那一步
3. **错误要能被 match**(🌟🌟)— 定义 `enum ParseError { Empty, NotANumber(String) }` + `to_int`,用 `map_err` 换错误类型
4. **把 match 链压成 `?`**(🌟🌟)— 给定一段啰嗦的双 match 版本,改写成三行以内
5. **先预测,再运行**(🌟)— `map` 和 `map_err` 分别作用在哪一侧?先猜后验
6. **一个坏的就整批作废**(🌟🌟🌟)— 写 `parse_all(&str) -> Result<Vec<i32>, String>`,`Vec<Result<T,E>>` 能 collect 成 `Result<Vec<T>,E>`

第 6 题是今天最值得做的一道:它逼你分清**「坏的跳过」和「有一个坏的就全盘失败」**这两种
语义。真实系统里天天要选 —— 行情解析通常跳过,订单解析必须整体失败。

## Questions I asked

- **Q:** 既然 `unwrap` 那么方便,为什么不到处用?
  **A:** 因为它把 `None` 分支的决定权交给了 panic,而 panic 不可恢复。可以用的场合只有一种:
  你能证明它逻辑上不可能是 `None`(比如刚 `push` 完再 `pop`)。别的地方用它,等于把一个
  编译期能挡住的错误推迟到运行期,而且是最难查的那种 —— 崩在调用点,原因在别处。

- **Q:** `Result<T, String>` 不是更省事吗,为什么要专门定义一个错误 enum?
  **A:** 省的是写代码的事,亏的是调用方。`String` 不能被穷尽匹配,所以调用方无法写
  「只处理除零、别的原样往上抛」这种逻辑,只能整个打印出来。错误 enum 让失败本身变成
  可分派的数据,这是后面写库的时候的硬需求。

- **Q:** `?` 为什么不能在 `main` 里随便写?
  **A:** `?` 的语义是「失败就 `return Err(..)`」,所以它要求所在函数的返回类型能承载
  这个 `Err`。默认的 `fn main()` 返回 `()`,承载不了。把签名改成
  `fn main() -> Result<(), Box<dyn Error>>` 就可以了。

- **Q:** `ok_or` 和 `ok_or_else` 用哪个?
  **A:** `ok_or(e)` 的参数 `e` 无论成功失败都会被求值;`ok_or_else(|| e)` 只在失败时才求值。
  错误值是现成的常量就用前者,要 `format!` 拼出来就用后者 —— 成功路径上白拼一个字符串是浪费。

- **Q:** 「引入规则 / 消去规则」这套说法有什么实际用处,还是只是好听?
  **A:** 有实际用处。它给了一个判断标准:任何一个新 API,先问它是在造值还是在用值。
  `Some`/`Ok` 是引入,`match`/`unwrap`/`map`/`?` 全是消去的不同形态。分清之后,记 API
  就从背方法名变成了认形态 —— `Result` 上那一整排方法,其实只是同一条消去规则的不同写法。

## Errors I hit

- **`error: expected item, found keyword 'let'` —— 把 `let` 写在了函数外面。**
  ```rust
  let f = File::open("hello.txt").err();   // ❌ 写在文件末尾、任何 fn 之外
  ```
  → **cause:** 模块顶层只能放 item(`fn` / `struct` / `enum` / `const` / `static` / `use` / `mod`),
  `let` 是语句,不是 item。编译器的建议是改用 `static` 或 `const`,但那不是我要的 ——
  我只是随手记了一行没放进 `main`。
  → **fix:** 挪进 `fn main()` 里。

- **`error[E0308]: mismatched types` —— 函数体最后一行多了个分号。**
  ```rust
  fn first(arr: &[i32]) -> Result<&i32, String> {
      arr.get(0).ok_or("out of index".to_string());   // ❌ 这个分号
  }
  ```
  → **cause:** 有分号 = 语句,值被丢掉,函数体的值变成 `()`;声明的返回类型却是 `Result`。
  Rust 里「表达式」和「语句」的区别就差这一个分号,而这一个分号改变的是**函数返回什么**。
  → **fix:** 去掉分号。这是 Day 2 表达式那一节的内容,到今天才第一次真的踩到。

- **`cannot find type 'File' in this scope` —— 用了 `File::open` 却没 `use`。**
  → **cause:** `std::fs::File` 不在 prelude 里。`Option`/`Result`/`String` 在 prelude 里所以
  从来不用 import,这造成了一种错觉:标准库的东西都不用 import。
  → **fix:** `use std::fs::File;`。

- **想在 `main` 里直接写 `let mut f = File::open("hello.txt")?;` —— 编译不过。**
  → **cause:** `main` 返回 `()`,`?` 无处可 return。
  → **fix:** 把演示挪进一个返回 `Result` 的辅助函数(`div_then_sqrt`),`main` 里只调用它。
  这个错反过来把 `?` 的原理讲清楚了 —— 它不是「解包」,是「提前返回」。

## Plan for Day 8

- day8 已经开好:`module` —— 模块系统、`mod` / `use` / `pub`、可见性规则
- 今天欠着的:**`String` 三件套还差的那一件**(day7 就欠了),`push_str` 与 `+` 的所有权差别、
  `format!`、为什么 `String` 不能用下标索引
- 泛型今天只讲到了「四层职责」这一层,**自己写 `impl<T>` 还是零** —— 这一条要留到项目里补
