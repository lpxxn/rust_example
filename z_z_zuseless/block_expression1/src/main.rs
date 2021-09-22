use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, r) => Some(r),
            Nil => None,
        }
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::downgrade(&Rc::new(Nil)))));
    let b = Rc::new(Cons(2, RefCell::new(Rc::downgrade(&a))));
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }

    let mut d: Option<Rc<List>> = None;
    if let Some(l) = a.tail() {
        d = l.borrow().upgrade();
    }

    for _ in 1..=1 {
        d = {
            let mut v: Option<Rc<List>> = None;
            // d has no &
            if let Some(x) = d {
                if let Some(l) = x.tail() {
                    v = l.borrow().upgrade();
                }
            }
            v
        };
        println!("---{:?}", d);
    }
    for _ in 1..=1 {
        // d has &
        // if removed & will have error
        if let Some(x) = &d {
            if let Some(l) = x.tail() {
                let v = l.borrow().upgrade();
                d = v
            }
        }
        println!("---{:?}", d);
    }

    let a = String::from("hey");
    let b = a;
    println!("{}", &a);
}
