const ABC: u32 = 123445;
fn main() {
    let a = 1;
    println!("a = {}", a);
    
    let mut b: u32 = 1;
    println!("b = {}", b);
    b = 3;
    println!("b = {}", b);
    println!("const value = {}", ABC);
    println!("Hello, world!");
}
