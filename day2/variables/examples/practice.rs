// Day 2 practice — do the exercises in order.
//
// Run with:  cargo run --example practice
//
// This file compiles as it is. Each exercise asks you to uncomment code,
// fix it, or write a few lines yourself. After each change, run the file
// again and check the output. Solutions are NOT given here on purpose —
// check yourself against the rubric in day2/NOTES.md.

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

    println!("--- done ---");
}
