//! Day 4 — Array & Slice
//!
//! 2026-08-07
//!
//! 今天的主线:数组的长度是类型的一部分,而切片是「指向其中一段的引用」——
//! 切片没有所有权,所以 Day 3 的借用规则原封不动地作用在它身上。

fn main() {
    // ---------- 1. 数组:长度不可变,值可变 ----------
    // 数组 array 在过程中不能改长度,只能更新值,不能删除。
    // 写法: let 变量名 : [类型; 长度] = [值1, 值2, ...]
    let arr: [i32; 4] = [10, 20, 30, 40];
    let arr1: [i32; 4] = [10; 4]; // 全是 10 的、长度 4 的数组
    println!("arr  = {:?}", arr);
    println!("arr1 = {:?}", arr1);

    // ---------- 2. 遍历数组的三种写法 ----------
    // (a) 按索引。注意:for 的循环变量**不能写类型标注**。
    //     `for index: i32 in 0..4` 是编译不过的 —— for 后面跟的是「模式」,
    //     不是变量声明,所以没有 `: 类型` 这一节。要指定类型得写在区间上:`0..4i32`。
    for index in 0..4 {
        println!("arr[{}] = {}", index, arr[index]);
    }

    // (b) 迭代器 arr.iter() —— 拿到的是 &i32
    for value in arr.iter() {
        println!("iter: {:?}", value);
    }

    // (c) 一行搞定:闭包 + for_each
    arr.iter().for_each(|value: &i32| println!("for_each: {}", value));

    // ---------- 3. 可变数组与不可变数组 ----------
    let fixed: [i32; 4] = [10; 4];
    let mut changeable: [i32; 4] = [10; 4];
    changeable[0] = 99; // 有 mut 才能改元素
    println!("fixed = {:?}, changeable = {:?}", fixed, changeable);

    // ---------- 4. 直传递:数组是 Copy,函数改不到原来的 ----------
    // i32 实现了 Copy,所以 [i32; 3] 整个也是 Copy。
    // 传进函数的是一份**拷贝**,函数里怎么改都不影响调用方 —— 这点和 Vec 相反。
    let a: [i32; 3] = [10, 20, 30];
    println!("before update      : {:?}", a);
    update(a);
    println!("after  update      : {:?}", a); // 还是 [10, 20, 30]

    // ---------- 5. 引用传递:&mut 才能真的改到 ----------
    let mut b: [i32; 3] = [10, 20, 30];
    println!("before update_mut  : {:?}", b);
    update_mut(&mut b);
    println!("after  update_mut  : {:?}", b); // 变成 [0, 0, 0]

    // ---------- 6. 切片解决的问题:一个和数据脱钩的返回值 ----------
    // 不用切片时,返回 usize 下标的问题在于:它是一个和 s 完全脱钩的独立数字,
    // 编译器不会帮你维护它俩的一致性。
    let mut s = String::from("hello world");
    let word_end = first_word_index(&s); // 5
    println!("first_word_index = {}", word_end);
    s.clear(); // s 现在是 ""
    // word_end 仍然是 5,但已经没有任何字符串能让它有意义 —— 这就是 out of sync。
    println!("s cleared, but word_end is still {}", word_end);
    //
    // 所以切片本质上也是一种引用,它并没有所有权。

    // ---------- 7. 什么是字符串切片 ----------
    // A string slice is a reference to part of a String.
    // rust 的字符 char 是 Unicode,固定占 4 个 byte;但字符串是 UTF-8 编码,
    // 也就是说字符串里的一个字符占 1 到 4 个字节不等。
    // 语法 [starting_index..ending_index],**左闭右开**。
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} / {}", hello, world);

    // 省略写法:起点是 0、终点是结尾时两端都能省
    let t = String::from("hello");
    let len = t.len();
    println!("{:?} {:?} {:?}", &t[..2], &t[3..], &t[..]);
    println!("{:?}", &t[0..len]);

    // ---------- 8. UTF-8 边界 ----------
    // ⚠️ 切片索引必须落在 UTF-8 字符边界上,否则**运行时直接 panic**。
    // 中文一个字符 3 字节,所以 &cn[0..2] 会炸,&cn[0..3] 才是 "你"。
    let cn = String::from("你好");
    println!("cn[0..3] = {:?}, cn.len() = {} (字节数,不是字符数)", &cn[0..3], cn.len());
    println!("cn 的字符数 = {}", cn.chars().count());
    println!("size_of::<char>() = {}", std::mem::size_of::<char>()); // 4

    // ---------- 9. slice 常用函数 ----------
    // len() / is_empty() / contains() / repeat() / reverse() / join()
    // swap() / windows() / starts_with()
    let nums: [i32; 10] = std::array::from_fn(|i| (i + 1) as i32);
    println!("len={} is_empty={} contains(3)={}", nums.len(), nums.is_empty(), nums.contains(&3));
    println!("starts_with([1,2]) = {}", nums.starts_with(&[1, 2]));

    let mut v = vec![1, 2, 3];
    v.swap(0, 2); // 交换两个索引上的元素
    println!("after swap  = {:?}", v);
    v.reverse();
    println!("after reverse = {:?}", v);
    println!("repeat(2) = {:?}", [1, 2].repeat(2));
    println!("join = {:?}", ["a", "b", "c"].join("-")); // flatten 后用分隔符连接

    // windows(n):以 n 大小的窗口滚动迭代
    for w in nums.windows(3).take(3) {
        println!("windows: {:?}", w);
    }

    // ---------- 10. 字符串的两种创建方式 ----------
    // 引用只能借用本体的内存,不对本体内存的释放负责。两种创建方式对应两种归属:
    //
    // (1) 字面值:被直接硬编码进可执行文件,编译期就定下来了(静态区)。
    //     引用方并没有获得所有权 —— 这不是坏事,因为并不是所有时候你都需要所有权。
    let lit: &str = "hello world"; // 类型就是 &str
    println!("lit = {} (&str, 硬编码进可执行文件)", lit);

    // (2) 运行时动态分配。String 在 Rust 里是个复合类型,定义大致是
    //     pub struct String { vec: Vec<u8> } —— Vec<u8> 申请在 heap 上,
    //     所以用 String 的时候我们是拥有所有权的。
    let owned = String::from("hello world");
    println!("owned = {} (String, 数据在 heap 上)", owned);

    // ---------- 11. String → &str 的三种写法 ----------
    let s = String::from("hello world");
    say_hello(&s); // &String -> &str,自动解引用 (deref coercion)
    say_hello(&s[..]); // 本质上在做切片,必须落在 UTF-8 边界上
    say_hello(s.as_str()); // 直接返回自身的 &str
    //
    // 反过来 &str -> String 成本高一些,因为要重新在 heap 上申请内存:
    let back: String = "hello world".to_string(); // 或 String::from(...)
    println!("back = {}", back);

    // ---------- 12. 字符串拼接 ----------
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let result = s1 + &s2; // 要拼接的第二个参数必须是字符串 slice 形式
                           // 注意 s1 在这里被 move 走了,之后不能再用
    println!("result = {}", result);

    // 同样可以用 format! 来拼接,而且它不会拿走任何一方的所有权
    let s3 = format!("{}, {}", result, s2);
    println!("s3 = {}", s3);
    // 我笔记里写的 `format!("{}, {}, s1, s2")` 是错的 ——
    // 变量要作为参数传进去,不能写在字符串字面量里面。

    // ---------- 13. 函数参数用 &str,不要用 &String ----------
    // 签名改成 &str 之后,同一个函数**同时能接受 String 和字面量**。
    let my_string = String::from("hello world");
    println!("{:?}", first_word(&my_string[0..6]));
    println!("{:?}", first_word(&my_string[..]));
    println!("{:?}", first_word(&my_string)); // &String 自动转 &str
    println!("{:?}", first_word("hello world")); // 字面量本身就是 &str

    // ---------- 14. 数组也能切,机制完全一样 ----------
    let a5 = [1, 2, 3, 4, 5];
    let slice = &a5[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("array slice = {:?} (type is &[i32])", slice);

    // ---------- 15. 内存布局:切片是个「胖指针」 ----------
    // 普通引用 = 1 个字(只存地址);切片引用 = 2 个字 = 首元素指针 + 长度。
    // 所以一个切片引用固定占 16 字节,和它引用了多少元素无关。
    let chars: [char; 3] = ['中', '国', '人'];
    let cslice = &chars[..2];
    // 注意不是 8!数组本身是 2 个 char × 4 字节 = 8,
    // 但 cslice 是引用:指针 8 + 长度 8 = 16。
    assert_eq!(std::mem::size_of_val(&cslice), 16);
    println!("size_of_val(&cslice) = {}", std::mem::size_of_val(&cslice));

    // ---------- 16. 三个容易混的类型 ----------
    // [T; N]  数组      —— 长度在编译期确定,是类型的一部分
    // [T]     切片本体  —— 长度运行时才知道,是 DST,**不能直接用**
    // &[T]    切片引用  —— 长度存在胖指针里,日常写的都是这个
    //
    // 下面两行放开都编译不过,因为 [i32] 和 str 都是 DST:
    // let bad1: [i32] = a5[0..2];
    // let bad2: str = "hello, world";
    let ok1: &[i32] = &a5[0..2];
    let ok2: &str = "hello, world";
    println!("{:?} {:?}", ok1, ok2);
}

// ---------- 上面用到的函数 ----------

// 第 4 节:数组是 Copy,所以这里改的是拷贝,调用方看不到。
fn update(mut arr: [i32; 3]) {
    println!("  update before: {:?}", arr);
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("  update after : {:?}", arr);
}

// 第 5 节:拿 &mut,改的就是调用方那一份。
fn update_mut(arr: &mut [i32; 3]) {
    println!("  update_mut before: {:?}", arr);
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("  update_mut after : {:?}", arr);
}

// 第 6 节:返回下标 —— 和原字符串脱钩的写法。
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 第 13 节:返回切片 —— 和原字符串绑在一起,而且 &str 比 &String 更通用。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 第 11 节:参数用 &str,String 和字面量都能传进来。
// 注意 println! 里变量要作为参数传,`println!("{}, s")` 是编译不过的。
fn say_hello(s: &str) {
    println!("hello, {}", s);
}
