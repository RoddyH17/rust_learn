// Day 3 practice — ownership, moves, and borrowing.
//
// Run with:  cargo run --example practice
//
// This file compiles as it is. Each exercise asks you to uncomment code,
// fix it, or write a few lines yourself. After each change, run the file
// again and check the output. Solutions are NOT given here on purpose —
// check yourself against the rubric in day3/NOTES.md.
//
//   1–6   ownership: what a move is, and when a value is copied instead
//   7–10  borrowing: references, the two rules, and NLL
//
// Adapted from Rust By Practice, graded the same way that site does:
//   🌟 warm-up   🌟🌟 the one that actually teaches something   🌟🌟🌟 challenge
//   https://practice-rust-zh.beatai.org/ownership/ownership.html
//   https://practice-rust-zh.beatai.org/ownership/borrowing.html

fn main() {
    println!("--- Exercise 1: a move, fixed as many ways as you can (🌟🌟) ---");
    // Uncomment. It does not compile: `x` is moved into `y`, then used again.
    // Find as MANY different fixes as you can — there are at least four, and
    // they are not equally good. For each one, say what it costs.
    //   (hint: one duplicates the heap data, one avoids owning at all,
    //    one changes what the value even is, one just reorders)
    //
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{}, {}", x, y);
    println!("(see the comments)");

    println!("--- Exercise 2: give the ownership back (🌟🌟) ---");
    // `take_ownership` at the bottom of this file swallows the String and
    // never returns it, so `s2` has nothing to bind to.
    // Fix it by changing ONLY the function — do not touch these three lines.
    //
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);
    println!("{}", s2);

    println!("--- Exercise 3: fix it without deleting a line (🌟🌟) ---");
    // Uncomment. `print_str` takes the String by value, so the println!
    // after it fails. Fix it WITHOUT removing any line.
    // Two approaches exist: change the function signature, or change the
    // call. Do both, then decide which you would ship and why.
    //
    let s = String::from("hello, world");
    print_str(&s);
    println!("{}", s);

    println!("--- Exercise 4: copy instead of clone (🌟🌟) ---");
    // This works, but `.clone()` here is doing real heap work. Change the
    // tuple so that `let y = x;` copies instead — WITHOUT calling clone.
    // Then explain what property the tuple has to have.
    //
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
    // TODO: make the line below work with `let y = x;` and no clone.

    println!("--- Exercise 5: mutability can change on transfer (🌟) ---");
    // Uncomment. `s` is immutable, so `push_str` fails — but ownership is
    // moving to `s1` anyway. Modify ONE line so it compiles.
    // Question to answer: whose property is mutability, the value's or the
    // binding's?
    //
    // let s = String::from("hello, ");
    // let s1 = s;
    // s1.push_str("world");
    // println!("{}", s1);

    println!("--- Exercise 6: partial move (🌟🌟) ---");
    // (a) Uncomment. Moving `t.0` out moves PART of the tuple, so `t` as a
    //     whole can no longer be printed. Change only the println! so it
    //     works, without using `_s`.
    //
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{:?}", t.1);
    //
    // (b) Now fill this in so that s1, s2 AND t are all still usable.
    //     Expected: "hello", "world", ("hello", "world")
    //     (hint: the `ref` keyword, or borrowing the whole tuple)
    //
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = &t;
    println!("{:?}, {:?}, {:?}", s1, s2, t);

    println!("--- Exercise 7: taking a reference (🌟) ---");
    // (a) Fill in so the program prints the memory address of `n`.
    //     `{:p}` is the pointer format specifier.
    //
    let n = 5;
    let p = &n;
    println!("the address of n is {:p}", p);
    //
    // (b) Uncomment. This fails: you cannot compare an `&i32` to an `i32`.
    //     Fix it by changing ONLY the assert line.
    //
    let m = 5;
    let r = &m;
    assert_eq!(5, *r);
    println!("n = {}", n);

    println!("--- Exercise 8: passing borrows to functions (🌟) ---");
    // Both calls below are wrong. Uncomment and fix each with one character.
    // `borrow_object` and `push_world` are at the bottom of this file.
    //
    let mut s = String::from("hello, ");
    borrow_object(&s);
    push_world(&mut s);
    println!("{}", s);

    println!("--- Exercise 9: the two rules (🌟🌟) ---");
    // (a) Uncomment. Two `&mut` to the same value at once — illegal.
    //     Make it work by REMOVING part of the code. You may not delete a
    //     whole line.
    //
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    //
    // (b) Uncomment. This one fails for a different reason — read the error
    //     carefully, it is not the same rule. Fix it by changing one line.
    //
    // let t = String::from("hello, ");
    // borrow_mut_object(&mut t);
    //
    // (c) Why does this one compile with no error at all? Say it in terms
    //     of rule 1.
    //
    // let mut u = String::from("hello, ");
    // borrow_object(&u);
    // u.push_str("world");

    println!("--- Exercise 10: NLL and ref (🌟🌟🌟) ---");
    // (a) Uncomment. It fails. Fix it by COMMENTING OUT exactly one line —
    //     and be able to say why removing that line ends r1's borrow early.
    //
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    //println!("{}", r1);
    //
    // (b) The reverse drill: this compiles right now. Add ONE line at the
    //     end that deliberately causes
    //     "cannot borrow `s` as mutable more than once at a time".
    //
    // let mut s = String::from("hello, ");
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // (c) 🌟🌟🌟 `ref` does the same job as `&`, but on the other side of
    //     the `=`. Fill in the blank without touching any other line.
    //     Both asserts must pass — the second one proves no copy was made.
    //
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("--- done ---");
}

// ---------- functions used by the exercises ----------

// Exercise 2: change ONLY this function so the caller can bind its result.
fn take_ownership(s: String) -> String {
     println!("{}", s);
     s
 }

// Exercise 3: this takes the String by value.
fn print_str(s: &String) {
     println!("{}", s);
 }

// Exercise 8 and 9(c): a read-only borrow.
fn borrow_object(s: &String) {
     println!("borrowed: {}", s);
}

// Exercise 8: a mutable borrow.
fn push_world(s: &mut String) {
     s.push_str("world");
}

// Exercise 9(b): needs a mutable borrow.
// fn borrow_mut_object(s: &mut String) {
//     s.push_str("world");
// }

// Exercise 10(c): formats the address a reference points at.
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
 }
