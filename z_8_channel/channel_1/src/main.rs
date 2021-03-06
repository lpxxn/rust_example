use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 0..10 {
            let value = format!("hi {}", i);
            // 调用send的时候会发生send
            let rn = Arc::new(value);
            let s1 = rn.clone();
            tx.send(s1).unwrap();
            // 所以后面不能再使用value
            //println!("value {}", value);

            println!("value {}", rn);
            println!("value {}", rn);
        }
    });
    while let Ok(received) = rx.recv() {
        println!("received: {}", received);
    }
    //let received = rx.recv();

    println!("Hello, world!");
}

// mpsc::channel 创建通道， mpsc是多个生产者，单个消费者；
// spmc::channel 创建通道， spmc是一个生产者，多个消费者；
// 创建通道后返回的是发送者和消费者，示例：
// let (tx, rx) = mpsc::channel();
// let (tx, rx) = spmc::channel();
