use std::fmt::Debug;

trait State: Debug {
    // self: Box<Self>。这个语法意味着这个方法调用只对这个类型的 Box 有效
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    // self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type.
    // This syntax takes ownership of Box<Self>,
    // * invalidating the old state so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[derive(Debug)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    /*
    We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value.
    Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned.
    If we didn’t call as_ref,
    we would get an error because we can’t move state out of the borrowed &self of the function parameter.
    At this point, when we call content on the &Box<dyn State>,
    deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait.
    */
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // 调用 take 方法将 state 字段中的 Some 值取出并留下一个 None，
    // 因为 Rust 不允许在结构体中存在空的字段。
    // 这使得我们将 state 值移动出 Post 而不是借用它。接着将博文的 state 值设置为这个操作的结果
    // 这里需要将 state 临时设置为 None,而不是直接赋值。 先取出再赋值，如果是None就不操作
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
