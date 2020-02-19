// trait 抽象方式定义共享行为 类似于其他语言的interface

pub trait GetInformation{
    fn get_name() -> &String;
    fn get_age() -> u32;
}

#[derive(Debug)]
pub struct Student {
    pub name : String,
    age: i32,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        &self.age
    }
}

fn main() {
    let s = Student{name: "xiao", age: 10};
    println!("s name: {}, age: {}", s.get_name(), s.get_age()());
    println!("Hello, world!");
}
