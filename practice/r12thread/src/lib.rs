use std::thread;

#[test]
fn it_works() {
    let h = thread::spawn(||{
        for i in 1..10 {
            println!("current: {}", i);
        }
    });

    h.join().unwrap();
}




