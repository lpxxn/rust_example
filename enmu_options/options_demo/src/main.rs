fn main() {
    println!("Hello, world!");

    match options() {
        Some(a) => {
            println!("a: {}", a);
        }
        None => {
            println!("match none")
        }
    }
}

fn options() -> Option<i32> {
    //let y: Option<i32> = Some(1);
    let y: Option<i32> = None;
    let y1 = y?;
    // 如果是None 就直接返回了，不会执行再下面的代码了
    println!("options y1 = {}", y1);
    y
}