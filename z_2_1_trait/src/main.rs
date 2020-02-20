trait GetName {
    fn get_name(&self) -> &String;
}

trait PrintName {
    fn print_name(&self);
}

// 对任何实现了特定trait的类型有条件的实现trait
// 对实现了 GetName trait的类型 又实现 PrintName 相当于继承interface
impl<T: GetName> PrintName for T {
    fn print_name(&self) {
        println!("printname: {}", self.get_name());
    }
}

#[derive(Debug)]
struct Student {
    name: String,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}


fn main() {
    let s = Student{name: String::from("abc")};
    s.print_name();
    println!("Hello, world!");


    let vec: Vec<Box<dyn Trait>> = vec![Box::new(Foo), Box::new(Bar)];

    for e in &vec {
        e.trait_method();

        if let Some(r) = downcast::<Foo>(&**e) {
            r.foo_method();
        }
    }
}


use std::any::Any;

trait Trait {
    fn trait_method(&self);
    fn as_any(&self) -> &Any;
}

struct Foo;

impl Foo {
    fn foo_method(&self) {
        println!("In foo_method");
    }
}

impl Trait for Foo {
    fn trait_method(&self) {
        println!("In trait_method in Foo");
    }

    fn as_any(&self) -> &Any {
        self
    }
}

struct Bar;

impl Trait for Bar {
    fn trait_method(&self) {
        println!("In trait_method in Bar");
    }

    fn as_any(&self) -> &Any {
        self
    }
}

fn downcast<T: Trait + 'static>(this: &Trait) -> Option<&T> {
    this.as_any().downcast_ref()
}
