// Day 2 — Variables: const, static, mutability, shadowing, and references
//
// Run the examples:  cargo run
// Do the exercises:  cargo run --example practice

// ---------- Constants and statics (module level, outside main) ----------

// `const` — a compile-time constant.
// - The type annotation (`: u32`) is required.
// - The value must be known at compile time and can never change.
// - The compiler copies the value into every place it is used.
// - Naming convention: SCREAMING_SNAKE_CASE.
const MAX_DAYS: u32 = 100;

// `static` — a global variable.
// - Lives at one fixed memory address for the whole program.
// - Exists for the entire run of the program (the `'static` lifetime).
// - Like normal variables, it is immutable by default.
static LANGUAGE: &str = "Rust";

// `static mut` (a mutable global) exists, but every read or write requires
// an `unsafe` block, because two parts of a program could change it at the
// same time. Takeaway: Rust makes global mutable state hard on purpose.
// Prefer passing values into functions instead.

fn main() {
    // ---------- 1. Using const and static ----------

    println!("Learning {} for up to {} days.", LANGUAGE, MAX_DAYS);
    //MAX_DAYS = 200;   // <- uncomment: error, a const is not even a variable
    //LANGUAGE = "Go";  // <- uncomment: error, cannot assign to a static

    // ---------- 2. When can a variable change? ----------

    // A `let` variable is immutable by default: bind once, never reassign.
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // <- uncomment: error: cannot assign twice to immutable variable

    // Add `mut` to make it changeable. This is the ONLY way a local
    // variable can be reassigned.
    let mut y = 5;
    y += 1;
    println!("y = {}", y);

    // Summary of "who can change":
    //   const      -> never, fixed at compile time
    //   static     -> no (without unsafe)
    //   let        -> no
    //   let mut    -> yes

    // ---------- 3. Shadowing ----------

    // `let` with the same name creates a NEW variable that hides the old one.
    // This is not mutation: the old variable still exists, it is just hidden.
    let spaces = "   ";        // &str (a string)
    let spaces = spaces.len(); // usize (a number) — the type changed
    println!("spaces = {}", spaces);

    // Difference from `mut`:
    // - `mut` changes the value of ONE variable; the type must stay the same.
    // - shadowing makes a NEW variable; the type may change,
    //   and the new variable is still immutable.

    // ---------- 4. References ----------

    // Passing a String to a function normally moves it into the function,
    // and the caller cannot use it afterwards.
    // A reference lends the value instead: `&s` points to the value
    // but does not own it. This is called borrowing.
    let s = String::from("borrowing");
    let len = length_of(&s);
    println!("'{}' has length {}", s, len); // s is still usable here

    // `&mut` lends the value AND allows the function to change it.
    let mut msg = String::from("hello");
    add_world(&mut msg);
    println!("after add_world: {}", msg);

    // The borrowing rule, enforced by the compiler:
    // at any time, EITHER any number of `&T` (read-only)
    // OR exactly one `&mut T` (writable) — never both.
    let a = &msg;
    let b = &msg;            // two read-only borrows: fine
    println!("{} / {}", a, b);
    //let c = &mut msg;     // <- uncomment while a/b are in use: error
}

// `&String` = this function borrows the string, read-only.
// `-> usize` = it returns a number. The last expression, written without
// a semicolon, is the return value.
fn length_of(text: &String) -> usize {
    text.len()
}

// `&mut String` = this function borrows the string and may change it.
fn add_world(text: &mut String) {
    text.push_str(", world");
}
