fn main() {
    // 栈
    // 栈上的数据只有一份。 赋值操作相当于深拷贝.
    // 椎上的数据由栈上的指针指向椎上的内容，默认情况下赋值操作是浅拷贝 深拷贝用clone()
    // 基本的数据类型 bool i32 char 元组等实现了 copy trait 在赋值操作后还可以用
    let a: i32 = 123;
    let b = a;
    println!("a = {}", a);
    println!("b = {}", b);

    let mut s1 = String::from("hello");
    s1.push_str(" world");

    // 堆
    // move 浅拷贝
    // 如果你在其他语言中听说过术语 浅拷贝（shallow copy）和 深拷贝（deep copy），
    // 那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。
    // 不过因为 Rust 同时使第一个变量无效了，这个操作被称为 移动（move），而不是浅拷贝
    {
        let s2 = &mut s1;
        // 之前的s1 无效了
        //println!("s1 = {}", s1);
        println!("s2 = {}", s2);
        // 出了 } s2 就无效了，归还所有权
    }
    println!("s1 = {}", s1);
    let s2 = &mut s1;
    // clone
    let s3 = s2.clone();
    s2.push_str(" aaa");
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let iv1: i32 = 2;
    i32_borow_test1(iv1);
    println!("iv1 = {}", iv1);

    let s1 = String::from("hello");
    str_borrow_test1(s1);
    // s1 moved  value borrowed here after move
    //println!("s1 : {}", s1);


    let v1 = vec![1, 2, 3];
    vec_borrow_test1(&v1);
    println!("{:?}", v1);
}

fn str_borrow_test1(str1: String) {
    println!("str : {}", str1);
}

fn i32_borow_test1(i: i32) {
    println!("i := {}", i);
}

// 编译时类型大小是固定的，就分配到栈上
// 类型大小不是固定的，分配到堆上

// if have no & will be move
fn vec_borrow_test1(v: &Vec<i32>) {
    for &item in v.iter() {
        println!("{}", item)
    }
}