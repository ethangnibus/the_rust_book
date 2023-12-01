enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor,
}

impl Message {
    fn call(&self) {
        // body
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}