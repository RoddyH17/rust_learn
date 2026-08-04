---
day: 2
date: 2026-08-04
topic: variables
mood: [zen, frage] # zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 2 — Variables: const, static, Mutability, Shadowing, References (2026-08-04)

> ✍️ Blog post: [Day 2 · Variables](https://roddyh17.github.io/posts/rust/day-2-variables/) (the blog has the story; this file has the technical details)

## Goals — what to master

| # | Topic | You should be able to... | Self-check |
|---|-------|--------------------------|------------|
| 1 | `const` | Declare a compile-time constant with a type annotation | Why does `const` require the type, and why can it never change? |
| 2 | `static` | Declare a global variable and explain how it differs from `const` | Where does a `static` live, and how long does it exist? |
| 3 | Mutability | Say for each of `const` / `static` / `let` / `let mut` whether it can change | Which one is the only reassignable local variable? |
| 4 | Shadowing | Rebind a name with `let` and change its type | What are the two differences between shadowing and `mut`? |
| 5 | References | Write a function taking `&T` and one taking `&mut T` | State the borrowing rule in one sentence |

## Concepts and examples

Examples live in `variables/src/main.rs`, in four numbered sections. Run with `cargo run`.

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

### References and borrowing

- `&s` lends a value without giving up ownership; `&mut s` lends it writable (the variable itself must be `mut`).
- **The borrowing rule**: at any time, either any number of read-only references, or exactly one mutable reference.

### Note on "static methods"

What other languages call a "static method" is an **associated function** in Rust (`impl` block, no `self`, called as `TypeName::function()`). Not related to the `static` keyword.

## Practice

All in `variables/examples/practice.rs`; run with `cargo run --example practice`. Status: all five completed.

1. **Make it compile, two ways** — fix a reassignment error with `mut`, then with shadowing.
2. **const and static** — declare `MAX_SCORE` (const) and `COURSE` (static), print, try to reassign, read the error.
3. **Shadowing with a type change** — string `"42"` → number `43`, one variable name. Expected: `42 + 1 = 43`
4. **Write a borrowing function** — `fn shout(text: &mut String)` appending `"!"`. Expected: `cheer = go!!`
5. **Break the borrowing rule on purpose** — predict which line the compiler rejects, then verify.

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

- Structs: defining my own types
- Methods and `impl` blocks
