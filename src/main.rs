fn main() {
    let msg1:Message = Message::Quit;
    let msg2:Message = Message::Move { x: 10, y: 20 };
    let msg3:Message = Message::Write("Hello".to_string());
    let msg4:Message = Message::ChangeColor(255, 255, 0);

    print_message(msg4);
}

enum Message{
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print_message(msg:Message) {
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to coordinates: {}, {}", x, y),
        Message::Write(text) => println!("Write message : {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB : {}, {}, {}", r, g, b),
    }
}