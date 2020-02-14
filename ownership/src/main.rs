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
    // 浅拷贝
    let s2 = &mut s1;
    // 移动，之前的s1 无效了
    //println!("s1 = {}", s1);
    println!("s2 = {}", s2);

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
}

fn str_borrow_test1(str1: String) {
    println!("str : {}", str1);
}

fn i32_borow_test1(i: i32) {
    println!("i := {}", i);
}

// 编译时类型大小是固定的，就分配到栈上
// 类型大小不是固定的，分配到堆上