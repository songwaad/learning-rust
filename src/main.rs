fn main() {
    let result: Result<f64, String> = divide(10.0, 0.0);

    let value: f64 = result.unwrap();
    println!("Value: {}", value);
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divivde by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}