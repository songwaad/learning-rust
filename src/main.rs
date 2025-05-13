fn main() {
    let result: i32 = division(5,0);
    println!("result : {}", result);
}

fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}