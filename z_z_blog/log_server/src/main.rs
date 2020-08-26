use blog::blog::*;
fn main() {
    let mut post = Post::new();
    let str1 = "I ate a salad for lunch today";
    post.add_text(str1);
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    let approve = post.approve();
    let approve2 = post.approve();
    assert_eq!(str1, post.content());

    println!("post: {}", post.content());
}
