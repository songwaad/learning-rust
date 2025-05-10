fn main() {
    let mut v = Vec::new();
    v.push('a');
    v.push('b');

    for i in &v {
        print!("{}", i);
    }

    println!();
}
