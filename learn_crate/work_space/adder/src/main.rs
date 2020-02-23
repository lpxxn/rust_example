use add_one;
use add_two;
use add_two::tfns::AverCollect;
fn main() {
    println!("add_one {}", add_one::add_one_fn::add_one(1));
    println!("add_two {}", add_two::add_two(1));
    let mut v = AverCollect::new();
    println!("remove value {:?}", v.remove());
    println!("average value {:?}", v.average());
    v.add(1);
    println!("average value {:?}", v.average());
    println!("remove value {:?}", v.remove());
    println!("Hello, world!");
}
