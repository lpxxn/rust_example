use std::thread;

fn main() {
    let mut v = vec![1, 2, 3];
    let handler = thread::spawn(move || {
        v.push(5);
        println!("v: {:?}", v);
    });
    // println!("in main v: {:?}", v);
    handler.join().unwrap();
    println!("Hello, world!");
}

// fn main() {
//     let v = vec![1, 2, 3];
//     let handler = thread::spawn(|| {
//         println!("v: {:?}", v);
//     });
//     // drop(v);
//     handler.join().unwrap();
//     println!("Hello, world!");
// }
