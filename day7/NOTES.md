---
day: 7
date: 2026-08-11
topic: data-structure-cookbook
mood: # tags, e.g. [zen, frage] — zen = calm | sorge = frustrated | frage = unresolved question
---

# Day 7 — data-structure-cookbook (2026-08-11)

> 📝 Live notes: [`data-structure-cookbook/src/main.rs`](data-structure-cookbook/src/main.rs) — written while watching, and the source everything else is generated from
>
> ✍️ Blog post: [Day 7](https://roddyh17.github.io/posts/rust/day-7-data-structure-cookbook/) (the blog has the story; this file has the technical details)

标准库的两个容器:`Vec` 是「按位置放」,`HashMap` 是「按名字放」。今天真正的收获不是
API 列表,而是**两者共用同一条访问规则**:`get` 都返回 `Option`,都逼你 `match`。
「下标越界」和「这个 key 不存在」在类型上被当成了同一件事。

三件套还差一件 —— `String` 今天没碰,留给后面。

## Goals — what to master

| #  | Topic | You should be able to... | Self-check |
|----|-------|--------------------------|------------|
| 1  | 创建 Vec | 用 `Vec::new()` 和 `vec![]` 各造一个 | 为什么 `Vec::new()` 必须写类型而 `vec![1,2]` 不用? |
| 2  | 添加元素 | 说明为什么 `push` 要求 `mut` | `mut` 修饰的是 Vec 还是绑定? |
| 3  | 索引 vs get | 写出越界不会 panic 的取值 | 什么时候可以放心用 `v[i]`? |
| 4  | 遍历与修改 | 用 `&v` 读、用 `&mut v` 改 | 不写 `&` 会发生什么?为什么改值要 `*i` ? |
| 5  | 装多种类型 | 用 enum 让一个 Vec 装 Int/Float/Text | Vec 要求同类型,那 enum 是怎么绕过去的? |
| 6  | capacity | 区分 `len()` 和 `capacity()` | 容量满了再 push,容量怎么长? |
| 7  | 哈希 | 说清 HashMap 为什么能 O(1) 查找 | 哈希冲突不可避免,那它凭什么还是对的? |
| 8  | insert 的所有权 | 判断 insert 之后原变量还能不能用 | `String` 和 `&str` 做 key,这里的行为一样吗? |
| 9  | 覆盖更新 | 说出重复 insert 的返回值是什么 | 被顶掉的旧值去哪了? |
| 10 | get + match | 处理「查无此键」 | 和第 3 条的 Vec 是同一套规则吗? |
| 11 | 遍历 HashMap | 写出 `for (k, v) in &map` | 输出顺序稳定吗?能依赖吗? |
| 12 | 合并两个 map | 分别写出「覆盖」和「保留」两种合并 | `entry()` 单独调用会发生什么? |

## Concepts and examples

代码在 `data-structure-cookbook/src/main.rs`,12 个编号节,`cargo run` 一次跑完。

### Vec —— 按位置放

**取值的两条路,差别只在越界时**(第 3 节):

| 写法 | 返回 | 越界时 | 什么时候用 |
|------|------|--------|-----------|
| `&v[i]` | `&i32` | **panic**,程序结束 | 下标由你自己算出来、逻辑上不可能越界 |
| `v.get(i)` | `Option<&i32>` | `None` | 下标来自外部输入、循环边界、用户 |

`get` 把「可能没有」写进了类型里,所以编译器会强制你处理这种情况。这是 Day 5
`Option` 那套东西第一次在容器上派上用场。

**遍历要写 `&`**(第 4 节)。不写 `&` 就是把 Vec 移动进循环,循环结束它就被丢掉了。
要改元素则借 `&mut`,循环变量的类型是 `&mut i32`,赋值要解引用:

```rust
for i in &mut v {
    *i += 10;
}
```

**enum 是「装多种类型」的正解**(第 5 节)。Vec 要求所有元素同类型 —— enum 不是绕过
这条规则,而是**满足**它:三种东西被包成同一个类型 `SpreadsheetCell`,取出来时再
`match` 分派回去。类型统一发生在编译期,取值的分支判断发生在运行期。

**`capacity` 不是 `len`**(第 6 节):

| | 含义 | `with_capacity(10)` 之后 |
|---|------|------------------------|
| `len()` | 真的放了几个 | `0` |
| `capacity()` | 已经要来几个位置 | `10` |

预先要够,`push` 的时候就不必重新分配再把数据搬过去。容量不够时 Vec 不是加一格 ——
它成倍增长,为的是让搬家的次数随元素个数只增长 O(log n) 次。

### HashMap —— 按名字放

**哈希与冲突**(第 7 节)。哈希函数把 key 压成一个 `u64`,靠这个数字直接算出该去哪里
找,所以查找是 O(1)。`key1` 和 `key2` 的哈希值确实不同,但**哈希值的位数有限而 key
的数量无限,冲突必然存在** —— 标准库能保证的不是「不冲突」,而是「冲突足够少」加上
「冲突时结果仍然正确」。

**`insert` 拿走所有权**(第 8 节)。它收的是值不是引用:

```rust
map.insert(field_name.clone(), field_value.clone());   // 不 clone 的话,下面这行用不了
println!("{}", field_name);
```

**同一个 key 再 insert 就是覆盖**(第 9 节),但旧值没有凭空消失 —— 它从返回的
`Option` 里还给你:

```rust
let old = scores.insert(String::from("blue"), 25);   // old == Some(10)
```

**`get` 和 Vec 完全一样**(第 10 节),返回 `Option`,查无此键给 `None`。这是今天最值得
记住的一条:两个结构完全不同的容器,在「取不到怎么办」这件事上给了同一个答案。

**合并两个 map 有两种语义**(第 12 节),选错就是数据丢失:

| 写法 | 语义 | `map1={a:1,b:2}` 合并 `map2={b:3,c:4}` |
|------|------|--------------------------------------|
| `for (k,v) in &m2 { out.insert(*k,*v); }` | 后来的**覆盖**先来的 | `{a:1, b:3, c:4}` — b 的 2 丢了 |
| `for (k,v) in &m2 { out.entry(*k).or_insert(*v); }` | 先来的**保留**,缺的才补 | `{a:1, b:2, c:4}` — b 保住了 |

遍历顺序**不保证**(第 11 节),每次运行都可能不同,不要依赖它。

## Practice

```bash
cd day7/data-structure-cookbook && cargo run --example practice
```

1. **越界了也不要崩**(🌟)— 写 `describe(v, idx)`,不许用 `v[idx]`;越界返回 `"越界"` 而不是 panic
2. **合并,但先来的说了算**(🌟🌟)— 写 `merge_keep(base, extra)`,重复的 key 保留 `base` 的值
3. **insert 之后它去哪了**(🌟🌟)— 一段会报 E0382 的代码,先说清报错原因,再用**两种**方式修好
4. **一列里放三种东西**(🌟🌟)— 定义 `enum Cell`,写 `sum_numeric` 只把数字加起来、跳过文字。期望 `14.5`
5. **先预测,再运行**(🌟)— `with_capacity(3)` 之后 push 四个,`len` 和 `capacity` 各是多少?先猜后验
6. **数一数每个词出现几次**(🌟🌟🌟)— 写 `word_count(text)`,`"a b a c a"` → `{a:3, b:1, c:1}`。函数体三行

## Questions I asked

<!-- 这几条是从 main.rs 现场注释里反推出来的,当天真正问过的如果有遗漏,补在下面。 -->

- **Q:** 两个 key 的哈希值确实不同,但是不是所有 key 的哈希值都会不同?
  **A:** 不是。哈希值是固定位数(`u64`)而 key 的数量无限,所以冲突一定存在,这叫鸽笼原理。
  HashMap 的正确性不依赖「不冲突」,而依赖「冲突时也能分辨」—— 落到同一个桶里的 key 会
  再逐个比较真值。哈希函数的目标是让冲突少到不影响 O(1) 的平均表现。

- **Q:** `map.insert(field_name, ...)` 之后 `field_name` 为什么就不能用了?
  **A:** `insert` 的参数是 `K` 不是 `&K` —— 它要把 key 存进 map 里长期持有,所以必须拥有它。
  `String` 不是 `Copy`,传进去就是移动。要两边都留就 `clone`。key 换成 `&str` 就没这问题,
  因为 `&str` 是 `Copy`(但那样 map 就得借着别人的数据活,受生命周期约束)。

- **Q:** `entry()` 到底返回什么?为什么单写一行 `map.entry(k);` 什么都没发生?
  **A:** 它返回一个 `Entry` 枚举 —— 代表「map 里 k 这个位置」,`Occupied` 或 `Vacant`。
  它只是把那个位置交给你,你得接着说要拿它干什么(`or_insert` / `or_insert_with` /
  `and_modify`)。什么都不接就等于没动过。

- **Q:** `capacity` 和 `len` 差在哪?
  **A:** `len` 是真的放了几个,`capacity` 是已经要来几个位置。`with_capacity(10)` 之后
  `len == 0` 而 `capacity == 10`。预分配省掉的是重新分配 + 搬数据的开销。

## Errors I hit

- **`entry()` 只写一半 —— 编译过、不报警、什么都不做。**
  写成 `scores.entry(String::from("yellow"));`,注释里以为它是「存在就不插入,不存在就插入」。
  → **cause:** `entry()` 返回的 `Entry` 不带 `#[must_use]`,所以丢掉它不产生任何警告;
  而 `Entry` 本身只是「位置」,不是「操作」。
  → **fix:** 接上 `.or_insert(0)`。今天最值得记的一个错 —— 它没有任何信号,只有结果是错的。

- **`error[E0382]: borrow of moved value` —— insert 之后再用原变量。**
  ```rust
  map.insert(field_name, field_value);
  println!("{}", field_name);   // ❌ E0382
  ```
  → **cause:** `insert(K, V)` 按值接收,`String` 不是 `Copy`,所以是移动不是复制。
  → **fix:** `map.insert(field_name.clone(), field_value.clone())`;或者调整顺序,
  在 insert 之前把要打印的东西先用掉。

- **`warning: unused import` —— practice.rs 出题时的自伤。**
  `use std::collections::HashMap;` 写在文件顶部,但答案槽还空着,没有一处用到它。
  → **cause:** 骨架文件的固有状态:import 是为将来写的答案准备的。
  → **fix:** 暂时挂 `#[allow(unused_imports)]`,题做完就把它删掉。

## Plan for Day 8

- **补上 `String`** —— 三件套 Vec / String / HashMap 今天只做了两件。重点在
  `push_str` 与 `+` 的所有权差别、`format!`、以及**为什么 String 不能用下标索引**
  (字节 ≠ 字符,这一条会直接接上 Day 4 的切片)
- day8 已经开好:`module` —— 模块系统、`mod` / `use` / `pub`、把今天这堆散落在
  `fn main` 里的东西拆进模块
- practice 里的第 6 题(`word_count`)是 `entry` 的实战形态,做完再看一遍第 12 节
