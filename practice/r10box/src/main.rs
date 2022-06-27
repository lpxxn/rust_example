use std::ops::Deref;

fn main() {
    let y = MyBox::new(123);
    println!("v: {}", *y);
    println!("v: {}", *y);
    // 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)
    displayMyBox(&y);

    let s = String::from("hello world");
    display(&s);
}
fn displayMyBox(b: &i32) {
    println!("{}", b);
}

fn display(s: &str) {
    println!("{}", s);
}
/*
String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
&s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)
 */

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
