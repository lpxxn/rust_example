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
    // data type
    // bool 
    let verify: bool = true;
    println!("verify: {}", verify);

    // char rust里 char是32位的也就是说一个car可以是一个汉字，c++里是8位
    let a = 'a';
    println!("a = {}", a);
    let b: char = '中';
    println!("b = {}", b);
   
    
    // i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
    let i8v: i8 = -123;
    println!("i8v = {}", i8v);

    let f32v: f32 = 0.00013;
    println!("f32v = {}", f32v);

    // 自适应 isize, usize
    println!("max = {}", usize::max_value());
    
    // 数组 [type; size]
    let arr: [u32; 5] = [1, 2 ,3 ,4, 5];
    println!("arr = {:?}", arr);
    let arr1: [u32; 3] = [1, 2, 3];
    show(arr1);
    println!("end array");

    // 元组
    let tup1: (i32, f64, char) = (123, 0.23, '元');
    println!("t1= {}, t2= {}, t3= {}", tup1.0, tup1.1, tup1.2);
    let (x, y, z) = tup1;
    println!("x= {}, y= {}, z= {}", x, y, z);
    
    println!("add = {}", add(1, 2));

    // 表达式 可以有返回值
    let y = {
        let x = 1;
        // 也是没有;
        x + 2
    };
    println!("y = {}", y);

    //控制流
    let control = true;
    let x = if control {
        5
    } else {
        8
    };
    println!("x = {}", x);

    // loop
    let mut  control = 0;
    loop {
       println!("in loop");
       // rust 没有 i++ 因为认为不安全 https://prev.rust-lang.org/en-US/faq.html
       control +=  1;
       if control == 10 {
          break;
       }
    }

    control = 0;

    let x = loop {
       println!("in loop");
       control +=  1;
       if control == 10 {
          break control * 5;
       }
    };
    println!("x = {}", x);

    control = 0;
    while control != 10 {
        control += 1
    }
    println!("control = {}", control);
    let arr: [u32; 5] = [1, 2 ,3 ,4, 5];   
    // 可以这样
    // for ele in arr.iter() {
    // 也可以这样
    for ele in &arr {
        println!("element = {}", ele);
    }
}

fn add(a: i32, b: i32) -> i32 {
   // 可以这么写 注意最后没有;
   // let result = a + b;
   // result
   // 也可以这么写
   a + b
}

/// 测试文档
/// cargo doc
/// cargo doc --open
fn show(arr: [u32; 3]) {
    for i in &arr {
        println!("{}", i)
    }
}

/// Add one to the number given
/// 
/// #Example
/// 
/// ```
/// let five = 5
/// assert_eq!(6, add_one(five))
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}