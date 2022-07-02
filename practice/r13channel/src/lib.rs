use std::sync::mpsc;
use std::thread;

// 多发送者，单接收者`
#[test]
fn it_works() {
    // std::sync::mpsc mpsc multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        for i in 3..10 {
            tx.send(i).unwrap();
        }
    });
    println!("receive {}", rx.recv().unwrap());
}
