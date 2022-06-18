use std::fmt::{Debug, Display};
mod d1;
fn main() {
    println!("Hello, world!");
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {}

// 多重约束
pub fn notify3(item: &(impl Summary + Display)){}
pub fn notify4<T: Summary + Display>(item: &T) {}

// where 约束
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

fn some_function2<T, U>(t: &T, u: &U)
    where T: Display + Clone,
          U: Display + Clone {
}



