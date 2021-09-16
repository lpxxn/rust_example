use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target =T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


struct A;
struct B;
impl Deref for A {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        let x = &B;
        return x
    }
}

fn foo(a: A) -> Option<()> {
    let v = &&a;
    bar(v);
    bar(&&a);
    bar((&a).deref());
    Some(())
}

fn bar(b: &B) {

}