use std::ops::Deref;

fn main() {
    let y = MyBox::new(123);
    println!("v: {}", *y);
    println!("v: {}", *y);
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // 返回的是引用
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
