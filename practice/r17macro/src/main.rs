use hello_macro_derive::HelloMacro;
use r17macro::HelloMacro;

#[derive(HelloMacro)]
struct Test;

fn main() {
    println!("Hello, world!");
    Test::hello_macro();
}
