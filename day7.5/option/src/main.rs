//! Day 7.5 — option 泛型的逻辑
//!
//! 2026-08-12 ·
//!
//! 这节的主要主旨有两个:
//! 1. 复习, 以及更加深刻理解 Option在rust中的地位, 以及option - enum的关系
//! 2. 深刻理解generic programming, functional programming的演化历史. 
//! 
//! 
//! 
//! 
// 在类型论里，每个类型都由两组规则定义：引入规则（怎么造出它）和消去规则（怎么用掉它）。
// Option<T> 的引入规则是 Some 和 None；消去规则就是 match。
// 再往前推一步就更彻底了。Böhm–Berarducci 编码告诉你：任何代数数据类型都同构于一个关于结果类型的多态函数类型。
//


// 1. 回顾option的抽象模式
// enum Option<T>{
//     Some(T), 
//     None, 
// }
// 所以当我们在写的时候, 要去思考:

fn main() {
    // let five = Some(5);
    // let six = plus_one(five);
    // println!("{:?}", six);
    // let none = plus_one(None);
    // println!("{:?}", none);

    // unwrap 
    let mut s = String::from("hello");
    let p1 = s.pop().unwrap();//有值取值,没有值panick
    println!("{:?}", p1);

    
    // 使用is_some或者is_none来判断
}


// 因为option是一个enum, 所以我们可以使用match来进行处理
// 这里的抽象层非常高----rust定义加法的逻辑是一种结果匹配起源, 然后制造过程的逻辑
// 泛型的逻辑: 
// 泛型 T        决定“里面可以装什么类型”
// enum          决定“值可能有哪些形态”
// match         判断当前值属于哪种形态
// 函数          定义输入如何转换为输出


fn _plus_one(x:Option<i32>) -> Option<i32>{
    match x{
        None => None ,  
        Some(i) => Some(i+1) ,
    }
}


