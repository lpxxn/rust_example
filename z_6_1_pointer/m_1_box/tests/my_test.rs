extern crate m_1_box;
// 在代码顶部有一个 extern crate adder。
// 这是因为在测试目录里测试是一个完全独立的箱，所以我们需要导入我们的函数库。
// 这也是为什么 tests 是一个编写集成风格测试的合适的地方：他们使用函数库和其他消费者。
// importing common mod
mod common;

#[test]
fn myfirst_test() {
    println!("abcvdfe");

    m_1_box::my_box::new_box();

    common::setup();
    // https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html
    // cargo test myfirst_test -- --nocapture
}
