---
day: 4
date: 2026-08-07
topic: array and slice
mood: # tags, e.g. [zen, frage] — zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 4 — Array and Slice (2026-08-07)

> 📝 Live notes: [`arr_slice_enum/src/main.rs`](arr_slice_enum/src/main.rs) — written while watching, and the source everything else is generated from
>
> ✍️ Blog post: [Day 4](https://roddyh17.github.io/posts/rust/day-4-arrays-and-slices/) (the blog has the story; this file has the technical details)
>
> 📓 Notion: **Day4: Array & Slice**

## Goals — what to master

| # | Topic | You should be able to... | Self-check |
|---|-------|--------------------------|------------|
| 1 | Array type | Declare `[T; N]` and say why `N` is part of the type | Why are `[i32; 3]` and `[i32; 4]` different types? |
| 2 | Array init | Use `[value; count]` and `std::array::from_fn` | What does `[10; 4]` produce? |
| 3 | Iterating | Write index / `.iter()` / `.for_each()` versions | Why can't you write `for i: i32 in 0..4`? |
| 4 | Array is `Copy` | Predict whether a function can modify the caller's array | What decides it — the array, or the element type? |
| 5 | `&mut [T; N]` | Make a function that really does modify the caller's array | Why does the `Copy` version silently do nothing useful? |
| 6 | Slice basics | Write `&s[a..b]`, and the `[..b]` / `[a..]` / `[..]` forms | Is the range inclusive on the right? |
| 7 | UTF-8 boundaries | Say why `&"你好"[0..2]` panics | What is `len()` counting — bytes or characters? |
| 8 | `char` vs `String` | State the size of `char` and the encoding of `String` | Why is `char` 4 bytes but a character in a string 1–4? |
| 9 | Slice methods | Use `len` / `contains` / `swap` / `reverse` / `join` / `windows` / `starts_with` | What does `windows(3)` yield? |
| 10 | String creation | Distinguish a literal from `String::from` | Where do the bytes live in each case? |
| 11 | `String` ↔ `&str` | Convert both directions, three ways for `&String` → `&str` | Which direction costs an allocation, and why? |
| 12 | Concatenation | Use `+` and `format!` | Which one moves its left operand? |
| 13 | `&str` over `&String` | Write a function that accepts both a `String` and a literal | What is deref coercion doing here? |
| 14 | Fat pointer | Say how many bytes a slice reference is | Why 16 and not 8? |
| 15 | DST | Explain why `[T]` and `str` can't be used bare | What does "size not known at compile time" mean? |

## Concepts and examples

Examples live in `arr_slice_enum/src/main.rs`, in 16 numbered sections. Run with `cargo run`.

### Array vs Vec — what the type says

| | `[T; N]` | `Vec<T>` |
|---|---|---|
| Length | Part of the **type**, fixed at compile time | Runtime, can grow |
| Where the data lives | Stack | Heap |
| `Copy`? | Yes **iff `T` is `Copy`** | Never |
| Passing to a function | Copies (if `Copy`), caller unaffected | Moves |

That third row is the one that bites. `[i32; 3]` is `Copy` because `i32` is; `[String; 2]` is not,
because `String` isn't. Same syntax, opposite behaviour — the element type decides.

### The three loop forms

```rust
for index in 0..4 { arr[index] }        // by index — bounds-checked every iteration
for value in arr.iter() { }             // &T
arr.iter().for_each(|v: &i32| { });     // closure
```

`for` takes a **pattern**, not a variable declaration, so `for i: i32 in 0..4` does not parse.
To pin the type, put it on the range: `0..4i32`.

### Slice = pointer + length

| Type | Signature | Length known when |
|---|---|---|
| Array | `[T; N]` | Compile time, part of the type |
| Slice (bare) | `[T]` | Runtime → DST, **cannot be used directly** |
| Slice reference | `&[T]` | Runtime, stored in the fat pointer → this is what you write |

A slice reference is **2 words = 16 bytes** on 64-bit: pointer to the first element + length.
Independent of how many elements it refers to.

### `char` vs string encoding

- `char` is a Unicode scalar value, **always 4 bytes** (`size_of::<char>() == 4`)
- `String` / `str` are **UTF-8**, so one character occupies **1–4 bytes**
- Therefore `"你好".len() == 6` (bytes) while `.chars().count() == 2` (characters)
- Slicing must land on a character boundary or it **panics at runtime**

### String creation and conversion

| | Literal `"hello"` | `String::from("hello")` |
|---|---|---|
| Type | `&'static str` | `String` |
| Bytes live in | Read-only section of the binary | Heap |
| Owns the data? | No | Yes |

`&String` → `&str` is free, three ways: `&s`, `&s[..]`, `s.as_str()`.
`&str` → `String` costs an allocation (`to_string()` / `String::from`), because the bytes have
to be copied onto the heap.

`s1 + &s2` **moves `s1`**; `format!("{}, {}", a, b)` moves nothing.

## Practice

```bash
cd day4/arr_slice_enum && cargo run --example practice
```

Ten exercises, graded 🌟 warm-up / 🌟🌟 the one that teaches something / 🌟🌟🌟 challenge.
Some adapted from [Rust By Practice](https://practice-rust-zh.beatai.org/).

1. 🌟 Length is part of the type — fix a type mismatch by changing one thing.
2. 🌟 Out of bounds is a **runtime** panic, not a compile error. Predict which iteration blows up.
3. 🌟🌟 Is an array `Copy`? Predict, then find out why `[String; 2]` behaves differently.
4. 🌟🌟 Make a function actually modify the caller's array.
5. 🌟🌟 Ranges are half-open — fill in the blanks, then rewrite without the digit `0`.
6. 🌟🌟 UTF-8 boundaries — answer three questions about `"你好"` before uncommenting the panic.
7. 🌟🌟 The borrow checker catching a slice bug. Fix it by reordering statements only.
8. 🌟🌟 `&str` or `&String` — how many of the four call sites compile, and what changes if the
   signature takes `&String`?
9. 🌟🌟🌟 How fat is a fat pointer? Then compare with a plain `&char`.
10. 🌟🌟🌟 DST — why `[i32]` and `str` need the `&`.

## Questions I asked

- **Q:** 为什么 `for index: i32 in 0..4` 编译不过?
  **A:** `for` 后面跟的是**模式**,不是变量声明,所以没有 `: 类型` 这一节。要指定类型写在区间上:`0..4i32`。

- **Q:** 切片有所有权吗?
  **A:** 没有。切片本质上是一种引用 —— 所以 Day 3 那套借用规则原封不动地作用在它身上。

- **Q:** 为什么切片引用是 16 字节而不是 8?
  **A:** 普通引用只存地址(1 个字);切片引用要同时存首元素指针和长度(2 个字),所以叫胖指针。

## Errors I hit

Six of these came from typing notes fast while watching. The first one is not a typo — it is a
language rule.

- `for index: i32 in 0..4` — for 的循环变量不能标类型 → 写成 `for index in 0..4`
- `println!("{:?"), arr)` — 括号错位 → `println!("{:?}", arr)`
- `let mut arr_mut ; [i32;4] = [10;4];` — 分号写成了冒号 → `let mut arr_mut: [i32; 4] = [10; 4];`
- `let arr1 : [i32;4] = [10;4]` — 缺分号
- `format!("{}, {}, s1, s2")` — 变量要作为参数传,不能写进字符串字面量 → `format!("{}, {}", s1, s2)`
- `println!("{}, s")` — 同上,`{}` 没有对应参数,编译不过

## Plan for Day 5

- Enum(今天没学,挪到 Day 5)
- `match` 与 `Option`

