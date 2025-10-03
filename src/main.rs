fn main() {
    let s = String::from("Forty two"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    println!("So we can't access it here anymore");

    let x = 42; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    println!("So we can still access it: {x}");
} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("Takes ownership of: {some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("Makes copy of: {some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
