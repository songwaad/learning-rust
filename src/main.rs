fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[0..3];

    println!("slice : {}",slice[2]);
}
