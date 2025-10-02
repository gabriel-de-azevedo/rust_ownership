fn main() {
    let some_text = "some_text";
    let some_more_text = ", world";

    println!("{some_text}");

    let mut some_text = String::from(some_text);
    some_text.push_str(some_more_text);

    println!("{some_text}");
}
