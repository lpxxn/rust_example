pub mod blog;

#[cfg(test)]
mod tests {
    use super::blog::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();
        let str1 = "I ate a salad for lunch today";
        post.add_text(str1);
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(str1, post.content());
    }
}
