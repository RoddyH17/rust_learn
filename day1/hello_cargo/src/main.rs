// Day 1 — cargo 管理的标准项目结构
//
// cargo new hello_cargo   → 生成 Cargo.toml + src/main.rs + git
// cargo build             → 编译到 target/debug/
// cargo run               → 编译 + 运行
// cargo build --release   → 优化编译到 target/release/(等价 C++ 的 -O3)
// cargo check             → 只做类型检查不生成二进制,比 build 快很多
//
// 对比 C++:这一套等于 CMake + vcpkg/conan + git init 一条命令全包了

fn main() {
    let name = "Roddy";
    let day = 1;
    // {} 占位符自动调用 Display trait,类似 C++20 std::format("{}")
    println!("Hello, {}! This is day {} of learning Rust.", name, day);
}
