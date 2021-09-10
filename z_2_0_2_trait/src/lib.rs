use std::fmt::{Display, Formatter};

pub trait Summary {
    fn summary(&self) -> String;
}
pub struct NewsArticle {
    pub head_line: String,
    pub content: String
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("display content");
        std::fmt::Result::Ok(())
    }
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("article head line: {}, content: {}", self.head_line, self.content)
    }
}



pub struct Tweet {
    pub username: String,
    pub content: String
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("tweet user: {} content: {}", self.username, self.content)
    }
}

pub fn notify(s: impl Summary) {
    println!("news: {}", s.summary());
}

pub fn notify2<T: Summary>(s: T) {
    println!("news: {}", s.summary());
}
// 使用 + 指定多个trait
pub fn notify3(s: impl Summary + Display) {
    println!("news: {}", s.summary());
}

pub fn notify4<T: Summary + Display>(s: T) {
    println!("news: {}", s.summary());
}
// 或者使用 where 指定多个trait

pub fn notify5<T>(s: T) where T: Summary + Display {
    println!("news: {}", s.summary());
}