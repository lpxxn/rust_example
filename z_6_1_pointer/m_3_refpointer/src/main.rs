enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons(i32, Rc<List2>),
    Nil
}
//use self::List::{Cons, Nil};
use crate::List::{Cons, Nil};
use std::rc::Rc;


fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // value used here after move
    // let c = Cons(4, Box::new(a));

    let r1 = Rc::new(List2::Cons(1, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    let r2 = List2::Cons(2, Rc::clone(&r1));
    let r3 = List2::Cons(3, r1.clone());
    let r4 = List2::Cons(4, r1.clone());

    println!("r1 {:?}", r1);
    println!("r1 {:?}", &r1);
    println!("r1 {:?}", *r1);
    println!("r2 {:?}", r2);
    typeid(&r1);

    println!("{}", type_of(&r1));
    print_type_of(&(*r1));
    //let _: () = r1;    // found struct `std::rc::Rc`
    //let _: () = &r1;   // found reference
    //let _: () = *r1;   // found enum `List2`
}

fn typeid<T: std::any::Any>(_: &T) {
    println!("{:?}", std::any::TypeId::of::<T>());
}

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}