// 1. 内部可变性：允许在使用不可变引用时改变数据。
// 2. 通过RefCell<T>在运行时检查借用规则（通常情况下，是在编译时检查借用规则），RefCell<T>代表其数据的
//    唯一所有权。
// 3. 类似于Rc<T>, RefCell<T>只能用于单线程场景。
// Rc<T> 允许一个数据有多个所有者；Box<T>和RefCell<T> 有单一所有者
// Box<T> 允许在编译时执行不可变或者可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；
// RefCell<T> 允许在运行时执行不可变或可变检查。
// 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便RefCell<T>自身是不可变的情况下改变
// 其内部的值
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    println!("a {:?}", a);
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    *value.borrow_mut() += 5;
    // let i: () = *value; // found struct `std::cell::RefCell`
    println!("a {:?}", a);
    println!("b {:?}", b);
    println!("c {:?}", c);

    println!("value {:?}", *value);
    println!("Hello, world!");
}
