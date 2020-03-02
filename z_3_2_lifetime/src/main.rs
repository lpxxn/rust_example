// 静态生命周期 'static
// 生命周期存活于整个程序期间，所有字符字面值都拥有static生命周期。
// let s: &'static str = "hello";

use std::fmt::Display;
fn do_something<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("ann is {}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let ann = 123;
    let r = do_something(s1.as_str(), s2.as_str(), ann);
    println!("r {}", r);
    

    println!("Hello, world!");
}

struct Foo {
    v: Vec<u8>
}

impl Foo {
    fn bar1(&self) -> impl Iterator<Item=u8> + '_ {
        self.v.iter().cloned()
    }

    fn bar2(&self) -> std::iter::Cloned<std::slice::Iter<u8>> {
        self.v.iter().cloned()
    }

    fn bar3<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
        self.v.iter().cloned()
    }
}
