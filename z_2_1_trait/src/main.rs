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
}
