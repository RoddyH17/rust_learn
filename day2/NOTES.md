---
day: 2
date: 2026-08-04
topic: variables, types, functions, flow control
mood: [zen, frage] # zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 2 — Variables, Types, Functions, Flow Control (2026-08-04)

> ✍️ Blog post: [Day 2 · Variables](https://roddyh17.github.io/posts/rust/day-2-variables/) (the blog has the story; this file has the technical details)
>
> 📓 Notion: everything in **1.1 Variables and Mutability**, up to but not including **1.2 Ownership and Memory**.

## Goals — what to master

| # | Topic | You should be able to... | Self-check |
|---|-------|--------------------------|------------|
| 1 | `const` | Declare a compile-time constant with a type annotation | Why does `const` require the type, and why can it never change? |
| 2 | `static` | Declare a global variable and explain how it differs from `const` | Where does a `static` live, and how long does it exist? |
| 3 | Mutability | Say for each of `const` / `static` / `let` / `let mut` whether it can change | Which one is the only reassignable local variable? |
| 4 | Shadowing | Rebind a name with `let` and change its type | What are the two differences between shadowing and `mut`? |
| 5 | Tuples | Destructure a tuple, and return two values from one function | What is `t.0` doing that indexing an array is not? |
| 6 | Arrays | Declare `[T; N]`, and say why the length is part of the type | Why is an out-of-bounds index a *runtime* panic and not a compile error? |
| 7 | Functions | Annotate every parameter type; return with a trailing expression | Why does Rust require parameter types but infer local variable types? |
| 8 | Statements vs expressions | Predict the value of a block, with and without a trailing semicolon | What is the type of a block ending in `;`? |
| 9 | `if` as an expression | Assign the result of an `if` to a variable | Why must both arms have the same type? |
| 10 | Loops | Use `for` / `while` / `loop`, ranges, `.rev()`, `.enumerate()`, labels | When can `break` carry a value, and when can it not? |
| 11 | Loops and ownership | Say what `for x in v` / `&v` / `&mut v` each give you | Why does the array of `i32` survive the loop but the array of `String` not? |
| 12 | References | Write a function taking `&T` and one taking `&mut T` | State the borrowing rule in one sentence |

## Concepts and examples

Examples live in `variables/src/main.rs`, in numbered sections. Run with `cargo run`.

### const vs static vs let

| | `const` | `static` | `let` | `let mut` |
|---|---|---|---|---|
| Where | module or function | module (global) | inside a function | inside a function |
| Can change? | never | no (without `unsafe`) | no | yes |
| Type annotation | required | required | optional (inferred) | optional (inferred) |
| Memory | inlined at each use, no address | one fixed address, whole program | stack, dropped at end of scope | stack, dropped at end of scope |

- Naming convention for `const` and `static`: `SCREAMING_SNAKE_CASE`.
- `static mut` exists but every access requires `unsafe`; Rust makes global mutable state hard on purpose.

### Shadowing

- `let` with an existing name creates a **new variable** that hides the old one.
- Two differences from `mut`: (1) the type may change; (2) the new variable is still immutable.

### Tuples and arrays

- A tuple holds **different** types: `let x: (i32, f64, u8) = (500, 6.4, 1);`, read with `x.0`, `x.1`, `x.2`.
- Destructuring is the idiomatic way in: `let (a, b, c) = tup;` — positional, so the *names* mean nothing and the *order* means everything.
- An array holds **one** type and has a **fixed length**, and that length is part of the type: `[i32; 5]`. `let a = [3; 5]` fills five slots with `3`.
- Arrays live on the **stack**; a `Vec` is the heap-allocated version that can grow or shrink.
- `arr[i]` panics at runtime on an out-of-range index; `arr.get(i)` returns `Option<T>` and is the safe form.
- `std::mem::size_of_val(&['a','b','c'])` is **12**, not 3 — a `char` is a 4-byte Unicode scalar value, not a byte.

### Functions, statements, expressions

- Every parameter needs a type annotation. That is a deliberate design choice: it means the compiler almost never needs annotations anywhere else, and it can give better error messages.
- Return type goes after `->`. The **last expression, with no semicolon**, is the return value.
- The one rule that explains most Day 2 compiler errors:
  - **no semicolon = expression** — it has a value
  - **semicolon = statement** — its value is `()`
- So `fn sum(x: i32, y: i32) -> i32 { x + y; }` fails: the `;` makes the body return `()`.
- A function with no `->` returns the unit type `()`. If it only prints, that is what you want.
- A block is an expression too, so `let y = { let x = 3; x + 3 };` gives `y == 6`.

### if and loops

- `if` is an expression: `let number = if condition { 5 } else { 6 };`. **Both arms must have the same type**, because the variable gets exactly one type.
- Ranges: `1..100` is left-closed right-open (1–99); `1..=100` includes 100. Same convention as slice indexing.
- `.rev()` reverses a range; `.iter().enumerate()` gives `(index, value)` pairs with no bounds-check cost and no chance of going out of range — prefer it over `while index < len` with `a[index]`.
- **`break value` works only in `loop`**, not in `for` or `while`, because those two are not guaranteed to reach the `break`:
  ```rust
  let result = loop { counter += 1; if counter == 10 { break counter * 2; } };  // 20
  ```
- Labels start with a single quote: `'outer: loop { ... break 'outer; }`. `continue 'label` works too.

### Loops and ownership — the important one

`for x in collection` **moves** the collection into the loop. This is the ownership rule wearing loop syntax:

| Written as | Desugars to | Each item is | Collection afterwards |
|---|---|---|---|
| `for x in &v` | `v.iter()` | `&T` | usable |
| `for x in &mut v` | `v.iter_mut()` | `&mut T` | usable, and modified |
| `for x in v` | `v.into_iter()` | `T` | **consumed** — unless the element type is `Copy` |

- `[String; 2]` is destroyed by `for name in names`; `[i32; 3]` is not, because `i32` is `Copy` so the array is copied into the loop.
- `Vec` is never `Copy`: copying it would mean either two `Vec`s pointing at one heap block (double free) or a hidden deep copy. Rust refuses both, so `Vec` can only move.
- Default to writing `&` unless you actually mean to consume the collection.
- Note: before Rust 1.53, `for e in array` gave references. Old code may look different.

### References and borrowing

- `&s` lends a value without giving up ownership; `&mut s` lends it writable (the variable itself must be `mut`).
- **The borrowing rule**: at any time, either any number of read-only references, or exactly one mutable reference.

### Note on "static methods"

What other languages call a "static method" is an **associated function** in Rust (`impl` block, no `self`, called as `TypeName::function()`). Not related to the `static` keyword.

## Practice

All in `variables/examples/practice.rs`; run with `cargo run --example practice`.

**1–5 — binding a value.** Status: all five completed.

1. **Make it compile, two ways** — fix a reassignment error with `mut`, then with shadowing.
2. **const and static** — declare `MAX_SCORE` (const) and `COURSE` (static), print, try to reassign, read the error.
3. **Shadowing with a type change** — string `"42"` → number `43`, one variable name. Expected: `42 + 1 = 43`
4. **Write a borrowing function** — `fn shout(text: &mut String)` appending `"!"`. Expected: `cheer = go!!`
5. **Break the borrowing rule on purpose** — predict which line the compiler rejects, then verify.

**6–10 — using a value.** Adapted from [Rust By Practice](https://practice-rust-zh.beatai.org/), same 🌟 grading. Status: to do.

6. 🌟🌟 **Statements vs expressions** — (a) make `sum_of` return `3` by deleting one character; (b) make a block evaluate to `3`, two different ways; (c) explain why `let v = (let x = 3);` can never work.
   *Source: [语句与表达式](https://practice-rust-zh.beatai.org/basic-types/statements-expressions.html)*
7. 🌟🌟 **`if` is an expression** — one block, two errors: a type mismatch between the arms and a stray semicolon. Predict both before compiling.
   *Source: [流程控制 · if/else](https://practice-rust-zh.beatai.org/flow-control.html)*
8. 🌟🌟 **Tuples** — (a) destructure `(1, 6.4, "hello")` in one line so the asserts pass (the order is deliberately not the obvious one); (b) find the argument that makes `sum_multiply` return `(5, 6)`.
   *Source: [元组](https://practice-rust-zh.beatai.org/compound-types/tuple.html)*
9. 🌟🌟 **Arrays** — (a) predict `size_of_val(&['a','b','c'])`; (b) build a 100-element array of `1` without typing 100 ones; (c) `.get()` vs `[]`, and why the compiler lets an obviously out-of-range index through.
   *Source: [数组](https://practice-rust-zh.beatai.org/compound-types/array.html)*
10. 🌟🌟🌟 **Loops and ownership** — the day's centrepiece. (a) one array survives its `for` loop and one does not; predict which, fix with one character, explain via `Copy`; (b) make `loop` return `20` through `break`; (c) hand-trace the nested `'outer`/`'inner1`/`'inner2` puzzle before running it.
    *Source: [流程控制 · for / loop](https://practice-rust-zh.beatai.org/flow-control.html)*

### Rubric — check yourself after each one

- **6** — Can you state the semicolon rule in one sentence without looking? If not, redo (c).
- **7** — Did you predict *both* errors? Getting only the type error means the semicolon rule from 6 has not landed yet.
- **8** — If the destructuring order surprised you: tuple patterns match by **position**, never by name.
- **9** — If you guessed 3 for the size: a `char` is a Unicode scalar value (4 bytes), not a byte. If you expected a compile error on `people[2]`: index bounds are a runtime check; `.get()` is how you make it a compile-time-safe `Option`.
- **10** — The real test is writing the three-row borrow table from memory. If you cannot, that is the thing to review before Day 3, because ownership is the whole of Notion 1.2.

## Questions I asked

- **Q:** I put a breakpoint on the `static LANGUAGE` line and the debugger never paused. Can Rust not step line by line?
  **A:** It can — but `const` and `static` are evaluated at **compile time**. That line has no runtime instruction, so a breakpoint there never fires. Breakpoints belong on executable statements inside functions.
- **Q:** Why were Variables / Watch / Call Stack all empty after the run?
  **A:** Those panels only show data **while the program is paused**. The breakpoint never hit, the program ran to the end and exited, so there was nothing left to inspect.
- **Q:** What are the "Registers" under Variables, and why are the values hex like `0x00...`?
  **A:** Raw CPU state (ARM registers on this Mac). The debugger shows them because it can, but they are for assembly-level debugging — ignore them; use Local and Static. Bonus observed: `variables::LANGUAGE` shows up under Static (it has an address); `MAX_DAYS` does not exist at runtime at all (const, inlined) — the const/static difference made visible.
- **Q:** What does `variables::main` in the call stack mean? Isn't `main` syntax?
  **A:** `main` is a normal function; `variables` is this crate's name (from Cargo.toml) and `::` is the path separator, so it reads "the main function of the variables crate". The frames below it (`std::rt::lang_start` etc.) are the standard library's startup code — main itself is called by the runtime.
- **Observation (exercise 5):** after `let r3 = &mut word;`, using `r1` again is rejected — but if `r1`/`r2` are never used after that point, the code compiles. A borrow ends at its **last use**, not at the end of the scope.

## Errors I hit

- `error[E0384]: cannot assign twice to immutable variable` — reassigned a plain `let` — fix with `let mut`, or rebind with shadowing.
- `error: cannot assign to this expression` (on `MAX_SCORE = 50;`) — a `const` is not a variable at all — deleted the line.
- `error[E0502]: cannot borrow ... as mutable because it is also borrowed as immutable` — took `&mut word` while `r1`/`r2` were still used later — either drop the read-only borrows (stop using them) or take the `&mut` after their last use.

## Plan for Day 3

- Notion 1.2 Ownership and Memory — stack vs heap, move, `clone`, the `Copy` trait
- References and borrowing, the two rules, NLL
- This is where today's `for` loop puzzle gets its real answer
