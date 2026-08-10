---
day: 5
date: 2026-08-10
topic: enum_struct_match
mood: # tags, e.g. [zen, frage] — zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 5 — Enum and Match (2026-08-10)

> 📝 Live notes: [`enum_struct_match/src/main.rs`](enum_struct_match/src/main.rs) — written while watching, and the source everything else is generated from
>
> ✍️ Blog post: [Day 5](https://roddyh17.github.io/posts/rust/day-5-enums-and-match/) (the blog has the story; this file has the technical details)

Struct was **not** covered today — the crate is named `enum_struct_match` because struct comes next
and will land in the same crate. Practice is deferred until then, so one exercise set covers both.

## Goals — what to master

| # | Topic | You should be able to... | Self-check |
|---|-------|--------------------------|------------|
| 1 | Variant shapes | Write tuple / struct / unit variants in one enum | What can a Rust variant carry that a C++ one can't? |
| 2 | Constructing | Build a value of each variant shape | What is `Pets::Cat` on its own, if `Cat(String)`? |
| 3 | `impl` on enum | Distinguish a method from an associated function | Why does `dog.log()` fail but `Pets::log()` work? |
| 4 | `#[derive]` | Say what `derive` generates and when it runs | What does it cost at runtime? |
| 5 | `match` exhaustiveness | Cover every variant; use `_` for open types | Why can't you `match` an `i32` without `_`? |
| 6 | Patterns vs values | Explain why `o => {}` makes later arms unreachable | Is `o` a comparison or a binding? |
| 7 | `if let` | Use it for the one-case check | Why does `if let cat = Pets::Cat` warn? |
| 8 | `Option<T>` | Handle `Some`/`None`; recognise APIs that return it | What does `HashMap::get` actually return? |
| 9 | `Result<T, E>` | Handle `Ok`/`Err`; return it from `main` | What does the `E` carry that `None` doesn't? |
| 10 | Conversions | Use `ok_or` and `ok()` | Which direction needs an extra argument, and why? |
| 11 | Combinators | Replace a `match` with `map` / `and_then` / `or_else` | When is `unwrap` a lie? |
| 12 | Memory layout | Compute the size of an enum | Why is `Mixed { A(u8), B(u32) }` 8 bytes and not 5? |

## Concepts and examples

Everything lives in `enum_struct_match/src/main.rs`, written progressively — each section commented
out as the next one started. Run with `cargo run`.

### Enum in Rust vs TS / C++

| | C++ / TS `enum` | Rust `enum` |
|---|---|---|
| Variant is | A name for an integer | A **variant of a sum type** |
| Can carry data | No (C++ needs a separate union/variant) | Yes, per variant, any shape |
| Exhaustive check | No | Yes — `match` fails to compile if a case is missed |

```rust
#[derive(Debug)]
enum Pets {
    Cat(String),                        // tuple variant
    Dog { names: String, ages: usize }, // struct variant
    Bird,                               // unit variant
}
```

**`Pets::Cat` on its own is a function, not a value.** For a tuple variant, the bare path is the
constructor: its type is `fn(String) -> Pets`. `let cat = Pets::Cat;` compiles and binds a function.
Only unit variants (`Pets::Bird`) are values as written.

### Methods vs associated functions

| | Method | Associated function |
|---|---|---|
| Signature | takes `&self` / `&mut self` / `self` | no `self` |
| Belongs to | the **value** | the **type** |
| Called with | `dog.speak()` | `Pets::log(..)` |

`::` is a namespace lookup, `.` is a value lookup. Constructors (`String::from`, `Vec::new`) are
just associated functions by convention — that's why it's `::new()`, never `.new()`.

### `#[derive(..)]`

An attribute macro, and specifically a **procedural macro**: at compile time the compiler passes the
type's syntax tree (TokenStream) to the macro, which returns code that is spliced into the source.

- Pure compile-time codegen, **zero runtime overhead**
- Output is identical to what you'd hand-write
- `#[derive(Debug, Clone, PartialEq)]` ≡ three hand-written `impl` blocks
- `{:?}` needs `Debug`; `==` needs `PartialEq`

### `match` and patterns

- **Exhaustive**: every variant covered, or it doesn't compile
- A **lowercase identifier in a pattern is a binding, not a comparison** — it matches any value.
  `o => {}` makes every following arm unreachable (compiler: *`o` matches any value* /
  *no value can reach this*)
- `_` is the same catch-all, without binding a name
- Open types like `i32` therefore require `_`: you can't enumerate 4 billion arms
- `if let` needs a **refutable** pattern. `if let cat = Pets::Cat` warns *irrefutable `if let`
  pattern* — `cat` is a binding, so it always matches. Write `if let Pets::Cat = pet`.

### `Option` and `Result`

| | `Option<T>` | `Result<T, E>` |
|---|---|---|
| Variants | `Some(T)` / `None` | `Ok(T)` / `Err(E)` |
| Says | "there is no value" | "it failed, because **E**" |
| Typical source | `HashMap::get` → `Option<&V>`, `iter().last()` → `Option<&T>` | `"24".parse()` → `Result<usize, ParseIntError>` |

`main` defaults to returning `()`, but may return `Result<(), E>` — then `Err` becomes a non-zero
exit code. End such a `main` with `Ok(())`.

Two hats worn by `_`:

- in a **pattern** (`Some(_)`) — there is a value, I don't care what it is
- in a **type** (`Vec<_>`) — compiler, infer this
- `()` is neither: a concrete type, the empty tuple

### Converting between them

Why the conversion exists: the two carry different amounts of information, and the two sides of a
function boundary often need different amounts. Conversion happens when you cross an abstraction
layer.

```rust
opt.ok_or("error")   // Option → Result — you supply the reason it never had
res.ok()             // Result → Option — the reason is discarded
```

`ok_or` takes an argument, `ok()` loses information. That asymmetry is the whole difference.

### Combinators — getting out of `match`

```rust
option.map(|n| n + 1)         // transform if present, stay in Option
option.and_then(|v| Some(v))  // like map, but the closure returns an Option
option.or_else(|| Some(1))    // fallback when None
option.unwrap()               // leave Option — PANICS on None
option.expect("msg")          // same, with your own message
```

### Memory layout — tagged union

Memory = **discriminant tag + largest variant payload + alignment padding**.

| Enum | `size_of` | Why |
|---|---|---|
| `enum A { a, b, c }` | 1 | tag only; 3 cases fit in a byte |
| `enum One { A = 255 }` | **0** | one variant → ZST, even with an explicit discriminant |
| `enum Never {}` | 0 | no values exist |
| `enum Mixed { A(u8), B(u32) }` | 8, align 4 | **not 5** — the `u32` forces padding |
| `enum Big { A(String), B }` | 24 | = `size_of::<String>()`, niche optimization |
| `Option<&i32>` | 8 | = `size_of::<&i32>()`, null pointer is the niche |

Single-variant enum is a ZST because the type has exactly one possible value — "which value is it"
carries no information, so there is nothing to store, and the tag would be unreadable anyway.

`Mixed` padding:

```text
offset:  0        1  2  3        4  5  6  7
        [ tag ] [ 3 bytes pad ] [    u32    ]
```

**Niche optimization**: when a payload has an impossible bit-pattern (a `String`'s pointer is never
null), the compiler uses that pattern as the tag instead of allocating one. So `Option<&T>` is
byte-for-byte a nullable pointer — the same layout C would have written, with the check enforced.

All figures above measured with `std::mem::size_of` / `align_of`, not quoted.

## Practice

Deferred — see the note at the top. `examples/practice.rs` is still the empty scaffold:

```bash
cd day5/enum_struct_match && cargo run --example practice
```

## Questions I asked

- **Q:** `let cat = Pets::Cat;` 为什么能编译,但 `cat` 用起来不对?
  **A:** 元组变体的裸路径是**构造函数**,类型是 `fn(String) -> Pets`。要值就得给参数:
  `Pets::Cat("Tom".to_string())`。只有单元变体的裸路径本身就是值。

- **Q:** 为什么 `dog.log()` 报错说找不到方法?
  **A:** `log` 没有 `self`,是关联函数,属于类型而不属于值,只能通过路径调用:`Pets::log(..)`。

- **Q:** `match num { o => {} 1 => {} }` 为什么 `1` 那一支永远进不去?
  **A:** 模式里的小写标识符是**绑定**,不是比较。`o` 匹配一切,等于带名字的 `_`。

- **Q:** Option 和 Result 为什么要互相转换?
  **A:** 两者携带的信息量不同,而函数边界两侧对信息的需求经常不一样。Option 说的是「没有」,
  Result 说的是「失败了,原因是 X」。转换发生在你跨越抽象层的时候。

## Errors I hit

- `let cat = Pets::Cat;` — 元组变体缺参数,拿到的是构造函数而不是值 → `Pets::Cat("Tom".to_string())`
- `dog.log("alen".to_string())` — 关联函数不能用点调用 → `Pets::log("alen".to_string())`
- `warning: unreachable pattern` — `o => {}` 是绑定模式,吞掉了后面所有分支 → 用具体值或把 `_` 放最后
- `warning: irrefutable if let pattern` — `if let cat = Pets::Cat` 恒真 → `if let Pets::Cat = pet`
- `let len: Result<uszie, ParseIntError>` — 拼写 → `usize`

## Plan for Day 6

- struct(字段、`impl`、关联函数与方法)
- 补上 enum + struct 合并的 `examples/practice.rs`
