fn main() {
    let mut str1 = String::from("hello");
    let l = ref_test1(&str1);
    println!("str1 = {}", str1);
    println!("len = {}", l);
    str_add(&mut str1);
    println!("str1 = {}", str1);


    // 只能有一个可写，可以有多个可读。
    let mut str2 = String::from("yes ");
    let r2 = &str2;
    let r3 = &str2;
    println!("r2: {}, r3: {}, st2: {}", r2, r3, str2);
    
    let b1 = &mut str2;
    str_add(b1);
    // 不能再访问 r2 r3 同时读同时写不是允许的 只能有多个可读，或者一个可读写
    //println!("r2: {}, r3: {}, b1: {}", r2, r3, b1);
    println!("b1: {}", b1);
    // str2 就不能再访问了
    println!("st2: {}", str2);
}

fn ref_test1(s: &String) -> usize {
    s.len()
}

fn str_add(s: &mut String) {
    s.push_str("abcdef end.")
}

// 引用 & 指向变量的地址，并不拥有它，所以引用离开其值指向的作用域后也不会被
// 丢弃
// 借用 &mut 只能有一个可写