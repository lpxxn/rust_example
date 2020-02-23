use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec!["a2".to_string(), "b2".to_string(), "c2".to_string()];
        for v in vals {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec!["tx a".to_string(), "tx b".to_string(), "tx c".to_string()];
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        println!("receive: {}", r);
    }
    println!("Hello, world!");
}
