#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => {
                if let Some(v) = item.borrow().upgrade() {
                    println!("v {:?}", *v);
                }
                if item.borrow().upgrade().is_none() {
                    println!("item is none")
                } else {
                    println!("not none")
                }
                Some(item)
            },
            Nil=> None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use self::List::{Cons, Nil};
use std::borrow::Borrow;

fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Weak::new())));
    println!("1, a strong coutn = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());

    {
        let b = Rc::new(Cons(2, RefCell::new(Weak::new())));
        println!("2, b tail = {:?}", b.tail());
        // b 指向 a
        if let Some(link) = b.tail() {
            *link.borrow_mut() = Rc::downgrade(&a);
        }
        println!("2, b tail = {:?}", b.tail());
        println!("2, a strong coutn = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
        println!("2, b strong coutn = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::downgrade(&b);
        }
        println!("2, a strong coutn = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
        println!("2, b strong coutn = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
        println!("2, b tail = {:?}", b.tail());
    }
    println!("1, a strong coutn = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());
}
// 弱引用 Week<T>
// 特点：
// 1. 弱引用通过Rc::downgrade 传递实例的引用，调用Rc::downgrade会得到Week<T>实例的智能指针，
//    同时将weak_count加1（而不是将 strong_count加1）。
// 2. 区别在于 weak_count 无需计数为 0 就能使 Rc 实例被清理。只需strong_count为0就可以。
// 3. 可以通过Rc::upgrade 方法为返回Option<Rc<T>>对象。