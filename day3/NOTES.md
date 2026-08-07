---
day: 3
date: 2026-08-06
topic: ownership and memory
mood: # tags, e.g. [zen] — zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 3 — Ownership and Memory (2026-08-06)

> 📓 Notion: all of **1.2 Ownership and Memory**.
>
> ✍️ Blog post: not written yet.

Day 2 ended on a puzzle: `for name in names` destroyed the array, but the identical loop over numbers did not. This is the chapter that explains it — and it is the chapter the rest of Rust is built on.

The one question underneath everything here: **when a value lives on the heap, who is responsible for freeing it?**

| Language | Answer | Cost |
|---|---|---|
| C / C++ | You are | Forget → leak. Too early → dangling pointer. Twice → double free. |
| Java / Go / Python | A garbage collector | Runtime overhead, unpredictable pauses |
| **Rust** | Exactly one variable **owns** it; freed when the owner leaves scope | Checked at **compile time**, zero runtime cost |

## Goals — what to master

| # | Topic | You should be able to... | Self-check |
|---|-------|--------------------------|------------|
| 1 | Stack vs heap | Say which values go where, and why | Why must stack values have a fixed size? |
| 2 | `String` vs `&str` | Explain why `String` needs the heap and a literal does not | What three words does a `String` keep on the stack? |
| 3 | Scope and drop | Point at the exact line where heap memory is freed | What runs at the closing brace, and who calls it? |
| 4 | Move | Say what is copied and what is invalidated by `let s2 = s1;` | What bug is Rust preventing by killing `s1`? |
| 5 | `clone` | Deep-copy on purpose | Why is this a visible method call and not automatic? |
| 6 | `Copy` | List which types are `Copy` and which can never be | Why can no heap-owning type ever be `Copy`? |
| 7 | Ownership and functions | Predict whether a value survives a function call | How do you use a value after passing it, without references? |
| 8 | References | Write `&T` and `&mut T` parameters | Why is nothing dropped when a reference goes out of scope? |
| 9 | The two rules | State them in one sentence | Which real-world concurrency primitive is this the same idea as? |
| 10 | NLL | Explain why two `&mut` in one block can compile | Does a borrow end at the closing brace or somewhere else? |
| 11 | Dangling references | Say why `fn dangle() -> &String` cannot work | What is the fix, and why does it need no reference at all? |

## Concepts and examples

Examples live in `ownership/src/main.rs`, in 11 numbered sections. Run with `cargo run`.

### Stack and heap

- **Stack**: LIFO, every value a known fixed size. Pushing is fast because there is never a search — the next slot is always the top.
- **Heap**: for data whose size is unknown at compile time or can change. You request space, the allocator finds a spot and returns a **pointer**. The pointer is fixed-size so it lives on the stack; the data lives on the heap.
- Allocating on the heap is slower than pushing: the allocator has to find room and do bookkeeping.
- A `String` is **24 bytes on the stack** (pointer + length + capacity) pointing at bytes on the heap. A string literal is baked into the binary — immutable, no allocation.

### Move — the shallow copy that invalidates

> 重点: 一旦我们用 `let s2 = s1` 这种 statement 了, 那么其实, `s1` 就已经被 free 了

- `let s2 = s1;` copies the three stack words but **not** the heap data.
- If both stayed valid, both would free the same heap block at end of scope — a **double free**, a genuine memory-safety bug.
- Rust's fix is not to deep-copy. It is to **invalidate the original**. That is a move.
- Consequence: **Rust never deep-copies automatically.** Anything expensive has to be asked for by name.

### clone and Copy

- `.clone()` duplicates the heap data. It is a visible method call precisely so that expensive work is visible when you read the code.
- `Copy` is for types entirely on the stack with a fixed size: all integers, floats, `bool`, `char`, and tuples made **only** of `Copy` types. Assignment duplicates them instead of moving.
- Nothing that owns heap data can ever be `Copy` — that is exactly the case where "just duplicate the bytes" is wrong.
- **This is Day 2's loop puzzle solved**: `[i32; 3]` is `Copy` so the `for` loop copied it; `[String; 2]` is not, so the loop moved it.

### Ownership and functions

- Passing a value to a function is a move, exactly like assignment. Returning one moves ownership back out.
- Threading ownership in and back out just to read a value is tedious — which is the argument for references.

### References and the two rules

- `&s` is an address we can follow to data **owned by someone else**. Nothing is dropped when the reference dies, because the reference never owned anything.
- `&mut` is required to modify, and the owner itself must be `mut`.
- While a `&mut` is alive there may be **no other reference at all** — not another `&mut`, not even a `&`.

> **1.** At any given time you may have *either* one mutable reference *or* any number of immutable references.
> **2.** References must always be valid.

一句话记: **读可以并发, 写必须独占.** Same mental model as a read-write lock, except Rust checks it at compile time so it costs nothing at runtime.

| Currently alive | Take another `&T` | Take another `&mut T` |
|---|---|---|
| nothing | ✅ | ✅ |
| `&T` (used later) | ✅ readers can share | ❌ a reader does not expect the value to change |
| `&mut T` (used later) | ❌ | ❌ two writers = data race |

### NLL — Non-Lexical Lifetimes

> 重点: 借用的生命周期不看大括号在哪, 只看它**最后一次被使用**在哪
> 两个借用的"使用区间"不重叠 → 编译器就放行

The borrow checker looks at the range over which a reference is actually **used**, not its lexical scope. So `&s`, `&s`, then `&mut s` all compile in one block, as long as the first two are never touched after the `&mut` appears. Add one `println!` using the old reference and the ranges overlap again — instant error.

### Dangling references

- The classic C++ trap: return a pointer to a local, the local dies on the way out, and you hold an address to freed memory.
- Rust rejects it at **compile time**: *"this function's return type contains a borrowed value, but there is no value for it to be borrowed from."*
- The fix is not to fight the borrow checker but to **return the `String` itself** — move ownership to the caller, and nothing gets freed on the way out.

## Practice

All in `ownership/examples/practice.rs`; run with `cargo run --example practice`. Status: to do.

Adapted from Rust By Practice ([所有权](https://practice-rust-zh.beatai.org/ownership/ownership.html), [引用和借用](https://practice-rust-zh.beatai.org/ownership/borrowing.html)), keeping that site's 🌟 grading.

**1–6 — ownership.**

1. 🌟🌟 **A move, fixed as many ways as you can** — at least four fixes exist for `let y = x; println!("{}, {}", x, y);`. Find them all and say what each costs.
2. 🌟🌟 **Give the ownership back** — make `take_ownership` return what it swallowed, changing only the function.
3. 🌟🌟 **Fix it without deleting a line** — two approaches (change the signature, or change the call). Do both, then pick the one you would ship.
4. 🌟🌟 **Copy instead of clone** — change the tuple so plain assignment duplicates it, and name the property that makes that possible.
5. 🌟 **Mutability on transfer** — one line. Is mutability a property of the value or of the binding?
6. 🌟🌟 **Partial move** — (a) print what is left after `t.0` moves out; (b) use `ref` so `s1`, `s2` and `t` all survive.

**7–10 — borrowing.**

7. 🌟 **Taking a reference** — print an address with `{:p}`; then fix an `&i32` vs `i32` comparison.
8. 🌟 **Passing borrows** — two calls, one character each.
9. 🌟🌟 **The two rules** — (a) two `&mut` at once, fixed by removing part of a line; (b) a failure that is *not* the same rule — read the error; (c) one that compiles, and why.
10. 🌟🌟🌟 **NLL and `ref`** — (a) comment out exactly one line to fix it; (b) the reverse drill, *add* a line that deliberately triggers "cannot borrow `s` as mutable more than once"; (c) `ref` vs `&`, where the second assert proves no copy happened.

### Rubric — check yourself after each one

- **1** — If you only found `.clone()`, you are thinking like a C++ programmer. Borrowing is usually the right answer; cloning is the one that costs a heap allocation.
- **2 / 3** — If you reached for `.clone()` again here, reread 1. The point of these two is that ownership can travel *through* a signature.
- **4** — Say out loud why a tuple containing a `String` can never be `Copy`.
- **5** — Mutability belongs to the **binding**, not the value. If that surprised you, it is worth a note.
- **6** — Partial move is the first time "the value" stops being one indivisible thing. Expect to reread it.
- **7 / 8** — Pure mechanics. If these are slow, the `&` / `&mut` / `*` symbols have not become automatic yet.
- **9** — Distinguishing (a) from (b) is the real test: one violates rule 1, the other is about where `mut` is written.
- **10** — The centrepiece. If you cannot explain (a) *and* produce the error in (b) on demand, NLL has not landed — and it is what makes every later chapter's borrow errors readable.

## Plan for Day 4

- Notion 1.3 The Slice Type — string slices, `&str` in signatures, fat pointers, and why a slice is just a borrow with a length attached
