fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("{word}")
    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

// I'm really happy that this is something addressed in the Rust Book
// It means that me and whoever wrote the book think about code in a similar way
// This is the first thing I thought of checking
// It's also very refreshing to work with a language that addresses these things all the way
