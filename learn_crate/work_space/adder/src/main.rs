use add_one;
use add_two;
fn main() {
    println!("add_one {}", add_one::add_one_fn::add_one(1));
    println!("add_two {}", add_two::add_two(1));
    println!("Hello, world!");
}
