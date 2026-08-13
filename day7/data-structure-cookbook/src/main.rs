//! Day 7 — data-structure-cookbook
//!
//! 2026-08-11 · 视频进度:Vec 与 HashMap
//!
//! 今天的主线:两个容器,同一条访问规则 —— `Vec::get` 和 `HashMap::get` 都返回
//! `Option`,都逼你 `match`。「下标越界」和「查无此键」在类型上是同一件事。
//!

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// 辅助函数放在 main 之前:放在最后一个编号节后面会被当成那一节的代码。
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    // ---------- 1. 创建 Vec ----------
    // Vec 是动态数组:长度能增能减,元素连续放在堆上。
    // 两种造法的区别只在「类型从哪来」。
    let empty: Vec<i32> = Vec::new();
    // 用 new 必须自己写类型 —— 一个元素都没有,编译器无从推断。
    println!("1. empty = {:?}", empty);

    let v = vec![1, 2, 3, 4, 5];
    // 用 vec! 宏就不用写:从字面量推断出 Vec<i32>。
    println!("1. v = {:?}", v);

    // ---------- 2. 添加元素必须是 mut ----------
    // push 改变的是这个 Vec 本身,所以绑定得声明成 mut。
    let mut nums: Vec<i32> = Vec::new();
    nums.push(1);
    nums.push(2);
    println!("2. nums = {:?}", nums);

    // ---------- 3. 访问元素:索引 vs get ----------
    // 两种取值方式,写法的差别不重要,越界时的差别才重要。
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    // 直接索引:越界当场 panic,程序结束。
    println!("3. v[2] = {}", third);

    // println!("{}", v[100]);   // 取消注释:panic —— index out of bounds

    match v.get(2) {
        // get 返回 Option:有就是 Some,没有就是 None,不会崩。
        Some(n) => println!("3. v.get(2) = {}", n),
        None => println!("3. 没有下标 2"),
    }

    match v.get(100) {
        Some(n) => println!("3. v.get(100) = {}", n),
        None => println!("3. v.get(100) = None,越界不 panic"),
    }

    // ---------- 4. 遍历与修改 ----------
    // for 里写 & 是借用。不写 & 就把 Vec 移动进循环,循环结束它就没了。
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("4. 读 {}", i);
    }

    for i in &mut v {
        // 要改就借可变引用,拿到的是 &mut i32,得用 * 解引用才能赋值。
        *i += 10;
    }
    println!("4. 改完 v = {:?},而且 v 还在", v);

    // ---------- 5. 用 enum 让一个 Vec 装多种类型 ----------
    // Vec 要求所有元素同一个类型。想混着装,就用一个 enum 把它们包成同一个类型 ——
    // 类型统一了,取出来时再用 match 分派回去。
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello")),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("5. Int({})", i),
            SpreadsheetCell::Float(f) => println!("5. Float({})", f),
            SpreadsheetCell::Text(s) => println!("5. Text({})", s),
        }
    }

    // ---------- 6. 容量:capacity 不是 len ----------
    // capacity 是「已经要来的位置」,len 是「真正放了几个」。
    // 事先要够,push 的时候就不必重新分配再搬家。
    let mut v: Vec<i32> = Vec::with_capacity(10);
    println!("6. 刚创建:len = {}, capacity = {}", v.len(), v.capacity());
    v.push(1);
    println!("6. push 之后:len = {}, capacity = {}", v.len(), v.capacity());

    // ---------- 7. HashMap 靠哈希定位 ----------
    // 哈希函数把 key 压成一个数字,靠它决定去哪里找 —— 所以查找是 O(1)。
    // 不同的 key 通常给出不同的哈希值,但哈希值位数有限而 key 的数量无限,
    // 冲突必然存在。标准库能做的是让冲突尽量少,并保证冲突时结果仍然正确。
    let key1 = String::from("key1");
    let key2 = String::from("key2");
    println!("7. hash(key1) = {}", calculate_hash(&key1));
    println!("7. hash(key2) = {}", calculate_hash(&key2));

    // ---------- 8. insert 会拿走所有权 ----------
    // insert 收的是值不是引用。String 交进去之后,原来那个变量就不能再用了。
    // 想两边都留着,就先 clone。
    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());
    println!("8. map = {:?}", map);
    println!("8. field_name 还在:{}", field_name);
    println!("8. field_value 还在:{}", field_value);

    // map.insert(field_name, field_value);
    // println!("{}", field_name);   // 取消注释:error[E0382] borrow of moved value

    // ---------- 9. 同一个 key 再 insert 就是覆盖 ----------
    // 一个 key 只对应一个值。再 insert 一次,旧值被顶掉 ——
    // 它没有丢,而是从返回的 Option 里还给你。
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("9. scores = {:?}", scores);

    let old = scores.insert(String::from("blue"), 25);
    println!("9. blue 改成 25,顶掉的旧值 = {:?}", old);

    // ---------- 10. get 也返回 Option ----------
    // 和第 3 节的 Vec 一模一样:get 给 Option,查无此键就是 None。
    // 「下标越界」和「这个 key 不存在」,在类型上被当成了同一件事。
    let team_name = String::from("blue");
    match scores.get(&team_name) {
        Some(s) => println!("10. {} 的分数是 {}", team_name, s),
        None => println!("10. 没有 {} 这个队", team_name),
    }

    match scores.get("green") {
        Some(s) => println!("10. green 的分数是 {}", s),
        None => println!("10. 没有 green 这个队"),
    }

    // ---------- 11. 遍历 HashMap ----------
    // 同样要写 &,否则 map 被移动进循环。
    // 顺序不保证,每次运行都可能不一样 —— 不要依赖它。
    for (team, score) in &scores {
        println!("11. {} -> {}", team, score);
    }

    // ---------- 12. 合并两个 HashMap:覆盖还是保留 ----------
    // 循环 insert 是「后来的盖掉先来的」;entry().or_insert() 是「先来的不动,缺的才补」。
    // 这一节最容易写错的地方在最后一行。
    let mut map1: HashMap<&str, i32> = HashMap::new();
    map1.insert("a", 1);
    map1.insert("b", 2);

    let mut map2: HashMap<&str, i32> = HashMap::new();
    map2.insert("b", 3);
    map2.insert("c", 4);

    let mut merged = map1.clone();
    for (k, v) in &map2 {
        merged.insert(*k, *v);
    }
    println!("12. insert 合并 = {:?}  → b 变成 3,map1 原来的 2 没了", merged);

    let mut kept = map1.clone();
    for (k, v) in &map2 {
        kept.entry(*k).or_insert(*v);
    }
    println!("12. entry  合并 = {:?}  → b 保住 2,只补进来一个 c", kept);

    // kept.entry("d");
    // 取消注释:编译通过、没有警告、也什么都不会发生。
    // entry() 只是把「这个位置」交给你,不接 or_insert 就等于没动过。
}
