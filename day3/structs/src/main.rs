// Day 3 — 结构体(struct):把相关的数据打包成一个自定义类型
//
// 前两天用的都是语言自带的类型(数字、字符串)。
// struct 让我们定义自己的类型,把几个相关的值组合成一个整体。

// 定义一个结构体:一个"用户"由这四个字段(field)组成
struct User {
    username: String,
    email: String,
    sign_in_count: u64, // u64:64 位无符号整数
    active: bool,       // bool:布尔值,true 或 false
}

// 元组结构体(tuple struct):字段没有名字,按位置访问
struct Point(i32, i32);

// 带方法的结构体:方法是"属于某个类型的函数"
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块:给 Rectangle 实现方法
impl Rectangle {
    // &self 表示方法借用调用者自己(rect.area() 里的 rect)
    // 只读取数据所以用不可变借用——这里直接用上了 Day 2 学的借用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法也可以接收其他参数:判断自己能否装下另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 不带 self 的叫关联函数(associated function),常用来当构造器
    // 用 类型名::函数名 调用,比如 Rectangle::square(20)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // ---------- 1. 创建和使用结构体实例 ----------

    let user1 = User {
        username: String::from("roddy"),
        email: String::from("roddy@example.com"),
        sign_in_count: 1,
        active: true,
    };
    // 用 . 访问字段
    println!("用户 {} 的邮箱是 {}", user1.username, user1.email);
    println!("登录 {} 次,活跃:{}", user1.sign_in_count, user1.active);

    // 整个实例声明为 mut 才能修改字段(不能只让某个字段可变)
    let mut user2 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 0,
        active: false,
    };
    user2.sign_in_count += 1;
    println!("{} 登录了 {} 次", user2.username, user2.sign_in_count);

    // 结构体更新语法:..user2 表示剩下的字段从 user2 拿
    // 注意:String 字段会发生 move,之后 user2.email 就不能再用了
    let user3 = User {
        username: String::from("bob"),
        ..user2
    };
    println!("user3: {} / {}", user3.username, user3.email);

    // ---------- 2. 元组结构体 ----------

    let origin = Point(0, 0);
    println!("原点:({}, {})", origin.0, origin.1);

    // ---------- 3. 方法 ----------

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // 方法用 实例.方法名() 调用
    println!("矩形面积 = {}", rect.area());

    let small = Rectangle {
        width: 10,
        height: 40,
    };
    println!("rect 能装下 small 吗?{}", rect.can_hold(&small));

    // 关联函数用 :: 调用
    let sq = Rectangle::square(20);
    println!("正方形面积 = {}", sq.area());
}
