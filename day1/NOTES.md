# Day 1 — Rust 本地环境 + Hello World + Cargo (2026-07-31)

## 今天完成

1. **安装 rustup 工具链**
   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```
   - 装好后:`rustc 1.94.0` + `cargo 1.94.0`,PATH 写入 `~/.zshenv`
   - 踩坑:rustup 尝试写 `~/.tcshrc` 时 Permission denied(该文件属主是 root)。
     不影响使用——zsh 只读 `.zshenv`。可选修复:`sudo chown roddy:staff ~/.tcshrc`

2. **hello_world/** — 裸 rustc 编译
   ```bash
   rustc main.rs && ./main
   ```

3. **hello_cargo/** — cargo 标准项目
   ```bash
   cargo new hello_cargo
   cargo run              # dev profile,带 debuginfo
   cargo build --release  # 优化编译,≈ g++ -O3
   cargo check            # 只查类型不出二进制,日常最常用
   ```

## 关键概念

- `println!` 带 `!` 是**宏**,编译期展开,格式串在编译期检查(C++ 的 printf 是运行期炸)
- `Cargo.toml` = 项目清单(≈ CMakeLists.txt + package.json 合体)
- `Cargo.lock` = 锁定依赖版本(binary 项目要提交进 git)
- dev / release 两个 profile,默认 dev(快编译),release 才开优化

## 明日计划(Day 2)

- 变量与可变性:`let` vs `let mut`,shadowing
- 基础类型 + 所有权(ownership)第一课
