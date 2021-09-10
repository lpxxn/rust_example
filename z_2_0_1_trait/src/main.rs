use z_2_0_1_trait::{Tweet, Summary};
// 如果没有 Summary 会报错
// method not found in `Tweet` trait 必须在作用域内才能使用他的方法

fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("zhang san"),
        content: String::from("content")
    };
    println!("summary: {} \n about: {}", tweet.summary(), tweet.about())
}
