fn main() {
    let black:Color = Color(0,0,0);
    print!("Black color RGB : {}, {}, {}", black.0, black.1, black.2);
}

struct Color(i32, i32, i32);