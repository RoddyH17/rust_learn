// Day 1 — 最原始的编译方式:直接用 rustc,不经过 cargo
// 编译:rustc main.rs
// 运行:./main
//
// 对应 C++ 的:g++ main.cpp -o main && ./main

fn main() {
    // println! 是宏(macro),不是函数 —— 感叹号是标志
    // C++ 里最接近的是 std::format / fmt::print,但那些是函数+模板,不是宏展开
    println!("Hello, world!");
}
