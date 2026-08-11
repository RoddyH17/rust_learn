---
day: 6
date: 2026-08-11
topic: oop_basic
mood: # tags, e.g. [zen, frage] — zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 6 — Struct, impl, Trait (2026-08-11)

> 📝 Live notes: [`oop_basic/src/main.rs`](oop_basic/src/main.rs) — written while watching, and the source everything else is generated from
>
> ✍️ Blog post: [Day 6](https://roddyh17.github.io/posts/rust/day-6-structs-and-traits/) (the blog has the story; this file has the technical details)
>
> 📓 Notion: **Day6: Struct & Trait**

Rust 没有 class,没有继承。OO 的三件事在这里是三个独立特性:**struct 是数据**,
**`impl` 挂行为**,**trait 是跨无关类型的共享行为**。今天主要是看清这三者之间的接缝。

Practice 推迟 —— 内容比较直观,不单独出题;struct + enum 合并的一套仍然欠着。

## Goals — what to master

| # | Topic | You should be able to... | Self-check |
|---|-------|--------------------------|------------|
| 1 | 定义与实例化 | 写出 struct 并初始化全部字段 | 字段有默认值吗?顺序重要吗? |
| 2 | 简洁写法 | 用字段初始化简写 | 什么时候可以省略 `field: field`? |
| 3 | 可变性 | 说明为什么不能只让一个字段可变 | `mut` 修饰的是字段还是绑定? |
| 4 | 更新语法 | 用 `..other` 构造新实例 | 它是拷贝还是移动? |
| 5 | 部分移动 | 判断 `..user1` 之后 user1 哪些字段还能用 | 为什么 `active` 能读,`username` 不能? |
| 6 | 生命周期 | 在 struct 里存 `&str` | 为什么必须写 `<'a>`?换成 String 呢? |
| 7 | 元组/单元结构体 | 写出 `Point(i32,i32,i32)` 和 `AlwaysEqual` | 什么时候类型该有名字而字段不用? |
| 8 | `impl` | 区分方法与关联函数 | 和 Day 5 在 enum 上的规则一样吗? |
| 9 | self 的三种形态 | 选对 `self` / `&self` / `&mut self` | `mut self` 到底做了什么? |
| 10 | Debug / Display | 派生 Debug,手写 Display | 为什么 Display 不能 derive? |
| 11 | Trait | 为两个无关类型实现同一个 trait | trait 和 interface 差在哪? |
| 12 | 静态分发 | 用 `&impl Trait` 做参数 | 为什么不写两个具体类型的参数? |

## Concepts and examples

代码在 `oop_basic/src/main.rs`,现场记录风格(旧段落逐段注释掉)。`cargo run` 跑当前这一段。

### struct 只是数据

- **每个字段都必须初始化**,没有默认值
- 字段有名字,所以**构造时顺序自由**,逗号分隔
- 字段初始化简写:局部变量同名时可省略 `field: field`(和 JS 一样)

### `mut` 修饰的是绑定,不是字段

```rust
let r3 = Rectangle { width: 1, height: 1 };
r3.width = 5;
// error[E0594]: cannot assign to `r3.width`, as `r3` is not declared as mutable
```

**Rust 不能把某一个字段单独标记为可写,必须对整体标记。** 这是借用规则的推论:
`&mut` 给出的是对整个值的独占访问,所以「可变」的粒度不可能细过你交出去的那个引用。

### 结构体更新语法是**移动**,不是拷贝

```rust
let user2 = User { email: String::from("another@example.com"), ..user1 };

println!("{}", user1.active);     // ✅ bool 是 Copy,还能读
println!("{}", user1.username);   // ❌ error[E0382]: borrow of moved value: `user1.username`
```

`user1` 不是整体作废,而是**部分移动(partially moved)**:Copy 字段留下,`String` 字段走了,
编译器按字段追踪。这就是 Day 3 的移动语义作用在每个字段上——
也是「一旦一个字段发生所有权移动,整个结构体就不能再整体传递或赋值」的来源。

### 存 `&str` 需要生命周期

```rust
struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}
```

结构体不拥有那些字节,只是借用,所以必须承诺自己不会活得比被指向的数据久,`'a` 就是这个承诺。
改存 `String` 就不需要了,因为那时结构体拥有数据。

### 三种结构体形态

| 形态 | 写法 | 用途 |
|---|---|---|
| 具名字段 | `struct User { .. }` | 常规 |
| 元组结构体 | `struct Point(i32, i32, i32);` | **类型**值得有名字,字段不需要;`origin.0` 访问 |
| 单元结构体 | `struct AlwaysEqual;` | 不关心数据,只关心要挂上去的行为 |

### `impl`:和 Day 5 在 enum 上完全同一条规则

| | 方法 | 关联函数 |
|---|---|---|
| 第一个参数 | `self` / `&self` / `&mut self` | 无 `self` |
| 属于 | 值 | 类型 |
| 调用 | `rect.area()` | `Rectangle::new(..)` |

`impl` 把行为挂到类型上,**不关心该类型是 struct 还是 enum**。`new` 写成自由函数也能跑,
放进 `impl` 是为了组织和命名空间——让它成为 `Rectangle::new` 而不是一个碰巧返回 Rectangle 的散函数。

### self 的三种形态 —— 今天最大的坑

```rust
fn set_width(mut self, new_width: u32) { self.width = new_width; }  // ❌
```

能编译,但什么也没做,还把接收者吃掉了。`mut self` 的意思是**拿走接收者的所有权,并允许我改我自己那份**,
方法结束时那份就被 drop 了。之后再用 `rect2`:

```text
error[E0382]: borrow of moved value: `rect2`
  `rect2` moved due to this method call
```

| 接收者 | 含义 |
|---|---|
| `self` | 消耗掉调用者 |
| `&self` | 只读 |
| `&mut self` | 原地修改 ← 我想要的 |
| `mut self` | 长得像第三种,行为是第一种 |

### Debug 可以 derive,Display 不行

```rust
#[derive(Debug)]
pub struct Rectangle { width: u32, height: u32 }
```

- `{:?}` 单行 / `{:#?}` 展开一行一个字段 / `dbg!(&rect)` 还会打印 file:line 并把值返回
- `{}` 是另一个 trait,**没有 `#[derive(Display)]`**:一个类型该怎么呈现给用户,是宏替你做不了的判断

手写 Display(今天用它把矩形画成 `#` 方块):

```rust
impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.height {
            let mut s = String::new();
            for _ in 0..self.width { s.push('#'); }
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}
```

### Trait:共享行为,不是共享父类

```rust
trait Shape { fn area(&self) -> f64; }

impl Shape for Rectangle { fn area(&self) -> f64 { self.width as f64 * self.height as f64 } }
impl Shape for Circle    { fn area(&self) -> f64 { std::f64::consts::PI * self.r * self.r } }
```

不同类型具有相同的行为 → 定义一个特征,再为这些类型实现它。`Rectangle` 和 `Circle`
不共享数据、不共享父类、不共享内存布局,只共享「都能回答 area()」这一件事。

叫它「Rust 的 interface」入门够用,但少说了一点:**trait 可以为你没写的类型实现**。
没有基类可以插进去,「这个类型算不算数」由一个独立的 `impl` 块回答,而不是由类型自己的定义回答。

### 为什么是 `&impl Shape` 而不是两个具体参数

```rust
fn print_area(shape: &impl Shape) { println!("{}", shape.area()); }
```

写 `print_area(rectangle: &Rectangle, circle: &Circle)` 是在回答错误的问题:它把「存在哪些类型」
写死了,每多一种形状就要多一个参数。`&impl Shape` 说的是「任何能告诉我面积的东西」,类型集合保持开放。

| | `&impl Trait` | `&dyn Trait` |
|---|---|---|
| 分发 | **静态**,编译期为每个具体类型单态化出一份 | **动态**,运行时查 vtable |
| 开销 | 零,直接调用 | 一次间接跳转 |
| 何时用 | 类型编译期已知 | 类型要在运行时才决定 |

## Practice

推迟。`examples/practice.rs` 仍是空骨架:

```bash
cd day6/oop_basic && cargo run --example practice
```

## Questions I asked

- **Q:** 为什么不能只把某一个字段标记为可变?
  **A:** `mut` 修饰绑定不修饰字段。`&mut` 给的是对整个值的独占访问,可变粒度不可能细过引用的粒度。

- **Q:** `..user1` 之后 user1 还能用吗?
  **A:** 部分能。它是**移动**不是拷贝:Copy 字段(`active`)还能读,`String` 字段(`username`)已经走了。

- **Q:** `rect2.set_width(10)` 为什么没生效,而且之后 rect2 用不了了?
  **A:** 签名是 `mut self`,拿走了所有权,改的是方法自己那一份,结束就 drop 了。要 `&mut self`。

- **Q:** 为什么 `print_area` 不写成 `(rectangle: &Rectangle, circle: &Circle)`?
  **A:** 那样把类型集合写死了。`&impl Shape` 对任何实现了 Shape 的类型都成立,而且是静态分发,零开销。

## Errors I hit

- `error[E0594]: cannot assign to r3.width, as r3 is not declared as mutable` — `mut` 修饰绑定 → `let mut r3`
- `error[E0382]: borrow of moved value: rect2` — `mut self` 吃掉了接收者 → 改成 `&mut self`,并把绑定写成 `let mut`
- `error[E0382]: borrow of moved value: user1.username` — `..user1` 移动了非 Copy 字段 → 需要就先 clone
- `warning: unused Result that must be used` — `write!(f, "{}\n", s);` 丢掉了 Result → `writeln!(f, "{}", s)?;`
- `warning: struct Circle is never constructed` — Circle 实现了 Shape 但没被实例化
- 溢出:`(self.width * self.height) as f64` 先在 u32 里乘再转 → 100000×100000 触发
  `attempt to multiply with overflow`(debug panic,release 静默出错)→ 先转再乘:
  `self.width as f64 * self.height as f64`
- `3.14` → 用 `std::f64::consts::PI`
- `println!())` — 括号错位(注释掉的旧段落里)

## Plan for Day 7

- 泛型 generics
- 补上 struct + enum 合并的 `examples/practice.rs`
