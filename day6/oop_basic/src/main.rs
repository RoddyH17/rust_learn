// // //! Day 6 — oop
// // //!

// // //!

// // fn main() {
// //     // 每一个字段都必须初始化, 没有默认赋值, 顺序不一定一样, 且要用逗号分隔
// //     // 和js一样,rust也有初始化的简洁语法


// //     // let mut user1 = User{
// //     //     email,
// //     //     username,
// //     //     active:true,
// //     //     sign_in_count: 1,
// //     // };

// //     // // 访问结构体字段的方式
// //     // println!("{}", user1.email);

// //     // // 访问并且改造
// //     // user1.email = String::from("another@example.com");

// //     // println!("{}", user1.email);
// //     // // rust并不能对具体某个字段标记为可写,必须对整体进行标记. 

// //     // // // rust的语法糖: 假设另一个user和user1的内容一模一样, 此时该怎么办?
// //     // // let user2 = User{
// //     // //     email: String::from("another@example.com"),
    
// //     // //     ..user1 // 结构体更新语法
// //     // // };

// //     // // 元组的实例化
// //     // let origin = Point(0,1,2); 
// //     // println!("{}", origin.0);

// //     // // 单元的实力化
// //     // let subject = AlwaysEqual;

// //     // 结构体的所有权范围: 每个字段都有一个所有权
// //     // 一旦结构体的一个字段发生了所有权移动,那么整个结构体都不能被传递和赋值了.


// //     let user1 = User{
// //         email:"someone@example.com", //字符串没有copy特性, 会发生move
// //         username:"someoneusername123",  
// //         active:true, //基本的copy特性,发生赋值
// //         sign_in_count: 1,
// //     };

// //     // let email = user1.email;
// //     println!("{}", user1.email);

// //     // let user2 =User{
// //     //     email: String::from("another@example.com"),
// //     //     ..User1
// //     // };
// // }


// // // 如果需要使用切片, 我们需要使用加入生命周期的标记. 
// // struct User<'a>{
// //     active: bool,
// //     username: &'a str, //使用切片类型 
// //     email: &'a str,
// //     sign_in_count: u64,
// // }

// // // 元组结构体: 结构体必须要有名称, 但是结构体的字段可以没有名称, 这种结构体长得很像元祖
// // // 比如point元组, 可以直接对点进行类型赋于,而非需要先说明他们是x, y, z
// // struct Point(i32, i32, i32);


// // // 单元结构体: 不关心该类型聂荣, 只关心他的行为. 没有字段
// // struct AlwaysEqual;

// // 结构体的行为也叫做方法, 方法和函数不一样. 
// // 方法的第一个参数总是self, 代表调用该方法的结构体实例化. 
// // 使用impl 来包裹函数fn 创建方法


// #[derive(Debug)]    
// struct Rectangle{
//     width: u32, 
//     height: u32,
// }

// pub struct Rectange{
//     width: u32, 
//     height: u32,
// }
// // 方法公开或者结构体公开
// // 使用函数创建结构体, 问题: 函数和结构体关系不够紧密
// fn new(width: u32, height: u32) -> Rectangle{
//     Rectangle{width, height}
// }

// // 使用关联函数, 并且impl是一种组织类型, 可以将不同方法更好的进行组织和绑定
// impl Rectangle{
//     // 方法被定义在结构体的上下文
//     fn new(width: u32, height: u32) -> Rectangle{
//         Rectangle{width, height}
//     }
//     fn set_width(mut self, new_width:u32){
//         self.width = new_width;
//     }

//     fn area(&self) -> u32{
//         self.width * self.height // 有点像python里的this, 
//     }
// }

// fn main(){
//     let rect1 = Rectangle{ width: 30, height: 50};

//     println!(" The area of the Rectangle is {} square pixels.", rect1.area());

//     let rect2 = Rectangle{ width: 20, height: 30};
//     rect2.set_width(10);

//     // 使用关联函数来进行实例化
//     let rect3 = Rectangle::new(10, 30);
//     println!())
// }


// 结构体的核心概念; 对trait的实现 (而非接口)
// 不同类型具有相同的行为, 那么我们就可以定义一个特征, 然后为这些类型实现该特征
// 定义特征是去把方法组合在一起, 目的是定一一个实现某些目标所必须的行为的集合. 

trait Shape{
    fn area(&self)->f64;
}

//派生: 使得结构体进行正常的打印
#[derive(Debug)]
// 或者使用debug宏


pub struct Rectangle{
    width: u32, 
    height: u32,
}

struct Circle{
    r: f64,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// 打印任意我们想要的
impl std::fmt::Display for Rectangle{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        for _ in 0..self.height{
            let mut s = String::new();
            for _ in 0..self.width{
                s.push('#');
            }
            write!(f, "{}\n", s);
        }
        return Ok(());
    }
}






impl Shape for Rectangle{
    fn area(&self) -> f64{
       return (self.width * self.height) as f64; 
    }
}




impl Shape for Circle{
    fn area(&self) -> f64{
       return (3.14 * self.r * self.r) as f64; 
    }
}

// 仔细观察,为什么不写print_area(rectangle: &Rectangle, circle:&Circle )
fn print_area(shape: &impl Shape) {
    println!("{}", shape.area());
}




fn main(){
    let rect4 = Rectangle::new(10,30);

    print_area(&rect4);
    //println!("rect4 is {:#?}", rect4);
    //dbg!(rect4);
    println!("{}", rect4);

}


