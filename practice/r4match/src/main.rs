enum Message {
    Quit,
    Move {x: i32, y: i32},
    Writer(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [Message::Quit, Message::Move { x: 1, y: 2 }, Message::ChangeColor(1,2,3), Message::Writer("abceef".to_string())];

    let msg = Message::Move{x: 1, y: 2};

    if let Message::Move{x: a, y: b} = msg {
        println!("a: {}, b: {}", a, b);
    } else {
        panic!("不要让这行代码运行！");
    }
    // for msg in msgs.iter() {
    //     show_message(msg);
    // }

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y : b} => {
            println!("a: {}, b: {}", a, b);
        },
        Message::ChangeColor(a, b, c) => {
            println!("a: {}, b: {}, c: {}", a, b, c);
        },
        Message::Writer(s) => {
            println!("s: {}", s);
        },
        _ => ()
    }
}
