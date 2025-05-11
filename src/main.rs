fn main() {
    let number:u16 = 4;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("The number is sonething else"),
    }
}
