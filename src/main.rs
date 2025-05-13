fn main() {
    let result = divide(10.0, 0.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divivde by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}