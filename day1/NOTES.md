---
day: 1
date: 2026-07-31
topic: hello world & cargo
mood: 3 # 1-5, how the day felt
pace: 2 # 1-5, how fast progress felt
---

# Day 1 — Setup + Hello World + Cargo (2026-07-31)

> ✍️ Blog post: [Day 1 · Hello Rust](https://roddyh17.github.io/posts/rust/day-1-hello-rust/) (the blog has the story; this file has the technical details)

## What I did

1. **Installed the Rust toolchain with rustup**
   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```
   - rustup is the Rust installer and version manager. After install: `rustc 1.94.0` (compiler) + `cargo 1.94.0` (project tool). PATH is added to `~/.zshenv`.
   - Problem I hit: rustup got "Permission denied" writing to `~/.tcshrc` (that file is owned by root). This does not matter for zsh, which only reads `.zshenv`. Optional fix: `sudo chown roddy:staff ~/.tcshrc`

2. **hello_world/** — compile directly with rustc
   ```bash
   rustc main.rs && ./main
   ```
   - What "compiling" means: rustc turns the `.rs` source file into a binary file the computer can run.

3. **hello_cargo/** — a standard cargo project
   ```bash
   cargo new hello_cargo  # create a project: folder structure + git repo
   cargo run              # compile and run; use this for daily work
   cargo build --release  # optimized build; the program is faster but compiling is slower
   cargo check            # check for errors only, no binary; fastest, use it often while writing
   ```

## Key concepts

- A program starts at `fn main()`. `fn` defines a function.
- `println!` prints a line. The `!` means it is a **macro**, not a normal function. The `{}` in the format string is a placeholder; variables fill it in order. If the format string is wrong, the compiler reports it at compile time, not when the program runs.
- `let` declares a variable: it gives a name to a value.
- `Cargo.toml` = project manifest: project name, version, and dependencies.
- `Cargo.lock` = exact versions of all dependencies, so builds are the same on any machine (commit it for binary projects).
- Two build profiles: dev (default, fast compile, easy to debug) and release (optimized).

## Plan for Day 2

- Variables: `const`, `static`, `let` vs `let mut`, shadowing
- References and borrowing
