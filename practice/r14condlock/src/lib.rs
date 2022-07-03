use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn it_works() {
    // 通过 Rc实现Mutex的多所有权
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

    println!("Result: {}", *counter.lock().unwrap());
}
/*
需要小心使用的 Mutex
如果有其它语言的编程经验，就知道互斥锁这家伙不好对付，想要正确使用，你得牢记在心：

在使用数据前必须先获取锁
在数据使用完成后，必须及时的释放锁

Atomic和Mutex一样，Atomic的值具有内部可变性，你无需将其声明为mut
 */