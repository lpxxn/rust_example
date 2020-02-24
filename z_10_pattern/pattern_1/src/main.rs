fn main() {
    let v = vec!['a', 'b', 'c'];
    for (idx, v) in v.iter().enumerate() {
        println!("idx: {}, v {}", idx, v);
    }
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);
    let (x, .., z) = (1, 2, 3);
    println!("x: {}, z: {}", x, z);
    let x = (1, 2);
    // 1和2 匹配模式x,y
    print_point(&x);
    println!("x: {:?}", x);
    let x = Some(5);
    match x {
        Some(5) => println!("5"),
        Some(y) => println!("y {}", y),
        _ => println!("others"),
    }
    let x = 1;
    match x {
        1|2 => println!("1 or 2"),
        // 这要加=号
        3..=10 => println!("3..10"),// 3, 4, 5, 6, 7...10
        _ => println!("others"),
    }
}

fn print_point(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}
// 字面值 let x =1; let x="abcdef";
// 模式有两种：refutable(可反驳的)和irrefutable(不可反驳的)。能匹配任何传递的可能值的模式称
// 为不可反驳。对值进行匹配可能失败的模式移库可反驳的。
// 只能接受不可反驳模式的有：函数、let语句、for循环。原因：因为通过不匹配的值程序无法进行工作。
// let Some(x) = a; 这种是不对的，没有匹配None  let语句只能接受不可反驳模式
// 3. if let 和while let 表达式被限制为只能接受可反驳的模式，因为他们定义主是为了处理可能失败的情况。