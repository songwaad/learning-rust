fn main() {
    let tup:(i32, f64, char) = (50, 6.4, 'a');
    let (x, y, z) = tup;
    println!("Tuple : {}, {}, {}", x , y, tup.2);

    let arr:[i32; 3] = [1, 2, 3];
    println!("Array : {}", arr[1]);

}
