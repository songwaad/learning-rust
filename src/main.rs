fn main() {
    let mut s:String = "Hello".to_string();

    // let r1:&String = &s;
    // let r2:&String = &s;
    let r3:&String = &mut s;

    // println!("{} and {}", r1, r2);
    println!("{}", r3);
}