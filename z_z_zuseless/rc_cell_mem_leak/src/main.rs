use std::cell::{RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::borrow::Borrow;

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        //println!("tail fn: {:?}", self);
        match self {
            Cons(_, r) => {
                Some(r)
            }
            Nil => {
                None
            }
        }
    }
}

fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::downgrade(&Rc::new(Nil)))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(2, RefCell::new(Rc::downgrade(&a))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // leak
    println!("a next item = {:?}", a.tail());
    println!("b next item = {:?}", b.tail());
    let mut t: Rc<List> = Rc::new(Nil);
    if let Some(l) = a.tail() {
        let r: Rc<List> = l.borrow().upgrade().unwrap();
        //println!("---{:?}", r);
        t = r;
    }
    println!("---------------");
    for _ in 1..=5 {
        println!("---{:?}", t);
        if let Some(l) = t.tail() {
            let r = l.borrow().upgrade().unwrap();
            t = r;
        }
    }

    let mut d: Option<Rc<List>> = None;
    if let Some(l) = a.tail() {
        d = l.borrow().upgrade();
    }
    println!("---------------");
    for _ in 1..=5 {
        println!("---{:?}", d);
        if let Some(x) = &d {
            if let Some(l) = x.tail() {
                let v = l.borrow().upgrade();
                // 直接这样就报错
                //d = l.borrow().upgrade();
                d = v
            }
        }
        //  match t {
        //      Some(x) => {
        //          if let Some(z) = x.tail() {
        //              t = z.borrow().upgrade();
        //          }
        //      }
        //      _ =>{}
        // }
    }
}
