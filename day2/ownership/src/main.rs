// Day 2 — 变量、可变性、所有权(ownership)与借用(borrowing)
//
// 所有权是 Rust 最核心的概念:每一个值都有一个"主人"(owner),
// 主人离开作用域时,值会被自动清理。这让 Rust 不需要垃圾回收器
// 也能保证内存安全。

fn main() {
    // ---------- 1. 变量与可变性 ----------

    // 变量默认是不可变的(immutable):绑定之后不能再改
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // ← 取消注释会编译报错:cannot assign twice to immutable variable

    // 想要可变,必须显式写 mut
    let mut y = 5;
    y = y + 1;
    println!("y = {}", y);

    // shadowing(遮蔽):用 let 重新声明同名变量,旧的被"盖住"
    // 和 mut 不同:shadowing 可以改变类型,而且新变量本身仍是不可变的
    let spaces = "   ";        // 字符串
    let spaces = spaces.len(); // 变成了数字——同一个名字,新的变量
    println!("spaces = {}", spaces);

    // ---------- 2. 所有权与 move ----------

    // String 是存放在堆上的可增长字符串,它的值有唯一的主人
    let s1 = String::from("hello");

    // 把 s1 赋给 s2,所有权发生了"移动"(move):现在 s2 是主人,s1 失效了
    let s2 = s1;
    // println!("{}", s1);  // ← 取消注释会报错:value borrowed here after move
    println!("s2 = {}", s2);

    // 如果真的想要两份数据,用 clone() 显式复制
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3); // 两个都能用,因为是两份独立的数据

    // 注:整数等简单类型存在栈上,赋值时直接复制,不会发生 move
    let a = 10;
    let b = a;
    println!("a = {}, b = {}", a, b); // 都能用

    // ---------- 3. 函数与所有权 ----------

    let s = String::from("ownership");
    takes_ownership(s); // s 的所有权交给了函数
    // println!("{}", s);  // ← 报错:s 已经被移走了

    // ---------- 4. 引用与借用 ----------

    // 大多数时候我们不想交出所有权,只想"借"给函数看一眼
    // &s 创建一个引用(reference):指向值但不拥有它,这个行为叫借用(borrow)
    let s = String::from("borrowing");
    let len = calculate_length(&s);
    println!("'{}' 的长度是 {}", s, len); // s 还能用,因为只是借出去过

    // 可变引用 &mut:允许函数修改借来的值
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("修改后:{}", s);

    // 借用规则(编译器强制执行):
    // 同一时刻,要么有任意多个不可变引用,要么只有一个可变引用
    // 这条规则在编译期就杜绝了数据竞争
}

// 参数类型是 String:调用时值的所有权会移动进来,函数结束时值被清理
fn takes_ownership(text: String) {
    println!("我拿到了所有权:{}", text);
}

// 参数类型是 &String:只是借用,不拿所有权
// -> usize 表示函数返回一个 usize 类型的数(无符号整数,常用来表示长度)
fn calculate_length(text: &String) -> usize {
    text.len() // 函数最后一个表达式不写分号,就是返回值
}

// &mut String:可变借用,可以修改原值
fn append_world(text: &mut String) {
    text.push_str(", world");
}
