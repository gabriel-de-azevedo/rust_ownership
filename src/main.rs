fn main() {
    let some_text = "some_text";
    let some_more_text = ", world";

    println!("{some_text}");

    let mut some_text = String::from(some_text); // A string literal cannot grow in size, so we need to cast our variable to a String
    some_text.push_str(some_more_text);

    println!("{some_text}");
}
