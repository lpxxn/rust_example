use std::sync::{Arc, Mutex};
use std::thread;
// RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T>
// Mutex<T> 提供内部可变性，类似于RefCell
// RefCell<T>/Rc<T>是非线程安全的，Mutex<T>/Arc<T>是线程安全的
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter: {}", *counter.lock().unwrap());
}
