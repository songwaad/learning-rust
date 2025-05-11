fn greet() {
    println!("Hello, world!");
}

fn print_sum(a:u16, b:u16) {
    println!("Sum : {}, {} = {}" , a, b, a + b);
}

fn square(x:u16) -> u16 {
    x * x
}

fn add(a:u16, b:u16) -> u16 {
    a + b
}

fn main() {
    greet();
    print_sum(5, 10);

    let result:u16 = square(5);
    println!("Square of 5 : {}", result);

    let sum = add(3,  7);
    println!("Sum of 3 and 7 = {}", sum);
}
