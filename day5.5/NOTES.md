---
day: 5.5
date: 2026-08-11
topic: more_match
mood: # tags, e.g. [zen, frage] — zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 5.5 — More Match (2026-08-11)

> 📝 Live notes: [`more_match/src/main.rs`](more_match/src/main.rs) — written while watching, and the source everything else is generated from
>
> ✍️ Blog post: [Day 5.5](https://roddyh17.github.io/posts/rust/day-5-5-pattern-matching/) (the blog has the story; this file has the technical details)
>
> 📓 Notion: **Day5.5: More Match**

补课性质的一天:Day 5 只学了 `match` 的皮,这一天把模式(pattern)本身拆开看。
`match` 真正的能力不是「代替 if-else」,而是**按数据的结构来分支**。

## Goals — what to master

| # | Topic | You should be able to... | Self-check |
|---|-------|--------------------------|------------|
| 1 | struct vs enum | 用一句话说清两者表达什么 | AND 和 OR 分别对应哪个? |
| 2 | 字面量模式 | 对数字和 `&str` 直接匹配 | 字符串字面量能匹配,`String` 呢? |
| 3 | `@` 绑定 | 写出 `var @ 42` | 它比直接写 `42` 多做了什么? |
| 4 | 范围模式 | 写出 `n @ 1..=10` | `..=` 和 `..` 差在哪? |
| 5 | 嵌套 `@` | 写出 `whole @ Message::Number(1..=10)` | 绑定到的是整个变体还是里面的数? |
| 6 | 模式里的所有权 | 预测 `match` 一个 String 会不会 move | 什么时候需要 `ref`? |
| 7 | 结构体解构 | 写出 `Point { x, y: 0 }` | 混用绑定和字面量意味着什么? |
| 8 | 守卫 guard | 写出 `n if n % 2 == 0` | 守卫分支的顺序重要吗? |
| 9 | 复杂枚举匹配 | 匹配 `ChangeColor(r, g, b)` 这类元组变体 | 什么时候该用 `_ => ()`? |
| 10 | `if let` / `while let` | 只关心一种情况时替代 match | `while let` 配什么方法最常见? |
| 11 | `ref` / `ref mut` | 在模式里借用而不是移动 | `ref mut` 之后怎么改到原值? |
| 12 | 迭代器惰性 | 区分适配器和消费者 | 只写 `map` 不写 `collect` 会发生什么? |

## Concepts and examples

代码在 `more_match/src/main.rs`,现场记录风格(旧段落逐段注释掉)。

### struct 是 AND,enum 是 OR

这是今天自己写下的一句话,后面所有模式匹配都是它的推论:

| | 表达 | 描述的是 |
|---|---|---|
| `struct` | **同时拥有**这些东西(AND) | 一个对象由什么组成 |
| `enum` | **只能是**这些可能性之一(OR) | 一个值可能处于哪一种情况 |

而且 enum 的每种情况还可以携带不同的数据 —— 这正是 `match` 要解构的东西:

```rust
enum OrderStatus {
    Pending,
    Filled { execution_price: f64, quantity: f64 },
    Canceled { reason: String },
}
```

### 模式的几种形态

```rust
// 1. 字面量:数字、&str 都能直接匹配
match y { "Hello" => .., _ => .. }

// 2. @ 绑定:既检查右边的模式,又把匹配到的完整值绑到左边
match x { var @ 42 => println!("{}", var), _ => .. }

// 3. @ + 范围
match x { n @ 1..=10 => println!("{} 在 1 到 10 之间", n), _ => .. }

// 4. @ 绑定整个变体
match Message::Number(8) {
    whole @ Message::Number(1..=10) => println!("匹配到完整结构:{:?}", whole),
    _ => {}
}

// 5. 结构体解构:绑定和字面量可以混用
match p {
    Point { x, y: 0 } => println!("on the x axis at {}", x),
    Point { x: 0, y } => println!("on the y axis at {}", y),
    Point { x, y }    => println!("on neither axis: {}, {}", x, y),
}

// 6. 守卫 guard:用计算出来的条件参与匹配
match x { n if n % 2 == 0 => println!("Even"), _ => println!("Odd") }

// 7. 嵌套:字段内部再 @ 绑定
match msg {
    Message::Hello { id: message_id @ 3..=7 } => ..,
    Message::Hello { id: phone_id @ 10..=12 } => ..,
    Message::Hello { id } => ..,
}
```

**守卫分支必须写在更宽泛的分支前面。** match 是自上而下、第一个匹配上的赢,
把带守卫的那一支放到 `Filled { .. }` 后面,编译器会给 `warning: unreachable pattern`
(*matches all the relevant values* / *no value can reach this*)—— 是警告不是错误,
所以程序照跑,只是那一支永远不生效。和 Day 5 那个 `o => {}` 是同一个教训。

### 模式里的所有权 —— `ref` 和 `ref mut`

`match` 一个 `String` 会把它**移进**绑定里:

```rust
let x = String::from("Hello");
match x {
    var => println!("{}", var),   // x 被移走了,之后不能再用
}
```

想借用而不是转移(递归、或者想改数据本身时),在模式里用 `ref` / `ref mut`:

```rust
let x = String::from("Hello");
match x {
    ref var => println!("The value of x is {}", var),   // var 是 &String
}

let mut x = String::from("Hello");
match x {
    ref mut var => {
        *var = String::from("world");   // 原地改
        println!("{}", var);
    }
}
println!("{}", x);   // world —— 真的改到了
```

不用 `ref` 而直接解构一个在 `&mut` 后面的 enum 字段,会得到:

```text
error[E0507]: cannot move out of `self.status.reason` as enum variant `Canceled`
              which is behind a mutable reference
```

注意这和 Day 6 `..user1` 的部分移动**不是同一个错误**:那个是 E0382(用了已移动的值),
这个是 E0507(想从借用后面移出去)。现代 Rust 也可以直接写 `match &mut self.status { .. }`,
match ergonomics 会自动把绑定变成可变引用,两种写法都对。

### `if let` / `while let`

只关心一种情况时,不必写完整的 match:

```rust
let opt = Some(5);
if let Some(x) = opt { println!("Matched {:?}", x); }

let mut iter = vec![1, 2, 3].into_iter();
while let Some(x) = iter.next() { println!("matched {:?}", x); }
```

`while let Some(x) = v.pop()` 是最常见的搭配 —— `pop()` 返回 `Option`,空了自然停。

### 迭代器是惰性的(顺带复习)

拆 `zip` 的签名可以看出来:

```rust
fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
where Self: Sized, U: IntoIterator
{
    Zip::new(self, other.into_iter())
}
```

`iter()` 会发生所有权变化,而 `zip` 自己也实现了 `Iterator`,返回的只是一个 `Zip` 结构体。
**只要没有人要元素,就只是在搭管道,不会真的算。**

| | 例子 | 作用 |
|---|---|---|
| 适配器 | `map` `filter` `zip` `skip` | 只搭建处理管道 |
| 消费者 | `next` `collect` `sum` `for` | 真正驱动管道执行 |

### match 的实际用武之地

i. 处理错误 ii. 解析命令行参数 iii. 解析配置文件与数据包 —— 共同点都是
**拿到一坨结构未知的数据,按结构分支**。

## Practice

这一天配的是一个 **mini project**,不是零散小题 —— 也是之前欠着的 struct + enum 合并练习:

```bash
cd day5.5/more_match && cargo run --example practice
```

**迷你订单引擎**,8 个 Stage,约 100 行自己写的代码,跨 Day 5 / 5.5 / 6:

| Stage | 内容 | 级别 |
|---|---|---|
| 1 | `Side` / `OrderStatus` 两个 enum,三种变体形态各出现一次 | 🌟 |
| 2 | `Order` 结构体 | 🌟 |
| 3 | `impl`:关联函数 `new` + 方法 `notional` / `fill` / `cancel` | 🌟🌟 |
| 4 | `annotate_cancel` —— 用 `ref mut` 原地追加 enum 里的 String | 🌟🌟🌟 |
| 5 | `classify` —— 四条规则三个变体,多出来的靠守卫 | 🌟🌟🌟 |
| 6 | `size_bucket` —— `@` 绑定 + 范围模式 | 🌟🌟 |
| 7 | `trait Describe` + `&[&dyn Describe]` 异质切片 | 🌟🌟🌟 |
| 8 | `settle` —— `while let` + `if let` | 🌟🌟 |

main 里是逐段取消注释的验收断言,写完一个 Stage 放开一段。

## Questions I asked

- **Q:** `match x { var @ string => .. }` 为什么不对?
  **A:** `string` 是小写标识符,在模式位置上它是**绑定**不是类型 —— 和 Day 5 的 `o => {}`
  一模一样的坑。想匹配字符串内容要写字面量 `"Hello"`,想按类型分支 match 做不到。

- **Q:** 守卫分支放错位置会怎样?
  **A:** `warning: unreachable pattern`,**是警告不是错误**,程序照常编译运行,
  那一支静默失效。这是今天最值得警惕的一条。

- **Q:** `ref mut` 和 `&mut` 在 match 里有什么区别?
  **A:** `ref mut` 写在模式一侧(`match x { ref mut var => ..}`),`&mut` 写在被匹配的
  表达式一侧(`match &mut x { .. }`)。结果都是拿到可变引用,现代 Rust 更常用后者。

- **Q:** 为什么 `map` 之后什么都没发生?
  **A:** 适配器是惰性的,只搭管道。要有消费者(`collect` / `sum` / `for` / `next`)才会执行。

## Errors I hit

- `match x { var @ string => .. }` — `string` 是绑定不是类型 → 用字面量模式
- `error[E0507]: cannot move out of ... which is behind a mutable reference` —
  想从 `&mut` 后面解构出 `String` → 用 `ref mut`,或改成 `match &mut ...`
- `warning: unreachable pattern` — 守卫分支排在了更宽泛的分支后面 → 守卫要靠前
- `Point { x: 1, y }` 的注释写成了「on the y axis」— 应该是 `x: 0` 才对得上语义

## Plan for Day 7

- 做完 `examples/practice.rs` 的迷你订单引擎
- 泛型 generics
