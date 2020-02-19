// trait 抽象方式定义共享行为 类似于其他语言的interface

pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> i32;
}

trait School {
    fn get_school_name(&self) -> String {
        //String::from("小星星学校")
        "小星星学校".to_string()
    }
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

impl School for Student {
    fn get_school_name(&self) -> String {
        "abcde school".to_string()
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
impl School for Teacher {
    
}

fn print_information(item: &impl GetInformation) {
    println!("items name: {}, age: {}", item.get_name(), item.get_age());
}

fn print_information2<T: GetInformation>(item: &T) {
    println!("T name: {}, age: {}", item.get_name(), item.get_age());
}

fn print_information3<T: GetInformation + School>(item: &T) {
    println!("T3fn school: {}, name: {}, age: {}", item.get_school_name(), item.get_name(), item.get_age());
}

fn print_school_information(item: impl School) {
    println!("item school name: {}", item.get_school_name());
}


fn main() {
    let s = Student{name: "xiao".to_string(), age: 10};
    println!("s name: {}, age: {}", s.get_name(), s.get_age());
    print_information(&s);
    println!("school {}", s.get_school_name());
    print_information3(&s);
    print_school_information(s);
    let t = Teacher{name: "da".to_string(), age: 20};
    print_information2(&t);
    print_school_information(t);


    println!("Hello, world!");
}
/*
https://stackoverflow.com/questions/29781331/why-cant-i-return-an-str-value-generated-from-a-string
fn description(&self) -> &str
// Can be rewritten as
fn description<'a>(&'a self) -> &'a str

*/