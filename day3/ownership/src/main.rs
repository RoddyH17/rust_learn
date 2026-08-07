// Day 3 — Ownership and Memory (Notion 1.2)
//
// Run the examples:  cargo run
// Do the exercises:  cargo run --example practice
//
// Day 2 ended on a puzzle: `for name in names` destroyed the array, but the
// identical loop over numbers did not. This is the chapter that explains it.
//
// The one question underneath everything here:
//   when a value is stored on the heap, WHO is responsible for freeing it?
//
//   - C/C++ answer: you are. Forget -> leak. Too early -> dangling pointer.
//     Twice -> double free.
//   - Java/Go/Python answer: a garbage collector, at runtime, at a cost.
//   - Rust answer: exactly one variable OWNS it, and the memory is freed when
//     that owner goes out of scope. Checked at COMPILE time, costs nothing
//     at runtime.

fn main() {
    // ---------- 1. Stack vs heap ----------

    // Stack: last in, first out. Every value must have a known, fixed size.
    // Pushing is fast because there is never a search — the next slot is
    // always the top of the stack.
    let stack_number = 42;          // i32, 4 bytes, size known at compile time
    let stack_tuple = (1, 2.5);     // also fixed size
    println!("on the stack: {} {:?}", stack_number, stack_tuple);

    // Heap: for data whose size is unknown at compile time or can change.
    // You ask the allocator for space, it finds a big enough spot, marks it
    // used, and hands back a POINTER. The pointer itself is fixed-size, so
    // the pointer lives on the stack and the data lives on the heap.
    let mut heap_string = String::from("hello");
    heap_string.push_str(", world!");   // can grow — this is why it needs the heap
    println!("on the heap: {}", heap_string);

    // A String is three words on the stack — pointer, length, capacity —
    // pointing at the actual bytes on the heap.
    println!(
        "String stack size = {} bytes (ptr + len + capacity)",
        std::mem::size_of::<String>()
    );

    // A string literal is different: it is baked into the binary, so it is
    // immutable and needs no allocation at all.
    let literal = "I am hardcoded into the executable";
    println!("{}", literal);

    // ---------- 2. Scope and drop ----------

    // The owner is freed the moment it goes out of scope. There is no `free`
    // to call and no garbage collector deciding when — the closing brace is
    // the deallocation point.
    {
        let scoped = String::from("I live inside these braces");
        println!("{}", scoped);
    } // <- `drop` runs HERE, automatically. The heap memory is returned.
    //println!("{}", scoped);   // <- uncomment: error, `scoped` no longer exists

    // ---------- 3. Move — the shallow copy that invalidates ----------

    // 重点: 一旦我们用 let s2 = s1 这种 statement 了, 那么其实, s1 就已经被 free 了
    //
    // Assigning a String copies the three stack words (pointer, len, capacity)
    // but NOT the heap data. That would leave two owners pointing at one
    // heap block — and both would try to free it at the end of scope.
    // That is a DOUBLE FREE, a real memory-safety bug.
    //
    // Rust's fix is not to copy the heap data. It is to invalidate the
    // original. This is called a MOVE.
    let s1 = String::from("hello");
    let s2 = s1;                 // s1 is moved into s2, and s1 is now dead
    println!("s2 = {}", s2);
    //println!("s1 = {}", s1);   // <- uncomment: error[E0382]: borrow of moved value

    // So Rust never does a deep copy automatically. Anything that would be
    // expensive is something you have to ask for by name.

    // ---------- 4. clone — the explicit deep copy ----------

    let original = String::from("copy me");
    let deep = original.clone();     // heap data IS duplicated here
    println!("original = {}, deep = {}", original, deep); // both still valid

    // The point of making this a visible method call: when you read `.clone()`
    // you know some arbitrary and possibly expensive work is happening.

    // ---------- 5. The Copy trait — why numbers behave differently ----------

    // Types entirely on the stack with a fixed size can implement `Copy`.
    // Copying them is trivial (no allocation, no ownership question), so
    // assignment duplicates instead of moving.
    let a = 5;
    let b = a;                   // a is COPIED, not moved
    println!("a = {}, b = {}", a, b);   // both still valid

    // Copy types: all integers, floats, bool, char, and tuples made only
    // of Copy types. Not String, not Vec — anything owning heap data.
    let copyable = (1, 2.5, 'x', true);
    let also_copyable = copyable;
    println!("{:?} and {:?}", copyable, also_copyable);

    // THIS is the answer to Day 2's loop puzzle: `[i32; 3]` is Copy so the
    // for loop copied it; `[String; 2]` is not, so the for loop moved it.

    // ---------- 6. Ownership and functions ----------

    // Passing a value to a function is a move, exactly like assignment.
    let gift = String::from("take this");
    takes_ownership(gift);
    //println!("{}", gift);     // <- uncomment: error, gift was moved into the function

    let number = 5;
    makes_copy(number);
    println!("number is still usable: {}", number);  // fine — i32 is Copy

    // Returning a value moves ownership back out to the caller.
    let given = gives_ownership();
    println!("given = {}", given);

    let round_trip = takes_and_gives_back(given);
    println!("round_trip = {}", round_trip);

    // Handing ownership in and back out just to use a value is tedious.
    // That is what references are for.

    // ---------- 7. References — borrowing instead of taking ----------

    // `&s` creates a reference: an address we can follow, pointing at data
    // owned by someone else. Because the reference does not own the value,
    // nothing is dropped when the reference goes away.
    let text = String::from("borrow me");
    let length = calculate_length(&text);
    println!("'{}' has length {}", text, length);  // text is still ours

    // Creating a reference is called BORROWING. Like real life: you can use
    // it, you have to give it back, and you do not get to destroy it.

    // ---------- 8. Mutable references ----------

    // A plain `&` borrow is read-only. To change a borrowed value you need
    // `&mut`, and the owner itself has to be declared `mut`.
    let mut editable = String::from("hello");
    append_world(&mut editable); 
    println!("after append_world: {}", editable);

    // The big restriction: while a `&mut` is alive you can have NO other
    // reference to that value — not another `&mut`, not even a plain `&`.
    //
    // Why: two mutable references = a data race. And a reader holding `&`
    // does not expect the value to change underneath them.
    let mut guarded = String::from("value");
    let w1 = &mut guarded;
    w1.push_str("!");
    //let w2 = &mut guarded;    // <- uncomment while w1 is still used below: error
    println!("w1 = {}", w1);

    // Multiple read-only borrows are fine — nobody reading can disturb
    // anyone else reading.
    let shared = String::from("read me");
    let r1 = &shared;
    let r2 = &shared;
    println!("{} / {}", r1, r2);

    // ---------- 9. NLL — a borrow ends at its LAST USE ----------

    // 重点: 借用的生命周期不看大括号在哪, 只看它最后一次被使用在哪
    //       两个借用的"使用区间"不重叠 -> 编译器就放行
    //
    // This is called Non-Lexical Lifetimes. The borrow checker looks at the
    // range over which a reference is actually USED, not at its lexical
    // scope. So this compiles even though all three references live in the
    // same block:
    let mut nll = String::from("hello");

    let n1 = &nll;
    let n2 = &nll;
    println!("{} and {}", n1, n2);   // last use of n1 and n2 -> their borrows END here

    let n3 = &mut nll;               // so taking a &mut now is fine
    n3.push_str(", world");
    println!("{}", n3);

    // But add `println!("{}", n1);` after this point and the two ranges
    // overlap again — instant error. The rule is about USE, not braces.

    // ---------- 10. Dangling references ----------

    // The classic C++ trap: return a pointer to a local, and the local dies
    // on the way out, leaving you holding an address to freed memory.
    // Rust rejects this at COMPILE time rather than crashing at runtime.
    //
    // fn dangle() -> &String {           // <- does not compile
    //     let s = String::from("hello");
    //     &s                             // returning a reference to a local...
    // }                                  // ...which is dropped right here
    //
    // error: this function's return type contains a borrowed value, but
    //        there is no value for it to be borrowed from
    //
    // The fix is not to fight the borrow checker — it is to return the
    // String itself, moving ownership out to the caller. Then nothing is
    // freed here at all.
    println!("no_dangle() = {}", no_dangle());

    // ---------- 11. The rules of references ----------

    // 1. At any given time you may have EITHER one mutable reference
    //    OR any number of immutable references.
    // 2. References must always be valid.
    //
    // In one line: 读可以并发, 写必须独占 — readers can share, a writer needs
    // exclusivity. It is the same mental model as a read-write lock, except
    // Rust checks it at compile time, so it costs nothing at runtime.
    //
    //   currently alive   | take another &T | take another &mut T
    //   ------------------|-----------------|--------------------
    //   nothing           | yes             | yes
    //   &T  (used later)  | yes             | no
    //   &mut T (used later)| no             | no
}

// ---------- functions used above ----------

// Takes the String by value: ownership moves in, and when this function
// ends, `some_string` goes out of scope and the memory is freed.
fn takes_ownership(some_string: String) {
    println!("takes_ownership got: {}", some_string);
}

// i32 is Copy, so the caller's value is duplicated and survives.
fn makes_copy(some_integer: i32) {
    println!("makes_copy got: {}", some_integer);
}

// Moves its return value out to whoever calls it.
fn gives_ownership() -> String {
    String::from("freshly made")
}

// Takes ownership, then hands it straight back.
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// `&String` — borrows, read-only. The value is NOT dropped when this returns,
// because this function never owned it.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// `&mut String` — borrows and is allowed to modify.
fn append_world(s: &mut String) {
    s.push_str(", world");
}

// The fix for `dangle`: move the String out instead of lending a reference
// to something that is about to die.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
