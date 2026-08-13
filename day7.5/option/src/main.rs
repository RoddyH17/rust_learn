//! Day 7.5 — option 泛型的逻辑
//!
//! 2026-08-12 · 视频进度:Option 复习 · 泛型的来历 · 错误处理
//!
//! 这节的主要主旨有三个:
//! 1. 复习, 以及更加深刻理解 Option 在 rust 中的地位, 以及 option - enum 的关系
//! 2. 深刻理解 generic programming, functional programming 的演化历史
//! 3. 学习 Rust 的报错机制, 主动和被动触发错误
//!
//! Rust 中的错误处理将可恢复和不可恢复的错误进行区分:可恢复错误是 `Result<T, E>`,
//! 不可恢复就是 `panic!`。同时我们可以使用 `?` 来传播错误。

use std::fs::File;

// 辅助函数放在 main 之前:放在最后一个编号节后面会被当成那一节的代码。

// 因为 option 是一个 enum, 所以我们可以使用 match 来进行处理。
// 这里的抽象层非常高 —— rust 定义加法的逻辑是一种结果匹配起源, 然后制造过程的逻辑。
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 显然, 我们可以将 Result 报错对象变成一个 Type。
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn div(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        // 我们现在命名一种失败的逻辑, 但是这种逻辑暗示着: 一定失败, 不如把失败包装在
        // Err 中返回 —— 失败是一种常态。
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// ? 的真正用途:把「失败就提前返回」这件事压成一个字符。
// 下面这个函数里有两次可能失败,却一个 match 都没写。
fn div_then_sqrt(a: f64, b: f64) -> Result<f64, MathError> {
    let q = div(a, b)?; // 失败 → 立刻 return Err(...);成功 → q 是 f64,不是 Result
    let r = sqrt(q)?;
    Ok(r)
}

// option 和 result 的互换:option 用 ok_or()。
fn first(arr: &[i32]) -> Result<&i32, String> {
    arr.get(0).ok_or("out of index".to_string())
}

fn main() {
    // ---------- 1. Option 的定义:引入规则与消去规则 ----------
    // 在类型论里, 每个类型都由两组规则定义:引入规则(怎么造出它)和消去规则(怎么用掉它)。
    // Option<T> 的引入规则是 Some 和 None;消去规则就是 match。
    // 再往前推一步就更彻底了 —— Böhm–Berarducci 编码告诉你:任何代数数据类型都同构于
    // 一个关于结果类型的多态函数类型。
    // 标准库里 Option 的定义就两行:`Some(T)` 和 `None`(完整写法见 NOTES.md)。
    let some_number = Some(5);
    let no_number: Option<i32> = None;
    // None 必须标类型:光看 None 无从知道 T 是什么。
    println!("1. {:?} / {:?}", some_number, no_number);

    // ---------- 2. match 是 Option 唯一的正门 ----------
    // 所以当我们在写的时候, 要去思考:值可能有哪些形态,每种形态怎么处理。
    let five = Some(5);
    let six = plus_one(five);
    println!("2. plus_one(Some(5)) = {:?}", six);

    let none = plus_one(None);
    println!("2. plus_one(None) = {:?}", none);
    // 注意 None 那条分支不是「跳过」,是「返回 None」—— 空也是一种结果。

    // ---------- 3. unwrap:省掉 match 的代价 ----------
    // 有值取值, 没有值 panic。
    let mut s = String::from("hello");
    let p1 = s.pop().unwrap();
    println!("3. pop().unwrap() = {:?}, 剩下 {:?}", p1, s);

    let empty = String::from("");
    // println!("{}", empty.pop().unwrap());   // 取消注释:panic —— called `Option::unwrap()` on a `None` value
    println!("3. 空串 pop() = {:?},unwrap 它就会 panic", empty.clone().pop());

    // expect 和 unwrap 一样会 panic,区别只是能自带一句话。
    println!("3. expect 的消息会出现在 panic 里,便于定位是哪一个 unwrap 崩的");

    // ---------- 4. 两类错误:可恢复与不可恢复 ----------
    // 可恢复错误: Result<T, E>,写进类型里,逼调用方处理。
    // 不可恢复:  panic!,直接终止,不给你处理的机会。
    // 主动的触发可以通过 panic!() 函数来去获得, 后面会显示在 debug 界面中的 backtrace。
    //
    // panic!("这是主动触发");            // 取消注释:程序当场终止
    // RUST_BACKTRACE=1 cargo run 可以看到完整调用栈。
    println!("4. Result 是值,panic 是终止 —— 前者能被 match,后者不能");

    // ---------- 5. 越界即 panic:为什么这是安全设计 ----------
    // Buffer overflow 是一种经典的安全系统攻击行为, 所以一些语言的设计就是为了避免这种行为。
    // 在 rust 中, 一旦访问越界数组, 程序会直接 panic。
    let v = vec![1, 2, 3];
    // println!("{}", v[99]);   // 取消注释:panic —— index out of bounds
    println!("5. v.get(99) = {:?} —— 想不 panic 就走 Option 这条路", v.get(99));

    // ---------- 6. 用 match 处理 Result:打开文件 ----------
    // 使用程序命令逻辑, 将变量打开系统内的文件。
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => {
            println!("6. 打开成功");
            Some(file)
        }
        Err(error) => {
            // 原写法是 panic!("Problem opening the file: {:?}", error)
            // 这里改成打印,是为了让整个文件跑得完 —— 语义上两者是同一个分支。
            println!("6. 打开失败: {:?}", error.kind());
            None
        }
    };
    // 所以对于错误的处理, 我们也必须要遵循文件管理的思维。

    // 显然, 我们还能够通过使用 unwrap 来取打开文件:
    // let f = File::open("hello.txt").unwrap();   // 取消注释:文件不存在时 panic

    // ---------- 7. map:Result<T, E> → Result<U, E> ----------
    // 显然, 我们也可以使得两种报错系统互相转换。
    // map 只作用在成功的那一侧,Err 原样穿过去 —— 这就是「只改一半」。
    let line = "1\n2\nthree\n4";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            // 这里 parse() 本来返回的也是一个 Result
            Ok(n) => println!("7. {} → {}", num, n),
            Err(..) => println!("7. {} → 解析失败,跳过", num),
        }
    }

    // ---------- 8. 把错误做成类型 ----------
    // 用这种方式封装错误, 会显得非常整齐:错误不再是字符串,而是可以被 match 的枚举。
    println!("8. div(6.0, 3.0) = {:?}", div(6.0, 3.0));
    println!("8. div(6.0, 0.0) = {:?}", div(6.0, 0.0));

    match div(1.0, 0.0) {
        Ok(v) => println!("8. 得到 {}", v),
        Err(MathError::DivisionByZero) => println!("8. 除零 —— 调用方能精确认出是哪种失败"),
        Err(MathError::NegativeSquareRoot) => println!("8. 负数开根"),
    }
    // 换成 Result<f64, String> 就做不到上面这种分支:字符串没法被穷尽匹配。

    // ---------- 9. ? 运算符:传播而不是处理 ----------
    // 为了省略 match, rust 支持直接使用 ? 作为 match 检查。
    // ? 只能写在返回 Result(或 Option)的函数里 —— 它要 return,就得知道返回什么。
    println!("9. div_then_sqrt(8.0, 2.0) = {:?}", div_then_sqrt(8.0, 2.0));
    println!("9. div_then_sqrt(8.0, 0.0) = {:?}", div_then_sqrt(8.0, 0.0));
    println!("9. div_then_sqrt(-8.0, 2.0) = {:?}", div_then_sqrt(-8.0, 2.0));
    // 三次调用走了三条不同的路径,函数体里却一个 match 都没有。

    // ---------- 10. Option 与 Result 互转 ----------
    // option: ok_or() —— 把「没有」升级成「因为某某原因而没有」。
    let arr = [10, 20, 30];
    println!("10. first(&arr) = {:?}", first(&arr));
    println!("10. first(&[]) = {:?}", first(&[]));

    // 反方向:Result 用 .ok() 丢掉错误信息,降级成 Option。
    let f = File::open("hello.txt").ok();
    println!("10. File::open(..).ok().is_some() = {:?}", f.is_some());
    // .err() 则是反过来只留错误那一侧。
    println!("10. .err() 只留错误侧 = {:?}", File::open("hello.txt").err().is_some());

    // ---------- 11. 泛型的逻辑 ----------
    // 四层职责各管一件事,拆开看就不神秘了:
    //
    // | 层     | 决定什么             |
    // |--------|----------------------|
    // | 泛型 T | 里面可以装什么类型   |
    // | enum   | 值可能有哪些形态     |
    // | match  | 当前值属于哪种形态   |
    // | 函数   | 输入如何转换为输出   |
    //
    // Option<T> 之所以强,是因为它同时用满了前三层:T 管内容,Some/None 管形态,
    // match 管分派。Result<T, E> 是同一个模子,只是把 None 换成了「带原因的 None」。
    let a: Option<i32> = Some(1);
    let b: Option<String> = Some("x".to_string());
    // 同一个 Option,装了两种完全不同的类型 —— 这是 T 在干活。
    println!("11. {:?} 和 {:?} 是同一个 enum 的两次单态化", a, b);
}
