pub trait Summary {
    fn summary(&self) -> String;
}
pub struct NewsArticle {
    pub head_line: String,
    pub content: String
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