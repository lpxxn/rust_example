// trait 抽象方式定义共享行为 类似于其他语言的interface

pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> i32;
}

#[derive(Debug)]
pub struct Student {
    pub name : String,
    pub age: i32,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

#[derive(Debug)]
pub struct Teacher {
    pub name : String,
    pub age: i32,
}

impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

fn print_information(item: impl GetInformation) {
    println!("items name: {}, age: {}", item.get_name(), item.get_age());
 
}


fn main() {
    let s = Student{name: "xiao".to_string(), age: 10};
    println!("s name: {}, age: {}", s.get_name(), s.get_age());
    print_information(s);
    let t = Teacher{name: "da".to_string(), age: 20};
    print_information(t);

    println!("Hello, world!");
}
/*
https://stackoverflow.com/questions/29781331/why-cant-i-return-an-str-value-generated-from-a-string
fn description(&self) -> &str
// Can be rewritten as
fn description<'a>(&'a self) -> &'a str

*/