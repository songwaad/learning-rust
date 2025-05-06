fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");

    let greeting: &str = "Hello, Rust!";

    println!("{}",s);
    println!("{}", greeting);


}
