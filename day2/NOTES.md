# Day 2 — Variables, Mutability, and Ownership (2026-08-03)

> ✍️ Blog post: [Day 2 · Ownership](https://roddyh17.github.io/posts/rust/day-2-ownership/) (the blog has the story; this file has the technical details)

## What I did

- **ownership/** — one program that shows, in order: variables and mutability, shadowing, ownership and move, clone, ownership in function calls, references and borrowing
  ```bash
  cd day2/ownership && cargo run
  ```
- Uncommented the two error lines in the code to see the compiler errors (`cannot assign twice to immutable variable` and `value borrowed here after move`) and read what they say. The error messages explain the problem clearly.

## Key concepts

### Variables and mutability

- Variables are **immutable** by default: after `let x = 5;` you cannot assign to x again.
- To change a variable, declare it with `let mut x = 5;`.
- **Shadowing**: declare a new variable with the same name using `let`. Different from `mut`: shadowing creates a new variable, so the type can change (for example, from string to number), and the new variable is still immutable.

### The three ownership rules

1. Every value has one **owner**.
2. A value has only one owner at a time.
3. When the owner goes out of scope, the value is cleaned up.

This is why Rust does not need a garbage collector: the cleanup point is known at compile time.

### Move and clone

- `let s2 = s1;` (where s1 is a String) → ownership **moves** to s2. s1 cannot be used after this.
- To get two copies of the data, call `s1.clone()`.
- Simple types like integers and booleans live on the stack. Assignment copies them, so there is no move.

### References and borrowing

- Passing a value to a function moves ownership into the function. Often we only want to lend the value. `&s` creates a **reference**: it points to the value but does not own it.
- `&T` — immutable borrow: read only.
- `&mut T` — mutable borrow: can modify.
- **The borrowing rule**: at any time, either any number of immutable references, or exactly one mutable reference. The compiler enforces this, which prevents data races at compile time.

### Other

- `-> usize` in a function signature is the return type. The last expression in a function body, written **without a semicolon**, is the return value.

## Plan for Day 3

- Structs: defining my own types
- Methods and `impl` blocks
