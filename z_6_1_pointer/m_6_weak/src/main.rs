use std::rc::Rc;
fn main() {
    let v = Rc::new(3);
    let weak_v = Rc::downgrade(&v);
    let strong_v: Option<Rc<_>> = weak_v.upgrade();
    println!("v {:?}", strong_v);
    drop(strong_v);
    drop(v);

    let strong_v: Option<Rc<_>> = weak_v.upgrade();
    println!("v {:?}", strong_v);
}

struct A(i32);