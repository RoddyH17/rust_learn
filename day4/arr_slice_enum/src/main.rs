//! Day 4 — Array, Slice, Enum
//!
//! 2026-08-07 
//!

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
    //     我笔记里写的 `for index :i32 in 0..4` 是编译不过的 ——
    //     for 后面跟的是「模式」,不是变量声明,所以没有 `: 类型` 这一节。
    //     要指定类型得在区间上写,比如 `0..4i32` 或 `(0..4).map(|x: i32| x)`。
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
    // 注意笔记里那行 `let mut arr_mut ; [i32;4] = [10;4];` 用的是分号不是冒号,
    // 类型标注要写成 `let mut arr_mut: [i32; 4] = [10; 4];`

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

    // ---------- 7. 什么是字符串切片 ----------
    // A string slice is a reference to part of a String.
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

    // ⚠️ 切片索引必须落在 UTF-8 字符边界上,否则**运行时直接 panic**。
    // 中文一个字符 3 字节,所以 &cn[0..2] 会炸,&cn[0..3] 才是 "你"。
    let cn = String::from("你好");
    println!("cn[0..3] = {:?}", &cn[0..3]);

    // ---------- 8. 用切片重写:让编译器替你抓 bug ----------
    // 返回 &str 之后,返回值就是一个借用,和 s 绑在一起了。
    let s2 = String::from("hello world");
    let w = first_word(&s2);
    println!("first_word = {:?}", w);
    // 下面这段放开就编译不过 —— clear() 要 &mut,而 w 还持有不可变借用:
    //   error: cannot borrow `s2` as mutable because it is also borrowed as immutable
    // 这正是 Day 3 借用规则的直接兑现:同一个 bug,别的语言要等线上崩了才发现。
    //
    // let mut s2 = String::from("hello world");
    // let w = first_word(&s2);
    // s2.clear();
    // println!("{w}");

    // ---------- 9. 字符串字面量本身就是切片 ----------
    // The type of `s` here is &str: a slice pointing to that point of the binary.
    // 这也是为什么字面量不可变 —— &str is an immutable reference.
    let literal = "Hello, world!";
    println!("literal = {} (type is &str)", literal);

    // ---------- 10. 函数参数用 &str,不要用 &String ----------
    // 签名改成 &str 之后,同一个函数**同时能接受 String 和字面量**。
    let my_string = String::from("hello world");
    println!("{:?}", first_word(&my_string[0..6]));
    println!("{:?}", first_word(&my_string[..]));
    println!("{:?}", first_word(&my_string)); // &String 自动转 &str (deref coercion)
    let my_literal = "hello world";
    println!("{:?}", first_word(my_literal)); // 字面量本身就是 &str

    // ---------- 11. 数组也能切,机制完全一样 ----------
    let a5 = [1, 2, 3, 4, 5];
    let slice = &a5[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("array slice = {:?} (type is &[i32])", slice);

    // ---------- 12. 内存布局:切片是个「胖指针」 ----------
    // 普通引用 = 1 个字(只存地址);切片引用 = 2 个字 = 首元素指针 + 长度。
    // 所以一个切片引用固定占 16 字节,和它引用了多少元素无关。
    let chars: [char; 3] = ['中', '国', '人'];
    let cslice = &chars[..2];
    // 注意不是 8!数组本身是 2 个 char × 4 字节 = 8,
    // 但 cslice 是引用:指针 8 + 长度 8 = 16。
    assert_eq!(std::mem::size_of_val(&cslice), 16);
    println!("size_of_val(&cslice) = {}", std::mem::size_of_val(&cslice));

    // ---------- 13. 三个容易混的类型 ----------
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

    // ---------- 14. Lifelong ----------
    // (Notion 上这一节还是空的,看视频时补)

    // ---------- 15. Enum ----------
    // (还没记,今天的第三块)
}

// ---------- 上面用到的函数 ----------

// 第 4 节:数组是 Copy,所以这里改的是拷贝,调用方看不到。
// 笔记里原来写的是 `for i:usize in 0..3` 且少一个 `{` —— for 的循环变量不能标类型。
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

// 第 8/10 节:返回切片 —— 和原字符串绑在一起,而且 &str 比 &String 更通用。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
