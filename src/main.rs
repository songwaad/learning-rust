fn main() {
    let number:u16 = 2;

    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "The number is sonething else",
    };

    println!("The reesult is : {}", result);
}
