fn main() {
    let a:u16 = 2;
    let b:u16 = 3;

    let result_and:u16 = a & b;
    let result_or:u16 = a | b;
    let result_xor:u16 = a ^ b;

    println!("{}", result_and);
    println!("{}", result_or);
    println!("{}", result_xor);


}
