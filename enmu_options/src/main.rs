fn main() {
    // 基本用法
    #[derive(Debug)]
    enum IpAddKind {
        V4, 
        V6,
    }
    #[derive(Debug)]
    struct IpAddr {
        Kind: IpAddKind,
        address: String,
    }

    let i1 = IpAddr {
        Kind: IpAddKind::V4,
        address: String::from("127.0.0.1")
    };

    println!("i1 = {:#?}", i1);

    // rust 方式
    #[derive(Debug)]
    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    let i1 = IpAddr2::V4(String::from("192.168.0.1"));
    println!("i1 = {:#?}", i1);


    let i1 = Message::Quit;
    i1.print();
    let i1 = Message::Move{x: 1, y: 2};
    println!("i1 = {:?}", i1);
    i1.print();
    let i1 = Message::Write(String::from("abcdef"));
    println!("i1 = {:?}", i1);
    i1.print();
    let i1 = Message::Change(1, 2, 3);
    i1.print();
}

// classic
#[derive(Debug)]
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32),
}
// 等同于
/*
struct QuitMessage;// 类单元结构体
struct MoveMessage {
    x: i32, 
    y: i32,
}
struct WriteMessage(String)
struct Change(i32, i32, i32)
*/

impl Message {
    fn print(&self) {
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("Move x = {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            Message::Write(ref s) => println!("s = {}", s),
        }
    }
}