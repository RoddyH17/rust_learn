// // //! Day 5 — enum_struct_match
// // //!
// // //! 2026-08-07 ·

// // // Rust实战笔记:
// // // TS和c++ 都有 enum的写法, c++中再加入类型 比如Direction dir = nBusca tu mente.orth

// // // Rust 的实现:
// // fn main() {
// //     #[derive(Debug)]
// //     enum Pets{
// //         Cat(String),
// //         Dog{
// //             names: String,
// //             ages: usize
// //         },
// //         Bird, //unit type
// //     }
// //     // using the enum
// //     let cat = Pets::Cat;
// //     let dog = Pets::Dog{names:"Allen".to_string(), ages:18 };
// //     println!("dog is {:?}", dog);

// //     // 给枚举值实现方法
// //     impl Pets{
// //         //methods 对当前值引用
// //         fn speak(&self){
// //              println!("hi");
// //         }
// //     }
// //     dog.speak();

// //     //定义一个关联函数

// //     impl Pets{
// //         //接自定义类型的引用
// //         fn log(name:String){
// //             println!("name is {name}");
// //         }
// //     }

// //     // dog.log() 不可以用, 会报错,因为log的方法找不到,需要用静态构建---构造函数
// //     //关联函数是通过路径来访问的,也就是双冒号, 也就是namespace
// //     Pets::log("alen".to_string());

// //     // discriminants判别式: 通过定义判别式的值来去识别枚举值
// //     // Fieldless: 只定义类型,不定义具体值
// //     // unit-only: 只包含unit类型

// // }

// // // enum的语法和规范

// fn main (){
//     // 我们现在想要去判断两个枚举值是否相等以及做比较
//     // #[derive(...)] 是一个属性宏，让编译器自动为你的类型生成某些 trait 的实现代码
//     // 例如, #[derive(Debug, Clone, PartialEq)]
//     // struct Point { x: i32, y: i32 }
//     // derive 属于过程宏（procedural macro）。编译时，编译器把类型定义的语法树（TokenStream）交给宏，宏返回一段新代码，
//     // 拼接到源码里。所以它是纯粹的编译期代码生成，零运行时开销，生成的代码和你手写的完全一样

//     // 这一行等价于你手写了三个 impl 块：

//     // impl Debug for Point { /* 逐字段打印 */ }
//     // impl Clone for Point { /* 逐字段 clone */ }
//     // impl PartialEq for Point { /* 逐字段比较 */ }

//     #[derive(PartialEq)] //加入这个derive
//     enum Pets{
//         Cat,
//         Dog,
//     }
//     let cat = Pets::Cat;
//     let dog = Pets::Dog;

//     // 使用宏包进行比较
//     // if cat == dog{
//     //     // q
//     // }

//     // 使用匹配match进行比较,也可以使用在结构体中, 以及usize
//     match cat{
//         Pets::Cat => {
//             println!("is cat");
//         }
//         // 所有条件都要匹配完整
//         Pets::Dog => {
//             println!("is dog");
//         }
//     }
//     // if let cat =Pets::Cat{
//     //     println!("is cat");
//     // }

//     let num =1;
//     match num{
//         o =>{}
//         1 =>{} // 这里num是i32类型, match匹配规则是匹配完整,不能从0匹配到i32完整, 我们直接定义match arm
//         _ =>{} // 进行模糊匹配
//     }

// }

// use std::collections::HashMap;

// 常见rust的枚举类型: Option和Result
// option: 优化空指针的问题 : 表示这个枚举下可能有值,也可能没有
// _ 的含义是"这里让编译器推断", () 是一个具体类型

// fn main() {
//     let num = Some(1); // 写法就是1.some()
//     let none: Option<usize> = None; // Option<T> 类
//     match num {
//         Some(_) => {},
//         None => {},
//     } 
//     // hashmap
//     let map:HashMap<&str, usize> = HashMap::new();
//     let a = map.get("a"); // 这里a拿到一个option类型, 而非a直接的值
//     // match a {
//     //     Some(val) => {},
//     //         let vec = vec![1,2,3];
//     //         let last_one = vec.iter().last();
//     //         // 获取方法: .match()
//     //         match last_one {
//     //             Some(val) => {},
//     //             None => {},
//     //          }   
//     //     None => {},
//     }


// result 的应用和语法
// fn main() ->Result<(), ()>{
//     let num:Result<usize, ()> = Ok(1);
//     //结合match去撇配
//     match num {
//         Ok(_) => {},
//         Err(_) => {},
//     }

//     // main函数其实没有任何访问类型的---访问一个元类型,但是他其实可以支持访问类型的
//     Ok(())
// }

// use std::{fs::File, num::ParseIntError};

// fn main() {
//     let num:Result<usize, ()> = Ok(1);
//     match num {
//         Ok(_) => {},
//         Err(_) => {},
//     }
//     let len: Result<uszie, ParseIntError> = "24".parse();
//     match len {
//         Ok(()) => {},
//         Err(_) => {},
//     }
// }

// Option和result的转换以及方法
//因为两者携带的信息量不同，而函数边界两侧对信息的需求经常不一样。
// Option 说的是「没有」，Result 说的是「失败了，原因是 X」。转换发生在你跨越抽象层的时候。

// fn main(){
//     let opt: Option<i32> = Some(42);
//     let result: Result<i32, &str>= opt.ok_or("error");
//     assert_eq!(result, Ok(42));

//     let none: Option<i32> = None;
//     let result: Result<i32, &str> = none.ok_or("error");
//     assert_eq!(result, Err("error"));
// }

// fn main(){
//     // let res: Result<i32, &str> = Ok(24);
//     // let option = res.ok();

//     let res:Result<i32, &str> = Err("error");
//     let option = res.ok();
//     assert_eq!(option,None);
// }


// 如何优化匹配机制 (match)

// fn main(){
//     let option = Some(1);
//     let b = option.map(|num| num + 1);

//     // option.unwrap(); //内容不能有none
//     let a = option.unwrap();
//     let a = option.expect("msg");
//     dbg!(a);

//     // 转换
//     // let c = option.and_then(|val| Some(val + 1));
//     // // 使用or_else
//     // let d = option.or_else(|| Some(1));
// }

use std::mem;

fn main(){
    enum A{ //size 默认为0
        // largest invariant : 分配字节来去管理枚举值内存空间 size
        a, 
        b, 
        c, 
    } // 这样只占据一个字节
    println!("size{}", mem::size_of::<A>())
}



// rust是如何分配内存管理的:
// Rust 的 enum 是带标签联合体（tagged union），内存 = 判别标签 + 最大变体的载荷：
// EnumA { A = 255 } → 0 字节

// 只有一个变体的无载荷 enum 是 ZST（零大小类型）。
// 原因很直白：这个类型只有一个可能的值，那么"值是什么"这件事不携带任何信息，不需要占内存。编译器根本不用存 tag——反正读出来也只能是 A。
// 对齐会导致"看不见的填充"，比如：
// enum Mixed { A(u8), B(u32) }   // size=8, align=4，不是 5
// 偏移:  0       1  2  3       4  5  6  7
//      [ tag ] [ 3 字节填充 ] [    u32     ]