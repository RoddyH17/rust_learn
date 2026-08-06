// Day 2 practice — do the exercises in order.
//
// Run with:  cargo run --example practice
//
// This file compiles as it is. Each exercise asks you to uncomment code,
// fix it, or write a few lines yourself. After each change, run the file
// again and check the output. Solutions are NOT given here on purpose —
// check yourself against the rubric in day2/NOTES.md.
//
//   1–5   binding a value: mut, const/static, shadowing, references
//   6–10  using a value:   expressions, if, tuples, arrays, loops
//
// Exercises 6–10 are adapted from Rust By Practice
// (https://practice-rust-zh.beatai.org/), graded the same way that site does:
//   🌟 warm-up   🌟🌟 the one that actually teaches something   🌟🌟🌟 challenge

// Exercise 2 asks you to fill these in at module level:
// TODO: define a `const` named MAX_SCORE with type u32 and value 100.
const MAX_SCORE: u32 = 100; 
// TODO: define a `static` named COURSE with type &str and value "Rust Day 2".
static COURSE: &str = "Rust Day 2"; 

fn main() {
    println!("--- Exercise 1: make it compile (two ways) ---");
    // The code below does not compile. First uncomment it and read the error.
    // Then fix it in BOTH of these ways, one after the other:
    //   (a) with `mut`
    //   (b) without `mut`, using shadowing
    //
    let mut count = 0;
    count = count + 1;
    println!("count = {}", count);

    println!("--- Exercise 2: const and static ---");
    // Fill in the two TODOs at the top of this file, then uncomment:
    //
    println!("{}: max score is {}", COURSE, MAX_SCORE);
    //
    // Then try to write `MAX_SCORE = 50;` here, read the compiler error,
    // and delete the line again.

    println!("--- Exercise 3: shadowing with a type change ---");
    // Start with the string "42" and end with the number 42 plus 1,
    // using the SAME variable name for both. One way to convert:
    //   let answer: u32 = answer.trim().parse().unwrap();
    //
    let answer = "42"; // Mut: can not change type; while shadowing can change the type
    let answer: u32 = answer.trim().parse().unwrap();
    println!("{} + 1 = {}", answer, answer + 1);
    // TODO: shadow `answer` so it becomes a number, add 1, print it.
    // Expected output: answer + 1 = 43


    println!("--- Exercise 4: write a borrowing function ---");
    // TODO: below main, write a function
    // that appends "!" to the string (use push_str).
    // Then uncomment:
    fn shout(text: &mut String){
        text.push_str("!");
    }
    let mut cheer = String::from("go");
    shout(&mut cheer);
    shout(&mut cheer);
    println!("cheer = {}", cheer);   // expected: go!!

    println!("--- Exercise 5: break the borrowing rule on purpose ---");
    // Predict first, then check: which of the four println! lines below
    // would the compiler reject if you uncomment the whole block? Why?
    //
    let mut word = String::from("hi");
    //let r1 = &word;
    //let r2 = &word;
    //println!("{} {}", r1, r2); // still right
    let r3 = &mut word;
    println!("{}", r3);// can work, but cannot work with the first two r1, r2 lines. 
    //println!("{}", r1);   // r1 alr gone? No! r1, r2 cannnot be borrowed. 
    //
    // Rule to check against: any number of `&T`, OR exactly one `&mut T`.

    println!("--- Exercise 6: statements vs expressions (🌟🌟) ---");
    // The whole rule, in one line:
    //   NO semicolon = expression, it HAS a value.
    //      semicolon = statement,  its value is `()`.
    //
    // (a) Uncomment `sum_of` at the bottom of this file. It does not compile.
    //     Read the error, then fix it by deleting ONE character.
    //     Then uncomment this:
    // println!("sum_of(1, 2) = {}", sum_of(1, 2));   // expected: 3
    //
    // (b) Uncomment the block below and make `v` equal 3 in TWO ways:
    //     first by removing a semicolon, then by putting `x` back explicitly
    //     as a final line. Both should pass the assert.
    //
    // let v = {
    //     let mut x = 1;
    //     x += 2;
    // };
    // assert_eq!(v, 3);
    // println!("v = {}", v);
    //
    // (c) Why does `let v = (let x = 3);` not compile in any language Rust
    //     would accept? Say it out loud using the words above.

    println!("--- Exercise 7: if is an expression (🌟🌟) ---");
    // Uncomment the block below. It has TWO separate errors — one about
    // types, one about semicolons. Predict both before you compile.
    //
    let n = 5;
    // let big_n =
    //     if n < 10 && n > -10 {
    //         println!("数字太小，先增加 10 倍再说");
    //         10 * n
    //     } else {
    //         println!("数字太大，我们得让它减半");
    //         n / 2.0 ;
    //     }
    //
    // println!("{} -> {}", n, big_n);
    //
    // Hint for error 1: what is the type of `10 * n`? And of `n / 2.0`?
    //   Every arm of an `if` used as an expression must produce the SAME type.
    // Hint for error 2: this is exercise 6 showing up again.
    println!("n = {}", n);

    println!("--- Exercise 8: tuples (🌟🌟) ---");
    // (a) Destructure this tuple in ONE line so all three asserts pass.
    //     Read the assert order carefully — it is not the obvious answer.
    //
    let tup = (1, 6.4, "hello");
    // let __ = tup;
    // assert_eq!(x, 1);
    // assert_eq!(y, "hello");
    // assert_eq!(z, 6.4);
    println!("tup.0 = {}, tup.2 = {}", tup.0, tup.2);
    //
    // (b) A tuple is also how a function returns two values at once.
    //     Uncomment `sum_multiply` at the bottom, then work out what to
    //     pass in so that x == 5 and y == 6. (Pen and paper is fine.)
    //
    // let (x, y) = sum_multiply(__);
    // assert_eq!(x, 5);
    // assert_eq!(y, 6);

    println!("--- Exercise 9: arrays (🌟🌟) ---");
    // (a) An array's length is part of its TYPE: [T; N]. Predict the number
    //     below before you uncomment it. A `char` in Rust is a Unicode
    //     scalar value, not a byte.
    //
    let letters: [_; 3] = ['a', 'b', 'c'];
    // assert!(std::mem::size_of_val(&letters) == __);
    println!("letters = {:?}", letters);
    //
    // (b) Fill in a 100-element array where every element is 1,
    //     without typing 100 ones.
    //
    // let list: [i32; 100] = __;
    // assert!(list[0] == 1 && list.len() == 100);
    //
    // (c) Two ways to read an element — one is safe, one can panic.
    //
    let people = [String::from("Sunfei"), "Sunface".to_string()];
    let first = people.get(0).unwrap();
    println!("first = {}", first);
    // let second = &people[2];   // uncomment: this compiles, then panics.
    //
    // Question to answer in NOTES: the index 2 is obviously out of range and
    // the length is known at compile time — so why is this a runtime panic
    // and not a compile error? What would you use instead?

    println!("--- Exercise 10: loops and ownership (🌟🌟🌟) ---");
    // This is the most important exercise of the day: it is the ownership
    // rule from exercises 4–5 hiding underneath ordinary loop syntax.
    //
    // (a) One of these two loops destroys its array and one does not.
    //     Uncomment BOTH println! lines. Only one will fail to compile.
    //     Predict which, then fix it by adding ONE character.
    //
    let words = [String::from("liming"), String::from("hanmeimei")];
    for word in words {
        let _ = word;
    }
    // println!("{:?}", words);

    let numbers = [1, 2, 3];
    for number in numbers {
        let _ = number;
    }
    // println!("{:?}", numbers);
    //
    // Then say why they differ, in terms of `Copy`. Write the three-row
    // table (`for x in v` / `&v` / `&mut v`) from memory in NOTES.md.
    //
    // (b) `loop` is an expression too — `break` can carry a value out of it.
    //     Fill in the blank so `result` is 20.
    //
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         __;
    //     }
    // };
    // assert_eq!(result, 20);
    //
    // (c) 🌟🌟🌟 Labels. Trace this by hand FIRST — write the value of
    //     `count` on paper at each step — then uncomment and check.
    //
    // let mut count = 0;
    // 'outer: loop {
    //     'inner1: loop {
    //         if count >= 20 { break 'inner1; }
    //         count += 2;
    //     }
    //     count += 5;
    //     'inner2: loop {
    //         if count >= 30 { break 'outer; }
    //         continue 'outer;
    //     }
    // }
    // assert!(count == __);

    println!("--- done ---");
}

// ---------- helpers for exercises 6 and 8 ----------

// Exercise 6(a): uncomment. Delete one character to make it compile.
// fn sum_of(x: i32, y: i32) -> i32 {
//     x + y;
// }

// Exercise 8(b): uncomment. This one is already correct — the exercise is
// working out what argument produces (5, 6).
// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }
