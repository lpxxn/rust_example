extern crate core;

use std::sync::{Arc, Condvar, Mutex, MutexGuard, Once};
use std::thread;

#[test]
fn it_works() {
    let h = thread::spawn(|| {
        for i in 1..10 {
            println!("current: {}", i);
        }
    });

    h.join().unwrap();
}


// 条件变量 condition variables 经常和Mutext一起使用，可以让线程挂起，直到某个条件发生后再继续执行

#[test]
fn test_condvar1() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));// 注意参数就是个（xxx）
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    while !*started {
        println!("main wait begin");
        started = cvar.wait(started).unwrap();
        println!("main wait end");
    }
    println!("started changed");
    /*
    main 线程首先进入 while 循环，调用 wait 方法挂起等待子线程的通知，并释放了锁 started
    子线程获取到锁，并将其修改为 true，然后调用条件变量的 notify_one 方法来通知主线程继续执行
     */
}


// once
static INIT: Once = Once::new();
static mut VAL: usize = 0;
#[test]
fn test_once() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| {
            println!("handle 1");
            unsafe { VAL = 1; }
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| {
            println!("handle 2");
            unsafe {VAL = 2;}
        });
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{}", unsafe {VAL});
}