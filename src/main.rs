fn main() {
    let result = square_root(4);

    match result {
        Some(value) => println!("Square root is : {}", value),
        None => println!("No square root found."),
    }
}

fn square_root(number: i64) -> Option<i64> {
    if number >= 0 {
        Some(number.isqrt())
    } else {
        None
    }
}