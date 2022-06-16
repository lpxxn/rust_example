use std::{arch::x86_64::_mm_testz_ps, f32::consts::PI};

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
    // 判断 保有x为1时才可以
    if let Message::Move{x: 1, y: b} = msg {
        println!("b: {}", b);
    } else {
        panic!("不要让这行代码运行！");
    }
    // for msg in msgs.iter() {
    //     show_message(msg);
    // }

    for msg in msgs {
        show_message(msg);
    }
    while_let();
    deconstruction();
    deconstruction2();
    match_gurard();
    match_at_binding();
    match_binding_deconstruction();
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

fn while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    let x = 5;
    match x {
        1..=5 => println!("some"),
        _ => println!("else")
    }

    let x = 'c';
    match x  {
        'a'..='j' => println!("abc"),
        'k'..='z' => println!("lll"),
        _ => println!("else")
    }
}

fn deconstruction() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point{x: 1, y: 2};
    let Point { x: a, y: b } = p;
    println!("a: {}, b: {}", a, b);

    // 同名
    let Point { x , y  } = p;
    println!("a: {}, b: {}", x, y);

    match p { 
        Point {x, y: 0} => println!("x {}", x),
        Point { x:0, y } => println!("y {}", &y),
        Point { x, y } => println!("x: {}, y: {}", x, y)
    }

    let numbers = (1, 23, 43);
    match numbers {
        (a, b, _) => {
            println!("a: {}, b: {}", a, b)
        }
    }

    match numbers {
        (a, b, ..) => {
            println!("a: {}, b: {}", a, b)
        }
    }
}

fn deconstruction2() {
    // 用 .. 忽略剩下的值
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

fn match_gurard() {
    // match 里有if条件
    let num = Some(1);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("x: {}", x),
        None => ()
    }
}

// @ 运算符允许为一个字段绑定另一个变量。
// 下面的例子 Message::hello的id字段是否位于 3..=7范围内，同时也希望能将其值绑定于id_variable
fn match_at_binding() {
    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("found an id in range {}", id_variable);
        },
        Message::Hello { id: 10..=12 } => {
            println!("found ann id in another range");
            // println!("found ann id in another range: {}", id); // error
        },
        Message::Hello { id } => {
            println!("hello id: {}", &id)
        }
    }
}

// @前绑定后解构(Rust 1.56 新增)
// 使用 @ 还可以在绑定新变量的同时，对目标进行解构：
fn match_binding_deconstruction() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 绑定新变量 p 同时对 Point 进行解构
    let p @ Point { x: px , y: py } = Point { x: 1, y: 2 };
    println!("x: {}, y: {}", px, py);

    let point = Point{x: 10, y: 5};
    // By default, struct and enum types are not Copy:
    if let p @ Point { x: 10, y } = &point {
        println!("x is 10 and y is {} in {:?}", y, p);
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num)
        }
        _ => {}
    }

    match point {
        Point { x , y: 0 } => println!("on the x axis at {}", x),
        Point { x:0..=10, y: y@ (5 | 10| 11) } => println!("on the y axis at {}", y),
        Point { x, y } => println!("x: {}, y: {} ", x, y),
    }
}