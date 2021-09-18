enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(1,  Rc::new(Cons(5, Rc::new(Nil)))));
    println!("a reference {}", Rc::strong_count(&a));
    {
        let b = Cons(2, Rc::clone(&a));
        println!("a reference {}", Rc::strong_count(&a));
        let c = Cons(3, a.clone());
        println!("a reference {}", Rc::strong_count(&a));
    }
    println!("a reference {}", Rc::strong_count(&a));
}
