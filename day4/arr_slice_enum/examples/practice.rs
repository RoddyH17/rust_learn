// Practice — Day 4: Array & Slice
// Run with: cargo run --example practice
//
// This file must always compile as shipped. Every exercise asks you to
// uncomment code, fix it, or write a few lines yourself — the broken code
// lives in comments so the file never stops running.
//
// Grading:  🌟 warm-up   🌟🌟 the one that actually teaches something   🌟🌟🌟 challenge
// Some exercises adapted from Rust By Practice:
//   https://practice-rust-zh.beatai.org/compound-types/array.html
//   https://practice-rust-zh.beatai.org/compound-types/slice.html

fn main() {
    println!("--- Exercise 1: 长度是类型的一部分 (🌟) ---");
    // 取消注释,让它编译通过。只允许改**一处**。
    // 提示:[i32; 3] 和 [i32; 4] 是两个不同的类型。
    //
    // let a: [i32; 4] = [1, 2, 3];
    // println!("{:?}", a);

    println!("--- Exercise 2: 越界是运行时 panic,不是编译错误 (🌟) ---");
    // 下面这段能通过编译,但跑起来会 panic。先预测它在第几次循环炸,再取消注释验证。
    // 想想:为什么编译器不拦下来?(和 Exercise 1 对比 —— 长度在类型里,索引却不在)
    //
    // let a = [1, 2, 3];
    // for i in 0..4 {
    //     println!("{}", a[i]);
    // }
    let a = [1, 2, 3];
    println!("安全写法: {:?}", a.get(3)); // get 返回 Option,不会炸

    println!("--- Exercise 3: 数组是 Copy 吗 (🌟🌟) ---");
    // 先预测这两个 println! 各打印什么,再运行核对。
    // 关键问题:什么决定了数组是不是 Copy?
    let nums: [i32; 3] = [1, 2, 3];
    takes_array(nums);
    println!("nums 还在吗: {:?}", nums);
    //
    // 然后把下面这段取消注释 —— 它编译不过。为什么?
    // let strs: [String; 2] = [String::from("a"), String::from("b")];
    // takes_string_array(strs);
    // println!("{:?}", strs);

    println!("--- Exercise 4: 让函数真的改到原数组 (🌟🌟) ---");
    // 只改 zero_out 的签名和这里的调用,让断言通过。不要在 main 里手写循环。
    let mut data: [i32; 3] = [7, 8, 9];
    zero_out(&mut data);
    assert_eq!(data, [0, 0, 0]);
    println!("data = {:?}", data);

    println!("--- Exercise 5: 左闭右开 (🌟🌟) ---");
    // 填空,让两个断言都通过。注意区间是左闭右开的。
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    assert_eq!(hello, "hello");
    assert_eq!(world, "world");
    // 再用**省略写法**写出同样的 hello,不要出现数字 0:
    let hello2 = &s[..5];
    assert_eq!(hello, hello2);
    println!("{} / {}", hello, world);

    println!("--- Exercise 6: UTF-8 边界 (🌟🌟) ---");
    // cn 是 "你好"。先回答三个问题,再取消注释验证:
    //   (a) cn.len() 是多少?  (b) cn.chars().count() 是多少?  (c) &cn[0..2] 会怎样?
    let cn = String::from("你好");
    println!("len={} chars={}", cn.len(), cn.chars().count());
    //
    // let boom = &cn[0..2];   // 取消注释会 panic,读一下报错信息
    // println!("{}", boom);
    println!("正确切法: {:?}", &cn[0..3]);

    println!("--- Exercise 7: 切片让编译器替你抓 bug (🌟🌟) ---");
    // 下面这段编译不过。先说出报错是什么,再想清楚为什么 —— 这正是 Day 3 的借用规则。
    // 然后只调整**语句顺序**让它通过,不要删掉任何一行。
    //
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // println!("the first word is: {word}");
    let mut s2 = String::from("hello world");
    let word = first_word(&s2);
    println!("the first word is: {word}");
    s2.clear(); // word 在这之后不再被使用,所以这样是可以的(NLL)
    println!("s2 cleared, len = {}", s2.len());

    println!("--- Exercise 8: &str 还是 &String (🌟🌟) ---");
    // first_word 的参数是 &str。下面四种调用里,有几种能通过编译?
    // 先预测,再运行。然后想想:如果签名改成 &String,还剩几种能用?
    let owned = String::from("hello world");
    println!("{:?}", first_word(&owned[0..6]));
    println!("{:?}", first_word(&owned[..]));
    println!("{:?}", first_word(&owned)); // 这一行为什么能通过?
    println!("{:?}", first_word("hello world"));

    println!("--- Exercise 9: 胖指针有多胖 (🌟🌟🌟) ---");
    // 填空,让断言通过。注意:问的是**切片引用**的大小,不是它引用的数据的大小。
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    assert_eq!(std::mem::size_of_val(&slice), 16);
    // 那么下面这个呢?先预测再取消注释 —— 它和上面一样吗?为什么?
    // let r: &char = &arr[0];
    // assert_eq!(std::mem::size_of_val(&r), 8);
    println!("size_of_val(&slice) = {}", std::mem::size_of_val(&slice));

    println!("--- Exercise 10: DST,为什么必须带 & (🌟🌟🌟) ---");
    // 下面两行编译不过。报错里会出现 "doesn't have a size known at compile-time"。
    // 修好它们 —— 只允许加字符,不允许删行。
    //
    // let s1: [i32] = arr2[0..2];
    // let s2: str = "hello, world";
    let arr2 = [1, 2, 3];
    let s1: &[i32] = &arr2[0..2];
    let s2: &str = "hello, world";
    println!("{:?} {:?}", s1, s2);
    // 最后一问:[T; N]、[T]、&[T] 三者,长度分别在什么时候确定?

    println!("--- done ---");
}

// ---------- 练习用到的函数 ----------

// Exercise 3
fn takes_array(a: [i32; 3]) {
    println!("  函数里收到: {:?}", a);
}

// Exercise 3(b):取消注释后配合使用
// fn takes_string_array(a: [String; 2]) {
//     println!("  {:?}", a);
// }

// Exercise 4:改这里的签名
fn zero_out(a: &mut [i32; 3]) {
    for i in 0..a.len() {
        a[i] = 0;
    }
}

// Exercise 7 / 8
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
