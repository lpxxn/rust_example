use std::thread;
use std::time::Duration;

fn main() {
    let handler = thread::spawn(|| {
        // 1-9
        for i in 1..10 {
            println!("number {} in spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //handler.join().unwrap();
    for i in 1..5 {
        println!("number {} in main !", i);
        thread::sleep(Duration::from_micros(3));
    }
    handler.join().unwrap();
    println!("Hello, world!");
}
