
#[derive(Debug)]
pub struct A {
    pub name: String,
    age: i32,
}

impl A {
    pub fn new_a(name: String, age: i32) -> A {
        A {
            name,
            age,
        }
    }
    // note: &
    pub fn age(&self) -> i32 {
            self.age
    }
    pub fn print_a(&self) {
        println!("a: {:#?}", self);
    }
}