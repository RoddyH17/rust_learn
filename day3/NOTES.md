---
day: 3
date: 2026-08-03
topic: structs
mood: # 1-5, fill in when the day is done
pace: # 1-5, fill in when the day is done
---

# Day 3 — Structs and Methods (2026-08-03)

> ✍️ Blog post: [Day 3 · Structs and Methods](https://roddyh17.github.io/posts/rust/day-3-structs/) (the blog has the story; this file has the technical details)

## What I did

- **structs/** — defined and used three kinds of structs: a normal struct `User`, a tuple struct `Point`, and `Rectangle` with methods
  ```bash
  cd day3/structs && cargo run
  ```
- Implemented the methods `area()` and `can_hold()` and the associated function `square()` for `Rectangle`. The `&self` parameter uses borrowing from Day 2.

## Key concepts

### Structs

- `struct` groups related values into one **custom type**. Each value is a field:
  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }
  ```
- When creating an instance, every field needs a value. Access fields with `.`.
- Mutability applies to the **whole instance**: `let mut user` makes all fields changeable. You cannot mark only one field as mutable.
- **Struct update syntax** `..user2`: take the remaining fields from another instance. Note: String fields are **moved** — ownership of the data transfers to the new instance, so `user2.email` cannot be used afterwards.
- **Tuple structs**: `struct Point(i32, i32);` — fields have no names. Access them by position with `.0`, `.1`. Useful when the meaning of each field is obvious.

### Methods and impl

- A method is a function that belongs to a type. Methods go inside an `impl TypeName { }` block.
- The first parameter is `&self`: the method **borrows** the instance it is called on (the `rect` in `rect.area()`).
  - Read only → `&self`; needs to modify → `&mut self`.
- Call a method with `instance.method()`.
- A function in an `impl` block without `self` is an **associated function**. Call it with `TypeName::function()`. Often used as a constructor, for example `Rectangle::square(20)`.

### Types used today

- `u32` / `u64`: unsigned integers (no negative values); the number is the bit width.
- `i32`: signed integer, Rust's default integer type.
- `bool`: `true` or `false`.

## Plan for Day 4

- Enums and `match` pattern matching
- `Option<T>`: how Rust expresses "there may be no value"
