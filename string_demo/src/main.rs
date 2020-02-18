fn main() {
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);

    let s1 = String::from("abc");
    // 或者
    let s1 = "abcss".to_string();
    println!("{}", s1);

    let s2 = " !".to_string();
    // 使用的是引用，所以后面还能用。
    s0.push_str(&s2);
    println!("{}", s0);
    println!("{}", s2);
    // 只能是一个字符串
    s0.push('中');
    println!("{}", s0);

    let s1 = String::from("hello");
    let s2 = ", world".to_string();
    // 这里相当于把s1的所有权给了s3
    // + 相当于s1的一个方法，s2是参数
    let s3 = s1 + &s2;
    // 所以s1 不能用了
    //println!("s1 {}", s1);
    println!("s2 {}", s2);
    println!("s3 {}", s3);

    // format
    let sf = format!("{}-{}", s2, s3);
    println!("sf = {}", sf);

    // String 是utf-8编码的，中文占3个字节长度
    // string 不能使用索引 eg. s3[0]
    let s3 = String::from("中文");
    // 6
    println!("s3 len = {}", s3.len());

    // slice 是可以的
    let hello = "你好";
    // 一定要整个字符的长度 error
    //let h5 = &hello[0..2];
    let h5 = &hello[0..3];
    println!("h5 {}", h5);

    // 遍历
    // chars
    for c in s3.chars() {
        println!("char {}", c);
    }

    // bytes
    for b in s3.bytes() {
        println!("b {}", b);
    }
}
