// cargo test defref_test1 -- --nocapture
#[test]
fn defref_test1() {
    let mut x: i32 = 5;
    let y: &i32 = &x;
    assert_eq!(x, 5);
    assert_eq!(5, *y); // 解引用

    let z = Box::new(x);
    x += 5;
    assert_eq!(5, *z);
    println!("x {}", x);
    println!("z {}", z);
    println!("*z {}", *z);
}

// 解引用
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // 并不获取T的所有权，所以只返回引用
    fn deref(&self) -> &T {
        &self.0
    }
}

// cargo test defref_test2 -- --nocapture
#[test]
fn defref_test2() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("*y {}", *y);

    let m = MyBox::new(String::from("abcdef"));
    // 解引用强制多态
    hello(&m); // 调用MyBox解引用 变为 &String,再调用String的解引用，变成字符串slicd &str
    /* 
         解引用多态与可变性交互
         1. 当T: Deref<Target=U>时，从&T到&U
         2. 当T: DerefMut<Target=U>时，从&mut T 到&mut U
         3. 当T: Deref<Target=U>时，从&mut T到 &U
         不能从不可变T到可变U
    */

}

fn hello(name: &str) {
    println!("Hello {}", name);
}