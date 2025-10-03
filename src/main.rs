fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let s1 = "Hi";
    let s2 = s1;

    println!("s1 = {s1}, s2 = {s2}");
}
