//! Day 9 — smart_pointer
//!
//! 2026-08-13 
//! 
//! 智能指针是一种数据结构---能实现deref + drop 两个trait

//! Deref: 可以使用"*“ 这种操作解引用. 
//! 例如Box<T>其实实现了Deref. 同时drop也会自动释放内存资源. 
//!  
//! 
//! Box<T> : 将类型T的值分配在heap上而非stack, 我们之前学到的rust type基本都在stack上
//! 底层逻辑: 内部包含了一个指向heap上分配的裸指针. 
//! 当Box<T>被销毁的时候, trait会调用这个指针. 
//! Box::new--- 初始化智能指针来分配内存. 


fn main() {
    // // heap 分配
    // let b = Box::new(5);
    // println!("b={}", b);

    // // 支持DST
    // let s: Box<str> = "hello, world".into();
    // println!("s = {}", s);

    // let arr: Box<[i32]> = vec![1,2,3,4,5].into_boxed_slic();
    // println!("arr = {:?}", arr);


    // 递归的数据结构, 需要指针类型来引用自身,
    let list: List = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
}

// linked list 结构
enum List{
    Cons(i32, Box<List>),
    Nil,
}



