extern crate m_1_box;

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