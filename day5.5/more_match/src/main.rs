//! Day 5.5 — more_match
//!

// match的核心功能: 代替if-else的control flow 逻辑
// 所以其可以检查数据的结构并进行相应操作, 尤其是对更复杂的数据做结构处理
// 对struct 和 enum的更深度的解析和思考:
// struct 表达“同时拥有这些东西”（AND）
// enum 表达“只能是这些可能性之一”（OR）
// struct：描述一个对象由什么组成
// enum：描述一个值可能处于哪一种情况, 并且enum 的每种情况还可以携带不同数据, 下面是一个例子
// // enum OrderStatus {
//     Pending,
//     Filled {
//         execution_price: f64,
//         quantity: f64,
//     },
//     Canceled {
//         reason: String,
//     },


fn main() {
    match_t();
    // let r:Result<i32, String> = divide(4, 2); //但调用时只按位置传值：
    // match r {
    //     Ok(n) =>println!("The result is {}", n), 
    //     Err(e) =>println!("The error is {}", e), 
    // }

    // match divide (4, 2){
    //     Ok(n) =>println!("The result is {}", n), 
    //     Err(e) =>println!("The error is {}", e), 
    // }


}

fn match_t(){
    // let number  = 13; 
    // match number{
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ =>println!("Others")
    // }
    // 1. 自变量的模式匹配
    // let y: &str = "Hello";
    // match y {
    //     "Hello" => println!("Hello"),
    //     _=>println!("Other!"),
    // }

    // 2. 变量匹配

    // let x: i32 = 42; // var变成x的值了
    // match x{
    //     var@ 42 => println!("The value of x is {}", var),
    //     _ => println!("Somethingelse"),
    // }

    // 3. 变量名 @ 匹配模式: 既检查右边的模式是否匹配，又把匹配到的完整值绑定到左边的变量。
    // let x = 7;
    // match x {
    //     n @ 1..=10 => println!("{} 在 1 到 10 之间", n),
    //     _ => println!("不在范围内"),
    // }


    // enum Message {
    //     Number(i32),
    // }

    // match Message::Number(8) {
    //     whole @ Message::Number(1..=10) => {
    //         println!("匹配到完整结构：{:?}", whole);
    //     }
    //     _ => {}
    // }


    // 4. 如果是字符串的匹配呢? 我们发现会被move, 因为此时已经被var移走了
    // 下面的例子在8 会被处理
    // let x = String::from("Hello");
    // match x{
    //     var@ string => println!("The value of x is {}", var),
    //     _ => println!("Somethingelse"),
    // }


    // 5. 结构模式的匹配
    // struct Point {x:i32, y:i32}

    // let p: Point = Point{x:0, y:7} ;
    
    // match p{
    //     Point{x, y:0} => println!("on the x axis at {}", x),
    //     Point{x:1,y} => println!("On the y axis at {}", y), 
    //     Point{x, y} => println!("On neither axis:{}, {}", x, y)
    // }


    // 6. 守卫: 使用计算的condition来进行match
    // let x = 5;
    // match x{
    //     n if n%2 ==0 => println!("Even"),
    //     _ => println!("Odd"),
    // }

    // 加一个表达式进行动态绑定
    // enum Message{
    //     Hello {id: i32}, 
    // }

    // let msg:Message = Message::Hello{id:5};

    // match msg{
    //     Message::Hello{id:message_id @3..=7} =>println!("Found and id in range:{}", message_id),
    //     Message::Hello{id:phone_id @10..=12} =>println!("Found and id in range:{}", phone_id),
    //     Message::Hello{id} =>println!("Found and id in range:{}", id),
    // }

    // 7.match的核心模式匹配的应用场景:
    // i.处理错误 ii. 解析命令行的参数 iii. 解析配置文件以及数据包
    // 下面带来更加复杂的嵌套匹配

    // Case1: 可以通过match, 将数值进行打印时更改, 而非储存. 
    // enum Message{
    //     Quit, 
    //     Move{x:i32, y:i32}, 
    //     Write(String),
    //     ChangeColor(i32, i32, i32), 

    // }

    // let msg: Message = Message::ChangeColor(0, 255, 0);

    // match msg{
    //     Message::ChangeColor(r,g,b) => {
    //         println!("change the color to red {}, green {}, and blue {}", r, g, b)
    //     } 
    //     _=>()
    // }

    // Case2: match和iter的结合使用, 对iterator的复习
    // 我们可以拆解iterator源代码:
    // fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter> 这里的iter()方法会发生所有权更改
    // 而zip自己也实现了iterator: 
    // where
    //     Self: Sized,
    //     U: IntoIterator,
    // {
    //     Zip::new(self, other.into_iter())
    // }
    // 我们发现, rust中的Iterator是惰性的, 只要元素没有处理, 只会创建struct
    // 适配器：map、filter、zip、skip
    //     ↓ 只搭建处理管道

    // 消费者：next、collect、sum、for
    //     ↓ 真正驱动管道执行

  
    // 7. 提供更简单的单个模式匹配

    // // if let 模式
    // let opt = Some(5);
    // // 如果option有值的话,本质上是对some的一种匹配
    // if let Some(x) = opt{
    //     println!("Matched {:?}", x);
    // }

    // // while let模式
    // let mut iter = vec![1,2,3].into_iter();

    // while let Some(x) = iter.next(){
    //     println!("matched {:?}", x);
    // }

    // 8. 对于被转移的string所有权类型 ,如果我们想借用数据而非转移 ,该怎么使用ref呢?
    // 例如,我们在递归的状态下, 我们不想改变数据的所有权形式, 或者如果我们想要修改数据本身?
    // let x = String::from("Hello");
    // match x{
    //     ref var => println!("The value of x is {}", var), // 这里var就是&String 了
    // }
    // 如果我们想要 进行一个变量的匹配:
    let mut x  = String::from("Hello");
    match x{
        ref mut var => {
            *var = String::from("world");
            println!("The result of x is {}", var);
        } 
    }
    println!("The value of x is {}", x);
  



}

fn _divide(a: i32, b: i32) -> Result<i32, String>{
    if b==0 {
        Err(String::from("Cannot divide by zero"))
    } else{
        Ok(a/b)
    }
}







