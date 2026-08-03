// Day 1 — cargo 管理的标准项目结构
//
// cargo 是 Rust 官方的项目管理工具,常用命令:
// cargo new hello_cargo   → 新建项目:生成 Cargo.toml + src/main.rs,并初始化 git
// cargo build             → 编译,产物放在 target/debug/
// cargo run               → 编译 + 运行,一步到位
// cargo build --release   → 优化编译,产物放在 target/release/,程序跑得更快但编译更慢
// cargo check             → 只检查代码有没有错,不生成可执行文件,速度最快,日常最常用

fn main() {
    // let 声明一个变量:给一个值起个名字,后面可以用
    let name = "Roddy";
    let day = 1;
    // 格式串里的 {} 是占位符,后面的变量会按顺序填进去
    println!("Hello, {}! This is day {} of learning Rust.", name, day);
}
