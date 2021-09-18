use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum  List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}
use crate::List::{Cons, Nil};

#[cfg(test)]
mod tests {
    use crate::List;
    use crate::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::borrow::Borrow;

    #[test]
    fn test_mute_ref() {
        let value = Rc::new(RefCell::new(1));
        let a = Cons(Rc::clone(&value), Rc::new(Nil));

        *value.borrow_mut() += 10;
        match &a {
            Cons(a, b) => {
                let b:&RefCell<i32> = a.borrow();
                println!("value: {}", b.borrow())
            }
            Nil => {

            }
        }
        *value.borrow_mut() += 10;

        println!("a: {:?}", a);
    }

    #[test]
    fn test_take() {
        let a = RefCell::new(5);
        let f = a.take();
        println!("v: {}", f);
        // will be default value
        println!("v: {}", a.take());
    }
}