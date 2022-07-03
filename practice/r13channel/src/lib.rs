use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

#[test]
fn test_try_recv() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });
    println!("receive {:?}", rx.try_recv());
}

// 传输具有所有权的数据
#[test]
fn test_sd_ownship() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("hello");
        tx.send(s).unwrap();
        //println!("val is {}", s);
        /*
        33 |         let s = String::from("hello");
   |             - move occurs because `s` has type `String`, which does not implement the `Copy` trait
34 |         tx.send(s).unwrap();
   |                 - value moved here
35 |         println!("val is {}", s);
   |                               ^ value borrowed here after move
         */
    });
    let received = rx.recv().unwrap();
    println!("got: {}", received);
}

// 使用for 循环接收
#[test]
fn test_for() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
    });
    for received in rx {
        println!("got : {}", received);
    }
}

// mpsc是multiple producer, single consumer
// 使用多发送者
#[test]
fn test_multiple_sender() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send("hello".to_string()).unwrap();
    });
    thread::spawn(move || {
        tx1.send("world".to_string()).unwrap();
    });
    for received in rx {
        println!("got: {}", received);
    }
}

// 上面使用的都是异步通道，无论接收者是不正在接收消息，消息发送者在发送消息时都不会阻塞
// 下面讲一下同步通道，消息是阻塞的，只有在消息被接收后才会解除阻塞

#[test]
fn test_sync_fun1() {
    // 我们传递了一个参数0: mpsc::sync_channel(0),这和go一样是个buffer的channel
    let (tx, rx) = mpsc::sync_channel(0);
    let handle = thread::spawn(move || {
        println!("before send");
        tx.send("hello").unwrap();
        println!("after send");
    });

    println!("before sleep");
    thread::sleep(Duration::from_secs(2));
    println!("after sleep");

    println!("received {}", rx.recv().unwrap());
    handle.join().unwrap();
}
/*
先将0改成1
println!("首次发送之前");
tx.send(1).unwrap();
println!("首次发送之后");
tx.send(1).unwrap();
println!("再次发送之后");
一切的关键就在于1上，该值可以用来指定同步通道的消息缓存条数，当你设定为N时，发送者就可以无阻塞的往通道中发送N条消息，当消息缓冲队列满了后，新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的消息，那么第N+1条消息就将触发发送阻塞)。
 */