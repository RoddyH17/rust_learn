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

    // ---------- 5. Tuples ----------

    // A tuple groups values of DIFFERENT types into one value.
    let point: (i32, f64, u8) = (500, 6.4, 1);
    println!("point.0 = {}, point.1 = {}", point.0, point.1);

    // Destructuring is the usual way to take one apart. It matches by
    // POSITION, not by name — the names on the left are yours to choose.
    let (id, score, level) = point;
    println!("id = {}, score = {}, level = {}", id, score, level);

    // A tuple is also how a function returns more than one value.
    let (total, product) = sum_multiply((2, 3));
    println!("sum = {}, product = {}", total, product);

    // ---------- 6. Arrays ----------

    // An array is one type, fixed length — and the LENGTH IS PART OF THE TYPE.
    // `[i32; 5]` and `[i32; 4]` are two different types.
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("nums = {:?}, len = {}", nums, nums.len());

    // Fill every slot with the same value: [value; count]
    let ones = [1; 5];
    println!("ones = {:?}", ones);

    // Arrays live on the stack. A `Vec` is the heap version that can grow.
    let mut growable = vec![1, 2, 3];
    growable.push(4);
    println!("growable = {:?}", growable);

    // A `char` is a 4-byte Unicode scalar value, not a byte — so three of
    // them take 12 bytes, not 3.
    let letters = ['a', 'b', 'c'];
    println!("size_of_val(&letters) = {}", std::mem::size_of_val(&letters));

    // Two ways to read an element: one panics, one does not.
    println!("nums.get(10) = {:?}", nums.get(10)); // None — safe
    //println!("{}", nums[10]);  // <- uncomment: compiles, then panics at runtime

    // ---------- 7. Statements and expressions ----------

    // NO semicolon = expression, it has a value.
    //    semicolon = statement,  its value is `()`.
    // A block is an expression, so it evaluates to its last expression.
    let y = {
        let inner = 3;
        inner + 3        // no semicolon: this is the value of the block
    };
    println!("y = {}", y);

    // The `allow` below only silences a warning; the compiler is pointing at
    // exactly the lesson here — "the arithmetic operation produces a value"
    // that the trailing semicolon then throws away.
    #[allow(unused_must_use)]
    let z = {
        let inner = 3;
        inner + 3;       // semicolon: the block's value is now ()
    };
    println!("z = {:?}", z); // prints ()

    // Same rule decides what a function returns — see `five()` below.
    println!("five() = {}", five());

    // ---------- 8. if as an expression ----------

    // `if` produces a value, so it can sit on the right of a `let`.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);
    // Both arms must have the SAME type, because `number` gets exactly one:
    //let bad = if condition { 5 } else { "six" };  // <- uncomment: type error

    // ---------- 9. Loops ----------

    // Ranges: `..` excludes the end, `..=` includes it.
    print!("1..4   ->");
    for n in 1..4 {
        print!(" {}", n);
    }
    print!("\n1..=4  ->");
    for n in 1..=4 {
        print!(" {}", n);
    }
    print!("\nreversed ->");
    for n in (1..=4).rev() {
        print!(" {}", n);
    }
    println!();

    // Index AND value, with no bounds-check cost and no way to overrun:
    for (i, v) in [4, 3, 2, 1].iter().enumerate() {
        println!("element {} is {}", i + 1, v);
    }

    // `loop` is an expression: `break` can carry a value out of it.
    // This works ONLY in `loop` — `for` and `while` cannot break with a value,
    // because neither is guaranteed to reach the break.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result = {}", result); // 20

    // Labels let an inner loop break out of an outer one.
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;              // leaves the inner loop only
            }
            if count == 2 {
                break 'counting_up; // leaves BOTH loops
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count = {}", count);

    // ---------- 10. Loops and ownership ----------

    // This is section 4's borrowing rule wearing loop syntax.
    // `for x in collection` MOVES the collection into the loop.

    let words = [String::from("liming"), String::from("hanmeimei")];
    for word in &words {          // borrow: `words` survives
        println!("word = {}", word);
    }
    println!("words still usable = {:?}", words);
    //for word in words {}          // <- this would move `words`...
    //println!("{:?}", words);      // <- ...and then this line fails

    // But an array of i32 is fine either way, because i32 is `Copy`:
    // the array is copied into the loop and the original is untouched.
    let numbers = [1, 2, 3];
    for n in numbers {
        print!("{} ", n);
    }
    println!("\nnumbers still usable = {:?}", numbers);

    // `&mut` lets the loop change the elements in place.
    let mut scores = vec![1, 2, 3];
    for score in &mut scores {
        *score *= 10;             // `*` to reach through the reference
    }
    println!("scores = {:?}", scores);

    // The three forms, worth memorising:
    //   for x in &v       -> v.iter()      -> &T      -> v still usable
    //   for x in &mut v   -> v.iter_mut()  -> &mut T  -> v still usable, changed
    //   for x in v        -> v.into_iter() -> T       -> v consumed (unless Copy)
    //
    // A `Vec` is never `Copy`: copying it would mean either two Vecs pointing
    // at the same heap block (double free) or a silent deep copy. Rust refuses
    // both, so a Vec can only ever move.
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

// Returning two values at once, packed into a tuple.
// Every parameter must have a type annotation — that is what lets the
// compiler infer types almost everywhere else.
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

// The body is a lonely `5` with NO semicolon, so it is an expression and
// becomes the return value. Adding a semicolon here would make the function
// return `()` and fail to compile against `-> i32`.
fn five() -> i32 {
    5
}
