use std::ops::Deref;
fn main() {
    let bar = Bar { f: Foo {} };
    bar.do_something();
    println!("Hello, world!");
}

struct Foo {}
impl Foo {
    fn do_something(&self) {
        println!("Foo: do something...");
    }
}

struct Bar {
    f: Foo,
}

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Foo {
        &self.f
    }
}
