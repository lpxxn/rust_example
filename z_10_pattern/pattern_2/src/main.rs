// 解构并分解值
// 解构元祖、结构体、枚举、引用
// 
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point{x: 1, y: 2};
    // a = x, b = y
    let Point{x: a, y: b} = p;
    assert_eq!(1, a);
    assert_eq!(2, b);

    let Point{x, y} = p;
    assert_eq!(1, x);
    assert_eq!(2, y); 

    let p = Point{x: 1, y: 0};
    match p {
        Point{x, y: 0} => println!("在x轴上"),
        Point{x: 0, y} => println!("在y轴上"),
        Point{x, y} => println!("others"),
    }
    let s = Some(String::from("abc"));
    let mut str1: String = String::new();
    if let Some(v) = s {
        println!("v: {}", v);
        str1 = v;
        println!("v: {}", str1);
    }
    println!("str {}", str1);
    println!("Hello, world!");
}
