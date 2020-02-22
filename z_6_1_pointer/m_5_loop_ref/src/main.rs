#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
use std::rc::Rc;
use std::cell::RefCell;
use self::List::{Cons, Nil};

fn main() {
    let a: Rc<List> = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));
    println!("1, a rc count {}", Rc::strong_count(&a));
    println!("1, a tail= {:?}", a.tail());
    {
        // b 引用a
        let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));
        println!("2, a rc count {}", Rc::strong_count(&a));
        println!("2, b rc count {}", Rc::strong_count(&b));
        println!("2, b tail= {:?}", b.tail()); 
        
        if let Some(link) = a.tail() {
            // a 引用 b 造成循环引用
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("3, a rc count {}", Rc::strong_count(&a));
        println!("3, b rc count {}", Rc::strong_count(&b));
    }
    println!("1, a rc count {}", Rc::strong_count(&a));
    // 再调用 a.tail();
    // 调用 tail() 造成无限死循环
    // a的Cons 引用的 b， b的Cons引用的 a
    // 输出时 无限输出 a、b、a、b、a、b。。。。。
    //println!("1, a tail= {:?}", a.tail());

}

/*
  a  --> [1|ref]
  ↑        |
  |        ↓
[2|ref] <--[b]
*/
